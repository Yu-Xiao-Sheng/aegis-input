# 实现计划: Linux 外置设备禁用内置输入

**分支**: `001-linux-input-disable` | **日期**: 2026-03-02 | **规格**: /home/yuxs/github_project/aegis-input/specs/001-linux-input-disable/spec.md
**输入**: 来自 /home/yuxs/github_project/aegis-input/specs/001-linux-input-disable/spec.md 的功能规格

**说明**: 本模板由 `/speckit.plan` 命令填充。执行流程见 /home/yuxs/github_project/aegis-input/.specify/templates/plan-template.md。

## 摘要

本功能面向 Linux 平台，实现可开启/关闭的自动禁用逻辑：当检测到外置键盘时禁用内置键盘，当检测到外置鼠标时禁用内置指点设备；按类型独立生效，多个同类型外设存在时仅在全部移除后恢复。设计保持低开销、最小权限与可恢复，并预留 Windows/macOS 扩展边界。

## 技术上下文

**语言/版本**: Rust（stable, 2024 edition）  
**主要依赖**: udev, evdev, serde, toml, tracing, tracing-subscriber  
**存储**: 本地配置文件（toml），用于启用状态与设备规则  
**测试**: cargo test + 端到端集成测试（设备插拔/禁用恢复路径）  
**目标平台**: Linux systemd service
**项目类型**: system service + 可选 CLI  
**性能目标**: 空闲 CPU ≤ 1%，内存 ≤ 50 MB；仅在设备变更时唤醒  
**约束**: 事件驱动、最小权限、设备异常可恢复、保持跨平台抽象边界  
**规模/范围**: 单机输入设备管理

**Language/Version**: Rust（stable, 2024 edition）  
**Primary Dependencies**: udev, evdev, serde, toml, tracing, tracing-subscriber  
**Storage**: 本地配置文件（toml）  
**Project Type**: system service + 可选 CLI

## 宪章核对

*门禁: 在 Phase 0 调研前必须通过，Phase 1 设计后复核。*

- 集成测试为硬性要求: 已在规格中为每条用户故事定义端到端集成测试，计划在 tests/integration 中落地并进入 CI
- 文档统一中文: 计划产出文档全部中文，英文仅用于技术关键词与命令
- 低开销与用户无干扰: 性能目标已明确（CPU/内存/唤醒），采用事件驱动模型
- 最小权限与故障可恢复: 仅访问必要设备节点，异常/设备变更需恢复内置设备可用性
- 跨平台抽象与可演进: 设计保留 platform 边界，规则与策略在核心层保持稳定

**Phase 1 复核结论**: 设计产物中保持以上约束，无需宪章例外

## 项目结构

### 文档（本功能）

```text
/home/yuxs/github_project/aegis-input/specs/001-linux-input-disable/
├── plan.md              # 本文件
├── research.md          # Phase 0 输出
├── data-model.md        # Phase 1 输出
├── quickstart.md        # Phase 1 输出
├── contracts/           # Phase 1 输出
└── tasks.md             # Phase 2 输出 (/speckit.tasks 创建)
```

### 源码结构（仓库根目录）

```text
/home/yuxs/github_project/aegis-input/
src/
├── core/                # 设备分类、状态机、禁用策略（平台无关）
├── platform/
│   ├── mod.rs           # 平台抽象接口
│   └── linux/           # Linux 实现（检测、禁用、恢复）
├── config/              # 配置解析与校验
├── service/             # 运行入口与守护进程封装
└── cli/                 # 启用/禁用/状态查询

tests/
├── integration/         # 端到端集成测试
└── unit/                # 单元测试
```

**结构决策**: 单项目结构，平台相关实现集中在 `src/platform/linux`，核心逻辑位于 `src/core`，以保证跨平台扩展边界清晰。

## 复杂度跟踪

无。当前计划无宪章违规项。
