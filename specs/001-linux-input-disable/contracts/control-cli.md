# 接口契约: 本地控制 CLI

## 目的

提供用户可见的启用/禁用/状态查询接口，用于控制自动禁用功能。

## 命令

### `aegis-input enable`

- **作用**: 启用自动禁用功能
- **输入**: 无
- **输出（stdout）**: 当前状态（enabled=true）
- **错误（stderr）**: 参数错误或权限不足等
- **退出码**: 0 表示成功，非 0 表示失败

### `aegis-input disable`

- **作用**: 关闭自动禁用功能（并恢复内置设备）
- **输入**: 无
- **输出（stdout）**: 当前状态（enabled=false）
- **错误（stderr）**: 参数错误或权限不足等
- **退出码**: 0 表示成功，非 0 表示失败

### `aegis-input status`

- **作用**: 查询当前启用状态与关键统计信息
- **输入**: 无
- **输出（stdout）**:
  - enabled: true/false
  - keyboard_external_count: 数量
  - pointing_external_count: 数量
  - keyboard_disabled: true/false
  - pointing_disabled: true/false
- **错误（stderr）**: 查询失败的原因
- **退出码**: 0 表示成功，非 0 表示失败

## 行为约束

- 命令应当幂等：多次执行 `enable` 或 `disable` 不应导致异常状态
- `disable` 必须保证内置设备恢复可用
- 输出需提供人类可读文本；如需机器可读格式，后续版本可扩展 `--json`
