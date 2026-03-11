# CI/CD 工作流契约

**类型**: GitHub Actions 工作流
**版本**: 1.0.0
**文件**: `.github/workflows/release.yml`

本文档定义自动构建和发布的 CI/CD 工作流规范。

---

## 触发条件

### Git Tag 触发

```yaml
on:
  push:
    tags:
      - 'v*.*.*'
```

**匹配规则**:
- ✅ `v1.0.0`
- ✅ `v0.3.0`
- ✅ `v2.1.3`
- ❌ `v1.0`（缺少 PATCH）
- ❌ `1.0.0`（缺少 v 前缀）
- ❌ `release-1.0.0`（不匹配模式）

### 手动触发（可选）

```yaml
on:
  workflow_dispatch:
    inputs:
      version:
        description: '版本号（如 0.3.0）'
        required: true
        type: string
```

---

## 构建矩阵

### 平台矩阵

| 目标 | 操作系统 | 使用 cross | 缓存 |
|------|----------|------------|------|
| `x86_64-unknown-linux-musl` | `ubuntu-latest` | `false` | ✅ |
| `aarch64-unknown-linux-musl` | `ubuntu-latest` | `true` | ✅ |

### 未来扩展（预留）

| 目标 | 操作系统 | 使用 cross | 优先级 |
|------|----------|------------|--------|
| `x86_64-apple-darwin` | `macos-latest` | `false` | P2 |
| `aarch64-apple-darwin` | `macos-latest` | `false` | P2 |
| `x86_64-pc-windows-msvc` | `windows-latest` | `false` | P3 |

### 矩阵配置

```yaml
strategy:
  fail-fast: false
  matrix:
    include:
      - target: x86_64-unknown-linux-musl
        os: ubuntu-latest
        use_cross: false

      - target: aarch64-unknown-linux-musl
        os: ubuntu-latest
        use_cross: true
```

---

## 构建步骤

### 1. 环境准备

```yaml
- name: 安装 Rust
  uses: actions-rs/toolchain@v1
  with:
    profile: minimal
    toolchain: stable
    override: true
    target: ${{ matrix.target }}
```

### 2. 代码检出

```yaml
- name: 检出代码
  uses: actions/checkout@v4
```

### 3. 缓存依赖

```yaml
- name: 缓存 Cargo registry
  uses: actions/cache@v4
  with:
    path: ~/.cargo/registry
    key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

- name: 缓存 Cargo index
  uses: actions/cache@v4
  with:
    path: ~/.cargo/git
    key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

- name: 缓存 Cargo build
  uses: actions/cache@v4
  with:
    path: target
    key: ${{ runner.os }}-cargo-build-target-${{ matrix.target }}-${{ hashFiles('**/Cargo.lock') }}
```

### 4. 编译二进制

```yaml
- name: 构建 ${{ matrix.target }}
  uses: actions-rs/cargo@v1
  with:
    use-cross: ${{ matrix.use_cross }}
    command: build
    args: --release --target ${{ matrix.target }}
```

### 5. 优化二进制

```yaml
- name: Strip 符号表
  run: |
    strip target/${{ matrix.target }}/release/aegis-input

- name: 压缩二进制（可选）
  run: |
    upx --best --lzma target/${{ matrix.target }}/release/aegis-input
```

### 6. 打包归档

```yaml
- name: 创建归档
  run: |
    tar czf aegis-input-${{ matrix.target }}.tar.gz \
      -C target/${{ matrix.target }}/release aegis-input

- name: 生成 SHA256
  run: |
    sha256sum aegis-input-${{ matrix.target }}.tar.gz >> SHA256SUMS.txt
```

### 7. 上传构建产物

```yaml
- name: 上传构建产物
  uses: actions/upload-artifact@v4
  with:
    name: binaries-${{ matrix.target }}
    path: |
      aegis-input-${{ matrix.target }}.tar.gz
      SHA256SUMS.txt
```

---

## Release 创建

### 合并校验和

```yaml
- name: 合并所有 SHA256SUMS
  run: |
    # 下载所有产物的 SHA256SUMS
    # 合并成单个文件
    # 重新上传
```

### 创建 Release

```yaml
- name: 创建 Release
  uses: softprops/action-gh-release@v2
  with:
    files: |
      artifacts/**/*.tar.gz
      SHA256SUMS.txt
    generate_release_notes: true
    draft: false
    prerelease: ${{ contains(github.ref_name, '-rc') || contains(github.ref_name, '-beta') }}
```

### Release 元数据

| 字段 | 值 | 说明 |
|------|-----|------|
| `tag_name` | 自动 | Git tag 名称 |
| `name` | 自动 | `Release {tag_name}` |
| `body` | 自动生成 | GitHub 自动生成 Release Notes |
| `draft` | `false` | 直接发布，非草稿 |
| `prerelease` | 自动检测 | 包含 `-rc` 或 `-beta` 时为 true |
| `files` | 所有 tar.gz | 多架构二进制文件 |
| `files` | SHA256SUMS.txt | 校验和文件 |

---

## 文件命名规范

### 二进制归档

```
aegis-input-{target}.tar.gz
```

**示例**:
- `aegis-input-x86_64-unknown-linux-musl.tar.gz`
- `aegis-input-aarch64-unknown-linux-musl.tar.gz`

### 校验和文件

```
SHA256SUMS.txt
```

**格式**:
```
2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  aegis-input-x86_64-unknown-linux-musl.tar.gz
486e4645420c8cd46d9cc694a09af05c1bb8b2dd2de4c4e1c6f7f40a4dd8d9f0  aegis-input-aarch64-unknown-linux-musl.tar.gz
```

---

## 性能要求

### 构建时间

| 目标 | 预期时间 | 备注 |
|------|----------|------|
| `x86_64-unknown-linux-musl` | <3 分钟 | 原生编译 |
| `aarch64-unknown-linux-musl` | <5 分钟 | 交叉编译 |
| 总计 | <10 分钟 | 并行构建 |

### 缓存效果

| 场景 | 无缓存 | 有缓存 | 提升 |
|------|--------|--------|------|
| 首次构建 | 10 分钟 | 10 分钟 | - |
| 依赖不变 | 8 分钟 | 3 分钟 | 62% |
| 仅代码变更 | 5 分钟 | 2 分钟 | 60% |

---

## 质量门禁

### 必须通过的检查

- ✅ 所有目标编译成功
- ✅ 二进制文件大小 <10MB
- ✅ SHA256 校验和文件生成
- ✅ Release 创建成功

### 失败处理

```yaml
- name: 构建失败通知
  if: failure()
  run: |
    echo "构建失败，请检查日志"
    exit 1
```

---

## 环境变量

### 构建时变量

| 变量 | 值 | 用途 |
|------|-----|------|
| `CARGO_TERM_COLOR` | `always` | 彩色输出 |
| `RUSTFLAGS` | `-C target-feature=+crt-static` | 静态链接 |
| `CARGO_PROFILE_RELEASE_LTO` | `true` | 链接时优化 |
| `CARGO_PROFILE_RELEASE_STRIP` | `true` | Strip 符号 |

### 自定义变量

```yaml
env:
  RUST_VERSION: stable
  CARGO_NET_RETRY: 10
  CARGO_NET_TIMEOUT: 30
```

---

## 秘钥管理

### 无需秘钥

本工作流不需要 GitHub Token 之外的秘钥：
- ✅ 读取代码：自动
- ✅ 创建 Release：`GITHUB_TOKEN` 自动提供
- ✅ 上传产物：`GITHUB_TOKEN` 自动提供

### 未来扩展

如果需要 GPG 签名：

```yaml
- name: 导入 GPG 密钥
  run: |
    echo -n "${{ secrets.GPG_PRIVATE_KEY }}" | gpg --import

- name: 签名二进制
  run: |
    gpg --detach-sign --armor aegis-input-${{ matrix.target }}.tar.gz
```

---

## 监控与日志

### 构建日志

```yaml
- name: 输出构建信息
  run: |
    echo "版本: ${{ github.ref_name }}"
    echo "提交: ${{ github.sha }}"
    echo "目标: ${{ matrix.target }}"
    echo "大小: $(stat -f%z aegis-input-${{ matrix.target }}.tar.gz)"
```

### 性能监控

```yaml
- name: 记录构建时间
  run: |
    echo "开始时间: $(date)"
    # ... 构建 ...
    echo "结束时间: $(date)"
```

---

## 回滚策略

### 发布失败回滚

如果 Release 创建失败：

```yaml
- name: 清理失败的 Release
  if: failure()
  uses: actions/github-script@v7
  with:
    script: |
      github.rest.repos.deleteRelease({
        owner: context.repo.owner,
        repo: context.repo.repo,
        release_id: ${{ steps.create_release.outputs.id }}
      })
```

### 手动回滚

```bash
# 删除失败的 Release
gh release delete v0.3.0 -y

# 删除 Git tag
git tag -d v0.3.0
git push origin :refs/tags/v0.3.0
```

---

## 测试策略

### 构建测试

```yaml
- name: 运行单元测试
  run: |
    cargo test --release --target ${{ matrix.target }}
```

### 集成测试

```yaml
- name: 运行集成测试
  run: |
    # 在容器中测试静态链接二进制
    docker run --rm -v $(pwd):/app alpine:latest \
      /app/target/${{ matrix.target }}/release/aegis-input --version
```

---

## 文档更新

### 自动更新文档

在 Release 创建后自动更新相关文档：

```yaml
- name: 更新安装文档
  run: |
    # 更新 README 中的安装命令
    # 更新 CHANGELOG
    # 提交文档更新
```
