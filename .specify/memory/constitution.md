<!-- Sync Impact Report
- Version change: template -> 1.0.0
- Modified principles: PRINCIPLE_1_NAME 占位符 -> I. 集成测试为硬性要求; PRINCIPLE_2_NAME 占位符 -> II. 文档统一中文; PRINCIPLE_3_NAME 占位符 -> III. 低开销与用户无干扰; PRINCIPLE_4_NAME 占位符 -> IV. 最小权限与故障可恢复; PRINCIPLE_5_NAME 占位符 -> V. 跨平台抽象与可演进
- Added sections: 文档与语言规范; 研发流程与质量门禁
- Removed sections: 无
- Templates requiring updates: ✅ .specify/templates/plan-template.md; ✅ .specify/templates/spec-template.md; ✅ .specify/templates/tasks-template.md; ✅ .specify/templates/checklist-template.md; ✅ .specify/templates/agent-file-template.md; ✅ .specify/templates/constitution-template.md; ✅ README.md
- Follow-up TODOs: 无
-->
# Aegis Input Constitution

## Core Principles

### I. 集成测试为硬性要求
- 每一项功能（以 `spec.md` 的用户故事/功能需求为粒度）必须具备至少 1 条端到端集成测试覆盖关键路径。
- 集成测试必须自动化、可重复，纳入 CI，并在合并/发布前全部通过。
- 若功能涉及设备接入/移除或状态迁移，集成测试必须覆盖该流程与恢复路径。
- 理由: 真实设备组合复杂，端到端验证能显著降低回归风险。

### II. 文档统一中文
- 所有 spec、plan、tasks、checklist、技术方案、运维/部署文档、README 等必须使用中文撰写。
- 仅允许对技术关键词、变量名、命令、路径、API 名称、专有名词使用英文。
- 引用英文资料时必须提供中文摘要与结论。
- 理由: 统一沟通语言，降低协作与维护成本。

### III. 低开销与用户无干扰
- 运行必须事件驱动，禁止无界轮询或高频定时器造成持续唤醒。
- 每个功能规格必须给出明确的资源预算（如 idle CPU、内存、唤醒次数）与验证方式。
- 任一变更若导致用户输入响应变慢或资源超出预算，必须修复后方可合并。
- 理由: 本项目为常驻服务，资源占用直接影响用户体验。

### IV. 最小权限与故障可恢复
- 仅访问完成需求所必需的设备节点与系统接口，默认不要求 root 权限。
- 在异常、退出或设备变更时必须释放抓取并恢复内置设备可用性。
- 所有失效路径必须有可观测日志与可重复的恢复步骤。
- 理由: 最小权限降低安全风险，可恢复性避免输入被意外锁死。

### V. 跨平台抽象与可演进
- 核心逻辑必须与平台实现解耦，平台差异仅存在于 `platform/*` 等明确边界内。
- 新功能不得破坏抽象边界，必须提供平台适配层的测试或替身实现。
- 规格文档必须明确平台范围（Linux 现行、Windows/macOS 预留）及兼容性影响。
- 理由: 未来扩展到 Windows/macOS 需要稳定的抽象边界。

## 文档与语言规范

- 新增或更新的文档必须为中文，标题、正文、图表注释均遵循该规则。
- 技术关键词、变量名、命令、路径、API 名称与专有名词可保留英文并使用代码样式标注。
- 引用英文资料时必须提供中文要点摘要，并标明来源。
- 文档中出现的性能预算与测试计划必须与对应规格保持一致。

## 研发流程与质量门禁

- 每个 spec 必须明确集成测试范围与通过标准，并在 tasks 中落实到具体测试任务。
- plan 必须包含“宪章核对”并逐条确认与本宪章一致。
- CI 必须执行集成测试与资源预算验证，任一失败都阻止合并。
- 代码评审必须检查宪章遵循情况，任何例外必须先修订本宪章再合并。

## Governance

- 本宪章为最高约束，若与其他文档冲突，以本宪章为准。
- 修订必须通过 PR 提交，包含变更说明、影响评估与必要的迁移/回滚方案，并获得维护者批准。
- 版本策略遵循语义化版本：MAJOR 用于原则移除或不兼容变更，MINOR 用于新增原则或实质性扩展，PATCH 用于澄清与文字修订。
- 合规审查为必经流程：spec/plan/tasks 必须含宪章核对，评审需逐条确认。

**Version**: 1.0.0 | **Ratified**: 2026-03-02 | **Last Amended**: 2026-03-02
