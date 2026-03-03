---

description: "功能实现的任务清单"
---

# 任务清单: Linux 外置设备禁用内置输入

**输入**: /home/yuxs/github_project/aegis-input/specs/001-linux-input-disable/
**前置条件**: plan.md, spec.md, research.md, data-model.md, contracts/

**测试**: 集成测试为必须项。每个用户故事至少包含 1 条端到端集成测试。单元测试可选。

**组织方式**: 任务按用户故事分组，确保每条故事可独立实现与测试。

## 格式: `[ID] [P?] [Story] 描述`

- **[P]**: 可并行执行（不同文件、无依赖）
- **[Story]**: 所属用户故事（例如: US1, US2, US3）
- 描述中必须包含明确的文件路径

## 路径约定

- 单项目结构，使用 `src/` 与 `tests/`

---

## Phase 1: 初始化（共享基础）

**目的**: 项目初始化与基础结构

- [x] T001 创建功能模块目录与占位文件: src/core/mod.rs, src/platform/mod.rs, src/platform/linux/mod.rs, src/config/mod.rs, src/service/mod.rs, src/cli/mod.rs
- [x] T002 [P] 初始化 Rust 工程配置与基础依赖清单更新: Cargo.toml
- [x] T003 [P] 配置基础质量工具与默认检查命令: rustfmt/clippy（记录到 docs 或 Makefile）

---

## Phase 2: 基础能力（阻塞前置）

**目的**: 所有用户故事实施前必须完成的核心基础设施

**⚠️ 关键**: 完成前不得开始任何用户故事开发

- [x] T004 设计配置结构与解析入口: src/config/config.rs
- [x] T005 建立核心数据模型与状态机骨架: src/core/state.rs, src/core/policy.rs
- [x] T006 建立平台抽象接口与统一事件模型: src/platform/mod.rs
- [x] T007 实现 Linux 设备发现与事件监听骨架: src/platform/linux/udev.rs
- [x] T008 实现 Linux 设备禁用/恢复能力骨架: src/platform/linux/evdev.rs
- [x] T009 建立服务运行入口与主循环框架: src/service/runner.rs, src/service/main.rs
- [x] T010 建立日志与可观测性基础: src/service/logging.rs
- [x] T011 建立 CLI 入口与命令骨架: src/cli/main.rs
- [x] T012 建立测试用平台替身以支持端到端集成测试: tests/integration/fake_platform.rs

**检查点**: 基础完成后，用户故事可并行推进

---

## Phase 3: 用户故事 1 - 外置键盘触发内置键盘禁用 (优先级: P1) 🎯 MVP

**目标**: 外置键盘接入/移除时，内置键盘自动禁用/恢复

**独立测试**: 插入/移除外置键盘后验证内置键盘禁用/恢复与 2 秒内响应

### 用户故事 1 的测试（必需）

- [x] T013 [P] [US1] [端到端] 集成测试: 外置键盘接入禁用内置键盘，路径: tests/integration/keyboard_disable.rs
- [x] T014 [P] [US1] 契约测试: CLI enable/disable/status 行为，路径: tests/contract/cli_control.rs

### 用户故事 1 的实现

- [x] T015 [US1] 实现外置键盘计数与内置键盘禁用规则: src/core/policy.rs
- [x] T016 [US1] 设备事件驱动更新计数: src/core/state.rs
- [x] T017 [US1] Linux 键盘设备识别与禁用/恢复调用: src/platform/linux/udev.rs, src/platform/linux/evdev.rs
- [x] T018 [US1] CLI enable/disable/status 完整行为与状态持久化: src/cli/main.rs, src/config/config.rs

**检查点**: 用户故事 1 可独立运行并通过集成测试

---

## Phase 4: 用户故事 2 - 外置鼠标触发内置指点设备禁用 (优先级: P2)

**目标**: 外置鼠标接入/移除时，内置指点设备自动禁用/恢复

**独立测试**: 插入/移除外置鼠标后验证内置指点设备禁用/恢复与 2 秒内响应

### 用户故事 2 的测试（必需）

- [x] T019 [P] [US2] [端到端] 集成测试: 外置鼠标接入禁用内置指点设备，路径: tests/integration/pointing_disable.rs
- [x] T020 [P] [US2] 契约测试: 配置文件解析与默认规则，路径: tests/contract/config_rules.rs

### 用户故事 2 的实现

- [x] T021 [US2] 实现外置鼠标计数与内置指点设备禁用规则: src/core/policy.rs
- [x] T022 [US2] Linux 指点设备识别与禁用/恢复调用: src/platform/linux/udev.rs, src/platform/linux/evdev.rs

**检查点**: 用户故事 1 与 2 均可独立运行与测试

---

## Phase 5: 用户故事 3 - 按类型独立禁用与多设备保持 (优先级: P3)

**目标**: 键盘/鼠标按类型独立影响，多个同类型外设存在时仅在全部移除后恢复

**独立测试**: 同时接入外置键盘与鼠标、以及多外置键盘场景，验证独立与计数逻辑

### 用户故事 3 的测试（必需）

- [x] T023 [P] [US3] [端到端] 集成测试: 多外置键盘计数与恢复时机，路径: tests/integration/multi_keyboard.rs
- [x] T024 [P] [US3] [端到端] 集成测试: 键盘与鼠标按类型独立禁用，路径: tests/integration/type_isolation.rs

### 用户故事 3 的实现

- [x] T025 [US3] 实现按类型独立计数与禁用状态同步: src/core/state.rs, src/core/policy.rs
- [x] T026 [US3] 处理多设备移除与恢复边界: src/core/state.rs

**检查点**: 所有用户故事均可独立运行与测试

---

## Phase N: 打磨与跨切关注点

**目的**: 影响多个用户故事的改进事项

- [x] T027 [P] 文档更新与验收步骤完善: specs/001-linux-input-disable/quickstart.md, docs/implementation.md
- [x] T028 [P] 性能预算验证与记录: docs/performance-budget.md
- [x] T029 失败路径恢复验证与补充测试: tests/integration/recovery_paths.rs
- [x] T030 配置热更新与回退策略完善: src/config/config.rs, src/service/runner.rs

---

## 依赖与执行顺序

### 阶段依赖

- **初始化（Phase 1）**: 无依赖，可立即开始
- **基础能力（Phase 2）**: 依赖 Phase 1 完成，阻塞所有用户故事
- **用户故事（Phase 3+）**: 依赖 Phase 2 完成
- **打磨（最终阶段）**: 依赖所有选定用户故事完成

### 用户故事依赖

- **用户故事 1 (P1)**: Phase 2 完成后可开始，无依赖
- **用户故事 2 (P2)**: Phase 2 完成后可开始
- **用户故事 3 (P3)**: Phase 2 完成后可开始

### 每条用户故事内部顺序

- 集成测试必须先写并确保失败
- 核心策略与状态机逻辑先于平台实现
- 平台实现先于服务整合
- 完成后必须独立验证

---

## 备注

- 每条用户故事至少 1 条端到端集成测试
- 避免跨故事耦合导致不可独立验证
