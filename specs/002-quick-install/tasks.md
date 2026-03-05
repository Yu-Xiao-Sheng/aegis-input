description: "Task list for quick install feature implementation"
---

# Tasks: 快速安装与服务化运行

**Input**: Design documents from `/specs/002-quick-install/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are included for installation flow and service behavior validation

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Installation scripts**: `install/linux/`
- **Rust installer code**: `src/installer/`
- **Tests**: `tests/installation/`
- **Systemd service**: `install/linux/aegis-input.service`

---

## Phase 1: Setup (项目初始化)

**Purpose**: Project initialization and basic structure

- [ ] T001 创建安装模块的项目结构 per implementation plan
- [ ] T002 [P] 在 Cargo.toml 中添加安装相关依赖（systemd integration, tokio）
- [ ] T003 [P] 配置安装模块的 linting 和 formatting 工具

---

## Phase 2: Foundational (基础构建)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 [P] 实现 Installer 抽象接口（install/uninstall/status）在 src/installer/mod.rs
- [ ] T005 [P] 实现 Linux 安装器骨架（InstallMetadata 结构）在 src/installer/linux.rs
- [ ] T006 创建安装元数据管理（读写 /var/lib/aegis-input/install.toml）
- [ ] T007 [P] 创建 Linux 安装脚本目录结构和基础文件
- [ ] T008 实现 systemd 服务管理器包装（start/stop/status/enable/disable）
- [ ] T009 配置错误处理和日志记录基础设施

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - 一键安装并自动运行 (Priority: P1) 🎯 MVP

**Goal**: 提供一键安装体验，安装完成后服务自动运行且功能立即生效

**Independent Test**: 执行安装流程 -> 验证服务处于运行状态 -> 验证功能已启用并可响应外设接入

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T010 [P] [US1] 安装流程集成测试在 tests/installation/test_install_flow.py
- [ ] T011 [P] [US1] 服务启动后功能验证测试在 tests/installation/test_service_functionality.py

### Implementation for User Story 1

- [ ] T012 [P] [US1] 实现 InstallMetadata 序列化/反序列化在 src/installer/mod.rs
- [ ] T013 [P] [US1] 实现 InstallConfig 结构和相关方法在 src/installer/mod.rs
- [ ] T014 [US1] 实现 Linux 安装器 install 方法在 src/installer/linux.rs
- [ ] T015 [US1] 创建 install/linux/install.sh 安装脚本
- [ ] T016 [US1] 实现 systemd unit 文件 install/linux/aegis-input.service
- [ ] T017 [US1] 添加安装完成后的服务启动逻辑
- [ ] T018 [US1] 添加安装流程的错误处理和用户提示
- [ ] T019 [US1] 添加权限检查（root 用户验证）
- [ ] T020 [US1] 添加 systemd 依赖检查

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - 停止服务即关闭功能 (Priority: P2)

**Goal**: 通过停止服务来关闭功能，停止后内置设备保持可用且不再自动禁用

**Independent Test**: 停止服务 -> 验证功能关闭 -> 外设插拔不再触发禁用行为

### Tests for User Story 2

- [ ] T021 [P] [US2] 服务停止后功能验证测试在 tests/installation/test_service_stop.py
- [ ] T022 [P] [US2] 重启后服务自动启动测试在 tests/installation/test_restart.py

### Implementation for User Story 2

- [ ] T023 [US2] 实现 Linux 安装器 stop 方法在 src/installer/linux.rs
- [ ] T024 [P] [US2] 创建 install/linux/uninstall.sh 卸载脚本
- [ ] T025 [US2] 实现卸载时的服务停止和资源清理
- [ ] T026 [US2] 实现 ServiceState 结构和状态管理
- [ ] T027 [US2] 添加卸载时的系统用户和组清理
- [ ] T028 [US2] 添加卸载完成后的状态验证

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - 具备跨平台安装扩展能力 (Priority: P3)

**Goal**: 安装方案具备清晰的平台抽象，便于未来为 Windows/macOS 添加各自的安装方式

**Independent Test**: 在 Linux 上使用抽象接口完成安装流程，且接口结构不依赖具体平台细节

### Tests for User Story 3

- [ ] T029 [P] [US3] 安装抽象接口测试在 tests/installation/test_installer_abstraction.py
- [ ] T030 [P] [US3] 跨平台兼容性验证测试

### Implementation for User Story 3

- [ ] T031 [P] [US3] 抽象 Installer trait 的平台无关部分
- [ ] T032 [US3] 添加平台检测模块（判断当前运行平台）
- [ ] T033 [P] [US3] 创建 install/windows/ 和 install/macos/ 目录结构
- [ ] T034 [US3] 添加 Windows 安装器接口实现
- [ ] T035 [US3] 添加 macOS 安装器接口实现
- [ ] T036 [US3] 添加跨平台配置管理
- [ ] T037 [US3] 更新主安装器选择逻辑

**Checkpoint**: All user stories should now be independently functional

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T038 [P] 更新 AGENTS.md 文档
- [ ] T039 [P] 更新项目 README.md 中的安装说明
- [ ] T040 [P] 更新 quickstart.md 验证
- [ ] T041 [P] 添加安装流程的性能优化
- [ ] T042 代码重构和清理
- [ ] T043 添加安装日志和调试信息
- [ ] T044 [P] 添加更多的单元测试覆盖
- [ ] T045 安全加固（文件权限、路径验证）
- [ ] T046 运行完整安装流程端到端测试

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "安装流程集成测试在 tests/installation/test_install_flow.py"
Task: "服务启动后功能验证测试在 tests/installation/test_service_functionality.py"

# Launch all models for User Story 1 together:
Task: "实现 InstallMetadata 序列化/反序列化在 src/installer/mod.rs"
Task: "实现 InstallConfig 结构和相关方法在 src/installer/mod.rs"
Task: "创建 Linux 安装脚本目录结构和基础文件"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- Total task count: 46 tasks
- User story task distribution:
  - User Story 1 (P1): 11 tasks
  - User Story 2 (P2): 8 tasks
  - User Story 3 (P3): 8 tasks
  - Setup & Foundational: 9 tasks
  - Polish: 10 tasks
- Parallel opportunities identified: 19 tasks marked [P]