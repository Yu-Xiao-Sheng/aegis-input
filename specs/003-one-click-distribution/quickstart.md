# 快速开始: 一键安装与自动化分发

**功能**: 003-one-click-distribution
**更新日期**: 2026-03-10

本文档面向开发者和用户，提供快速上手指南。

---

## 用户指南：一键安装

### 前置要求

- Linux 系统（Debian/Ubuntu/Mint 系列）
- systemd（用于服务管理）
- sudo 权限（系统级安装）
- 互联网连接（下载二进制文件）

### 安装命令

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/yourusername/aegis-input/main/install/remote/install.sh | bash
```

### 验证安装

```bash
# 检查版本
aegis-input --version

# 检查服务状态
systemctl status aegis-input

# 查看日志
journalctl -u aegis-input -f
```

### 升级

```bash
# 重新运行安装命令即可升级
curl --proto '=https' --tlsv1.2 -sSf https://.../install.sh | bash
```

### 卸载

```bash
# 停止服务
sudo systemctl stop aegis-input
sudo systemctl disable aegis-input

# 删除文件
sudo rm /usr/local/bin/aegis-input
sudo rm /etc/systemd/system/aegis-input.service
sudo rm -rf /etc/aegis-input /var/lib/aegis-input

# 重载 systemd
sudo systemctl daemon-reload
```

---

## 开发者指南：测试发布流程

### 本地测试构建

#### 1. 构建静态链接二进制

```bash
# 安装 musl target
rustup target add x86_64-unknown-linux-musl

# 构建
cargo build --release --target x86_64-unknown-linux-musl

# 验证静态链接
ldd target/x86_64-unknown-linux-musl/release/aegis-input
# 应输出: not a dynamic executable
```

#### 2. 在 Alpine 容器中测试

```bash
# 运行 Alpine 容器
docker run --rm -v $(pwd):/app alpine:latest /app/target/x86_64-unknown-linux-musl/release/aegis-input --version
```

#### 3. 打包测试

```bash
# 创建归档
tar czf aegis-input-x86_64-unknown-linux-musl.tar.gz \
  -C target/x86_64-unknown-linux-musl/release aegis-input

# 生成校验和
sha256sum aegis-input-x86_64-unknown-linux-musl.tar.gz > SHA256SUMS.txt
```

### 本地测试安装脚本

#### 1. 启动本地文件服务器

```bash
# 使用 Python 启动简单 HTTP 服务器
cd /path/to/aegis-input
python3 -m http.server 8000
```

#### 2. 修改安装脚本中的 URL

```bash
# 临时修改 DOWNLOAD_URL
export DOWNLOAD_URL="http://localhost:8000/aegis-input-x86_64-unknown-linux-musl.tar.gz"
export CHECKSUM_URL="http://localhost:8000/SHA256SUMS.txt"
```

#### 3. 测试安装

```bash
# 在虚拟机或容器中测试
bash install/remote/install.sh --version 0.3.0 --target x86_64-unknown-linux-musl
```

---

## 发布新版本

### 方法 1: 使用辅助脚本（推荐）

```bash
# 一键发布
./scripts/tag-release.sh 0.3.0
```

脚本会自动：
1. 创建 git tag `v0.3.0`
2. 推送 tag 到远程
3. 触发 GitHub Actions 构建
4. 创建 Release 并上传二进制文件

### 方法 2: 手动发布

```bash
# 1. 更新版本号
vim Cargo.toml  # 修改 version = "0.3.0"

# 2. 提交变更
git add Cargo.toml
git commit -m "bump: version to 0.3.0"

# 3. 创建 tag
git tag -a v0.3.0 -m "Release v0.3.0: 一键安装功能

- 新增一键安装脚本
- GitHub Actions 自动构建
- 多平台二进制支持"

# 4. 推送
git push origin main
git push origin v0.3.0
```

### 验证发布

```bash
# 使用 GitHub CLI
gh release view v0.3.0

# 或访问网页
open https://github.com/yourusername/aegis-input/releases/tag/v0.3.0
```

---

## GitHub Actions 工作流

### 触发构建

推送 tag 后，GitHub Actions 自动触发：

```bash
git push origin v0.3.0
```

### 监控构建

```bash
# 使用 GitHub CLI
gh run list
gh run view

# 或访问网页
open https://github.com/yourusername/aegis-input/actions
```

### 构建产物

构建完成后，Release 包含：

| 文件 | 描述 |
|------|------|
| `aegis-input-x86_64-unknown-linux-musl.tar.gz` | x86_64 架构二进制 |
| `aegis-input-aarch64-unknown-linux-musl.tar.gz` | aarch64 架构二进制 |
| `SHA256SUMS.txt` | 所有文件的校验和 |

---

## 故障排除

### 安装失败

**问题**: 下载失败
```
[ERROR] 无法连接到 GitHub
```

**解决**:
```bash
# 检查网络连接
ping github.com

# 检查 DNS
nslookup github.com

# 手动下载
wget https://github.com/.../releases/download/v0.3.0/aegis-input-x86_64-unknown-linux-musl.tar.gz
```

**问题**: 校验和验证失败
```
[ERROR] 校验和验证失败！
```

**解决**:
```bash
# 重新下载
curl ... | bash

# 或手动验证
sha256sum -c SHA256SUMS.txt
```

### 构建失败

**问题**: 交叉编译失败
```
error: linking with `cc` failed: exit code: 1
```

**解决**:
```bash
# 检查 cross 工具
cargo install cross

# 使用 cross 编译
cross build --release --target aarch64-unknown-linux-musl
```

**问题**: 静态链接失败
```
error: cannot link statically with udev
```

**解决**:
- udev crate 通常无法完全静态链接
- 在容器中运行或接受动态链接部分组件

### 版本检测失败

**问题**: GitHub API 速率限制
```
[ERROR] GitHub API 速率限制已用尽
```

**解决**:
```bash
# 等待限制重置（通常 1 小时）
# 或手动检查更新
open https://github.com/yourusername/aegis-input/releases/latest
```

---

## 常见问题

### Q: 为什么不使用 apt 安装？

A: 当前优先实现一键安装，提供最快的安装体验。APT 仓库支持在规划中（见 spec.md 未来扩展）。

### Q: 安装脚本安全吗？

A: 脚本使用 HTTPS + TLS 1.2+，验证 SHA256 校验和。你也可以先下载后检查：

```bash
curl -O https://.../install.sh
less install.sh  # 检查脚本
sudo bash install.sh
```

### Q: 支持哪些 Linux 发行版？

A: 当前支持 Debian/Ubuntu/Mint 系列（systemd）。其他发行版可能需要调整 systemd 配置。

### Q: 支持 macOS/Windows 吗？

A: 当前仅支持 Linux，macOS/Windows 支持在规划中。架构已预留，未来可以添加。

### Q: 二进制文件为什么这么大？

A: 静态链接会将所有依赖打包。可以优化：

```toml
[profile.release]
opt-level = "z"     # 优化大小
lto = true         # 链接时优化
strip = true       # 去除符号表
```

### Q: 如何回滚到旧版本？

A: 手动下载旧版本二进制：

```bash
# 下载旧版本
wget https://github.com/.../releases/download/v0.2.0/aegis-input-x86_64-unknown-linux-musl.tar.gz

# 安装
tar xzf aegis-input-x86_64-unknown-linux-musl.tar.gz
sudo cp aegis-input /usr/local/bin/
sudo systemctl restart aegis-input
```

---

## 开发环境设置

### 安装依赖

```bash
# Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 交叉编译工具
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

# Cross（可选，用于 aarch64）
cargo install cross
```

### 运行测试

```bash
# 单元测试
cargo test

# 集成测试
sudo ./tests/installation/test_install_flow.sh
```

### 代码风格

```bash
# 格式化
cargo fmt

# Lint
cargo clippy -- -D warnings
```

---

## 贡献指南

### 开发流程

1. Fork 仓库
2. 创建功能分支: `git checkout -b 003-your-feature`
3. 提交变更: `git commit -am "Add feature"`
4. 推送分支: `git push origin 003-your-feature`
5. 创建 Pull Request

### 提交信息规范

```
<type>: <description>

[optional body]

[optional footer]
```

**类型**:
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档变更
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具相关

**示例**:
```
feat: 添加一键安装脚本

实现 curl | bash 一键安装功能：
- 自动检测系统架构
- 从 GitHub Releases 下载二进制
- 验证 SHA256 校验和
- 自动配置 systemd 服务

Closes #123
```

---

## 相关资源

### 文档

- [功能规格](./spec.md) - 详细的功能需求
- [实现计划](./plan.md) - 技术实现方案
- [技术研究](./research.md) - 技术选型和研究
- [数据模型](./data-model.md) - 数据结构设计
- [接口契约](./contracts/) - 接口规范

### 外部资源

- [Rust 静态链接指南](https://doc.rust-lang.org/edition-guide/rust-2021/static-linking-with-musl.html)
- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [语义化版本](https://semver.org/lang/zh-CN/)
- [项目宪章](https://github.com/yourusername/aegis-input/blob/main/.specify/memory/constitution.md)

---

## 下一步

- [ ] 完成实现：运行 `/speckit.tasks` 生成任务列表
- [ ] 开始开发：按照 tasks.md 逐项实现
- [ ] 测试：在真实环境中测试安装流程
- [ ] 发布：创建首个 v0.3.0 Release
