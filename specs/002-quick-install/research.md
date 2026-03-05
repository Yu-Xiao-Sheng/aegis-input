# 调研记录: 快速安装与服务化运行

## 决策 1: Linux 快速安装方式

**Decision**: 使用 Shell 安装脚本作为入口，完成二进制部署与 systemd 服务创建。

**Rationale**: 兼容性最好、依赖最少，符合“快速安装 ≤ 3 步”的目标。

**Alternatives considered**:
- 使用发行版包管理器（deb/rpm）: 体验稳定但维护成本高，前期不利于快速迭代
- 使用图形安装器: 不适合服务类工具

## 决策 2: systemd 服务启停即功能开关

**Decision**: systemd 服务运行即启用，服务停止即关闭功能并恢复内置设备。

**Rationale**: 符合需求，控制方式简单明确。

**Alternatives considered**:
- 保留应用内开关: 与“服务运行即启用”的需求冲突

## 决策 3: 最小权限运行方式

**Decision**: 默认创建 `aegis-input` 系统用户并加入 `input` 组，服务以该用户运行。

**Rationale**: 满足最小权限原则，同时避免必须使用 root 运行服务。

**Alternatives considered**:
- 服务以 root 运行: 实现简单但违反最小权限原则

## 决策 4: 配置与状态路径

**Decision**: systemd unit 设置 `AEGIS_INPUT_CONFIG` 与 `AEGIS_INPUT_STATUS`，使用 `/etc/aegis-input/config.toml` 与 `/var/lib/aegis-input/status.toml`。

**Rationale**: 与系统服务运行方式一致，避免依赖用户 HOME。

**Alternatives considered**:
- 使用 `/root/.config` 或用户目录: 不适合系统级服务

## 决策 5: 跨平台安装抽象

**Decision**: 定义 Installer 抽象接口（install/uninstall/status），Linux 为首个实现。

**Rationale**: 保证未来扩展 Windows/macOS 时不影响业务逻辑。

**Alternatives considered**:
- 直接写死 Linux 脚本: 未来扩展成本高

## 决策 6: 卸载与回退策略

**Decision**: 提供 uninstall 脚本，卸载时删除 unit 文件、移除系统用户、清理安装目录与元数据。

**Rationale**: 避免残留无效服务，降低系统污染。

**Alternatives considered**:
- 不提供卸载: 用户体验差且违背需求
