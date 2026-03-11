# Rust 静态链接资源

本目录包含 Rust 静态链接和交叉编译的完整指南和工具。

## 文档

### [static-linking-guide.md](./static-linking-guide.md) - 完整指南
涵盖静态链接的所有方面：
- 交叉编译配置
- musl 工具链详解
- 二进制大小优化
- CI/CD 测试策略
- 常见问题解决

**适合**: 需要深入了解静态链接的开发者

### [static-linking-quickstart.md](./static-linking-quickstart.md) - 快速入门
快速开始使用静态链接：
- 5 分钟快速设置
- 常用命令速查
- 基础故障排除

**适合**: 新手或需要快速参考的开发者

## 项目配置

项目已配置以下文件用于静态链接：

### [Cargo.toml](../Cargo.toml)
优化的 release profile 配置：
- `opt-level = "z"` - 优化大小
- `lto = true` - 链接时优化
- `strip = true` - 自动移除符号
- `panic = "abort"` - 减小二进制大小

### [.cargo/config.toml](../.cargo/config.toml)
Target 特定配置：
- musl 静态链接 flag
- 交叉编译链接器设置

## 脚本工具

### [scripts/build-static.sh](../scripts/build-static.sh)
自动构建静态链接二进制

```bash
# 构建 x86_64 版本
./scripts/build-static.sh x86_64-unknown-linux-musl

# 构建 aarch64 版本
./scripts/build-static.sh aarch64-unknown-linux-musl
```

**功能**:
- 自动安装 targets
- 检查系统依赖
- 编译二进制
- 验证静态链接
- 在 Alpine 中测试

### [scripts/verify-static.sh](../scripts/verify-static.sh)
验证静态链接配置

```bash
./scripts/verify-static.sh target/x86_64-unknown-linux-musl/release/aegis-input
```

**检查项目**:
- 文件类型和架构
- 静态链接状态
- 符号表状态
- 二进制大小评估
- Alpine 兼容性

### [scripts/optimize-binary.sh](../scripts/optimize-binary.sh)
优化二进制大小

```bash
./scripts/optimize-binary.sh target/x86_64-unknown-linux-musl/release/aegis-input
```

**优化步骤**:
1. Strip 符号表
2. UPX 压缩
3. 功能验证
4. 大小对比报告

## CI/CD

### [.github/workflows/build-static.yml](../.github/workflows/build-static.yml)
GitHub Actions 工作流，自动：
- 为多个架构构建静态链接二进制
- 验证静态链接
- 在 Alpine 中测试
- 运行完整测试套件
- 创建 release 包

**触发条件**:
- Push to main/develop
- Pull requests
- Tags (v*)
- Manual workflow dispatch

## 快速开始

### 1. 安装依赖
```bash
sudo apt-get install musl-tools musl-dev gcc-aarch64-linux-gnu
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl
```

### 2. 构建静态链接二进制
```bash
./scripts/build-static.sh x86_64-unknown-linux-musl
```

### 3. 验证
```bash
./scripts/verify-static.sh target/x86_64-unknown-linux-musl/release/aegis-input
```

### 4. 优化（可选）
```bash
./scripts/optimize-binary.sh target/x86_64-unknown-linux-musl/release/aegis-input
```

### 5. 在 Alpine 中测试
```bash
docker run --rm -v $(pwd):/app alpine:latest \
  /app/target/x86_64-unknown-linux-musl/release/aegis-input --version
```

## 常用命令

### 构建
```bash
# 本地架构
cargo build --release --target x86_64-unknown-linux-musl

# 交叉编译
cargo build --release --target aarch64-unknown-linux-musl

# 使用 cross
cross build --release --target aarch64-unknown-linux-musl
```

### 验证
```bash
# 文件类型
file target/x86_64-unknown-linux-musl/release/binary

# 动态依赖
ldd target/x86_64-unknown-linux-musl/release/binary

# 大小
ls -lh target/x86_64-unknown-linux-musl/release/binary
```

### 优化
```bash
# Strip
strip --strip-all target/x86_64-unknown-linux-musl/release/binary

# UPX 压缩
upx --best --lzma target/x86_64-unknown-linux-musl/release/binary
```

## 依赖配置

### 推荐的静态链接依赖

```toml
[dependencies]
# OpenSSL (使用 vendored)
openssl = { version = "0.10", features = ["vendored"] }

# 推荐: 使用 rustls (纯 Rust)
reqwest = { version = "0.12", features = ["rustls-tls"], default-features = false }

# SQLite (bundled)
rusqlite = { version = "0.32", features = ["bundled"] }

# zlib
flate2 = { version = "1.0", default-features = false, features = ["zlib-ng"] }
```

## 性能参考

| 配置 | 大小 | 编译时间 | 运行时 |
|------|------|---------|--------|
| Debug | ~50MB | 快 | 慢 |
| Release | ~10MB | 中 | 快 |
| + musl | ~8MB | 中 | 快-5% |
| + LTO | ~5MB | 慢 | 快-2% |
| + strip | ~3MB | 慢 | 快-2% |
| + UPX | ~1.5MB | 慢 | 快-2% |

## 常见问题

### Q: 静态链接 vs 动态链接？
**A**: 静态链接的优点：
- 可移植性更好（单一二进制）
- 无需依赖库
- 更容易部署到容器

缺点：
- 二进制更大
- 更新库需要重新编译
- 某些功能可能受限（如 glibc NSS）

### Q: musl vs glibc？
**A**:
- **musl**: 完全静态链接，更小，更适合容器
- **glibc**: 性能更好，但静态链接有问题

### Q: 如何处理 OpenSSL？
**A**:
```toml
# 方案 1: 使用 vendored
openssl = { version = "0.10", features = ["vendored"] }

# 方案 2: 使用 rustls (推荐)
reqwest = { version = "0.12", features = ["rustls-tls"], default-features = false }
```

### Q: UPX 压缩安全吗？
**A**: UPX 是成熟的工具，广泛使用。但注意：
- 启动时会有轻微解压开销（毫秒级）
- 某些安全工具可能误报
- 总是测试压缩后的二进制

## 故障排除

### 编译错误
```bash
# cannot find -lm
sudo apt-get install musl-dev

# linker not found
sudo apt-get install gcc-aarch64-linux-gnu

# OpenSSL 错误
openssl = { version = "0.10", features = ["vendored"] }
```

### 运行时错误
```bash
# 动态链接器找不到
ldd target/x86_64-unknown-linux-musl/release/binary
# 应该输出: not a dynamic executable

# 在 Alpine 中失败
# 检查架构匹配
file target/x86_64-unknown-linux-musl/release/binary
```

## 参考资源

### 官方文档
- [Rust Edition Guide - musl Support](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html)
- [Cargo Configuration Reference](https://doc.rust-lang.org/cargo/reference/config.html)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)

### 工具
- [cross - Zero Setup Rust Cross Compilation](https://github.com/cross-rs/cross)
- [musl libc](https://musl.libc.org/)
- [UPX - Ultimate Packer for eXecutables](https://upx.github.io/)

### 社区资源
- [Rust Embedded Cookbook](https://rust-embedded.github.io/cookbook/)
- [Rust Cross-Compilation Guide](https://github.com/japaric/rust-cross)

## 贡献

欢迎改进这些文档！请：
1. 阅读现有文档
2. 测试所有命令和配置
3. 提交 PR 或 Issue

## 许可

与主项目相同。

---

**版本**: 1.0
**最后更新**: 2026-03-10
**维护**: aegis-input 项目
