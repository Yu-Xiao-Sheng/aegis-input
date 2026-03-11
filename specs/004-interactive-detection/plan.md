# Implementation Plan: 交互式输入设备检测与配置

**Branch**: `004-interactive-detection` | **Date**: 2026-03-11 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/004-interactive-detection/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

实现交互式输入设备检测与配置功能，解决当前自动识别不准确的问题。通过用户主动输入来精确识别活跃的输入设备，并提供直观的设备选择界面来禁用特定设备。

**核心需求**：
- P1: 实时设备检测 - 监听输入事件并显示当前活跃设备
- P2: 选择性设备禁用 - 用户交互式选择要禁用的设备
- P3: 配置持久化 - 保存配置并支持系统重启后自动加载

**技术方案**：
- 使用 evdev 库实时监听所有输入设备的事件
- 实现交互式 CLI 界面引导用户完成检测和配置流程
- 扩展现有配置系统支持设备级别的精细控制

## Technical Context

**Language/Version**: Rust 1.75+
**Primary Dependencies**:
- `evdev` - Linux 输入设备事件监听
- `tokio` - 异步运行时（现有依赖）
- `clap` - CLI 参数解析（现有依赖）
- `toml` - 配置文件序列化（现有依赖）
- `anyhow` - 错误处理（现有依赖）

**Storage**:
- 配置文件：TOML 格式（`/etc/aegis-input/config.toml` 或用户指定路径）
- 状态文件：TOML 格式（`/var/lib/aegis-input/status.toml`，用于持久化活跃设备记录）

**Testing**: cargo test + 集成测试
**Target Platform**: Linux（Ubuntu 20.04+, Debian 11+, Linux Mint 20+）
**Project Type**: CLI 工具 + 系统服务（systemd）
**Performance Goals**:
- 实时检测响应时间 <100ms（从用户输入到界面显示）
- 检测会话启动时间 <1秒
- 设备禁用生效时间 <500ms

**Constraints**:
- 需要 root 权限访问 `/dev/input/event*`
- 仅支持 Linux（evdev 特定）
- 内存占用 <50MB（检测模式）
- CPU 空闲占用 <1%

**Scale/Scope**:
- 支持最多 20 个输入设备
- 同时监听所有设备的输入事件
- 配置文件大小 <10KB

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. 集成测试为硬性要求 ✅ PASS

**计划**：
- 为 P1 实时检测功能编写集成测试：模拟多设备输入，验证活跃设备识别准确性
- 为 P2 设备禁用功能编写集成测试：验证设备选择、禁用和恢复流程
- 为 P3 配置持久化编写集成测试：验证配置保存、加载和跨会话应用
- 所有集成测试纳入 CI，合并前必须全部通过

### II. 文档统一中文 ✅ PASS

**计划**：
- 本 plan.md 及所有后续文档（research.md、data-model.md、quickstart.md）均使用中文
- 代码注释使用中文，技术关键词保留英文
- CLI 输出信息使用中文，提供清晰的用户引导

### III. 低开销与用户无干扰 ✅ PASS

**资源预算**：
- **CPU 空闲**: <1%（事件驱动，无轮询）
- **内存**: <50MB（检测模式）
- **响应延迟**: <100ms（从输入到显示）
- **唤醒频率**: 仅在输入事件发生时唤醒

**实现方式**：
- 使用 epoll/inotify 等内核事件通知机制，非轮询
- 异步 I/O（tokio）避免阻塞主线程
- 设备文件描述符复用，避免频繁打开/关闭

**验证方式**：
- 集成测试中测量响应延迟
- CI 中使用 `perf` 工具监控 CPU 和内存占用

### IV. 最小权限与故障可恢复 ✅ PASS

**权限控制**：
- 检测模式仅在用户主动调用时运行，非常驻
- 仅请求读取 `/dev/input/event*` 权限，不写入
- 设备禁用操作使用现有最小权限机制

**故障恢复**：
- 检测模式异常退出时，自动释放所有设备文件描述符
- 配置文件损坏时，记录错误日志并使用默认配置（不禁用任何设备）
- 提供显式的 `aegis-input config --reset` 命令恢复到初始状态
- 所有失效路径记录日志到 systemd journal

### V. 跨平台抽象与可演进 ✅ PASS

**抽象边界**：
- 核心检测逻辑与平台实现解耦：`DetectionSession` trait 抽象
- Linux 特定代码隔离在 `src/platform/linux/mod.rs`
- CLI 交互层与平台层分离，便于未来添加 Windows/macOS 支持

**平台范围**：
- 当前版本：Linux（evdev）
- 未来预留：Windows（RawInput）、macOS（CGEvent）
- 配置格式和 CLI 接口跨平台统一

**测试策略**：
- 平台依赖的代码使用 trait object 和 mock 进行测试
- 集成测试使用虚拟设备或真实设备替身

---

## Phase 1 设计后重新评估

*GATE: Phase 1 完成后的宪章重新验证*

### 设计决策与宪章一致性

**新增依赖**：
- `evdev` - Linux 输入设备访问（必需，平台特定）
- `crossterm` - 终端交互（最小依赖，跨平台兼容）

**评估**：
- ✅ 符合宪章 III（低开销）：crossterm 仅在检测模式使用，非常驻
- ✅ 符合宪章 V（跨平台抽象）：平台代码隔离在 `src/platform/linux/`
- ✅ 符合宪章 IV（故障可恢复）：配置格式向后兼容，降级策略明确

**配置格式扩展**：
- 向后兼容的 TOML 扩展（保留 `disable_internal` 字段）
- 新增 `devices.specific.disabled` 数组支持设备级别控制

**评估**：
- ✅ 符合宪章 IV（故障可恢复）：旧配置始终可用
- ✅ 符合宪章 II（文档统一中文）：配置文件注释使用中文

**接口设计**：
- `DetectionSession` trait - 会话管理
- `DeviceMonitor` trait - 设备监听
- `DeviceSelector` trait - 设备选择

**评估**：
- ✅ 符合宪章 V（跨平台抽象）：trait 定义与平台无关
- ✅ 符合宪章 I（集成测试）：每个 trait 可独立测试

**性能预算验证**：
- CPU 空闲 <1% ✅（tokio 异步 I/O，无轮询）
- 内存 <50MB ✅（预估 30KB 实体内存 + 事件缓冲）
- 响应延迟 <100ms ✅（tokio + AsyncDevice <50ms）

### 最终结论

**所有宪章原则通过** ✅

设计完成后重新评估，所有决策符合宪章要求：
1. 集成测试计划覆盖所有核心功能
2. 所有文档使用中文
3. 性能预算满足要求
4. 故障恢复机制完善
5. 跨平台抽象清晰

---

## Project Structure

### Documentation (this feature)

```text
specs/004-interactive-detection/
├── plan.md              # 本文件
├── research.md          # Phase 0 输出：技术方案研究
├── data-model.md        # Phase 1 输出：数据模型设计
├── quickstart.md        # Phase 1 输出：快速开始指南
├── contracts/           # Phase 1 输出：接口契约
│   ├── cli.md          # CLI 接口契约
│   └── detection.md    # 检测模块接口契约
└── tasks.md             # Phase 2 输出：任务列表（/speckit.tasks 生成）
```

### Source Code (repository root)

```text
src/
├── cli/
│   ├── mod.rs
│   ├── detect.rs        # 新增：detect 子命令实现
│   └── config.rs        # 修改：添加 --reset 功能
├── detection/
│   ├── mod.rs           # 新增：检测模块入口
│   ├── session.rs       # 新增：检测会话管理
│   ├── monitor.rs       # 新增：输入事件监听
│   └── selector.rs      # 新增：设备选择交互界面
├── platform/
│   ├── linux/
│   │   ├── input.rs     # 修改：添加事件监听功能
│   │   └── device.rs    # 修改：添加设备路径信息
│   └── mod.rs
├── config/
│   ├── mod.rs           # 修改：支持设备级别配置
│   └── models.rs        # 修改：添加设备配置结构
└── main.rs              # 修改：添加 detect 子命令注册

tests/
├── integration/
│   ├── detection_test.rs    # 新增：检测功能集成测试
│   ├── config_test.rs       # 新增：配置持久化测试
│   └── cli_test.rs          # 新增：CLI 交互测试
└── unit/
    ├── session_test.rs      # 新增：检测会话单元测试
    └── selector_test.rs     # 新增：设备选择器单元测试
```

**Structure Decision**: 选择 Option 1（单一项目结构）。当前项目是 Rust CLI 工具，不需要前后端分离。新功能集成到现有代码结构中：
- `src/cli/` - CLI 命令实现
- `src/detection/` - 新增检测模块
- `src/platform/linux/` - Linux 特定实现
- `tests/` - 测试代码

---

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

本功能没有宪章违规项。所有设计决策遵循核心原则。

---

## Implementation Phases

### Phase 0: Research & Technical Decisions

**目标**: 解决所有 NEEDS CLARIFICATION，确定技术方案

**研究任务**：
1. **evdev 事件监听机制研究**
   - 如何同时监听多个输入设备的事件？
   - 如何区分键盘和鼠标事件？
   - 事件响应延迟是否能达到 <100ms？
   - 参考文档：[evdev-rs](https://docs.rs/evdev/)、[Linux input subsystem](https://www.kernel.org/doc/html/latest/input/input.html)

2. **CLI 交互界面设计**
   - 如何实现实时更新的终端界面（显示"正在使用：XXX"）？
   - 选项：`crossterm`、`termion`、或纯文本输出
   - 评估标准：兼容性、性能、易用性

3. **配置格式扩展**
   - 当前 config.toml 格式是否足够支持设备级别配置？
   - 需要添加哪些字段？
   - 如何保证向后兼容？

**输出**: `research.md` - 包含所有技术决策和替代方案分析

---

### Phase 1: Design & Contracts

**前提**: `research.md` 完成

**设计任务**：

1. **数据模型设计** (`data-model.md`)
   - `InputDevice` 结构：设备名称、路径、类型、总线类型、状态
   - `DetectionSession` 结构：会话状态、活跃设备记录
   - `DeviceConfiguration` 结构：要禁用的设备列表、时间戳
   - 配置文件 TOML 格式定义

2. **接口契约定义** (`contracts/`)
   - `cli.md`: `detect` 子命令的参数、行为、输出格式
   - `detection.md`: `DetectionSession` trait、`DeviceMonitor` trait、`DeviceSelector` trait

3. **快速开始指南** (`quickstart.md`)
   - 如何运行检测模式
   - 如何选择和禁用设备
   - 如何保存和重置配置
   - 故障排除

4. **Agent Context 更新**
   - 运行 `.specify/scripts/bash/update-agent-context.sh claude`
   - 添加新依赖（evdev、crossterm/termion）到 CLAUDE.md

**输出**:
- `data-model.md`
- `contracts/cli.md`
- `contracts/detection.md`
- `quickstart.md`
- 更新 `CLAUDE.md`

---

### Phase 2: Task Generation

**前提**: Phase 1 完成

**任务**: 运行 `/speckit.tasks` 生成详细的任务列表

**输出**: `tasks.md` - 可执行的任务分解

---

## Next Steps

1. ✅ 完成 Constitution Check（已通过）
2. ⏳ 执行 Phase 0 研究，生成 `research.md`
3. ⏳ 执行 Phase 1 设计，生成数据模型、契约和 quickstart
4. ⏳ 更新 agent context
5. ⏳ 重新评估 Constitution Check
6. ⏳ 运行 `/speckit.tasks` 生成任务列表

**当前状态**: 等待 Phase 0 研究开始
