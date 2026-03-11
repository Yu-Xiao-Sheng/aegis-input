# CLI 接口契约

**Feature**: 004-interactive-detection
**Created**: 2026-03-11
**Version**: 1.0

## 概述

本文档定义 `aegis-input detect` 命令的 CLI 接口契约，包括参数、行为、输出格式和错误处理。

---

## 命令语法

```bash
aegis-input detect [OPTIONS]
```

### 参数

| 参数 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `--timeout <SECONDS>` | `u64` | 否 | `300` (5分钟) | 检测超时时间（秒） |
| `--output <FORMAT>` | `string` | 否 | `auto` | 输出格式：`auto`/`json`/`plain` |
| `--config-only` | `bool` | 否 | `false` | 仅配置模式（跳过检测，直接选择设备） |
| `-h, --help` | - | 否 | - | 显示帮助信息 |

---

## 行为规范

### 1. 正常流程

#### 步骤 1: 初始化

**前置条件**：
- 以 root 权限运行
- 至少有一个输入设备连接

**行为**：
1. 扫描 `/dev/input/event*` 设备
2. 过滤出键盘和鼠标设备
3. 显示设备列表

**输出**：
```
正在扫描输入设备...

检测到 3 个输入设备：

  [1] AT Translated Set 2 keyboard
      路径: /dev/input/event0
      类型: 键盘 | 总线: PS2

  [2] Logitech USB Receiver
      路径: /dev/input/event3
      类型: 键盘/鼠标 | 总线: USB

  [3] SynPS/2 Synaptics TouchPad
      路径: /dev/input/event5
      类型: 鼠标 | 总线: PS2
```

#### 步骤 2: 检测模式

**提示信息**：
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  交互式设备检测模式
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

请在所有连接的输入设备上输入（打字或移动鼠标）。
程序将实时显示当前正在使用的设备。

检测超时: 300 秒
按 Ctrl+C 结束检测

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**实时更新**（单行刷新）：
```
正在使用: Logitech USB Receiver
```

**空闲状态**：
```
等待输入... (已检测 3 个设备，活跃 2 个)
```

#### 步骤 3: 设备选择

**触发条件**：
- 用户按 Ctrl+C
- 超时（`--timeout` 秒）

**输出**：
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  检测完成
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

检测期间活跃的设备:

  ✓ [1] Logitech USB Receiver
  ✓ [2] SynPS/2 Synaptics TouchPad

未活跃的设备:

    [3] AT Translated Set 2 keyboard
```

**交互提示**：
```
要禁用哪些设备？（可多选）
  输入编号，如: 1,3
  输入 'all' 禁用所有活跃设备
  输入 'none' 跳过
>
```

#### 步骤 4: 确认保存

**用户输入后**：
```
已选择禁用:
  - Logitech USB Receiver
  - SynPS/2 Synaptics TouchPad

是否保存此配置？ (y/n):
```

**用户确认后**：
```
✓ 配置已保存到 /etc/aegis-input/config.toml
✓ 设备已禁用

要重新配置，请运行: aegis-input detect
要重置配置，请运行: aegis-input config --reset
```

---

### 2. 输出格式

#### `--output auto`（默认）

自动检测终端能力：
- 如果终端支持 ANSI 转义序列 → 使用彩色输出
- 否则 → 使用纯文本

#### `--output plain`

纯文本输出，无 ANSI 转义序列：
```
检测完成
正在使用: Logitech USB Receiver
```

#### `--output json`

机器可读的 JSON 格式：
```json
{
  "session_id": "detect-20260311-150000",
  "start_time": "2026-03-11T15:00:00Z",
  "end_time": "2026-03-11T15:00:30Z",
  "all_devices": [
    {
      "name": "AT Translated Set 2 keyboard",
      "path": "/dev/input/event0",
      "device_type": "keyboard",
      "bus_type": "ps2"
    }
  ],
  "active_devices": [
    "/dev/input/event3",
    "/dev/input/event5"
  ],
  "selected_devices": [
    "/dev/input/event3"
  ]
}
```

---

### 3. 错误处理

#### 错误代码

| 代码 | 名称 | 描述 |
|------|------|------|
| `1` | `PERMISSION_DENIED` | 权限不足，需要 root |
| `2` | `NO_DEVICES_FOUND` | 未检测到输入设备 |
| `3` | `DEVICE_ACCESS_FAILED` | 无法访问设备文件 |
| `4` | `CONFIG_WRITE_FAILED` | 配置文件写入失败 |
| `5` | `INTERRUPTED` | 用户中断（Ctrl+C） |

#### 错误消息格式

```bash
$ aegis-input detect
错误: 权限不足

此命令需要 root 权限来访问输入设备。
请使用 sudo 运行: sudo aegis-input detect
```

```bash
$ aegis-input detect
错误: 未检测到输入设备

请确认:
  - 输入设备已连接
  - 设备驱动正常工作
  - 您有权限访问 /dev/input/event*
```

---

### 4. 特殊模式

#### `--config-only`（仅配置模式）

跳过检测，直接显示当前所有设备并选择：

```
当前检测到的设备:

  [1] AT Translated Set 2 keyboard
  [2] Logitech USB Receiver
  [3] SynPS/2 Synaptics TouchPad

要禁用哪些设备？（可多选）
>
```

**使用场景**：
- 用户已知要禁用哪些设备
- 快速重新配置

---

## 集成测试要求

### 测试场景 1: 基本检测流程

**输入**：
```bash
sudo aegis-input detect
# 在两个键盘上输入
# 按 Ctrl+C
# 输入 "1"
# 输入 "y"
```

**预期输出**：
- 检测到 2 个活跃设备
- 配置已保存
- 设备已禁用

**验证**：
```bash
aegis-input status
# 显示设备已禁用
```

### 测试场景 2: 超时处理

**输入**：
```bash
sudo aegis-input detect --timeout 5
# 等待 5 秒无输入
```

**预期输出**：
- 显示超时消息
- 进入设备选择界面

### 测试场景 3: JSON 输出

**输入**：
```bash
sudo aegis-input detect --output json < /dev/null
# 模拟输入
# 按 Ctrl+C
```

**预期输出**：
- 有效的 JSON 对象
- 包含 `session_id`、`active_devices` 字段

---

## 性能要求

| 指标 | 要求 | 验证方法 |
|------|------|----------|
| 启动时间 | <1 秒 | 从命令调用到显示设备列表 |
| 响应延迟 | <100ms | 从输入到显示设备名称 |
| 内存占用 | <50MB | 检测模式下的 RSS |
| CPU 占用 | <5% | 响应输入时的峰值 CPU |

---

## 兼容性

### 终端兼容性

| 终端类型 | ANSI 支持 | 彩色支持 |
|----------|-----------|----------|
| Linux TTY | ✅ | ❌ |
| GNOME Terminal | ✅ | ✅ |
| Konsole | ✅ | ✅ |
| tmux | ✅ | ✅ |
| screen | ✅ | ❌ (需要配置) |

### 平台兼容性

| 平台 | 支持状态 | 备注 |
|------|----------|------|
| Ubuntu 20.04+ | ✅ | 完全支持 |
| Debian 11+ | ✅ | 完全支持 |
| Linux Mint 20+ | ✅ | 完全支持 |
| Arch Linux | ✅ | 完全支持 |
| Fedora | ⚠️ | 可能需要调整 |
| macOS | ❌ | 不支持（无 evdev） |
| Windows | ❌ | 不支持（无 evdev） |

---

## 安全考虑

### 权限要求

- **必需**: root 权限（读取 `/dev/input/event*`）
- **可选**: 写入配置文件权限

### 数据隐私

- **不记录**: 按键内容、鼠标移动轨迹
- **仅记录**: 设备路径、设备名称、活跃状态

### 输入验证

- 用户输入的设备编号必须有效
- 设备路径必须存在且可访问
- 配置文件路径必须合法

---

## 未来扩展

### 可能的新参数

- `--list-devices`: 仅列出设备，不进入检测模式
- `--test-device <PATH>`: 测试特定设备
- `--export-config`: 导出配置到文件

### 可能的新输出格式

- `--output yaml`: YAML 格式
- `--output toml`: TOML 格式

---

## 总结

**命令**: `aegis-input detect`

**核心功能**:
1. 扫描并显示输入设备
2. 实时检测活跃设备
3. 交互式选择要禁用的设备
4. 保存并应用配置

**关键约束**:
- 需要 root 权限
- 响应延迟 <100ms
- 支持最多 20 个设备

**测试覆盖**:
- 基本检测流程
- 超时处理
- JSON 输出
- 错误处理

**下一步**: 实现检测模块接口契约（detection.md）
