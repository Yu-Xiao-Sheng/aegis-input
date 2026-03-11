# Aegis Input 安装指南

本文档提供详细的安装说明和故障排除指南。

## 目录

- [快速安装](#快速安装)
- [系统要求](#系统要求)
- [安装方式](#安装方式)
- [配置](#配置)
- [升级](#升级)
- [卸载](#卸载)
- [故障排除](#故障排除)

---

## 快速安装

### 一键安装

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Yu-Xiao-Sheng/aegis-input/main/install/remote/install.sh | bash
```

此命令会：
1. 检测您的系统架构
2. 从 GitHub Releases 下载对应的二进制文件
3. 验证下载文件的完整性（SHA256）
4. 安装到 `/usr/local/bin`
5. 创建并启动 systemd 服务

**预计时间**: 60 秒（取决于网络速度）

---

## 系统要求

### 支持的操作系统

- **主要支持**:
  - Ubuntu 20.04+
  - Debian 11+
  - Linux Mint 20+

- **可能支持**（其他 systemd 发行版）:
  - Arch Linux
  - Fedora
  - openSUSE

### 支持的架构

- `x86_64` / `amd64` (Intel/AMD 64位)
- `aarch64` / `arm64` (ARM 64位)

### 依赖项

- systemd（用于服务管理）
- curl（用于下载）
- sha256sum（用于校验和验证）
- sudo 权限（系统级安装）

---

## 安装方式

### 方式 1: 一键安装（推荐）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Yu-Xiao-Sheng/aegis-input/main/install/remote/install.sh | bash
```

### 方式 2: 先下载后检查

如果您想先检查脚本内容：

```bash
# 下载安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Yu-Xiao-Sheng/aegis-input/main/install/remote/install.sh -o install.sh

# 查看脚本内容
less install.sh

# 确认后执行
sudo bash install.sh
```

### 方式 3: 指定版本

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Yu-Xiao-Sheng/aegis-input/main/install/remote/install.sh | bash -s -- --version 0.3.0
```

### 方式 4: 从 GitHub Releases 手动下载

1. 访问 [Releases 页面](https://github.com/Yu-Xiao-Sheng/aegis-input/releases)
2. 下载对应架构的 `.tar.gz` 文件
3. 下载 `SHA256SUMS.txt` 文件
4. 解压并安装：

```bash
# 解压
tar xzf aegis-input-x86_64-unknown-linux-musl.tar.gz

# 验证校验和
sha256sum -c SHA256SUMS.txt

# 安装
sudo mv aegis-input /usr/local/bin/
sudo chmod +x /usr/local/bin/aegis-input
```

---

## 配置

### 配置文件位置

- **配置文件**: `/etc/aegis-input/config.toml`
- **状态文件**: `/var/lib/aegis-input/status.toml`
- **安装信息**: `/var/lib/aegis-input/install.toml`

### 基本配置

编辑配置文件：

```bash
sudo nano /etc/aegis-input/config.toml
```

示例配置：

```toml
[devices.keyboard]
enabled = true
disable_internal = true

[devices.mouse]
enabled = true
disable_internal = true

[detection]
poll_interval_ms = 1000

[logging]
level = "info"
```

修改配置后重启服务：

```bash
sudo systemctl restart aegis-input
```

---

## 升级

### 检查更新

```bash
aegis-input --check
```

### 升级到最新版本

重新运行一键安装脚本即可：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Yu-Xiao-Sheng/aegis-input/main/install/remote/install.sh | bash
```

安装脚本会自动检测当前版本并升级到最新版本。

### 查看当前版本

```bash
aegis-input --version
```

---

## 卸载

### 一键卸载（推荐）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Yu-Xiao-Sheng/aegis-input/main/install/remote/uninstall.sh | sudo bash
```

此命令会：
1. 停止 aegis-input 服务
2. 禁用服务
3. 删除二进制文件
4. 删除 systemd 服务文件
5. 删除配置和数据目录
6. 删除系统用户和组（如果存在）
7. 重载 systemd

### 手动卸载

```bash
# 1. 停止服务
sudo systemctl stop aegis-input

# 2. 禁用服务
sudo systemctl disable aegis-input

# 3. 删除二进制文件
sudo rm /usr/local/bin/aegis-input

# 4. 删除服务文件
sudo rm /etc/systemd/system/aegis-input.service

# 5. 删除配置和数据
sudo rm -rf /etc/aegis-input
sudo rm -rf /var/lib/aegis-input

# 6. 删除系统用户和组
sudo userdel aegis-input
sudo groupdel aegis-input

# 7. 重载 systemd
sudo systemctl daemon-reload
```

---

## 故障排除

### 安装失败

#### 问题：权限不足

**错误信息**: `需要 sudo 权限来安装到 /usr/local/bin`

**解决**:
```bash
# 使用 sudo 运行安装脚本
sudo bash install.sh
```

#### 问题：不支持的平台

**错误信息**: `不支持的操作系统: macOS`

**解决**: 当前仅支持 Linux。macOS 和 Windows 支持正在开发中。

#### 问题：不支持 32 位系统

**错误信息**: `不支持的 32 位 x86 架构`

**解决**: Aegis Input 仅支持 64 位系统。

#### 问题：下载失败

**错误信息**: `下载失败，已重试 5 次`

**解决**:
1. 检查网络连接
2. 检查防火墙设置
3. 尝试手动下载安装

### 服务运行问题

#### 问题：服务无法启动

**检查步骤**:
```bash
# 1. 查看服务状态
sudo systemctl status aegis-input

# 2. 查看日志
sudo journalctl -u aegis-input -n 50

# 3. 检查配置文件
sudo aegis-input validate
```

#### 问题：功能未生效

**检查步骤**:
```bash
# 1. 确认服务正在运行
sudo systemctl is-active aegis-input

# 2. 检查外置设备是否被检测
sudo aegis-input status

# 3. 启用功能
sudo aegis-input enable
```

### 版本问题

#### 问题：版本检测失败

**错误信息**: `无法获取最新版本信息`

**解决**:
- GitHub API 可能暂时不可用
- 稍后重试
- 或手动访问 [Releases 页面](https://github.com/Yu-Xiao-Sheng/aegis-input/releases/latest)

#### 问题：校验和验证失败

**错误信息**: `SHA256 校验和验证失败！`

**解决**:
1. 删除已下载的文件
2. 重新运行安装脚本
3. 如果问题持续，请手动下载并验证

---

## 获取帮助

如果遇到问题：

1. 查看 [故障排除](#故障排除) 部分
2. 搜索 [Issues](https://github.com/Yu-Xiao-Sheng/aegis-input/issues)
3. 创建新 Issue 并包含：
   - 系统信息：`uname -a`
   - 服务状态：`systemctl status aegis-input`
   - 相关日志：`journalctl -u aegis-input -n 50`

---

## 相关文档

- [实现方案](docs/implementation.md)
- [发布指南](docs/RELEASE.md)
- [性能预算](docs/performance-budget.md)
