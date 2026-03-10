# Aegis Input 发布指南

本文档描述如何发布 Aegis Input 的新版本。

## 自动发布流程

Aegis Input 使用 GitHub Actions 自动化发布流程：

### 1. 创建版本标签

使用提供的脚本自动创建版本标签并触发构建：

```bash
./scripts/tag-release.sh 1.0.0
```

这会自动：
- 更新 `Cargo.toml` 中的版本号
- 创建 Git 标签
- 推送到远程仓库
- 触发 GitHub Actions 构建流程

### 2. GitHub Actions 构建流程

推送标签后，GitHub Actions 会自动：

1. **交叉编译多平台二进制文件**：
   - `x86_64-unknown-linux-musl` - Linux x86_64（静态链接）
   - `aarch64-unknown-linux-musl` - Linux ARM64（静态链接）
   - `armv7-unknown-linux-musleabihf` - Linux ARMv7（树莓派等）
   - `x86_64-apple-darwin` - macOS Intel
   - `aarch64-apple-darwin` - macOS Apple Silicon

2. **生成 SHA256 校验和**

3. **创建 GitHub Release**：
   - 上传所有构建产物
   - 自动生成 Release Notes
   - 包含安装说明

### 3. 下载和验证

用户可以从 GitHub Releases 页面下载对应平台的二进制文件：

```bash
# 下载 Linux 版本
wget https://github.com/yourusername/aegis-input/releases/download/v1.0.0/aegis-input-x86_64-unknown-linux-musl.tar.gz

# 解压
tar xzf aegis-input-x86_64-unknown-linux-musl.tar.gz

# 验证校验和
sha256sum -c SHA256SUMS.txt

# 安装
sudo mv aegis-input /usr/local/bin/
```

## 手动发布步骤

如果需要手动控制发布流程：

### 1. 更新版本号

编辑 `Cargo.toml`：

```toml
[package]
name = "aegis-input"
version = "1.0.0"  # 更新版本号
```

### 2. 提交更改

```bash
git add Cargo.toml
git commit -m "chore: bump version to 1.0.0"
```

### 3. 创建标签

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin main
git push origin v1.0.0
```

### 4. 等待构建完成

访问 GitHub Actions 页面查看构建状态。

### 5. 验证 Release

构建完成后，访问 GitHub Releases 页面验证：
- 所有平台的二进制文件都已上传
- SHA256SUMS.txt 文件存在
- Release Notes 正确生成

## 版本号规范

遵循 [语义化版本](https://semver.org/lang/zh-CN/) 规范：

- **主版本号（MAJOR）**：不兼容的 API 变更
- **次版本号（MINOR）**：向后兼容的功能新增
- **修订号（PATCH）**：向后兼容的问题修正

示例：
- `1.0.0` - 初始稳定版本
- `1.1.0` - 新增功能
- `1.1.1` - Bug 修复
- `2.0.0` - 重大变更

## 构建产物说明

### Linux 二进制文件

使用 musl 静态链接，具有以下优势：
- 不依赖系统的 glibc 版本
- 可以在旧版 Linux 发行版上运行
- 单文件部署，无需其他依赖

### macOS 二进制文件

提供两个版本：
- **Intel 版本**（`x86_64-apple-darwin`）：适用于 Intel Mac
- **Apple Silicon 版本**（`aarch64-apple-darwin`）：适用于 M1/M2/M3 Mac

## 本地构建测试

在发布前，可以本地测试构建：

```bash
# 安装 cross 工具
cargo install cross

# 构建 Linux x86_64 版本
cross build --release --target x86_64-unknown-linux-musl

# 构建 Linux ARM64 版本
cross build --release --target aarch64-unknown-linux-musl

# 测试二进制文件
./target/x86_64-unknown-linux-musl/release/aegis-input --version
```

## 故障排除

### 构建失败

1. 检查 GitHub Actions 日志
2. 确保所有依赖项在 `Cargo.toml` 中正确声明
3. 检查交叉编译配置是否正确

### Release 创建失败

1. 检查 `GITHUB_TOKEN` 权限
2. 确保 `.github/workflows/release.yml` 中的权限配置正确
3. 验证标签格式是否正确（`v*.*.*`）

### 二进制文件问题

1. 使用 `file` 命令验证二进制文件类型
2. 使用 `ldd` 检查动态链接依赖（Linux）
3. 使用 `otool -L` 检查依赖（macOS）

## 相关文档

- [GitHub Actions 工作流配置](/.github/workflows/release.yml)
- [Cross 交叉编译工具文档](https://github.com/cross-rs/cross)
- [Cargo 构建配置](/Cargo.toml)
