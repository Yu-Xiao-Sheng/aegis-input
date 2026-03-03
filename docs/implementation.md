# 实现方案调研与设计

本文档归纳 Linux 下可行的实现方案，并给出 Rust 项目落地设计。目标是：当检测到外置键盘或鼠标时，自动禁用笔记本内置键盘与触摸板；外置设备断开后恢复内置设备。后续预留 Windows/macOS 扩展。

**设计原则（优雅 & 低开销）**
- 事件驱动：仅在设备变更时响应，避免轮询
- 依赖克制：优先使用标准线程 + 阻塞监听，避免复杂运行时
- 最小特权：仅访问必要的 `/dev/input` 设备节点
- 故障自愈：设备重连/休眠后自动恢复抓取

**范围与目标**
- 目标平台：Linux（Linux Mint 优先），后续扩展 Windows/macOS
- 运行形态：系统服务，可选用户 CLI
- 核心行为：外置键盘/鼠标存在时禁用内置键盘/触摸板；外置设备移除时恢复

**非目标**
- 不处理屏幕合盖或电源管理策略
- 不处理特定应用级的输入屏蔽

## Linux 方案调研

**设备检测**
- 通过 udev 事件监听输入设备的 add/remove
- 依据 udev 属性判断设备类型与来源
- 常用属性
- `ID_INPUT_KEYBOARD=1`
- `ID_INPUT_MOUSE=1`
- `ID_INPUT_TOUCHPAD=1`
- `ID_BUS=usb` 或 `ID_BUS=bluetooth` 通常为外置
- `ID_PATH` 或 `ID_PATH_TAG` 常用于区分内置与外置

**禁用内置设备的可选方案**
1. `xinput` 禁用
- 只适用于 X11，会话内有效
- Wayland 下不可用或受限

2. 桌面环境设置
- 例如 GNOME 的触摸板开关
- 对键盘无统一开关，且依赖特定 DE

3. 内核输入层抓取（evdev `EVIOCGRAB`）
- 直接对 `/dev/input/event*` 设备进行独占抓取
- 与 X11/Wayland 无关，兼容性最好
- 需要足够权限读取设备节点
- 抓取期间内置设备事件不会传给其他客户端

**结论**
- 推荐采用 udev 监听 + evdev 独占抓取的方案
- 这是跨 X11/Wayland 的最稳妥方案

## 设计方案

**整体架构**
- `core`：状态机与策略逻辑（纯逻辑、可测试）
- `platform/linux`：设备扫描、监听与禁用实现
- `config`：规则与设备识别配置
- `cli`：调试与手动控制（可选）

**设备识别策略**
- 默认规则
- 外置：`ID_INPUT_KEYBOARD=1` 或 `ID_INPUT_MOUSE=1` 且 `ID_BUS` 为 `usb`/`bluetooth`
- 内置：`ID_INPUT_KEYBOARD=1` 且 `ID_BUS` 为 `i8042`/`serio`/`platform`/`i2c` 等
- 通过配置文件允许用户手动标记内置设备（按设备路径、名称或 `ID_PATH_TAG`）

**状态机**
- 初始扫描所有输入设备并分类
- 维护 `external_present` 标志
- 当 `external_present=true` 时：抓取内置键盘与触摸板
- 当 `external_present=false` 时：释放抓取
- 外置设备增删时更新状态并触发动作

**低开销事件循环**
- 采用单线程阻塞监听 udev 事件（或 1 线程 + mpsc）即可
- 不必引入 async 运行时；仅在设备变更时执行少量工作

**禁用/恢复实现**
- 使用 `evdev` crate 打开内置设备并执行独占抓取
- 维持设备文件句柄，直到解除禁用
- 设备移除或系统休眠后自动重建句柄与抓取

**权限与部署**
- 需要读写 `/dev/input/event*`
- 建议方案
- 系统级 systemd service
- 使用 `SupplementaryGroups=input` 或 udev 规则将相关设备节点授权给专用用户组
- 记录日志到 journald

**配置**
- `config.toml` 示例
- 指定外置设备识别规则（bus、name、path）
- 指定内置设备白名单（强制禁用列表）
- 日志级别与调试开关
- 默认路径: `~/.config/aegis-input/config.toml`（可通过 `AEGIS_INPUT_CONFIG` 覆盖）

## Rust 技术选型

**推荐 crate**
- `udev`：设备枚举与事件监听
- `evdev`：输入设备访问与抓取
- `serde` + `toml`：配置读取
- `tracing` + `tracing-subscriber`：日志
- 事件循环建议使用标准线程 + channel（避免 async 运行时）

## 服务化方案

**systemd**
- 提供 `aegis-input.service`
- 自动随系统启动
- 支持 `systemctl status` 查看状态

**CLI（可选）**
- `aegis-input status`
- `aegis-input enable/disable`
- 状态持久化到本地配置文件，重启后保持开关

## 兼容性与边界情况

- 外置设备通过 USB 与蓝牙两种通道都要支持
- 触摸板可能是 I2C 设备，识别规则要可配置
- 休眠/唤醒后设备节点可能变化，需要重建抓取
- 外置设备短暂抖动（蓝牙）要做去抖处理

## Windows/macOS 预留

- 抽象 `PlatformBackend` trait
- Linux 实现为 `UdevBackend + EvdevDisabler`
- Windows 计划通过 HID/Raw Input + 设备过滤驱动或钩子
- macOS 计划通过 IOKit + 设备属性监听
- 公共逻辑只依赖抽象层

## 下一步实现建议

1. 搭建最小可运行的 Linux 版本
- 设备扫描
- 外置识别
- 内置抓取与释放

2. 添加配置文件与日志
3. 添加 systemd 服务文件与示例
4. 进行 Linux Mint 实机测试
