# 实现计划: 一键安装与自动化分发

**分支**: `003-one-click-distribution` | **日期**: 2026-03-10 | **规格**: [spec.md](./spec.md)
**输入**: 来自 `/specs/003-one-click-distribution/spec.md` 的功能规格

## 摘要

实现一键安装脚本和自动化发布流程，使用户无需配置 Rust 编译环境即可通过 `curl | bash` 快速安装 aegis-input。通过 GitHub Actions 自动构建多平台静态链接二进制文件并发布到 GitHub Releases，安装脚本自动检测系统架构并下载对应二进制。

**技术方案**：
- 使用 GitHub Actions 在推送 tag 时自动构建 x86_64 和 aarch64 的 Linux 静态链接二进制
- 提供安装脚本 `install.sh`，自动检测架构并从 Releases 下载预编译二进制
- 实现版本检测机制，提示用户新版本可用
- 生成 SHA256 校验和文件确保下载完整性

## 技术上下文

**语言/版本**: Rust 1.75+（stable），Shell 脚本（POSIX 兼容的 sh/bash）
**主要依赖**:
- 构建工具：cross（交叉编译）、cargo（Rust 构建工具）
- GitHub Actions：softprops/action-gh-release（发布）、actions/checkout（代码检出）、actions/cache（缓存）
- 安装脚本：curl（下载）、sha256sum（校验）
**存储**: GitHub Releases（二进制文件存储）、Git（版本控制）
**测试**: cargo test（单元测试）、集成测试（安装流程测试）
**目标平台**:
- 主要：Linux（x86_64、aarch64），Debian/Ubuntu/Mint 系列（systemd）
- 未来：macOS、Windows（预留扩展）
**项目类型**: CLI 工具 + 系统服务
**性能目标**:
- 安装时间：<60 秒（从执行命令到服务运行）
- 构建时间：<10 分钟（GitHub Actions 完整构建和发布）
**约束**:
- 二进制文件必须静态链接（musl target）以提升兼容性
- 安装脚本必须与 POSIX 兼容（sh/bash）
- 必须验证下载文件的完整性（SHA256 校验和）
- 需要sudo权限进行系统级安装
**规模/范围**:
- 初始支持 2 个架构（x86_64、aarch64）
- 预计每个架构二进制文件大小 <5MB（优化后）

## 宪章核对

*GATE: Phase 0 研究前必须通过。Phase 1 设计后重新核对。*

### I. 集成测试为硬性要求 ✅

**要求**: 每项功能必须具备至少 1 条端到端集成测试覆盖关键路径。

**核对结果**:
- ✅ **P1 - 一键安装**: 将测试从 curl 命令执行到服务运行完整流程
- ✅ **P1 - 自动化构建**: 测试从 git tag 推送到 Release 创建的完整 CI/CD 流程
- ✅ **P2 - 智能降级**: 测试网络失败、校验失败等边界情况
- ✅ **P3 - 版本检测**: 测试版本比较和提示逻辑

**测试策略**:
- 在 `tests/installation/` 中创建集成测试
- 使用临时目录和 mock GitHub Releases 进行测试
- CI 中运行端到端安装测试

### II. 文档统一中文 ✅

**要求**: 所有文档必须使用中文，仅技术关键词保留英文。

**核对结果**:
- ✅ 所有规格文档（spec.md、plan.md、tasks.md）使用中文
- ✅ 代码注释和文档字符串使用中文
- ⚠️ **例外**: Shell 脚本中的错误消息可使用英文以保持 POSIX 兼容性，但需提供中文文档
- ✅ 技术关键词保留英文：GitHub Actions、cargo、musl、systemd 等

### III. 低开销与用户无干扰 ✅

**要求**: 事件驱动，明确的资源预算，不超出预算。

**核对结果**:
- ✅ **安装脚本资源预算**:
  - CPU: 单次下载和安装，无后台进程
  - 内存: <50MB（下载和校验过程）
  - 网络: 单次下载 <10MB（二进制文件）
  - 磁盘: <20MB（包含临时文件和最终安装）
- ✅ **版本检测资源预算**:
  - 网络: 可选功能，仅在用户主动执行时检查
  - CPU: 版本比较逻辑 <10ms
- ✅ **无后台轮询**: 版本检测仅在用户执行命令时触发，不创建后台守护进程

### IV. 最小权限与故障可恢复 ✅

**要求**: 仅访问必需资源，异常时释放资源，可观测日志。

**核对结果**:
- ✅ **最小权限**:
  - 安装脚本仅在必要时请求 sudo 权限
  - 下载过程无需特殊权限
  - 仅在写入系统目录时需要提升权限
- ✅ **故障恢复**:
  - 安装失败时清理临时文件
  - 下载失败时提供清晰的错误消息和手动下载指引
  - 支持重新安装和升级
- ✅ **可观测日志**:
  - 安装过程输出详细日志
  - 错误消息包含可操作的解决建议
  - 记录安装元数据到 `/var/lib/aegis-input/install.toml`

### V. 跨平台抽象与可演进 ✅

**要求**: 核心逻辑与平台实现解耦，明确平台边界。

**核对结果**:
- ✅ **架构抽象**:
  - 安装脚本中的平台检测逻辑封装为独立函数
  - 支持未来添加 macOS 和 Windows 安装路径
  - GitHub Actions 工作流使用矩阵构建，易于扩展新架构
- ✅ **明确的平台边界**:
  - Linux 特定逻辑：systemd 服务管理
  - 预留 macOS 和 Windows 扩展点
- ✅ **可演进**:
  - 二进制文件命名规范支持多平台：`aegis-input-{target}.tar.gz`
  - Release 元数据包含平台信息

**结论**: 所有宪章原则已核对通过，无违规项，可以进入 Phase 0 研究。

## 项目结构

### 文档（本功能）

```text
specs/003-one-click-distribution/
├── spec.md              # 功能规格
├── plan.md              # 本文件（实现计划）
├── research.md          # Phase 0 输出（技术研究）
├── data-model.md        # Phase 1 输出（数据模型）
├── quickstart.md        # Phase 1 输出（快速开始）
├── contracts/           # Phase 1 输出（接口契约）
│   ├── install-script.md    # 安装脚本接口
│   ├── release-workflow.md  # CI/CD 工作流契约
│   └── version-check.md     # 版本检测 API
└── tasks.md             # Phase 2 输出（任务列表）
```

### 源代码（仓库根）

```text
# 新增文件
.github/
└── workflows/
    └── release.yml          # 自动发布工作流

install/
└── remote/                 # 新增：远程安装脚本
    └── install.sh          # 一键安装脚本（curl | bash）

scripts/
└── tag-release.sh          # 发布辅助脚本

docs/
└── RELEASE.md              # 发布指南

# 修改文件
Cargo.toml                  # 添加 release 优化配置
.cargo/config.toml          # 静态链接配置（可能已存在）
```

**结构决策**: 采用单一项目结构，在现有代码基础上添加：
1. **CI/CD 配置**: `.github/workflows/release.yml` - 自动构建和发布
2. **安装脚本**: `install/remote/install.sh` - 远程一键安装脚本（与现有的 `install/linux/install.sh` 分离）
3. **文档**: 发布指南和快速开始文档
4. **辅助脚本**: 简化发布流程的 shell 脚本

## 复杂度追踪

> **仅在宪章核对有违规需要说明时填写**

本功能无宪章违规，此节留空。

---

## Phase 0: 研究与决策

### 研究主题

1. **GitHub Actions 发布工作流**
   - 决策: 使用 `softprops/action-gh-release@v2` + 矩阵构建
   - 理由: 业界标准，支持自动创建 Release 和上传多平台二进制
   - 替代方案: 自定义 shell 脚本（灵活性高但维护成本大）

2. **Rust 静态链接与交叉编译**
   - 决策: 使用 musl target + cross 工具
   - 理由: 完全静态链接，不依赖系统库，最大化兼容性
   - 替代方案: glibc 动态链接（兼容性差，依赖系统库版本）

3. **安装脚本架构检测**
   - 决策: 使用 `uname -m` 检测架构，`uname` 检测操作系统
   - 理由: POSIX 标准，广泛兼容
   - 替代方案: 使用 Rust 编写的安装器（需要预编译安装器本身，鸡生蛋问题）

4. **版本检测机制**
   - 决策: 通过 GitHub Releases API 查询最新版本
   - 理由: 简单可靠，无需额外基础设施
   - 替代方案: 自建版本检查服务（过度设计）

5. **下载完整性验证**
   - 决策: 生成 SHA256SUMS 文件，安装脚本验证校验和
   - 理由: 平衡安全性和复杂度，无需 GPG 密钥管理
   - 替代方案: GPG 签名验证（更安全但用户感知度低，实施复杂）

详细研究结果见 [research.md](./research.md)。

---

## Phase 1: 设计与契约

### 数据模型

详见 [data-model.md](./data-model.md)。

**核心实体**:
- **Release 元数据**: 版本号、发布日期、各架构二进制文件的 URL 和 SHA256
- **安装状态**: 当前版本、安装时间、安装来源
- **版本信息**: 本地版本、远程最新版本、比较结果

### 接口契约

详见 [contracts/](./contracts/) 目录。

**关键接口**:
1. **安装脚本接口** (`contracts/install-script.md`)
   - 命令行选项和参数
   - 环境变量配置
   - 退出码定义

2. **CI/CD 工作流契约** (`contracts/release-workflow.md`)
   - Git tag 触发规则
   - 构建矩阵定义
   - Release 创建规范

3. **版本检测 API** (`contracts/version-check.md`)
   - GitHub API 调用格式
   - 版本比较逻辑
   - 错误处理

### 快速开始指南

详见 [quickstart.md](./quickstart.md)。

**内容**:
- 开发者如何测试发布流程
- 用户如何使用一键安装
- 发布新版本的步骤

---

## Phase 2: 任务分解

*在执行 `/speckit.tasks` 时生成*

任务将按用户故事分组，每个任务包含：
- 描述
- 验收标准
- 文件路径
- 依赖关系
