# Rust 静态链接快速入门

本文档提供了静态链接的快速入门指南。详细文档请参阅 [static-linking-guide.md](./static-linking-guide.md)。

## 快速开始

### 1. 安装依赖

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y musl-tools musl-dev gcc-aarch64-linux-gnu

# Arch Linux
sudo pacman -S musl

# Alpine Linux (自带 musl)
apk add build-base
```

### 2. 添加 Rust targets

```bash
# 添加 x86_64 musl target
rustup target add x86_64-unknown-linux-musl

# 添加 aarch64 musl target
rustup target add aarch64-unknown-linux-musl
```

### 3. 构建静态链接二进制

#### 使用项目脚本（推荐）

```bash
# 构建 x86_64 静态链接版本
./scripts/build-static.sh x86_64-unknown-linux-musl

# 构建 aarch64 静态链接版本
./scripts/build-static.sh aarch64-unknown-linux-musl
```

#### 手动构建

```bash
# 基础构建
cargo build --release --target x86_64-unknown-linux-musl

# 完全静态链接（包括 C 库）
RUSTFLAGS="-C target-feature=+crt-static" \
  cargo build --release --target x86_64-unknown-linux-musl
```

### 4. 验证静态链接

```bash
# 使用验证脚本
./scripts/verify-static.sh target/x86_64-unknown-linux-musl/release/aegis-input

# 手动验证
file target/x86_64-unknown-linux-musl/release/aegis-input
ldd target/x86_64-unknown-linux-musl/release/aegis-input
```

### 5. 优化二进制大小

```bash
# 使用优化脚本
./scripts/optimize-binary.sh target/x86_64-unknown-linux-musl/release/aegis-input

# 手动优化
strip --strip-all target/x86_64-unknown-linux-musl/release/aegis-input
upx --best --lzma target/x86_64-unknown-linux-musl/release/aegis-input
```

### 6. 在 Docker 中测试

```bash
# 在 Alpine 容器中测试
docker run --rm -v $(pwd):/app alpine:latest \
  /app/target/x86_64-unknown-linux-musl/release/aegis-input --version
```

## 配置文件

项目已配置以下文件用于静态链接：

### `.cargo/config.toml`
```toml
[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "link-arg=-static"]

[target.aarch64-unknown-linux-musl]
rustflags = ["-C", "link-arg=-static"]
linker = "aarch64-linux-gnu-gcc"
```

### `Cargo.toml`
```toml
[profile.release]
opt-level = "z"     # 优化大小
lto = true         # 链接时优化
codegen-units = 1  # 更好的优化
strip = true       # 自动 strip
panic = "abort"    # 减小大小
```

## 常用命令

### 构建
```bash
# 本地架构
cargo build --release --target x86_64-unknown-linux-musl

# 交叉编译
cargo build --release --target aarch64-unknown-linux-musl

# 使用 cross 工具（自动处理交叉编译环境）
cross build --release --target aarch64-unknown-linux-musl
```

### 测试
```bash
# 单元测试
cargo test --release --target x86_64-unknown-linux-musl

# 集成测试
cargo test --release --target x86_64-unknown-linux-musl --test '*'

# 在 Docker 中测试
docker run --rm -v $(pwd):/app alpine:latest /app/path/to/binary
```

### 验证
```bash
# 检查文件类型
file target/x86_64-unknown-linux-musl/release/binary

# 检查动态依赖
ldd target/x86_64-unknown-linux-musl/release/binary

# 检查符号表
file target/x86_64-unknown-linux-musl/release/binary | grep stripped

# 检查大小
ls -lh target/x86_64-unknown-linux-musl/release/binary
```

## 依赖处理

### 常见静态链接依赖

```toml
# OpenSSL
[dependencies]
openssl = { version = "0.10", features = ["vendored"] }

# SQLite
[dependencies]
rusqlite = { version = "0.32", features = ["bundled"] }

# zlib
[dependencies]
flate2 = { version = "1.0", default-features = false, features = ["zlib-ng"] }

# 推荐使用纯 Rust 替代品
[dependencies]
reqwest = { version = "0.12", features = ["rustls-tls"], default-features = false }
```

## 脚本说明

项目提供以下实用脚本：

### `scripts/build-static.sh`
自动构建静态链接二进制，支持：
- 自动安装 target
- 检查依赖
- 构建验证
- Alpine 兼容性测试

### `scripts/verify-static.sh`
验证静态链接配置：
- 文件类型检查
- 架构检查
- 静态链接验证
- 符号表检查
- 大小评估
- Alpine 兼容性测试

### `scripts/optimize-binary.sh`
优化二进制大小：
- 移除符号表（strip）
- UPX 压缩
- 功能验证
- 大小对比报告

## CI/CD

项目包含 GitHub Actions 工作流 `.github/workflows/build-static.yml`，自动：
- 为多个架构构建静态链接二进制
- 验证静态链接
- 在 Alpine 中测试
- 运行测试套件
- 优化二进制大小
- 创建 release 包

## 故障排除

### 编译错误

**错误**: `cannot find -lm`
```bash
# 安装 musl 开发文件
sudo apt-get install musl-dev
```

**错误**: `linker not found`
```bash
# 安装交叉编译工具链
sudo apt-get install gcc-aarch64-linux-gnu
```

**错误**: OpenSSL 编译失败
```bash
# 使用 vendored 版本
# 在 Cargo.toml 中:
openssl = { version = "0.10", features = ["vendored"] }
```

### 运行时错误

**错误**: 动态链接器找不到
```bash
# 检查是否真的静态链接
ldd target/x86_64-unknown-linux-musl/release/binary

# 应该输出: not a dynamic executable
```

**错误**: 在 Alpine 中无法运行
```bash
# 检查架构匹配
file target/x86_64-unknown-linux-musl/release/binary

# 确保 x86_64 二进制在 x86_64 Alpine 上运行
# aarch64 二进制在 aarch64 Alpine 上运行
```

## 性能对比

| 配置 | 大小 | 编译时间 | 运行时性能 |
|------|------|---------|-----------|
| Debug | ~50MB | 快 | 慢 |
| Release (默认) | ~10MB | 中 | 快 |
| Release + musl | ~8MB | 中 | 快-5% |
| Release + musl + LTO | ~5MB | 慢 | 快-2% |
| Release + musl + LTO + strip | ~3MB | 慢 | 快-2% |
| Release + musl + LTO + strip + UPX | ~1.5MB | 慢 | 快-2% |

## 下一步

1. 阅读详细文档: [static-linking-guide.md](./static-linking-guide.md)
2. 查看示例: [examples/](../examples/)
3. 了解 CI/CD: [.github/workflows/build-static.yml](../.github/workflows/build-static.yml)

## 参考资源

- [Rust Edition Guide - musl Support](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html)
- [cross - Zero Setup Rust Cross Compilation](https://github.com/cross-rs/cross)
- [musl libc Official Site](https://musl.libc.org/)
- [Cargo Configuration Reference](https://doc.rust-lang.org/cargo/reference/config.html)

---

**版本**: 1.0
**最后更新**: 2026-03-10
