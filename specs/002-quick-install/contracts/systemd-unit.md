# 接口契约: systemd 服务单元

## 目的

定义服务在系统启动时自动运行，并以“运行即启用”的方式控制功能状态。

## 关键字段

- **Unit**: 描述与依赖
- **Service**:
  - `ExecStart`: 启动命令（运行守护进程）
  - `User`/`Group`: 最小权限运行用户与组（需包含 `input`）
  - `Environment`: `AEGIS_INPUT_CONFIG` 与 `AEGIS_INPUT_STATUS`
  - `Restart`: 异常退出后自动重启
- **Install**:
  - `WantedBy=multi-user.target`

## 行为约束

- 服务运行即功能启用
- 停止服务必须关闭功能并恢复内置设备
- 支持 `systemctl start/stop/status/enable/disable`
