# 技术研究: 一键安装与自动化分发

**功能**: 003-one-click-distribution
**创建日期**: 2026-03-10
**状态**: 已完成

本文档记录为实现"一键安装与自动化分发"功能进行的技术研究和决策。

---

## 研究主题 1: GitHub Actions 发布工作流

### 决策

使用 `softprops/action-gh-release@v2` 结合矩阵构建实现自动发布。

### 技术方案

#### 工作流配置

```yaml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-musl
            os: ubuntu-latest
            use_cross: false
          - target: aarch64-unknown-linux-musl
            os: ubuntu-latest
            use_cross: true

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - uses: actions-rs/cargo@v1
        with:
          use-cross: ${{ matrix.use_cross }}
          command: build
          args: --release --target ${{ matrix.target }}

      - run: |
          tar czf aegis-input-${{ matrix.target }}.tar.gz \
            -C target/${{ matrix.target }}/release aegis-input

      - uses: softprops/action-gh-release@v2
        with:
          files: artifacts/*/*.tar.gz
          generate_release_notes: true
```

### 关键特性

1. **多平台并行构建**: 使用 matrix 同时构建多个架构
2. **智能使用 cross**: 仅在必要时使用交叉编译（aarch64），原生编译更快
3. **三层缓存**: Cargo registry、index、build 缓存加速构建
4. **自动 Release Notes**: 使用 GitHub 内置功能生成 Release 说明
5. **Git Tag 触发**: 推送 `v*.*.*` 标签自动触发

### 性能优化

- **缓存策略**: 基于 `Cargo.lock` 的哈希值，依赖不变时命中缓存
- **并行构建**: 多个架构同时构建，不串行等待
- **条件编译**: x86_64 使用原生编译，速度提升 3-5 倍

### 替代方案评估

| 方案 | 优点 | 缺点 | 选择理由 |
|------|------|------|----------|
| softprops/action-gh-release | 业界标准，功能完善，社区活跃 | - | ✅ 采用 |
| 自定义 shell 脚本 | 灵活性高，完全可控 | 维护成本大，需手动处理 API | ❌ 过度设计 |
| goreleaser (多语言支持) | 功能强大，支持多语言 | 主要针对 Go，Rust 支持有限 | ❌ 不适合 |

### 参考资源

- [softprops/action-gh-release](https://github.com/softprops/action-gh-release)
- [Cross Compiling Rust in GitHub Actions](https://blog.urth.org/2023/03/05/cross-compiling-rust-projects-in-github-actions/)
- [Rust CI/CD Best Practices](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)

---

## 研究主题 2: Rust 静态链接与交叉编译

### 决策

使用 musl target + cross 工具实现完全静态链接。

### 技术方案

#### Cargo 配置

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]

[target.aarch64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]
```

#### 发布优化配置

```toml
# Cargo.toml
[profile.release]
opt-level = "z"        # 优化大小
lto = true            # 链接时优化
codegen-units = 1     # 更好的优化
strip = true          # 自动 strip 符号
panic = "abort"       # 减小大小
```

#### 构建命令

```bash
# 安装 targets
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

# 安装 cross（可选）
cargo install cross

# x86_64 原生编译
cargo build --release --target x86_64-unknown-linux-musl

# aarch64 交叉编译
cross build --release --target aarch64-unknown-linux-musl
```

### 依赖静态链接

由于项目使用 `udev` crate（通常依赖系统库），需要特殊处理：

```toml
# 如果需要网络功能，使用 rustls 而不是 openssl
reqwest = { version = "0.12", features = ["rustls-tls"], default-features = false }

# 如果需要 SQLite，使用 bundled 特性
rusqlite = { version = "0.32", features = ["bundled"] }
```

### 性能对比

| 配置 | 大小 | 编译时间 | 运行时性能 |
|------|------|---------|-----------|
| Debug | ~50MB | 快 | 慢 |
| Release | ~10MB | 中 | 快 |
| + musl | ~8MB | 中 | 快-5% |
| + LTO | ~5MB | 慢 | 快-2% |
| + strip | ~3MB | 慢 | 快-2% |

### 兼容性测试

在 Alpine Linux 容器中测试静态链接二进制：

```bash
docker run --rm -v $(pwd):/app alpine:latest \
  /app/target/x86_64-unknown-linux-musl/release/aegis-input --version
```

### 替代方案评估

| 方案 | 优点 | 缺点 | 选择理由 |
|------|------|------|----------|
| musl 静态链接 | 完全独立，兼容性最好 | 轻微性能损失 | ✅ 采用 |
| glibc 动态链接 | 性能最优 | 依赖系统库版本，兼容性差 | ❌ 不适合分发 |
| 动态链接打包 | 兼容性和性能平衡 | 安装复杂，需要部署多个文件 | ❌ 用户体验差 |

### 已知问题

- **udev 依赖**: udev crate 通常依赖系统库，可能无法完全静态链接
  - **解决方案**: 在容器中运行或动态链接 udev
  - **验证**: 在不同发行版中测试二进制兼容性

### 参考资源

- [Rust MUSL Static Binaries](https://doc.rust-lang.org/edition-guide/rust-2021/static-linking-with-musl.html)
- [min-sized-rust](https://github.com/johnthagen/min-sized-rust)
- [Cross Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)

---

## 研究主题 3: 安装脚本最佳实践

### 决策

使用 POSIX 兼容的 shell 脚本，通过 `uname -m` 检测架构，从 GitHub Releases 下载二进制。

### 技术方案

#### 安全措施

```bash
# 强制 HTTPS 和 TLS 1.2+
DOWNLOAD_URL="https://github.com/user/repo/releases/download/${VERSION}/aegis-input-${ARCH}.tar.gz"

curl --proto '=https' --tlsv1.2 -sSf "$DOWNLOAD_URL" -o "$tmp_file"
```

#### 架构检测

```bash
detect_architecture() {
    case "$(uname -m)" in
        x86_64|amd64)
            ARCH="x86_64-unknown-linux-musl"
            ;;
        aarch64|arm64)
            ARCH="aarch64-unknown-linux-musl"
            ;;
        *)
            abort "不支持的架构: $(uname -m)"
            ;;
    esac
}
```

#### 操作系统检测

```bash
detect_os() {
    OS="$(uname)"
    if [[ "$OS" == "Linux" ]]; then
        ON_LINUX=1
        # 检测发行版
        if [[ -f /etc/os-release ]]; then
            . /etc/os-release
            DISTRO="$ID"
        fi
    elif [[ "$OS" == "Darwin" ]]; then
        abort "macOS 支持即将推出，当前仅支持 Linux"
    else
        abort "不支持的操作系统: $OS"
    fi
}
```

#### 下载与校验

```bash
download_and_verify() {
    local version="$1"
    local arch="$2"

    # 下载二进制
    local download_url="https://github.com/.../releases/download/${version}/aegis-input-${arch}.tar.gz"
    curl --proto '=https' --tlsv1.2 -sSf "$download_url" -o "$tmp_file"

    # 下载校验和
    local checksum_url="https://github.com/.../releases/download/${version}/SHA256SUMS.txt"
    curl --proto '=https' --tlsv1.2 -sSf "$checksum_url" -o "$checksum_file"

    # 验证校验和
    local expected_checksum=$(grep "aegis-input-${arch}.tar.gz" "$checksum_file" | awk '{print $1}')
    local actual_checksum=$(sha256sum "$tmp_file" | awk '{print $1}')

    if [[ "$actual_checksum" != "$expected_checksum" ]]; then
        abort "校验和验证失败！文件可能已损坏。"
    fi
}
```

#### 错误处理

```bash
# 重试机制（指数退避）
retry() {
    local tries="$1" n="$1" pause=2
    shift

    if ! "$@"; then
        while [[ $((--n)) -gt 0 ]]; do
            warn "重试中（${pause}秒后）: $(shell_join "$@")"
            sleep "$pause"
            ((pause *= 2))  # 指数退避
            if "$@"; then
                return
            fi
        done
        abort "失败 ${tries} 次: $(shell_join "$@")"
    fi
}

# 使用
retry 5 curl -fsSL "$DOWNLOAD_URL" -o "$output_file"
```

#### GitHub API 速率限制处理

```bash
check_github_rate_limit() {
    if command -v jq >/dev/null 2>&1; then
        local rate_limit=$(curl -s https://api.github.com/rate_limit)
        local remaining=$(echo "$rate_limit" | jq -r '.rate.remaining')

        if [[ "$remaining" -lt 10 ]]; then
            local reset=$(echo "$rate_limit" | jq -r '.rate.reset')
            local reset_time=$(date -d "@$reset" "+%H:%M:%S" 2>/dev/null || date -r "$reset" "+%H:%M:%S")
            warn "GitHub API 速率限制较低（剩余 ${remaining}）。重置时间：${reset_time}"
        fi
    fi
}
```

### 版本检测与比较

```bash
# 获取最新版本
get_latest_version() {
    local repo="user/aegis-input"
    local latest_url="https://api.github.com/repos/${repo}/releases/latest"

    check_github_rate_limit

    local version=$(curl -s "$latest_url" | grep -oP '"tag_name":\s*"\K[^"]*')
    echo "$version"
}

# 获取当前版本
get_current_version() {
    if command -v aegis-input >/dev/null 2>&1; then
        aegis-input --version 2>/dev/null | grep -oP 'version\s+\K[0-9.]+' || echo "0.0.0"
    else
        echo "not-installed"
    fi
}

# 版本比较
version_gt() {
    [[ "${1%.*}" -gt "${2%.*}" ]] || [[ "${1%.*}" -eq "${2%.*}" && "${1#*.}" -gt "${2#*.}" ]]
}

# 检查更新
check_for_updates() {
    local current_version=$(get_current_version)
    local latest_version=$(get_latest_version)

    if version_gt "$latest_version" "$current_version"; then
        log_info "新版本可用: ${latest_version}（当前: ${current_version}）"
        return 0
    else
        log_info "已是最新版本: ${current_version}"
        return 1
    fi
}
```

### 安装流程

```bash
install_binary() {
    local version="$1"
    local arch="$2"

    # 1. 下载并验证
    download_and_verify "$version" "$arch"

    # 2. 解压
    tar xzf "$tmp_file" -C "$tmp_dir"

    # 3. 检测安装路径
    local install_dir="${INSTALL_DIR:-/usr/local/bin}"
    local binary_path="$install_dir/aegis-input"

    # 4. 停止现有服务（如果存在）
    if systemctl is-active --quiet aegis-input 2>/dev/null; then
        log_info "停止现有服务..."
        sudo systemctl stop aegis-input
    fi

    # 5. 安装二进制
    sudo mkdir -p "$install_dir"
    sudo cp "$tmp_dir/aegis-input" "$binary_path"
    sudo chmod +x "$binary_path"

    # 6. 创建 systemd 服务（如果不存在）
    if [[ ! -f /etc/systemd/system/aegis-input.service ]]; then
        create_systemd_unit
    fi

    # 7. 启动服务
    sudo systemctl daemon-reload
    sudo systemctl enable aegis-input
    sudo systemctl start aegis-input

    log_info "安装完成！"
    log_info "版本: $(aegis-input --version)"
    log_info "状态: $(systemctl is-active aegis-input)"
}
```

### 安装命令

```bash
# 一键安装（推荐）
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/user/aegis-input/main/install/remote/install.sh | bash

# 或下载后检查再执行
curl --proto '=https' --tlsv1.2 -sSf https://.../install.sh -o install.sh
less install.sh  # 检查脚本
sh install.sh
```

### 替代方案评估

| 方案 | 优点 | 缺点 | 选择理由 |
|------|------|------|----------|
| POSIX shell 脚本 | 兼容性最好，无需预编译 | 安全性争议（pipe to bash） | ✅ 采用 |
| Rust 编写的安装器 | 类型安全，易于维护 | 需要预编译安装器本身 | ❌ 鸡生蛋问题 |
| 包管理器（APT） | 标准化，易于升级 | 需要维护仓库，复杂性高 | ❌ 未来扩展 |

### 业界参考

- [Homebrew install.sh](https://github.com/Homebrew/install/blob/main/install.sh) - 最全面的示例（1167 行）
- [rustup-init.sh](https://github.com/rust-lang/rustup/blob/main/rustup-init.sh) - 安全聚焦，强制 TLS
- [Docker install.sh](https://github.com/docker/docker-install/blob/main/install.sh) - 简洁，基于发行版

### 参考资源

- [Security StackExchange: curl | bash 安全性](https://security.stackexchange.com/questions/213401/is-curl-something-sudo-bash-reasonably-safe)
- [StackOverflow: 架构检测](https://stackoverflow.com/questions/48678152/how-to-detect-architecture-via-shell)
- [GitHub API Rate Limits](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api)

---

## 研究主题 4: 版本管理策略

### 决策

遵循语义化版本（Semantic Versioning），使用 `vMAJOR.MINOR.PATCH` 格式。

### 版本号规范

```
vMAJOR.MINOR.PATCH

MAJOR: 不兼容的 API 变更
MINOR: 向后兼容的功能新增
PATCH: 向后兼容的问题修复
```

### 示例

- `v0.2.0` → `v0.3.0`: 新增一键安装功能（MINOR）
- `v0.3.0` → `v0.3.1`: 修复安装脚本 bug（PATCH）
- `v0.3.1` → `v1.0.0`: 稳定版本发布（MAJOR）

### 发布流程

```bash
# 使用辅助脚本
./scripts/tag-release.sh 0.3.0

# 或手动操作
git tag -a v0.3.0 -m "Release v0.3.0: 一键安装功能"
git push origin v0.3.0
```

### Release Notes

使用 GitHub Actions 的 `generate_release_notes: true` 自动生成：

```yaml
- uses: softprops/action-gh-release@v2
  with:
    generate_release_notes: true
    body: |
      ## 🚀 Release ${{ github.ref_name }}

      ### 安装
      curl --proto '=https' --tlsv1.2 -sSf https://.../install.sh | bash

      ### 变更
      - 新增一键安装功能
      - 自动构建多平台二进制
      - 版本检测和更新提示
```

---

## 总结

### 技术栈

- **构建**: Rust 1.75+, cargo, cross
- **CI/CD**: GitHub Actions, softprops/action-gh-release
- **安装脚本**: POSIX shell (sh/bash)
- **依赖**: curl, sha256sum
- **平台**: Linux (x86_64, aarch64), systemd

### 关键决策

1. ✅ 使用 GitHub Actions 自动化发布
2. ✅ musl 静态链接最大化兼容性
3. ✅ POSIX shell 脚本实现一键安装
4. ✅ SHA256 校验和确保下载完整性
5. ✅ 语义化版本管理

### 下一步

进入 Phase 1: 设计与契约
- 创建 [data-model.md](./data-model.md)
- 定义 [contracts/](./contracts/)
- 编写 [quickstart.md](./quickstart.md)
