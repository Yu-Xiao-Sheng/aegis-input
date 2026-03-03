# 实现计划: [FEATURE]

**分支**: `[###-feature-name]` | **日期**: [DATE] | **规格**: [link]
**输入**: 来自 `/specs/[###-feature-name]/spec.md` 的功能规格

**说明**: 本模板由 `/speckit.plan` 命令填充。执行流程见 `.specify/templates/plan-template.md`。

## 摘要

[从功能规格中提取: 主要需求 + 技术方案]

## 技术上下文

<!--
  需要操作: 将本节内容替换为项目实际技术细节。
  该结构用于指导迭代过程，可根据需要调整。
-->

**语言/版本**: [例如: Rust 1.75 或 NEEDS CLARIFICATION]  
**主要依赖**: [例如: udev, evdev 或 NEEDS CLARIFICATION]  
**存储**: [如适用, 例如: 文件 或 N/A]  
**测试**: [例如: cargo test 或 NEEDS CLARIFICATION]  
**目标平台**: [例如: Linux system service 或 NEEDS CLARIFICATION]
**项目类型**: [例如: system service/cli 或 NEEDS CLARIFICATION]  
**性能目标**: [领域指标, 例如: idle CPU/内存/唤醒次数 或 NEEDS CLARIFICATION]  
**约束**: [领域约束, 例如: <200ms 响应 或 NEEDS CLARIFICATION]  
**规模/范围**: [领域规模, 例如: 单机服务 或 NEEDS CLARIFICATION]

## 宪章核对

*门禁: 在 Phase 0 调研前必须通过，Phase 1 设计后复核。*

[根据宪章逐条填写必须满足的门禁项，例如: 集成测试覆盖、中文文档、低开销预算、最小权限]

## 项目结构

### 文档（本功能）

```text
specs/[###-feature]/
├── plan.md              # 本文件 (/speckit.plan 输出)
├── research.md          # Phase 0 输出 (/speckit.plan)
├── data-model.md        # Phase 1 输出 (/speckit.plan)
├── quickstart.md        # Phase 1 输出 (/speckit.plan)
├── contracts/           # Phase 1 输出 (/speckit.plan)
└── tasks.md             # Phase 2 输出 (/speckit.tasks - 非 /speckit.plan 创建)
```

### 源码结构（仓库根目录）
<!--
  需要操作: 将下方占位树替换为本功能的实际结构。
  删除未使用选项，并用真实路径展开所选结构。
  交付的计划中不得保留 Option 标签。
-->

```text
# [REMOVE IF UNUSED] Option 1: 单项目 (默认)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web 应用 (当检测到 "frontend" + "backend")
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: 移动端 + API (当检测到 "iOS/Android")
api/
└── [同 backend 结构]

ios/ 或 android/
└── [平台结构: 功能模块、UI 流程、平台测试]
```

**结构决策**: [记录所选结构，并引用上方的真实目录]

## 复杂度跟踪

> **仅在宪章核对存在违规且需要合理化时填写**

| 违规点 | 必要原因 | 放弃更简单方案的原因 |
|-----------|------------|-------------------------------------|
| [例如: 第 4 个项目] | [当前需求] | [为何 3 个项目不足] |
| [例如: 仓储模式] | [具体问题] | [为何直接访问不足] |
