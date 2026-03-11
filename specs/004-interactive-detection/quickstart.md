# 快速开始指南

**Feature**: 004-interactive-detection
**Version**: 1.0
**Last Updated**: 2026-03-11

## 概述

本指南帮助用户快速上手交互式输入设备检测与配置功能。

---

## 前置要求

### 系统要求

- **操作系统**: Linux（Ubuntu 20.04+, Debian 11+, Linux Mint 20+）
- **权限**: root 或 sudo（访问 `/dev/input/event*`）
- **设备**: 至少一个输入设备（键盘或鼠标）

### 安装 Aegis Input

如果尚未安装，请使用一键安装脚本：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Yu-Xiao-Sheng/aegis-input/main/install/remote/install.sh | bash
```

---

## 基本使用

### 场景 1: 检测并禁用内置触摸板

**目标**: 在使用外置鼠标时禁用笔记本触摸板。

#### 步骤 1: 运行检测命令

```bash
sudo aegis-input detect
```

#### 步骤 2: 扫描设备

程序显示所有检测到的设备：

```
正在扫描输入设备...

检测到 3 个输入设备:

  [1] AT Translated Set 2 keyboard
      路径: /dev/input/event0
      类型: 键盘 | 总线: PS2

  [2] Logitech USB Receiver
      路径: /dev/input/event3
      类型: 鼠标 | 总线: USB

  [3] SynPS/2 Synaptics TouchPad
      路径: /dev/input/event5
      类型: 鼠标 | 总线: PS2
```

#### 步骤 3: 检测活跃设备

程序进入检测模式：

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

#### 步骤 4: 使用设备

1. 在外置鼠标上移动 → 程序显示 `正在使用: Logitech USB Receiver`
2. 在触摸板上滑动 → 程序显示 `正在使用: SynPS/2 Synaptics TouchPad`
3. 按 Ctrl+C 结束检测

#### 步骤 5: 选择要禁用的设备

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  检测完成
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

检测期间活跃的设备:

  ✓ [2] Logitech USB Receiver
  ✓ [3] SynPS/2 Synaptics TouchPad

未活跃的设备:

    [1] AT Translated Set 2 keyboard

要禁用哪些设备？（可多选）
  输入编号，如: 1,3
  输入 'all' 禁用所有活跃设备
  输入 'none' 跳过
> 3
```

#### 步骤 6: 确认保存

```
已选择禁用:
  - SynPS/2 Synaptics TouchPad

是否保存此配置？ (y/n): y
```

#### 步骤 7: 完成

```
✓ 配置已保存到 /etc/aegis-input/config.toml
✓ 设备已禁用

要重新配置，请运行: sudo aegis-input detect
要重置配置，请运行: sudo aegis-input config --reset
```

---

### 场景 2: 仅配置模式（跳过检测）

如果你已经知道要禁用哪些设备，可以使用 `--config-only` 模式：

```bash
sudo aegis-input detect --config-only
```

程序直接显示设备列表，跳过检测步骤。

---

### 场景 3: JSON 输出（用于脚本）

如果需要机器可读的输出，使用 `--output json`：

```bash
sudo aegis-input detect --output json
```

输出示例：

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
    "/dev/input/event3"
  ]
}
```

---

## 配置管理

### 查看当前配置

```bash
sudo cat /etc/aegis-input/config.toml
```

示例输出：

```toml
version = "1.0"

[devices.keyboard]
enabled = true
disable_internal = false

[devices.mouse]
enabled = true
disable_internal = false

[devices.specific.disabled]
disabled = [
    { path = "/dev/input/event5" },
]
```

### 重置配置

删除所有自定义配置，恢复默认状态：

```bash
sudo aegis-input config --reset
```

**警告**: 这将删除所有设备禁用配置。

---

## 故障排除

### 问题 1: 权限不足

**错误信息**:
```
错误: 权限不足

此命令需要 root 权限来访问输入设备。
请使用 sudo 运行: sudo aegis-input detect
```

**解决方法**:
```bash
sudo aegis-input detect
```

---

### 问题 2: 未检测到输入设备

**错误信息**:
```
错误: 未检测到输入设备
```

**解决方法**:

1. 检查设备是否连接：
   ```bash
   ls -l /dev/input/event*
   ```

2. 检查设备驱动：
   ```bash
   dmesg | grep -i input
   ```

3. 测试设备：
   ```bash
   sudo evtest /dev/input/event0
   ```

---

### 问题 3: 设备无法禁用

**错误信息**:
```
警告: 设备 "AT Translated Set 2 keyboard" 不支持禁用
```

**原因**: 某些内置设备（如 PS/2 键盘）无法禁用。

**解决方法**: 选择其他设备，或使用 `disable_internal = false`。

---

### 问题 4: 配置文件损坏

**错误信息**:
```
错误: 配置文件格式错误
```

**解决方法**:

1. 备份当前配置：
   ```bash
   sudo cp /etc/aegis-input/config.toml /tmp/config.toml.backup
   ```

2. 重置配置：
   ```bash
   sudo aegis-input config --reset
   ```

3. 重新配置：
   ```bash
   sudo aegis-input detect
   ```

---

## 高级用法

### 自定义超时时间

默认超时 5 分钟（300 秒）。可以自定义：

```bash
sudo aegis-input detect --timeout 600  # 10 分钟
```

### 仅列出设备

查看所有检测到的设备，不进入检测模式：

```bash
sudo aegis-input detect --config-only
```

### 组合使用

组合多个选项：

```bash
sudo aegis-input detect --timeout 120 --output plain --config-only
```

---

## 验证配置

### 检查设备状态

```bash
sudo aegis-input status
```

输出示例：

```
Aegis Input 状态:

服务状态: active (running)
启用状态: enabled

当前配置:
  键盘: 启用
  鼠标: 启用

禁用的设备:
  - /dev/input/event5 (SynPS/2 Synaptics TouchPad)
```

### 测试设备

测试特定设备是否工作：

```bash
sudo evtest /dev/input/event5
```

在设备上输入，应该看到事件输出。

---

## 卸载

### 完全卸载

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Yu-Xiao-Sheng/aegis-input/main/install/remote/uninstall.sh | sudo bash
```

这将删除：
- 二进制文件
- systemd 服务
- 配置文件
- 数据目录

---

## 常见问题

### Q: 配置会持久化吗？

**A**: 是的。配置保存在 `/etc/aegis-input/config.toml`，系统重启后自动加载。

### Q: 如何临时禁用设备？

**A**: 运行检测模式，选择设备，保存配置时选择 `n`（不保存）。设备仅在当前会话禁用。

### Q: 设备路径会变化吗？

**A**: 可能会。配置支持按名称匹配作为后备。如果路径变化，程序会尝试按名称匹配设备。

### Q: 可以禁用多个设备吗？

**A**: 可以。输入时用逗号分隔，如 `1,3,5`。

### Q: 如何恢复默认配置？

**A**: 运行 `sudo aegis-input config --reset`。

---

## 下一步

- 查看 [完整文档](https://github.com/Yu-Xiao-Sheng/aegis-input)
- 报告问题: [GitHub Issues](https://github.com/Yu-Xiao-Sheng/aegis-input/issues)
- 贡献代码: [GitHub Pull Requests](https://github.com/Yu-Xiao-Sheng/aegis-input/pulls)

---

## 总结

**核心命令**:
- `sudo aegis-input detect` - 检测并配置设备
- `sudo aegis-input detect --config-only` - 仅配置模式
- `sudo aegis-input config --reset` - 重置配置
- `sudo aegis-input status` - 查看状态

**典型流程**:
1. 连接外置设备
2. 运行 `sudo aegis-input detect`
3. 在所有设备上输入
4. 按 Ctrl+C 结束检测
5. 选择要禁用的设备
6. 保存配置

**故障排除**:
- 权限问题 → 使用 sudo
- 无设备 → 检查 `/dev/input/event*`
- 配置损坏 → 运行 `config --reset`
