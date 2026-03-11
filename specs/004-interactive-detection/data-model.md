# Data Model: 交互式输入设备检测与配置

**Feature**: 004-interactive-detection
**Created**: 2026-03-11
**Status**: Draft

## 概述

本文档定义交互式检测与配置功能涉及的核心数据实体和关系。

---

## 核心实体

### 1. InputDevice（输入设备）

表示系统中的一个输入设备（键盘或鼠标）。

**字段**：

| 字段 | 类型 | 描述 | 示例 |
|------|------|------|------|
| `name` | `String` | 设备名称（来自 evdev） | `"AT Translated Set 2 keyboard"` |
| `path` | `PathBuf` | 设备文件路径 | `"/dev/input/event0"` |
| `device_type` | `DeviceType` | 设备类型（键盘/鼠标） | `DeviceType::Keyboard` |
| `bus_type` | `BusType` | 总线类型（USB/蓝牙/内置） | `BusType::Usb` |
| `vendor_id` | `Option<u16>` | 供应商 ID | `Some(0x046d)` |
| `product_id` | `Option<u16>` | 产品 ID | `Some(0xc52b)` |
| `phys` | `Option<String>` | 物理连接位置 | `"usb-0000:00:14.0-1/input0"` |
| `enabled` | `bool` | 当前是否启用 | `true` |
| `supports_disable` | `bool` | 是否支持禁用 | `true` |

**Rust 定义**：
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDevice {
    pub name: String,
    pub path: PathBuf,
    pub device_type: DeviceType,
    pub bus_type: BusType,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub phys: Option<String>,
    pub enabled: bool,
    pub supports_disable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Keyboard,
    Mouse,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusType {
    Usb,
    Bluetooth,
    Ps2,
    I2c,
    Platform,
    Unknown,
}
```

**验证规则**：
- `name` 非空
- `path` 必须是绝对路径且存在
- `device_type` 不能是 `Unknown`（除非无法识别）
- `enabled` 默认为 `true`

---

### 2. DetectionSession（检测会话）

表示一次交互式检测过程。

**字段**：

| 字段 | 类型 | 描述 | 示例 |
|------|------|------|------|
| `session_id` | `String` | 会话唯一 ID | `"detect-20260311-150000"` |
| `start_time` | `DateTime<Utc>` | 会话开始时间 | `2026-03-11T15:00:00Z` |
| `end_time` | `Option<DateTime<Utc>>` | 会话结束时间 | `Some(2026-03-11T15:00:30Z)` |
| `all_devices` | `Vec<InputDevice>` | 所有检测到的设备 | `[device1, device2, ...]` |
| `active_devices` | `HashSet<PathBuf>` | 检测期间活跃的设备路径 | `{event0, event3}` |
| `state` | `SessionState` | 会话状态 | `SessionState::Running` |
| `user_selection` | `Option<HashSet<PathBuf>>` | 用户选择的要禁用的设备 | `Some({event0})` |

**Rust 定义**：
```rust
#[derive(Debug, Clone)]
pub struct DetectionSession {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub all_devices: Vec<InputDevice>,
    pub active_devices: HashSet<PathBuf>,
    pub state: SessionState,
    pub user_selection: Option<HashSet<PathBuf>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Initializing,
    Running,
    UserSelection,
    Completed,
    Failed(String),
}
```

**状态转换**：
```
Initializing → Running → UserSelection → Completed
                          ↓
                       Failed
```

**验证规则**：
- `session_id` 格式：`detect-YYYYMMDD-HHMMSS`
- `all_devices` 非空（至少有一个输入设备）
- `active_devices` 是 `all_devices` 的子集

---

### 3. DeviceConfiguration（设备配置）

表示用户选择要禁用的设备配置。

**字段**：

| 字段 | 类型 | 描述 | 示例 |
|------|------|------|------|
| `version` | `String` | 配置版本 | `"1.0"` |
| `disabled_devices` | `Vec<DeviceRef>` | 要禁用的设备引用 | `[ref1, ref2]` |
| `created_at` | `DateTime<Utc>` | 配置创建时间 | `2026-03-11T15:01:00Z` |
| `updated_at` | `DateTime<Utc>` | 配置更新时间 | `2026-03-11T15:01:00Z` |

**Rust 定义**：
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfiguration {
    pub version: String,
    pub disabled_devices: Vec<DeviceRef>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRef {
    // 路径匹配（优先）
    pub path: Option<PathBuf>,

    // 名称匹配（后备）
    pub name: Option<String>,

    // 验证信息（用于警告）
    pub verified_at: Option<DateTime<Utc>>,
}
```

**匹配逻辑**：
1. 如果 `path` 存在且设备存在 → 匹配
2. 如果 `name` 存在且设备名称匹配 → 匹配
3. 否则 → 不匹配（记录警告）

**验证规则**：
- `version` 遵循语义化版本
- `disabled_devices` 最多 20 个
- 每个 `DeviceRef` 至少有 `path` 或 `name` 之一

---

## TOML 配置格式

### 配置文件结构

```toml
# 版本信息（新增）
version = "1.0"

# 现有配置（向后兼容）
[devices.keyboard]
enabled = true
disable_internal = true

[devices.mouse]
enabled = true
disable_internal = true

# 新增：设备级别配置
[devices.specific]
# 禁用特定设备
disabled = [
    # 按路径禁用（最可靠）
    { path = "/dev/input/event0" },

    # 按名称禁用（后备，路径可能变化）
    { name = "SynPS/2 Synaptics TouchPad" },

    # 组合匹配（路径和名称都匹配时才禁用）
    { path = "/dev/input/event5", name = "AT Translated Set 2 keyboard" },
]

# 元数据（新增）
[meta]
created_at = "2026-03-11T15:01:00Z"
updated_at = "2026-03-11T15:01:00Z"
```

### 向后兼容性

**读取配置时**：
1. 如果 `version` 字段不存在 → 假设版本 `0.1`（旧格式）
2. 如果 `devices.specific.disabled` 不存在 → 使用空列表
3. 合并 `disable_internal` 和 `devices.specific.disabled`

**写入配置时**：
- 始终写入 `version = "1.0"`
- 保留 `disable_internal` 字段（向后兼容旧版本）
- 写入 `devices.specific.disabled`（新功能）

---

## 实体关系图

```
DetectionSession (1)
    ├── 使用 (1..*) → InputDevice
    ├── 生成 (1) → DeviceConfiguration
    └── 验证 → Config File (TOML)

InputDevice (1..*)
    ├── 标识 → PathBuf (设备路径)
    ├── 分类 → DeviceType
    └── 分类 → BusType

DeviceConfiguration (1)
    ├── 引用 (1..*) → DeviceRef
    └── 持久化 → Config File (TOML)
```

---

## 数据流

### 检测流程

```
1. 用户运行 `aegis-input detect`
   ↓
2. 创建 DetectionSession (state: Initializing)
   ↓
3. 扫描 /dev/input/event* → Vec<InputDevice>
   ↓
4. 启动事件监听 (state: Running)
   ↓
5. 用户在设备上输入 → 记录 active_devices
   ↓
6. 用户按 Ctrl+C → state: UserSelection
   ↓
7. 显示 active_devices，用户选择 → user_selection
   ↓
8. 生成 DeviceConfiguration
   ↓
9. 写入 TOML 配置文件
   ↓
10. state: Completed
```

### 配置应用流程

```
1. 读取 TOML 配置文件
   ↓
2. 解析为 DeviceConfiguration
   ↓
3. 验证 disabled_devices 引用
   ↓
4. 匹配当前系统的 InputDevice
   ↓
5. 禁用匹配的设备
   ↓
6. 记录到状态文件
```

---

## 约束与验证

### 数量约束

- 最多支持 20 个输入设备（性能考虑）
- 配置中最多禁用 10 个设备（可用性考虑）

### 完整性约束

- `DetectionSession.active_devices` ⊆ `DetectionSession.all_devices`
- `DeviceConfiguration.disabled_devices` 中的引用必须可解析

### 业务规则

1. **设备可用性**：设备被禁用后，应在配置文件中标记
2. **配置验证**：应用配置前验证所有引用的设备存在
3. **降级策略**：如果设备路径变化，尝试按名称匹配
4. **冲突解决**：`disable_internal` 和 `devices.specific.disabled` 冲突时，后者优先

---

## 性能考虑

### 内存占用

| 实体 | 单个大小 | 数量 | 总计 |
|------|---------|------|------|
| `InputDevice` | ~1KB | 20 | 20KB |
| `DetectionSession` | ~5KB | 1 | 5KB |
| `DeviceConfiguration` | ~2KB | 1 | 2KB |

**总计**: ~30KB（远低于 50MB 预算）

### I/O 操作

- **配置读取**: 启动时一次（<10ms）
- **配置写入**: 用户保存时一次（<10ms）
- **事件监听**: 仅在检测模式下（非轮询）

---

## 迁移策略

### 从旧配置迁移

**场景**: 用户从 v0.3.0 升级到 v0.4.0

**步骤**：
1. 读取旧配置（没有 `version` 字段）
2. 保留 `disable_internal` 设置
3. 创建空的 `devices.specific.disabled` 列表
4. 写入新配置格式（添加 `version = "1.0"`）

**示例**：
```toml
# 旧配置（v0.3.0）
[devices.keyboard]
enabled = true
disable_internal = true

# 新配置（v0.4.0，自动迁移）
version = "1.0"

[devices.keyboard]
enabled = true
disable_internal = true

[devices.specific]
disabled = []
```

---

## 未来扩展

### 可能的增强

1. **设备组**：将多个设备分组（如"所有内置输入设备"）
2. **时间规则**：特定时间自动禁用/启用设备
3. **条件规则**：基于外置设备存在状态自动切换

### 当前不包含

- 设备驱动配置（按键映射、宏等）
- 输入事件记录或日志
- 远程设备配置

---

## 总结

**核心实体**：
1. `InputDevice` - 输入设备表示
2. `DetectionSession` - 检测会话管理
3. `DeviceConfiguration` - 设备配置持久化

**配置格式**：向后兼容的 TOML 扩展

**验证机制**：
- 实体完整性约束
- 配置文件格式验证
- 设备引用解析验证

**下一步**: 定义接口契约（contracts/）
