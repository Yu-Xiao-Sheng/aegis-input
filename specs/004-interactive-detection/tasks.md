# Implementation Tasks: 交互式输入设备检测与配置

**Feature**: 004-interactive-detection
**Branch**: `004-interactive-detection`
**Generated**: 2026-03-11
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)

---

## 目录

- [Phase 1: Setup](#phase-1-setup)
- [Phase 2: Foundational](#phase-2-foundational)
- [Phase 3: User Story 1 - 实时设备检测 (P1)](#phase-3-user-story-1---实时设备检测-p1)
- [Phase 4: User Story 2 - 选择性设备禁用配置 (P2)](#phase-4-user-story-2---选择性设备禁用配置-p2)
- [Phase 5: User Story 3 - 配置持久化与重新检测 (P3)](#phase-5-user-story-3---配置持久化与重新检测-p3)
- [Phase 6: Polish & Cross-Cutting Concerns](#phase-6-polish--cross-cutting-concerns)
- [Dependencies](#dependencies)
- [Parallel Execution Examples](#parallel-execution-examples)
- [Implementation Strategy](#implementation-strategy)

---

## Phase 1: Setup

**目标**: 初始化项目结构，添加新依赖

### 依赖配置

- [ ] T001 添加 evdev 依赖到 Cargo.toml
- [ ] T002 添加 crossterm 依赖到 Cargo.toml
- [ ] T003 [P] 添加 chrono 依赖到 Cargo.toml（用于时间戳）
- [ ] T004 [P] 添加 thiserror 依赖到 Cargo.toml（用于错误类型）

### 目录结构创建

- [ ] T005 创建 src/detection/ 目录及 mod.rs
- [ ] T006 [P] 创建 src/detection/session.rs
- [ ] T007 [P] 创建 src/detection/monitor.rs
- [ ] T008 [P] 创建 src/detection/selector.rs
- [ ] T009 创建 tests/integration/detection_test.rs
- [ ] T010 [P] 创建 tests/integration/config_test.rs
- [ ] T011 [P] 创建 tests/unit/session_test.rs
- [ ] T012 [P] 创建 tests/unit/selector_test.rs

**完成标准**: 所有目录创建完成，依赖添加到 Cargo.toml

---

## Phase 2: Foundational

**目标**: 实现所有用户故事的共享基础组件

### 数据模型实现

- [ ] T013 实现 InputDevice 结构体及枚举（DeviceType, BusType）
- [ ] T014 [P] 实现 DetectionSession 结构体及枚举（SessionState）
- [ ] T015 [P] 实现 DeviceConfiguration 结构体及 DeviceRef
- [ ] T016 实现 InputDevice 验证逻辑（非空检查、路径验证）
- [ ] T017 [P] 实现 DeviceConfiguration 序列化/反序列化（TOML）

### 错误类型定义

- [ ] T018 定义 DetectionError 枚举
- [ ] T019 [P] 实现 DetectionError 的 Display 和 Error trait
- [ ] T020 添加错误转换到 anyhow::Error

### 配置格式扩展

- [ ] T021 扩展 Config 结构支持 devices.specific.disabled 字段
- [ ] T022 [P] 实现配置向后兼容逻辑（旧配置升级）
- [ ] T023 实现设备引用解析逻辑（路径/名称匹配）
- [ ] T024 [P] 添加配置验证函数

**完成标准**: 数据模型可序列化，配置可读写，错误类型定义完整

---

## Phase 3: User Story 1 - 实时设备检测 (P1)

**目标**: 用户运行 detect 命令，程序实时显示当前活跃的输入设备

**独立测试标准**: 运行检测命令，在不同设备上输入，验证程序正确显示当前使用的设备名称

### CLI 入口点

- [ ] T025 [US1] 在 src/main.rs 中注册 detect 子命令
- [ ] T026 [US1] 实现 src/cli/detect.rs 基本框架（参数解析）
- [ ] T027 [US1] 实现 --timeout 参数处理
- [ ] T028 [US1] [P] 实现 --output 参数处理（auto/json/plain）
- [ ] T029 [US1] [P] 实现 --config-only 参数处理

### 设备扫描与显示

- [ ] T030 [US1] 实现扫描 /dev/input/event* 设备函数
- [ ] T031 [US1] 实现设备类型过滤（仅键盘和鼠标）
- [ ] T032 [US1] 实现设备信息提取（名称、路径、总线类型）
- [ ] T033 [US1] [P] 实现设备列表格式化输出（编号、类型、总线）
- [ ] T034 [US1] 实现终端初始化和清屏（crossterm）

### 设备监听

- [ ] T035 [US1] 实现 DeviceMonitor trait（LinuxDetectionDeviceMonitor）
- [ ] T036 [US1] 实现异步设备文件打开（AsyncDevice::new）
- [ ] T037 [US1] 实现事件监听循环（tokio::select!）
- [ ] T038 [US1] [P] 实现键盘事件识别
- [ ] T039 [US1] [P] 实现鼠标事件识别
- [ ] T040 [US1] 实现活跃设备记录（HashSet<PathBuf>）

### 实时显示更新

- [ ] T041 [US1] 实现实时界面更新（清除当前行、显示设备名）
- [ ] T042 [US1] 实现空闲状态显示（"等待输入..."）
- [ ] T043 [US1] [P] 实现活跃设备计数显示
- [ ] T044 [US1] 实现超时处理（默认 300 秒）

### 会话管理

- [ ] T045 [US1] 实现 DetectionSession trait（LinuxDetectionSession）
- [ ] T046 [US1] 实现会话启动（start 方法）
- [ ] T047 [US1] 实现会话等待（wait 方法）
- [ ] T048 [US1] [P] 实现用户中断处理（Ctrl+C）
- [ ] T049 [US1] 实现会话取消（cancel 方法）
- [ ] T050 [US1] 实现活跃设备查询（active_devices 方法）

### 集成测试

- [ ] T051 [US1] 编写检测流程集成测试（多设备输入）
- [ ] T052 [US1] [P] 编写超时处理测试
- [ ] T053 [US1] [P] 编写 JSON 输出测试
- [ ] T054 [US1] 编写响应延迟测试（<100ms）

**完成标准**: 用户可运行 detect 命令，输入时实时显示设备名，按 Ctrl+C 退出并显示检测总结

---

## Phase 4: User Story 2 - 选择性设备禁用配置 (P2)

**目标**: 检测完成后，用户选择要禁用的设备，程序立即禁用并保存配置

**独立测试标准**: 验证程序正确显示设备列表，接受用户输入，禁用选中设备

### 设备选择界面

- [ ] T055 [US2] 实现设备列表显示（标记活跃/未活跃设备）
- [ ] T056 [US2] 实现用户输入提示（编号、all、none）
- [ ] T057 [US2] 实现输入解析逻辑（逗号分隔编号）
- [ ] T058 [US2] [P] 实现 "all" 选项处理
- [ ] T059 [US2] [P] 实现 "none" 选项处理
- [ ] T060 [US2] 实现输入验证（编号有效性检查）

### DeviceSelector 实现

- [ ] T061 [US2] 实现 DeviceSelector trait（CliDeviceSelector）
- [ ] T062 [US2] 实现 prompt_selection 方法
- [ ] T063 [US2] [P] 实现 validate_selection 方法
- [ ] T064 [US2] 实现设备路径验证
- [ ] T065 [US2] [P] 实现设备禁用支持检查（supports_disable）

### 确认与保存

- [ ] T066 [US2] 实现确认提示（显示选中设备列表）
- [ ] T067 [US2] 实现临时禁用选项（不保存配置）
- [ ] T068 [US2] [P] 实现 save_prompt 方法
- [ ] T069 [US2] 实现配置确认处理（y/n）

### 设备禁用

- [ ] T070 [US2] 调用现有设备禁用逻辑（复用 run 命令）
- [ ] T071 [US2] 实现设备禁用确认消息
- [ ] T072 [US2] [P] 实现设备禁用失败处理

### 配置生成

- [ ] T073 [US2] 实现 DeviceConfiguration 生成（from_selection）
- [ ] T074 [US2] [P] 实现设备引用创建（DeviceRef with path/name）
- [ ] T075 [US2] 实现配置时间戳记录
- [ ] T076 [US2] 实现配置写入到文件（/etc/aegis-input/config.toml）

### 集成测试

- [ ] T077 [US2] 编写设备选择流程测试（1,3）
- [ ] T078 [US2] [P] 编写 "all" 选项测试
- [ ] T079 [US2] [P] 编写临时禁用测试
- [ ] T080 [US2] 编写配置保存测试

**完成标准**: 用户可选择设备，程序禁用并保存配置

---

## Phase 5: User Story 3 - 配置持久化与重新检测 (P3)

**目标**: 配置保存后系统重启自动加载，支持重新检测和配置重置

**独立测试标准**: 保存配置，重启服务，验证配置正确加载和应用

### 配置重置命令

- [ ] T081 [US3] 在 src/main.rs 中注册 config 子命令
- [ ] T082 [US3] 实现 --reset 参数处理
- [ ] T083 [US3] 实现配置文件删除逻辑
- [ ] T084 [US3] [P] 实现配置重置确认提示
- [ ] T085 [US3] 实现服务重启提示

### 配置加载与验证

- [ ] T086 [US3] 实现配置加载逻辑（启动时）
- [ ] T087 [US3] 实现配置版本检查
- [ ] T088 [US3] [P] 实现设备引用解析（启动时）
- [ ] T089 [US3] 实现配置验证（validate_config）
- [ ] T090 [US3] 实现配置损坏处理（降级到默认配置）

### 服务启动集成

- [ ] T091 [US3] 在服务启动时加载配置
- [ ] T092 [US3] 实现配置应用逻辑（禁用设备）
- [ ] T093 [US3] [P] 实现配置应用失败处理
- [ ] T094 [US3] 实现配置加载日志（journalctl）

### 重新检测支持

- [ ] T095 [US3] 实现检测前显示当前配置
- [ ] T096 [US3] 实现配置覆盖提示
- [ ] T097 [US3] [P] 实现配置备份（覆盖前）
- [ ] T098 [US3] 实现检测会话与配置集成（complete 方法）

### 状态管理

- [ ] T099 [US3] 扩展 Status 结构支持设备配置
- [ ] T100 [US3] [P] 实现状态文件写入（/var/lib/aegis-input/status.toml）
- [ ] T101 [US3] 实现状态查询命令（status）

### 集成测试

- [ ] T102 [US3] 编写配置保存和加载测试
- [ ] T103 [US3] [P] 编写服务重启测试
- [ ] T104 [US3] [P] 编写配置损坏恢复测试
- [ ] T105 [US3] 编写配置重置测试

**完成标准**: 配置持久化，重启后自动加载，支持重新检测和重置

---

## Phase 6: Polish & Cross-Cutting Concerns

**目标**: 完善错误处理、日志、文档、性能优化

### 错误处理完善

- [ ] T106 实现所有错误场景的错误消息（中文）
- [ ] T107 [P] 实现权限不足错误处理
- [ ] T108 [P] 实现设备未找到错误处理
- [ ] T109 实现配置写入失败错误处理

### 日志记录

- [ ] T110 添加结构化日志（tracing）
- [ ] T111 [P] 实现调试日志（设备事件）
- [ ] T112 实现错误日志到 systemd journal
- [ ] T113 [P] 实现性能日志（响应时间）

### 文档更新

- [ ] T114 更新 README.md 添加 detect 命令说明
- [ ] T115 [P] 更新 INSTALLATION.md 添加新功能说明
- [ ] T116 添加代码注释（中文）
- [ ] T117 [P] 编写 CHANGELOG 条目

### 性能优化

- [ ] T118 实现响应延迟监控（<100ms 验证）
- [ ] T119 [P] 实现 CPU 占用监控（<1% 验证）
- [ ] T120 实现内存占用监控（<50MB 验证）

### 单元测试完善

- [ ] T121 为 DetectionSession 补充单元测试
- [ ] T122 [P] 为 DeviceMonitor 补充单元测试
- [ ] T123 [P] 为 DeviceSelector 补充单元测试
- [ ] T124 为配置解析补充单元测试

### 最终集成测试

- [ ] T125 编写端到端测试（完整流程）
- [ ] T126 [P] 编写多设备场景测试
- [ ] T127 [P] 编写边界条件测试（热插拔、断开）
- [ ] T128 验证所有集成测试通过

**完成标准**: 所有测试通过，文档完整，性能满足要求

---

## Dependencies

### 任务依赖关系

**Phase 1 (Setup)**:
- T001-T004 无依赖，可并行执行
- T005-T012 依赖 T001-T004

**Phase 2 (Foundational)**:
- T013-T017 无依赖，可部分并行
- T018-T020 依赖 T013
- T021-T024 依赖 T015, T017

**Phase 3 (P1 - 实时检测)**:
- T025-T029 依赖 Phase 2 完成
- T030-T034 依赖 T025
- T035-T040 依赖 T030, T018
- T041-T044 依赖 T040
- T045-T050 依赖 T044, T035
- T051-T054 依赖 T050

**Phase 4 (P2 - 设备选择)**:
- T055-T065 依赖 Phase 3 完成
- T066-T072 依赖 T061
- T073-T076 依赖 T066
- T077-T080 依赖 T076

**Phase 5 (P3 - 配置持久化)**:
- T081-T085 依赖 Phase 4 完成
- T086-T090 依赖 T076
- T091-T094 依赖 T090
- T095-T098 依赖 T094
- T099-T101 依赖 T086
- T102-T105 依赖 T101

**Phase 6 (Polish)**:
- T106-T109 依赖 Phase 5 完成
- T110-T113 依赖 T106
- T114-T117 无依赖（可提前开始）
- T118-T120 依赖 Phase 3 完成
- T121-T124 依赖各自对应模块
- T125-T128 依赖所有之前阶段完成

### 关键路径

**MVP 最小路径** (仅 P1):
T001-T004 → T013-T020 → T025-T034 → T035-T050 → T051-T054

**完整功能路径**:
Setup → Foundational → P1 → P2 → P3 → Polish

---

## Parallel Execution Examples

### Phase 1 并行执行

```bash
# 可以同时执行的依赖添加任务
T001: 添加 evdev
T002: 添加 crossterm
T003: 添加 chrono
T004: 添加 thiserror

# 可以同时创建的文件
T006: 创建 session.rs
T007: 创建 monitor.rs
T008: 创建 selector.rs
T010: 创建 config_test.rs
T011: 创建 session_test.rs
T012: 创建 selector_test.rs
```

### Phase 2 并行执行

```bash
# 可以同时实现的数据模型
T014: 实现 DetectionSession
T015: 实现 DeviceConfiguration
T017: 实现 DeviceConfiguration 序列化

# 可以同时实现的错误处理
T019: 实现 Display/Error
T020: 添加错误转换
```

### Phase 3 并行执行

```bash
# 可以同时实现的参数处理
T028: 实现 --output 参数
T029: 实现 --config-only 参数

# 可以同时实现的事件识别
T038: 实现键盘事件识别
T039: 实现鼠标事件识别

# 可以同时实现的显示逻辑
T043: 实现活跃设备计数
T044: 实现超时处理
```

---

## Implementation Strategy

### MVP 范围（最小可行产品）

**包含**: Phase 1 + Phase 2 + Phase 3（仅 P1）

**交付内容**:
- 用户可运行 `aegis-input detect`
- 实时显示当前活跃的输入设备
- 按 Ctrl+C 退出并显示检测总结

**不包含**:
- 设备选择和禁用（P2）
- 配置持久化（P3）

**预计时间**: T001-T054（54 个任务）

### 增量交付策略

**Sprint 1**: 完成基础检测（T001-T054）
- 交付可用的检测命令
- 验证核心功能和性能

**Sprint 2**: 添加设备选择（T055-T080）
- 交付完整的配置流程
- 验证用户交互体验

**Sprint 3**: 配置持久化（T081-T105）
- 交付完整的持久化功能
- 验证服务集成

**Sprint 4**: 完善与优化（T106-T128）
- 完善错误处理和日志
- 优化性能
- 完成所有测试和文档

### 测试策略

**单元测试**: 每个模块完成后立即编写
- T121-T124: 单元测试补充

**集成测试**: 每个用户故事完成后编写
- T051-T054: P1 集成测试
- T077-T080: P2 集成测试
- T102-T105: P3 集成测试

**端到端测试**: 所有功能完成后编写
- T125-T128: 完整流程测试

---

## Summary

**总任务数**: 128 个任务

**分布**:
- Phase 1 (Setup): 12 个任务
- Phase 2 (Foundational): 12 个任务
- Phase 3 (P1): 30 个任务
- Phase 4 (P2): 26 个任务
- Phase 5 (P3): 25 个任务
- Phase 6 (Polish): 23 个任务

**可并行任务**: 约 40 个任务（标记为 [P]）

**关键路径**: 60-70 个任务（串行执行）

**预计 MVP 完成时间**: T001-T054（54 个任务）

**完整功能完成时间**: T001-T128（128 个任务）

---

## Next Steps

1. ✅ 规格、计划、研究完成
2. ✅ 任务列表生成完成
3. ⏳ 开始实现：`/speckit.implement`

**当前状态**: 准备开始实现

---

**生成时间**: 2026-03-11
**文档版本**: 1.0
