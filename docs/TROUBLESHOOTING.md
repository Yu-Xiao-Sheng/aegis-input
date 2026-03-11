# Aegis Input 故障排除指南

本文档提供常见问题的诊断和解决方案。

## 目录

- [安装问题](#安装问题)
- [服务问题](#服务问题)
- [功能问题](#功能问题)
- [性能问题](#性能问题)
- [网络问题](#网络问题)
- [获取帮助](#获取帮助)

---

## 安装问题

### 错误：需要 sudo 权限

**症状**: `需要 sudo 权限来安装到 /usr/local/bin`

**原因**: 安装到系统目录需要管理员权限

**解决方案**:
```bash
# 使用 sudo 运行安装脚本
sudo bash install.sh

# 或指定用户目录（无需 sudo）
curl ... | bash -s -- --dir ~/.local/bin
```

---

### 错误：不支持的操作系统

**症状**: `不支持的操作系统: macOS` 或 `不支持的操作系统: Windows`

**原因**: 当前版本仅支持 Linux

**解决方案**:
- Linux 用户：确保使用的是支持的发行版
- macOS/Windows 用户：请关注项目获取跨平台支持

---

### 错误：不支持的架构

**症状**: `不支持的架构: armv7l`

**原因**: Aegis Input 当前仅支持 64 位架构

**解决方案**:
- 确保使用 64 位操作系统（x86_64 或 aarch64）
- 32 位系统不支持

---

### 错误：下载失败

**症状**: `下载失败，已重试 5 次`

**可能原因**:
1. 网络连接问题
2. GitHub 访问受限
3. 防火墙阻止

**解决方案**:
```bash
# 1. 检查网络连接
ping github.com

# 2. 检查 DNS 解析
nslookup github.com

# 3. 手动下载
# 访问 https://github.com/Yu-Xiao-Sheng/aegis-input/releases
# 下载对应架构的 .tar.gz 文件
```

---

### 错误：校验和验证失败

**症状**: `SHA256 校验和验证失败！`

**可能原因**:
1. 下载过程中文件损坏
2. 不完整的下载
3. 校验和文件不匹配

**解决方案**:
```bash
# 1. 清理临时文件
rm /tmp/aegis-install-*

# 2. 重新运行安装
curl ... | bash

# 3. 如果问题持续，手动下载验证
wget https://github.com/Yu-Xiao-Sheng/aegis-input/releases/download/v0.3.0/aegis-input-x86_64-unknown-linux-musl.tar.gz
wget https://github.com/Yu-Xiao-Sheng/aegis-input/releases/download/v0.3.0/SHA256SUMS.txt
sha256sum -c SHA256SUMS.txt
```

---

## 服务问题

### 问题：服务无法启动

**症状**: `systemctl status aegis-input` 显示 `failed`

**诊断步骤**:
```bash
# 1. 查看详细状态
sudo systemctl status aegis-input

# 2. 查看日志
sudo journalctl -u aegis-input -n 50 --no-pager

# 3. 检查二进制文件
which aegis-input
aegis-input --version
```

**常见原因和解决方案**:

#### 原因 1: 配置文件错误

```bash
# 检查配置
sudo aegis-input validate

# 重新生成配置
sudo aegis-input config init
```

#### 原因 2: 权限不足

```bash
# 检查二进制权限
ls -l /usr/local/bin/aegis-input

# 确保可执行
sudo chmod +x /usr/local/bin/aegis-input
```

#### 原因 3: 缺少依赖

```bash
# 检查 systemd
systemctl --version

# 检查 input 组
getent group input
```

---

### 问题：服务启动后立即停止

**症状**: 服务状态 `active (exited)` 或 `inactive`

**诊断**:
```bash
# 查看日志
sudo journalctl -u aegis-input -n 20 --no-pager
```

**可能原因**:
1. 配置文件中有错误配置
2. 依赖的设备不可访问
3. 权限问题（无法访问 `/dev/input`）

**解决方案**:
```bash
# 1. 检查配置
sudo aegis-input validate

# 2. 检查设备访问
ls /dev/input/event*

# 3. 检查用户组
groups

# 4. 添加到 input 组
sudo usermod -aG input $USER
# 然后重新登录
```

---

### 问题：服务反复重启

**症状**: `systemctl status` 显示 `restart: always`

**诊断**:
```bash
# 查看重启次数
sudo systemctl show aegis-input | grep Restart

# 查看日志
sudo journalctl -u aegis-input -n 100 --no-pager | tail -20
```

**可能原因**:
1. 崩溃（panic）
2. 配置错误导致立即退出
3. 依赖缺失

**解决方案**:
1. 查看崩溃日志
2. 验证配置文件
3. 运行测试：`RUST_BACKTRACE=1 aegis-input run`

---

## 功能问题

### 问题：插入外置设备后内置设备未禁用

**诊断**:
```bash
# 1. 确认服务运行
sudo systemctl is-active aegis-input

# 2. 检查状态
sudo aegis-input status

# 3. 查看日志
sudo journalctl -u aegis-input -f
```

**可能原因**:
1. 功能未启用
2. 设备检测失败
3. 设备类型不匹配

**解决方案**:
```bash
# 1. 启用功能
sudo aegis-input enable

# 2. 检查配置
sudo aegis-input config show

# 3. 检查设备检测
sudo aegis-input detect
```

---

### 问题：移除外置设备后内置设备未恢复

**诊断**:
```bash
# 查看日志中的恢复事件
sudo journalctl -u aegis-input | grep -i "恢复\|restore"
```

**解决方案**:
1. 手动恢复内置设备：`sudo aegis-input restore`
2. 检查配置中的恢复设置
3. 重启服务

---

## 性能问题

### 问题：CPU 使用率高

**症状**: `top` 或 `htop` 显示 `aegis-input` CPU 使用率高

**正常范围**: CPU 使用率应该 < 1%

**诊断**:
```bash
# 测量 CPU 使用
sudo perf top -p $(pidof aegis-input)

# 检查事件循环频率
sudo aegis-input status
```

**解决方案**:
- 检查配置中的轮询间隔
- 确保使用了事件驱动而非轮询
- 查看性能预算文档：`docs/performance-budget.md`

---

### 问题：内存占用高

**症状**: 内存使用超过预期

**正常范围**: 内存使用应 < 50MB

**诊断**:
```bash
# 查看内存使用
ps aux | grep aegis-input
pmap $(pidof aegis-input)
```

**解决方案**:
1. 检查内存泄漏（长期运行后增长）
2. 重启服务
3. 报告问题并提供 `heap` 快照

---

## 网络问题

### 问题：无法连接到 GitHub

**症状**: 安装时无法下载二进制文件

**诊断**:
```bash
# 测试连接
curl -v https://github.com
curl -v https://api.github.com
```

**解决方案**:
1. 检查网络连接
2. 配置代理（如需要）：
   ```bash
   export https_proxy=http://proxy.example.com:8080
   ```
3. 使用镜像下载（如果可用）

---

### 问题：GitHub API 速率限制

**症状**: `GitHub API 速率限制较低（剩余 X）`

**原因**: GitHub API 对未认证请求限制为 60 次/小时

**解决方案**:
1. 等待 1 小时后重试
2. 或使用手动下载
3. 或配置 GitHub token（高级）

---

## 获取帮助

### 收集诊断信息

在报告问题前，请收集以下信息：

```bash
# 系统信息
echo "=== 系统信息 ===" > diagnostics.txt
uname -a >> diagnostics.txt
cat /etc/os-release >> diagnostics.txt

# 服务状态
echo "=== 服务状态 ===" >> diagnostics.txt
sudo systemctl status aegis-input >> diagnostics.txt

# 版本信息
echo "=== 版本信息 ===" >> diagnostics.txt
aegis-input --version >> diagnostics.txt

# 最近日志
echo "=== 最近日志 ===" >> diagnostics.txt
sudo journalctl -u aegis-input -n 100 >> diagnostics.txt

# 查看诊断信息
cat diagnostics.txt
```

### 报告问题

1. 搜索 [现有 Issues](https://github.com/Yu-Xiao-Sheng/aegis-input/issues)
2. 创建新 Issue 并包含：
   - 清晰的标题和描述
   - 复现步骤
   - 诊断信息（见上）
   - 期望行为
   - 实际行为

### 社区支持

- [GitHub Discussions](https://github.com/Yu-Xiao-Sheng/aegis-input/discussions)
- [Issues](https://github.com/Yu-Xiao-Sheng/aegis-input/issues)

---

## 调试技巧

### 启用详细日志

```bash
# 临时启用调试日志
sudo systemctl edit aegis-input
# 添加: Environment=RUST_LOG=debug
sudo systemctl daemon-reload
sudo systemctl restart aegis-input

# 查看日志
sudo journalctl -u aegis-input -f
```

### 手动运行（无服务）

```bash
# 停止服务
sudo systemctl stop aegis-input

# 手动运行（前台）
sudo aegis-input run
```

### 验证配置

```bash
# 检查配置文件语法
sudo aegis-input validate

# 查看当前配置
sudo aegis-input config show
```

---

## 常见错误代码

| 错误代码 | 含义 | 解决方案 |
|---------|------|----------|
| 1 | 一般错误 | 查看日志获取详情 |
| 2 | 网络错误 | 检查网络连接 |
| 3 | 校验和错误 | 重新下载文件 |
| 4 | 权限错误 | 使用 sudo |
| 5 | 平台错误 | 检查系统要求 |
| 6 | 依赖错误 | 安装缺失依赖 |

---

## 性能监控

### 实时监控

```bash
# CPU 和内存使用
watch -n 1 'ps aux | grep aegis-input'

# 服务状态
watch -n 1 systemctl status aegis-input

# 日志流
sudo journalctl -u aegis-input -f
```

### 长期监控

```bash
# 查看资源使用历史
sudo systemd-cgtop

# 查看服务资源限制
systemctl show aegis-input | grep -i memory
systemctl show aegis-input | grep -i cpu
```

---

## 更多资源

- [安装指南](INSTALLATION.md)
- [实现方案](implementation.md)
- [性能预算](performance-budget.md)
- [发布指南](RELEASE.md)
