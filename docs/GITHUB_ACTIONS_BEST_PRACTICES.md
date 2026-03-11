# GitHub Actions 发布 Rust 二进制文件最佳实践

本文档总结了使用 GitHub Actions 发布 Rust 二进制文件的最佳实践。

## 1. 交叉编译工作流

### 1.1 使用 `cross` 工具

`cross` 是 Rust 交叉编译的标准工具，通过 Docker 容器简化了交叉编译过程。

**安装方式**：
```bash
cargo install cross
```

**在 GitHub Actions 中使用**：
```yaml
- name: Install cross
  uses: taiki-e/install-action@cross
  with:
    tool: cross

- name: Build with cross
  run: cross build --release --target aarch64-unknown-linux-musl
```

### 1.2 静态链接配置（musl target）

使用 musl 目标创建完全静态链接的二进制文件：

**优势**：
- 不依赖系统的 glibc 版本
- 可移植性强，适用于各种 Linux 发行版
- 单文件部署，简化安装

**配置示例**：
```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]

[target.aarch64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]
```

### 1.3 构建矩阵设置

使用矩阵策略并行构建多个平台：

```yaml
strategy:
  matrix:
    include:
      - target: x86_64-unknown-linux-musl
        os: ubuntu-latest
        use_cross: false

      - target: aarch64-unknown-linux-musl
        os: ubuntu-latest
        use_cross: true

      - target: x86_64-apple-darwin
        os: macos-latest
        use_cross: false
```

**关键点**：
- `use_cross: false` - 原生编译，速度快
- `use_cross: true` - 需要使用 cross 工具
- 为不同平台设置 `fail-fast: false`，避免一个平台失败导致全部停止

## 2. 自动发布流程

### 2.1 触发条件：Git Tag 推送

```yaml
on:
  push:
    tags:
      - 'v*.*.*'  # 匹配 v1.0.0, v2.1.3 等
```

### 2.2 创建 GitHub Release

使用 `softprops/action-gh-release` 自动创建 Release：

```yaml
- name: Create Release
  uses: softprops/action-gh-release@v2
  with:
    files: |
      artifacts/*.tar.gz
      artifacts/SHA256SUMS.txt
    generate_release_notes: true
    draft: false
    prerelease: false
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

**关键配置**：
- `generate_release_notes: true` - 自动生成 Release Notes
- `files` - 支持通配符，可上传多个文件
- `GITHUB_TOKEN` - 自动提供，无需手动配置

### 2.3 上传构建产物

**步骤 1**：构建并创建压缩包
```yaml
- name: Build binary
  run: cargo build --release --target ${{ matrix.target }}

- name: Create tarball
  run: |
    cd target/${{ matrix.target }}/release
    tar czf ../../../${{ matrix.asset_name }} aegis-input
```

**步骤 2**：上传为临时 artifact
```yaml
- name: Upload artifact
  uses: actions/upload-artifact@v4
  with:
    name: ${{ matrix.asset_name }}
    path: ${{ matrix.asset_name }}
```

**步骤 3**：在 Release job 中下载所有 artifacts
```yaml
- name: Download all artifacts
  uses: actions/download-artifact@v4
  with:
    path: artifacts
```

### 2.4 SHA256 校验和文件生成

```yaml
- name: Generate SHA256 checksums
  run: |
    cd artifacts
    for dir in */; do
      cd "$dir"
      for file in *; do
        if [ -f "$file" ]; then
          sha256sum "$file" >> ../SHA256SUMS.txt
        fi
      done
      cd ..
    done
```

**用户验证**：
```bash
sha256sum -c SHA256SUMS.txt
```

## 3. Actions 市场推荐

### 3.1 核心 Actions

1. **softprops/action-gh-release** - 创建 GitHub Release
   - 仓库：https://github.com/softprops/action-gh-release
   - 用途：自动创建 Release 并上传文件
   - 特点：简单易用，支持自动生成 Release Notes

2. **dtolnay/rust-toolchain** - 安装 Rust 工具链
   - 仓库：https://github.com/dtolnay/rust-toolchain
   - 用途：安装指定版本的 Rust
   - 特点：支持多目标，缓存友好

3. **taiki-e/install-action** - 安装开发工具
   - 仓库：https://github.com/taiki-e/install-action
   - 用途：安装 cross、cargo-binstall 等工具
   - 特点：支持多种 Rust 工具

4. **actions/cache** - 缓存依赖
   - 仓库：https://github.com/actions/cache
   - 用途：缓存 Cargo 依赖和构建产物
   - 特点：显著加速构建

### 3.2 缓存策略

**三层缓存策略**：
```yaml
- name: Cache cargo registry
  uses: actions/cache@v4
  with:
    path: ~/.cargo/registry
    key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

- name: Cache cargo index
  uses: actions/cache@v4
  with:
    path: ~/.cargo/git
    key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

- name: Cache cargo build
  uses: actions/cache@v4
  with:
    path: target
    key: ${{ runner.os }}-cargo-build-target-${{ matrix.target }}-${{ hashFiles('**/Cargo.lock') }}
```

**缓存效果**：
- 首次构建：5-10 分钟
- 缓存命中：1-3 分钟

### 3.3 构建时间优化技巧

1. **使用 `--release` 配置优化**
   ```toml
   [profile.release]
   opt-level = 3
   lto = true
   codegen-units = 1
   ```

2. **并行编译**
   ```yaml
   strategy:
     matrix:
       include: [...]  # 多个平台并行构建
   ```

3. **条件性使用 cross**
   ```yaml
   - name: Build
     run: |
       if [ "${{ matrix.use_cross }}" = "true" ]; then
         cross build --release --target ${{ matrix.target }}
       else
         cargo build --release --target ${{ matrix.target }}
       fi
   ```

4. **Strip 符号表**
   ```yaml
   - name: Strip binary
     run: strip target/${{ matrix.target }}/release/aegis-input
   ```

## 4. Release 版本管理

### 4.1 版本号格式

遵循语义化版本（Semantic Versioning）：

```
vMAJOR.MINOR.PATCH
```

- **MAJOR**：不兼容的 API 变更
- **MINOR**：向后兼容的功能新增
- **PATCH**：向后兼容的问题修正

**示例**：
- `v1.0.0` - 初始稳定版本
- `v1.1.0` - 新增功能
- `v1.1.1` - Bug 修复
- `v2.0.0` - 重大变更

### 4.2 Release Notes 自动生成

使用 GitHub 内置的 Release Notes 生成：

```yaml
- uses: softprops/action-gh-release@v2
  with:
    generate_release_notes: true
```

**自定义模板**：
```yaml
body: |
  ## 🚀 Aegis Input ${{ github.ref_name }}

  ### 📦 下载说明

  **Linux (静态链接)**
  - `x86_64-unknown-linux-musl`: 适用于大多数 Linux 发行版
  - `aarch64-unknown-linux-musl`: 适用于 ARM64 Linux 服务器

  ### 🔐 验证校验和
  ```bash
  sha256sum -c SHA256SUMS.txt
  ```
```

### 4.3 多架构二进制文件命名规范

**推荐的命名格式**：
```
{project-name}-{target-triple}.tar.gz
```

**示例**：
- `aegis-input-x86_64-unknown-linux-musl.tar.gz`
- `aegis-input-aarch64-unknown-linux-musl.tar.gz`
- `aegis-input-x86_64-apple-darwin.tar.gz`
- `aegis-input-aarch64-apple-darwin.tar.gz`

**目标三元组格式**：
```
{arch}-{vendor}-{os}-{env}
```

常见目标：
- `x86_64-unknown-linux-musl` - Linux 64位静态链接
- `aarch64-unknown-linux-musl` - Linux ARM64 静态链接
- `x86_64-apple-darwin` - macOS Intel
- `aarch64-apple-darwin` - macOS Apple Silicon

## 5. 完整工作流示例

参见项目中的文件：
- **基础版本**：`.github/workflows/release.yml`
- **优化版本**：`.github/workflows/release-optimized.yml`

## 6. 安全最佳实践

### 6.1 权限配置

```yaml
permissions:
  contents: write  # 允许创建 Release
```

### 6.2 密钥管理

- 使用 `GITHUB_TOKEN`（自动提供）
- 避免在代码中硬编码敏感信息
- 使用 GitHub Secrets 存储必要的凭证

### 6.3 依赖验证

```yaml
- name: Verify dependencies
  run: cargo audit
```

## 7. 监控和调试

### 7.1 构建状态监控

- GitHub Actions 页面查看实时状态
- 设置通知（Email/Slack/Discord）

### 7.2 常见问题

1. **交叉编译失败**
   - 检查 Cross.toml 配置
   - 验证目标平台支持

2. **静态链接失败**
   - 检查依赖项是否支持静态链接
   - 使用 `cargo tree` 查看依赖树

3. **Release 创建失败**
   - 验证 `GITHUB_TOKEN` 权限
   - 检查标签格式

## 8. 相关资源

- [Cross 交叉编译工具](https://github.com/cross-rs/cross)
- [softprops/action-gh-release](https://github.com/softprops/action-gh-release)
- [Rust 发布最佳实践](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [GitHub Actions 文档](https://docs.github.com/en/actions)
