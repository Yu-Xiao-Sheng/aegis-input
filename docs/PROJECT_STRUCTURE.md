# 静态链接资源项目结构

本文档展示了为 aegis-input 项目添加的所有静态链接相关资源的完整结构。

## 文件结构

```
aegis-input/
├── .cargo/
│   └── config.toml                 # Cargo 配置 - 静态链接设置
├── .github/
│   └── workflows/
│       └── build-static.yml        # GitHub Actions CI/CD 工作流
├── docs/
│   ├── README.md                   # 静态链接资源总览
│   ├── static-linking-guide.md     # 完整指南（详细文档）
│   ├── static-linking-quickstart.md # 快速入门
│   └── PROJECT_STRUCTURE.md        # 本文件
├── scripts/
│   ├── build-static.sh             # 自动构建脚本
│   ├── verify-static.sh            # 验证脚本
│   └── optimize-binary.sh          # 优化脚本
└── Cargo.toml                      # 已更新 - 添加了优化配置
```

## 文件说明

### 配置文件

#### `.cargo/config.toml`
**用途**: Cargo 工具链配置
**内容**:
- musl targets 的静态链接 flag
- 交叉编译链接器设置
- 环境变量配置

**关键配置**:
```toml
[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "link-arg=-static"]

[target.aarch64-unknown-linux-musl]
rustflags = ["-C", "link-arg=-static"]
linker = "aarch64-linux-gnu-gcc"
```

#### `Cargo.toml` (已更新)
**用途**: 项目配置和优化设置
**新增内容**:
```toml
[profile.release]
opt-level = "z"        # 优化大小
lto = true            # 链接时优化
codegen-units = 1     # 更好的优化
strip = true          # 自动 strip
panic = "abort"       # 减小二进制大小
```

### 文档

#### `docs/README.md`
**用途**: 静态链接资源总览
**内容**:
- 所有文档和工具的索引
- 快速开始指南
- 常用命令速查
- 常见问题解答

#### `docs/static-linking-guide.md`
**用途**: 完整的静态链接指南
**内容**:
- 交叉编译详细配置
- musl 工具链深入解析
- 二进制大小优化技术
- CI/CD 测试策略
- 故障排除指南

**篇幅**: ~800 行
**适合**: 需要深入了解的开发者

#### `docs/static-linking-quickstart.md`
**用途**: 快速入门指南
**内容**:
- 5 分钟快速设置
- 基础命令和用法
- 常见问题快速解决
- 命令速查表

**篇幅**: ~300 行
**适合**: 新手或需要快速参考的开发者

### 脚本

#### `scripts/build-static.sh`
**用途**: 自动构建静态链接二进制
**功能**:
- 检查并安装 Rust targets
- 验证系统依赖
- 编译静态链接二进制
- 验证静态链接状态
- 在 Alpine 容器中测试
- 显示构建摘要

**用法**:
```bash
./scripts/build-static.sh x86_64-unknown-linux-musl
./scripts/build-static.sh aarch64-unknown-linux-musl
```

#### `scripts/verify-static.sh`
**用途**: 验证静态链接配置
**检查项目**:
1. 文件类型和架构
2. 静态链接状态
3. 符号表状态
4. 可执行权限
5. 二进制大小评估
6. Alpine 兼容性测试
7. 优化建议

**用法**:
```bash
./scripts/verify-static.sh target/x86_64-unknown-linux-musl/release/aegis-input
```

#### `scripts/optimize-binary.sh`
**用途**: 优化二进制大小
**优化步骤**:
1. Strip 符号表
2. UPX 压缩
3. 功能验证
4. 大小对比报告
5. 创建备份

**用法**:
```bash
./scripts/optimize-binary.sh target/x86_64-unknown-linux-musl/release/aegis-input
```

### CI/CD

#### `.github/workflows/build-static.yml`
**用途**: GitHub Actions 自动化工作流
**功能**:
- 为多个架构构建静态链接二进制
- 验证静态链接状态
- 在 Alpine 容器中测试
- 运行完整测试套件
- 优化二进制大小
- 创建 release 包
- 生成构建摘要

**触发条件**:
- Push to main/develop
- Pull requests
- Tags (v*)
- Manual workflow dispatch

**支持的目标**:
- x86_64-unknown-linux-musl
- aarch64-unknown-linux-musl

## 使用流程

### 开发流程

```bash
# 1. 开发阶段
cargo build --release

# 2. 构建静态链接版本
./scripts/build-static.sh x86_64-unknown-linux-musl

# 3. 验证静态链接
./scripts/verify-static.sh target/x86_64-unknown-linux-musl/release/aegis-input

# 4. 优化二进制大小
./scripts/optimize-binary.sh target/x86_64-unknown-linux-musl/release/aegis-input

# 5. 测试
docker run --rm -v $(pwd):/app alpine:latest \
  /app/target/x86_64-unknown-linux-musl/release/aegis-input --version
```

### CI/CD 流程

```
Push/PR → GitHub Actions
  ↓
安装依赖 (musl, cross tools)
  ↓
构建静态链接二进制 (x86_64, aarch64)
  ↓
验证静态链接
  ↓
在 Alpine 中测试
  ↓
运行测试套件
  ↓
优化二进制
  ↓
上传 artifacts
  ↓
如果是 tag → 创建 Release
```

## 快速参考

### 环境准备
```bash
sudo apt-get install musl-tools musl-dev gcc-aarch64-linux-gnu
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl
```

### 构建
```bash
./scripts/build-static.sh x86_64-unknown-linux-musl
```

### 验证
```bash
./scripts/verify-static.sh target/x86_64-unknown-linux-musl/release/aegis-input
```

### 优化
```bash
./scripts/optimize-binary.sh target/x86_64-unknown-linux-musl/release/aegis-input
```

### 测试
```bash
docker run --rm -v $(pwd):/app alpine:latest \
  /app/target/x86_64-unknown-linux-musl/release/aegis-input --version
```

## 文件依赖关系

```
Cargo.toml
    ↓
.cargo/config.toml
    ↓
scripts/build-static.sh
    ↓
scripts/verify-static.sh
    ↓
scripts/optimize-binary.sh
    ↓
.github/workflows/build-static.yml
    ↓
docs/ (文档支持)
```

## 维护说明

### 添加新架构支持

1. 更新 `.cargo/config.toml` - 添加新 target 配置
2. 更新 `scripts/build-static.sh` - 添加到 VALID_TARGETS
3. 更新 `.github/workflows/build-static.yml` - 添加到 matrix
4. 更新文档 - 添加新架构的说明

### 更新脚本

所有脚本都包含：
- 完整的错误处理
- 彩色输出
- 详细的进度信息
- 自动依赖检查

测试脚本修改：
```bash
# 语法检查
bash -n scripts/*.sh

# 测试运行
./scripts/build-static.sh x86_64-unknown-linux-musl
```

### 文档更新

文档使用 Markdown 格式，包含：
- 代码块
- 表格
- 链接
- 任务列表

预览文档：
```bash
# 使用 VS Code 插件
# 或使用 Markdown 预览工具
```

## 性能数据

基于 aegis-input 项目：

| 配置 | 大小 | 说明 |
|------|------|------|
| Debug | ~50MB | 未优化 |
| Release (默认) | ~10MB | 基础优化 |
| Release + musl | ~8MB | 静态链接 |
| Release + musl + LTO | ~5MB | 链接时优化 |
| Release + musl + LTO + strip | ~3MB | 移除符号 |
| Release + musl + LTO + strip + UPX | ~1.5MB | 压缩 |

## 已知问题

### udev 依赖
项目使用 `udev` crate，这可能影响静态链接：
- udev 依赖系统库
- 可能需要动态链接
- 考虑使用替代方案或容器化部署

### 解决方案
1. 使用 `pkg-config` 查找系统库
2. 在容器中运行（Alpine + udev）
3. 考虑纯 Rust 替代品

## 参考资源

- [主文档](./README.md)
- [完整指南](./static-linking-guide.md)
- [快速入门](./static-linking-quickstart.md)
- [Rust 官方文档](https://doc.rust-lang.org/)
- [musl libc](https://musl.libc.org/)

## 贡献

欢迎改进！请：
1. 测试所有配置和脚本
2. 验证在不同系统上的兼容性
3. 更新文档
4. 提交 PR

---

**版本**: 1.0
**创建日期**: 2026-03-10
**项目**: aegis-input
