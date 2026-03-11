# Aegis Input

Aegis Input 是一个低开销系统服务，当检测到外置键盘或鼠标时自动禁用笔记本内置键盘与触摸板。初始目标平台为 Linux（已在 Linux Mint 上验证），并保留清晰的抽象层以支持 Windows 和 macOS。

## 快速安装

### 一键安装（推荐）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Yu-Xiao-Sheng/aegis-input/main/install/remote/install.sh | bash
```

**安装要求**：
- Linux 系统（Debian/Ubuntu/Mint 系列）
- systemd（用于服务管理）
- sudo 权限（系统级安装）
- 互联网连接

安装完成后，服务会自动启动。使用 `systemctl status aegis-input` 查看状态。

### 使用检测功能

安装后，可以使用交互式检测功能来识别和配置输入设备：

```bash
sudo aegis-input detect
```

这将：
1. 扫描所有连接的输入设备
2. 实时显示当前正在使用的设备
3. 让您选择要禁用的设备
4. 保存配置并立即生效

其他命令：
```bash
sudo aegis-input detect --config-only  # 仅配置模式
sudo aegis-input config --reset         # 重置配置
sudo aegis-input status                  # 查看状态
```

### 从源码构建

如果要从源码构建：

```bash
# 克隆仓库
git clone https://github.com/Yu-Xiao-Sheng/aegis-input.git
cd aegis-input

# 构建
cargo build --release

# 安装
sudo cp target/release/aegis-input /usr/local/bin/
```

**运行（本地验证）**
建议先使用临时配置路径，避免权限影响系统配置:
```bash
export AEGIS_INPUT_CONFIG=/tmp/aegis-input-config.toml
export AEGIS_INPUT_STATUS=/tmp/aegis-input-status.toml
```

启用功能并运行服务:
```bash
./target/debug/aegis-input enable
RUST_LOG=info ./target/debug/aegis-input run
```

若提示权限不足访问 `/dev/input`，可二选一:
1. 将当前用户加入 `input` 组并重新登录
```bash
sudo usermod -aG input $USER
```
2. 临时使用 sudo 运行（保留环境变量）
```bash
sudo -E RUST_LOG=info ./target/debug/aegis-input run
```

**CLI**
```bash
./target/debug/aegis-input enable
./target/debug/aegis-input disable
./target/debug/aegis-input status
```

**systemd（计划）**
将提供 systemd 服务与可选 CLI。服务模型见 `docs/implementation.md`。

### 卸载

如需卸载 Aegis Input：

```bash
# 方式 1: 使用卸载脚本（推荐）
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Yu-Xiao-Sheng/aegis-input/main/install/remote/uninstall.sh | sudo bash

# 方式 2: 手动卸载
sudo systemctl stop aegis-input
sudo systemctl disable aegis-input
sudo rm /usr/local/bin/aegis-input
sudo rm /etc/systemd/system/aegis-input.service
sudo rm -rf /etc/aegis-input
sudo rm -rf /var/lib/aegis-input
sudo systemctl daemon-reload
```

**名称**
Aegis 意为“盾”，体现目标: 在使用外置外设时防止内置设备误触。
