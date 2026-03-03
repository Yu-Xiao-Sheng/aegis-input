# Aegis Input

Aegis Input 是一个低开销系统服务，当检测到外置键盘或鼠标时自动禁用笔记本内置键盘与触摸板。初始目标平台为 Linux（已在 Linux Mint 上验证），并保留清晰的抽象层以支持 Windows 和 macOS。

**状态**
已实现 Linux 版本的核心功能，支持启用/禁用与按类型独立的自动禁用逻辑。

**关键目标**
- 外置键盘或鼠标存在时自动禁用内置键盘与触摸板
- 外置设备移除后恢复内置设备
- 以系统服务形式运行并提供可靠日志
- 采用 Linux 优先、可扩展的跨平台架构

**文档**
- 实现方案与设计: `docs/implementation.md`

**构建**
```bash
cargo build
```

**运行（本地验证）**
建议先使用临时配置路径，避免权限影响系统配置:
```bash
export AEGIS_INPUT_CONFIG=/tmp/aegis-input-config.toml
export AEGIS_INPUT_STATUS=/tmp/aegis-input-status.toml
```

启用功能并运行服务:
```bash
./target/debug/aegis-input enable
RUST_LOG=info ./target/debug/aegis-input run
```

若提示权限不足访问 `/dev/input`，可二选一:
1. 将当前用户加入 `input` 组并重新登录
```bash
sudo usermod -aG input $USER
```
2. 临时使用 sudo 运行（保留环境变量）
```bash
sudo -E RUST_LOG=info ./target/debug/aegis-input run
```

**CLI**
```bash
./target/debug/aegis-input enable
./target/debug/aegis-input disable
./target/debug/aegis-input status
```

**systemd（计划）**
将提供 systemd 服务与可选 CLI。服务模型见 `docs/implementation.md`。

**名称**
Aegis 意为“盾”，体现目标: 在使用外置外设时防止内置设备误触。
