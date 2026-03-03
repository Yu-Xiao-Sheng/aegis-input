# 数据模型: Linux 外置设备禁用内置输入

## 实体与字段

### 设备（Device）

- **id**: 设备唯一标识（系统路径或稳定标识）
- **name**: 设备名称
- **type**: 设备类型（keyboard, pointing）
- **origin**: 设备来源（internal, external）
- **state**: 连接状态（connected, disconnected）
- **last_seen_at**: 最近一次状态变更时间

### 功能状态（FeatureState）

- **enabled**: 是否启用自动禁用功能
- **updated_at**: 最近一次开关变更时间

### 禁用策略（DisablePolicy）

- **keyboard_external_count**: 外置键盘计数
- **pointing_external_count**: 外置指点设备计数
- **keyboard_disabled**: 内置键盘是否被禁用
- **pointing_disabled**: 内置指点设备是否被禁用

## 关系

- 多个 **Device** 影响单个 **DisablePolicy**（按类型计数与禁用状态）
- **FeatureState** 控制 **DisablePolicy** 是否生效

## 规则与校验

- **origin=external** 的 Device 才计入计数
- **type=keyboard** 仅影响内置键盘禁用状态
- **type=pointing** 仅影响内置指点设备禁用状态
- 当计数从 0 -> 1 时触发禁用；当计数从 1 -> 0 时触发恢复

## 状态迁移

### FeatureState

- `disabled -> enabled`: 启用后立即计算当前外置设备计数并应用禁用规则
- `enabled -> disabled`: 立即恢复内置键盘与内置指点设备

### DisablePolicy

- `keyboard_external_count: 0 -> 1`: `keyboard_disabled = true`
- `keyboard_external_count: 1 -> 0`: `keyboard_disabled = false`
- `pointing_external_count: 0 -> 1`: `pointing_disabled = true`
- `pointing_external_count: 1 -> 0`: `pointing_disabled = false`

### Device

- `disconnected -> connected`: 触发分类与计数更新
- `connected -> disconnected`: 触发计数更新与可能的恢复逻辑
