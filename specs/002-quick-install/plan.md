# 实现计划: 快速安装与服务化运行

**分支**: `002-quick-install` | **日期**: 2026-03-03 | **规格**: /home/yuxs/github_project/aegis-input/specs/002-quick-install/spec.md
**输入**: 来自 /home/yuxs/github_project/aegis-input/specs/002-quick-install/spec.md 的功能规格

**说明**: 本模板由 `/speckit.plan` 命令填充。执行流程见 /home/yuxs/github_project/aegis-input/.specify/templates/plan-template.md。

## 摘要

本功能提供 Linux 平台的快速安装与服务化运行能力，安装后自动创建并启动 systemd 服务，服务运行即功能启用，停止服务即功能关闭。设计包含跨平台安装抽象边界，便于未来扩展 Windows/macOS 的安装方式。

## 技术上下文

**语言/版本**: Rust（stable, 2024 edition）  
**主要依赖**: systemd 服务单元、Shell 安装脚本、当前 Rust 运行时  
**存储**: systemd unit 文件、安装元数据（安装路径/版本/平台）  
**测试**: cargo test + 安装流程端到端集成测试  
**目标平台**: Linux systemd service
**项目类型**: system service + 安装器  
**性能目标**: 安装步骤 ≤ 3 步，首次安装 ≤ 3 分钟  
**约束**: 最小权限、可卸载回退、跨平台抽象边界清晰  
**规模/范围**: 单机安装与服务管理

**Language/Version**: Rust（stable, 2024 edition）  
**Primary Dependencies**: systemd, Shell install script  
**Storage**: systemd unit + install metadata  
**Project Type**: system service + installer

## 宪章核对

*门禁: 在 Phase 0 调研前必须通过，Phase 1 设计后复核。*

- 集成测试为硬性要求: 安装流程、服务启动/停止必须具备端到端测试
- 文档统一中文: 安装说明、运维与快速开始均使用中文
- 低开销与用户无干扰: 安装流程不引入常驻高开销逻辑
- 最小权限与故障可恢复: 安装默认采用最小权限，卸载/回退必须恢复系统状态
- 跨平台抽象与可演进: 安装接口抽象，Linux 为首个实现

**Phase 1 复核结论**: 设计产物中保持以上约束，无需宪章例外

## 项目结构

### 文档（本功能）

```text
/home/yuxs/github_project/aegis-input/specs/002-quick-install/
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
install/
└── linux/
    ├── install.sh
    ├── uninstall.sh
    └── aegis-input.service

src/
├── installer/
│   ├── mod.rs           # 安装抽象接口
│   └── linux.rs         # Linux 安装实现
└── service/             # 现有服务运行逻辑
```

**结构决策**: 在 `src/installer` 中建立跨平台安装抽象，Linux 安装脚本与 unit 文件放在 `install/linux`。

## 复杂度跟踪

无。当前计划无宪章违规项。
