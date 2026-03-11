# 任务列表: 一键安装与自动化分发

**输入**: 来自 `/specs/003-one-click-distribution/` 的设计文档
**前置条件**: plan.md, spec.md, research.md, data-model.md, contracts/

**测试**: 本功能要求集成测试覆盖所有用户故事（宪章要求）

**组织方式**: 按用户故事分组，实现每个故事的独立实现和测试

## 格式: `[ID] [P?] [Story] Description`

- **[P]**: 可并行执行（不同文件，无依赖）
- **[Story]**: 任务所属用户故事（如 US1, US2, US3, US4）
- 包含具体文件路径

## 路径约定

- CI/CD 配置: `.github/workflows/`
- 安装脚本: `install/remote/`
- 辅助脚本: `scripts/`
- 文档: `docs/`
- Rust 代码: `Cargo.toml`, `.cargo/config.toml`
- 测试: `tests/installation/`

---

## Phase 1: 项目初始化（共享基础设施）

**目的**: 创建项目结构和配置文件

- [x] T001 创建 `.github/workflows/` 目录结构
- [x] T002 创建 `install/remote/` 目录用于远程安装脚本
- [x] T003 [P] 更新 `Cargo.toml` 添加 release profile 优化配置
- [x] T004 [P] 创建或更新 `.cargo/config.toml` 配置 musl 静态链接
- [x] T005 创建 `docs/RELEASE.md` 发布指南文档
- [x] T006 [P] 创建 `scripts/` 目录并添加 `tag-release.sh` 发布脚本

---

## Phase 2: 基础设施（阻塞性前置条件）

**目的**: 所有用户故事依赖的核心基础设施

**⚠️ 关键**: 完成本阶段前，不能开始任何用户故事的实现

- [x] T007 实现版本比较逻辑函数（version_gt, version_eq, version_ge）在 `install/remote/install.sh` 中
- [x] T008 实现系统检测函数（detect_architecture, detect_os）在 `install/remote/install.sh` 中
- [x] T009 实现下载和验证函数（download_and_verify, sha256sum_check）在 `install/remote/install.sh` 中
- [x] T010 实现错误处理和日志函数（abort, warn, log_info）在 `install/remote/install.sh` 中
- [x] T011 配置 GitHub Actions 基础工作流结构（trigger, strategy, caching）在 `.github/workflows/release.yml`

**检查点**: 基础设施就绪 - 用户故事实现可以并行开始

---

## Phase 3: 用户故事 1 - 一键安装无需编译环境 (优先级: P1) 🎯 MVP

**目标**: 用户通过 `curl | bash` 一条命令完成安装，无需配置 Rust 环境

**独立测试**: 在全新 Linux 系统执行 `curl -sSL https://.../install.sh | bash`，60秒内完成安装并启动服务

### 用户故事 1 的集成测试 ⚠️

> **注意**: 先编写测试，确保测试失败后再实现功能

- [x] T012 [P] [US1] 编写安装流程端到端测试在 `tests/installation/test_install_flow.sh`
- [x] T013 [P] [US1] 编写架构检测集成测试在 `tests/installation/test_arch_detection.sh`
- [x] T014 [P] [US1] 编写下载验证集成测试在 `tests/installation/test_download_verify.sh`

### 用户故事 1 的实现

- [x] T015 [P] [US1] 实现主安装流程（main 函数，参数解析）在 `install/remote/install.sh`
- [x] T016 [US1] 实现 GitHub Releases API 查询逻辑（get_latest_version）在 `install/remote/install.sh`
- [x] T017 [US1] 实现二进制文件安装逻辑（install_binary）在 `install/remote/install.sh`
- [x] T018 [US1] 实现 systemd 服务创建和启动逻辑（create_systemd_unit）在 `install/remote/install.sh`
- [x] T019 [US1] 实现安装状态记录逻辑（write_install_state）在 `install/remote/install.sh`
- [x] T020 [US1] 添加交互式确认提示（用户确认安装）在 `install/remote/install.sh`
- [x] T021 [US1] 添加权限检查和提升逻辑（sudo 检测）在 `install/remote/install.sh`
- [x] T022 [US1] 添加支持的架构和操作系统验证在 `install/remote/install.sh`

**检查点**: 此时用户故事 1 应完全可用并可独立测试

---

## Phase 4: 用户故事 2 - 自动化构建与发布 (优先级: P1)

**目标**: 推送 Git tag 后自动构建多平台二进制并发布到 GitHub Releases

**独立测试**: 推送版本 tag（如 v0.3.0），GitHub Actions 自动构建 x86_64 和 aarch64 二进制，创建 Release 并上传产物

### 用户故事 2 的集成测试 ⚠️

- [ ] T023 [P] [US2] 编写 GitHub Actions 工作流测试在 `tests/installation/test_ci_workflow.sh`
- [ ] T024 [P] [US2] 编写多平台构建验证测试在 `tests/installation/test_multiarch_build.sh`
- [ ] T025 [P] [US2] 编写 Release 创建验证测试在 `tests/installation/test_release_creation.sh`

### 用户故事 2 的实现

- [ ] T026 [P] [US2] 实现 Rust 多架构编译配置（构建矩阵）在 `.github/workflows/release.yml`
- [ ] T027 [US2] 实现 Cargo 依赖缓存配置（registry, index, build）在 `.github/workflows/release.yml`
- [ ] T028 [US2] 实现静态链接编译步骤（musl target）在 `.github/workflows/release.yml`
- [ ] T029 [US2] 实现二进制优化步骤（strip, upx 压缩）在 `.github/workflows/release.yml`
- [ ] T030 [US2] 实现归档打包步骤（tar.gz）在 `.github/workflows/release.yml`
- [ ] T031 [US2] 实现 SHA256SUMS.txt 生成步骤在 `.github/workflows/release.yml`
- [ ] T032 [US2] 实现构建产物上传步骤（artifacts）在 `.github/workflows/release.yml`
- [ ] T033 [US2] 实现 Release 创建步骤（softprops/action-gh-release）在 `.github/workflows/release.yml`
- [ ] T034 [US2] 实现 `scripts/tag-release.sh` 辅助发布脚本
- [ ] T035 [US2] 添加构建失败通知和清理逻辑在 `.github/workflows/release.yml`
- [ ] T036 [US2] 配置工作流触发条件（git tag pattern）在 `.github/workflows/release.yml`

**检查点**: 此时用户故事 1 和 2 都应独立可用

---

## Phase 5: 用户故事 3 - 安装脚本智能降级 (优先级: P2)

**目标**: 下载失败时提供清晰错误提示和替代方案

**独立测试**: 模拟网络失败、校验失败等场景，验证错误消息清晰度和可操作性

### 用户故事 3 的集成测试 ⚠️

- [ ] T037 [P] [US3] 编写网络失败场景测试在 `tests/installation/test_network_failure.sh`
- [ ] T038 [P] [US3] 编写校验和失败场景测试在 `tests/installation/test_checksum_failure.sh`
- [ ] T039 [P] [US3] 编写不支持平台场景测试在 `tests/installation/test_unsupported_platform.sh`

### 用户故事 3 的实现

- [ ] T040 [US3] 实现网络重试机制（指数退避）在 `install/remote/install.sh`
- [ ] T041 [US3] 实现 GitHub API 速率限制检查在 `install/remote/install.sh`
- [ ] T042 [US3] 实现下载失败错误处理和手动下载指引在 `install/remote/install.sh`
- [ ] T043 [US3] 实现校验和验证失败处理在 `install/remote/install.sh`
- [ ] T044 [US3] 实现不支持架构/操作系统的错误提示在 `install/remote/install.sh`
- [ ] T045 [US3] 实现临时文件清理逻辑（trap EXIT）在 `install/remote/install.sh`
- [ ] T046 [US3] 添加详细的错误日志和可操作的解决建议在 `install/remote/install.sh`

**检查点**: 此时用户故事 1、2、3 都应独立可用

---

## Phase 6: 用户故事 4 - 版本检测与更新提示 (优先级: P3)

**目标**: 运行 `aegis-input --version` 时检测新版本并提示升级

**独立测试**: 安装旧版本后，执行 `aegis-input --check` 验证能正确识别新版本并显示升级提示

### 用户故事 4 的集成测试 ⚠️

- [ ] T047 [P] [US4] 编写版本检测逻辑测试在 `tests/installation/test_version_check.sh`
- [ ] T048 [P] [US4] 编写版本比较逻辑测试在 `tests/installation/test_version_compare.sh`
- [ ] T049 [P] [US4] 编写 GitHub API 速率限制处理测试在 `tests/installation/test_rate_limit.sh`

### 用户故事 4 的实现

- [ ] T050 [US4] 实现当前版本读取逻辑（get_current_version）在 `install/remote/install.sh`
- [ ] T051 [US4] 实现最新版本查询逻辑（get_latest_version）在 `install/remote/install.sh`
- [ ] T052 [US4] 实现版本比较和更新类型判断逻辑（check_for_updates）在 `install/remote/install.sh`
- [ ] T053 [US4] 实现 `--check` 命令行选项在 `install/remote/install.sh`
- [ ] T054 [US4] 实现版本信息缓存机制（减少 API 调用）在 `install/remote/install.sh`
- [ ] T055 [US4] 在 `aegis-input --version` 输出中集成版本检测提示（修改 CLI 代码）

**检查点**: 所有用户故事现在都应独立可用

---

## Phase 7: 文档与发布

**目的**: 跨所有用户故事的改进

- [ ] T056 [P] 更新 `README.md` 添加一键安装说明
- [ ] T057 [P] 更新 `docs/RELEASE.md` 发布指南（包含测试结果）
- [ ] T058 [P] 创建 `docs/INSTALLATION.md` 详细安装文档
- [ ] T059 [P] 创建 `docs/TROUBLESHOOTING.md` 故障排除指南
- [ ] T060 [P] 在 `install/remote/install.sh` 顶部添加脚本使用说明注释
- [ ] T061 [P] 验证并更新 `quickstart.md` 中的所有命令和示例

---

## Phase 8: 完善与优化

**目的**: 性能优化、安全加固、代码质量

- [ ] T062 [P] 添加安装脚本性能优化（进度条、并发下载）
- [ ] T063 [P] 添加 GitHub Actions 工作流性能优化（缓存命中率）
- [ ] T064 运行完整的安装流程端到端测试并修复发现的问题
- [ ] T065 在 Alpine Linux 容器中测试静态链接二进制兼容性
- [ ] T066 安全加固：添加下载来源验证（HTTPS 证书检查）
- [ ] T067 添加更多的单元测试覆盖（安装脚本函数测试）
- [ ] T068 代码清理和重构（移除重复代码、提取公共函数）
- [ ] T069 添加 CI 集成测试到 GitHub Actions 工作流（在发布前运行测试）
- [ ] T070 性能验证：确保安装时间 <60 秒，构建时间 <10 分钟

---

## 依赖关系与执行顺序

### 阶段依赖

- **项目初始化（Phase 1）**: 无依赖 - 可立即开始
- **基础设施（Phase 2）**: 依赖项目初始化完成 - 阻塞所有用户故事
- **用户故事（Phase 3-6）**: 都依赖基础设施完成
  - 用户故事可以并行进行（如果有人力）
  - 或按优先级顺序执行（P1 → P2 → P3 → P4）
- **文档与发布（Phase 7）**: 依赖所有需要的用户故事完成
- **完善与优化（Phase 8）**: 依赖所有用户故事完成

### 用户故事依赖

- **用户故事 1 (P1)**: 可在基础设施完成后开始 - 无其他故事依赖
- **用户故事 2 (P1)**: 可在基础设施完成后开始 - 无其他故事依赖
- **用户故事 3 (P2)**: 可在基础设施完成后开始 - 可能与 US1 集成但应独立可测试
- **用户故事 4 (P3)**: 可在基础设施完成后开始 - 可能与 US1/US2 集成但应独立可测试

### 每个用户故事内部

- 测试必须先编写并在实现前失败
- 基础函数在组合逻辑之前
- 核心实现在集成之前
- 故事完成后才能进入下一个优先级

### 并行机会

- 所有标记 [P] 的项目初始化任务可并行运行
- 所有标记 [P] 的基础设施任务可在 Phase 2 内并行
- 基础设施完成后，所有用户故事可并行开始（如果团队容量允许）
- 每个用户故事中标记 [P] 的测试可并行运行
- 不同用户故事可由不同团队成员并行工作

---

## 并行示例: 用户故事 1

```bash
# 一起启动用户故事 1 的所有测试：
Task: "安装流程端到端测试在 tests/installation/test_install_flow.sh"
Task: "架构检测集成测试在 tests/installation/test_arch_detection.sh"
Task: "下载验证集成测试在 tests/installation/test_download_verify.sh"

# 然后按顺序运行实现任务
# T015 → T016 → T017 → T018 → T019 → T020 → T021 → T022
```

---

## 实现策略

### MVP 优先（仅用户故事 1）

1. 完成 Phase 1: 项目初始化
2. 完成 Phase 2: 基础设施（关键 - 阻塞所有故事）
3. 完成 Phase 3: 用户故事 1
4. **停止并验证**: 独立测试用户故事 1
5. 如果就绪则部署/演示

### 增量交付

1. 完成项目初始化 + 基础设施 → 基础就绪
2. 添加用户故事 1 → 独立测试 → 部署/演示（MVP！）
3. 添加用户故事 2 → 独立测试 → 部署/演示
4. 添加用户故事 3 → 独立测试 → 部署/演示
5. 添加用户故事 4 → 独立测试 → 部署/演示
6. 每个故事都增加价值而不破坏之前的故事

### 并行团队策略

多开发者情况：

1. 团队一起完成项目初始化 + 基础设施
2. 基础设施完成后：
   - 开发者 A: 用户故事 1（安装脚本）
   - 开发者 B: 用户故事 2（CI/CD）
   - 开发者 C: 用户故事 3（错误处理）+ 用户故事 4（版本检测）
3. 故事独立完成并集成

---

## 备注

- [P] 任务 = 不同文件，无依赖
- [Story] 标签将任务映射到具体用户故事以便追溯
- 每个用户故事应可独立完成和测试
- 实现前验证测试失败
- 每个任务或逻辑组后提交
- 在任何检查点停止以独立验证故事
- 避免：模糊任务、同文件冲突、破坏独立性的跨故事依赖

---

## 任务统计

- **总任务数**: 70
- **项目初始化**: 6 任务
- **基础设施**: 5 任务
- **用户故事 1 (P1)**: 13 任务（3 测试 + 10 实现）
- **用户故事 2 (P1)**: 14 任务（3 测试 + 11 实现）
- **用户故事 3 (P2)**: 10 任务（3 测试 + 7 实现）
- **用户故事 4 (P3)**: 9 任务（3 测试 + 6 实现）
- **文档与发布**: 6 任务
- **完善与优化**: 9 任务
- **并行机会**: 35 任务标记 [P]

### MVP 范围（建议）

MVP 应包含：
- ✅ 项目初始化（Phase 1）
- ✅ 基础设施（Phase 2）
- ✅ 用户故事 1（Phase 3）- 一键安装功能

**总计**: 24 任务实现 MVP，可在 60 秒内完成安装。
