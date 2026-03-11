# 检测模块接口契约

**Feature**: 004-interactive-detection
**Created**: 2026-03-11
**Version**: 1.0

## 概述

本文档定义检测模块的核心接口契约，包括会话管理、设备监听和设备选择器。

---

## 核心 Trait 定义

### 1. DetectionSession Trait

检测会话管理接口。

```rust
/// 检测会话管理
#[async_trait]
pub trait DetectionSession: Send + Sync {
    /// 启动检测会话
    async fn start(&mut self) -> Result<SessionStartInfo>;

    /// 等待会话完成（用户结束或超时）
    async fn wait(&mut self) -> Result<SessionResult>;

    /// 获取当前活跃的设备列表
    fn active_devices(&self) -> HashSet<PathBuf>;

    /// 取消会话
    async fn cancel(&mut self) -> Result<()>;

    /// 完成会话并生成配置
    async fn complete(&mut self, selection: HashSet<PathBuf>) -> Result<DeviceConfiguration>;
}
```

**方法说明**：

#### `start() -> Result<SessionStartInfo>`

启动检测会话。

**返回值**：
```rust
pub struct SessionStartInfo {
    pub session_id: String,
    pub all_devices: Vec<InputDevice>,
    pub start_time: DateTime<Utc>,
}
```

**错误**：
- `NoDevicesFound`: 未检测到输入设备
- `PermissionDenied`: 权限不足
- `IoError`: I/O 错误

**行为**：
1. 扫描 `/dev/input/event*` 设备
2. 过滤键盘和鼠标设备
3. 为每个设备创建异步监听器
4. 返回设备列表和会话信息

#### `wait() -> Result<SessionResult>`

等待会话完成。

**返回值**：
```rust
pub struct SessionResult {
    pub session_id: String,
    pub end_time: DateTime<Utc>,
    pub active_devices: HashSet<PathBuf>,
    pub duration: Duration,
    pub completion_reason: CompletionReason,
}

pub enum CompletionReason {
    UserInterrupt,  // 用户按 Ctrl+C
    Timeout,        // 超时
    Error(String),  // 错误
}
```

**行为**：
1. 监听所有设备输入事件
2. 记录活跃设备
3. 等待用户中断或超时
4. 返回会话结果

#### `active_devices() -> HashSet<PathBuf>`

获取当前活跃的设备列表。

**返回值**：活跃设备的路径集合

**线程安全**: 是

#### `cancel() -> Result<()>`

取消会话。

**行为**：
1. 停止所有设备监听
2. 释放设备文件描述符
3. 清理资源

#### `complete() -> Result<DeviceConfiguration>`

完成会话并生成配置。

**参数**：
- `selection`: 用户选择的要禁用的设备路径

**返回值**：设备配置对象

**行为**：
1. 验证设备路径有效性
2. 创建配置对象
3. 标记时间戳

---

### 2. DeviceMonitor Trait

设备事件监听接口。

```rust
/// 设备事件监听器
#[async_trait]
pub trait DeviceMonitor: Send + Sync {
    /// 创建监听器
    async fn new(device: &InputDevice) -> Result<Self>

    where
        Self: Sized;

    /// 开始监听事件
    async fn start(&mut self) -> Result<()>;

    /// 获取下一个事件
    async fn next_event(&mut self) -> Result<InputEvent>;

    /// 停止监听
    async fn stop(&mut self) -> Result<()>;

    /// 检查设备是否支持禁用
    fn supports_disable(&self) -> bool;
}
```

**方法说明**：

#### `new() -> Result<Self>`

创建监听器。

**错误**：
- `DeviceNotFound`: 设备文件不存在
- `PermissionDenied`: 权限不足
- `IoError`: I/O 错误

**行为**：
1. 打开设备文件
2. 设置非阻塞模式
3. 创建异步事件流

#### `start() -> Result<()>`

开始监听事件。

**行为**：
1. 启动事件循环
2. 注册信号处理（Ctrl+C）

#### `next_event() -> Result<InputEvent>`

获取下一个事件。

**返回值**：
```rust
pub struct InputEvent {
    pub device_path: PathBuf,
    pub event_type: InputEventType,
    pub timestamp: DateTime<Utc>,
}

pub enum InputEventType {
    Keyboard { key_code: u16 },
    Mouse { x: i32, y: i32 },
    Unknown,
}
```

**行为**：
- 阻塞等待下一个事件
- 超时返回 `Error::Timeout`

#### `stop() -> Result<()>`

停止监听。

**行为**：
1. 停止事件循环
2. 关闭设备文件
3. 清理资源

#### `supports_disable() -> bool`

检查设备是否支持禁用。

**返回值**：
- `true`: 可以禁用（外置设备）
- `false`: 不可禁用（内置或系统关键设备）

---

### 3. DeviceSelector Trait

设备选择器接口。

```rust
/// 设备选择器
pub trait DeviceSelector: Send + Sync {
    /// 显示设备列表并获取用户选择
    fn prompt_selection(
        &self,
        all_devices: &[InputDevice],
        active_devices: &HashSet<PathBuf>,
    ) -> Result<HashSet<PathBuf>>;

    /// 验证用户选择
    fn validate_selection(
        &self,
        selection: &HashSet<PathBuf>,
        available: &[InputDevice],
    ) -> Result<()>;
}
```

**方法说明**：

#### `prompt_selection() -> Result<HashSet<PathBuf>>`

显示设备列表并获取用户选择。

**参数**：
- `all_devices`: 所有检测到的设备
- `active_devices`: 检测期间活跃的设备

**返回值**：用户选择的设备路径集合

**错误**：
- `InvalidInput`: 无效的用户输入
- `IoError`: I/O 错误

**行为**：
1. 显示设备列表（标记活跃设备）
2. 提示用户输入
3. 解析用户输入（编号、all、none）
4. 返回选择的设备路径

**交互示例**：
```
检测期间活跃的设备:

  ✓ [1] Logitech USB Receiver
  ✓ [2] SynPS/2 Synaptics TouchPad

未活跃的设备:

    [3] AT Translated Set 2 keyboard

要禁用哪些设备？（可多选）
> 1,2

已选择禁用:
  - Logitech USB Receiver
  - SynPS/2 Synaptics TouchPad

是否保存此配置？ (y/n): y
```

#### `validate_selection() -> Result<()>`

验证用户选择。

**参数**：
- `selection`: 用户选择的设备路径
- `available`: 可用的设备列表

**错误**：
- `InvalidDevice`: 设备不存在
- `DeviceNotSupported`: 设备不支持禁用

**行为**：
1. 检查所有设备是否存在
2. 检查设备是否支持禁用
3. 返回验证结果

---

## 实现要求

### 1. 线程安全

所有 trait 必须实现 `Send + Sync`，支持多线程调用。

### 2. 错误处理

使用 `anyhow::Result<T>` 作为统一错误类型。

```rust
pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;
```

### 3. 异步支持

所有异步方法必须使用 `tokio::async_trait`。

### 4. 资源清理

实现 `Drop` trait 确保资源释放：

```rust
impl Drop for ConcreteDetectionSession {
    fn drop(&mut self) {
        // 清理资源
        tokio::runtime::Handle::try_current()
            .and_then(|handle| {
                handle.block_on(self.cancel())
            });
    }
}
```

---

## 性能要求

### 响应延迟

- `next_event()`: <100ms（从事件发生到返回）
- `prompt_selection()`: 即时（I/O 瓶颈）

### 内存占用

- `DetectionSession`: <10MB
- `DeviceMonitor`: <1MB/设备
- 总计: <50MB（20 个设备）

### CPU 占用

- 空闲: <1%
- 事件处理: <5%（峰值）

---

## 测试要求

### 单元测试

**测试覆盖率**: >80%

**测试用例**:
1. `DetectionSession::start()` - 正常启动
2. `DetectionSession::wait()` - 用户中断
3. `DetectionSession::wait()` - 超时
4. `DeviceMonitor::next_event()` - 键盘事件
5. `DeviceMonitor::next_event()` - 鼠标事件
6. `DeviceSelector::prompt_selection()` - 有效输入
7. `DeviceSelector::prompt_selection()` - 无效输入

### 集成测试

**测试场景**:
1. 完整检测流程（多设备）
2. 配置保存和加载
3. 错误恢复（设备断开）

**测试环境**:
- 使用虚拟设备或真实设备替身
- 模拟键盘和鼠标输入

---

## 依赖关系

### 外部依赖

- `evdev`: 设备访问
- `tokio`: 异步运行时
- `crossterm`: 终端交互
- `anyhow`: 错误处理

### 内部依赖

- `src/config`: 配置管理
- `src/platform/linux`: Linux 特定实现

---

## 平台抽象

### Linux 实现

**文件**: `src/platform/linux/detection.rs`

**实现**:
- `LinuxDetectionSession`
- `LinuxDeviceMonitor`
- `CliDeviceSelector`

### 未来平台支持

**Windows**: 使用 `RawInput` API
**macOS**: 使用 `CGEvent` API

---

## 错误类型

### 错误定义

```rust
#[derive(Debug, thiserror::Error)]
pub enum DetectionError {
    #[error("未检测到输入设备")]
    NoDevicesFound,

    #[error("权限不足: {0}")]
    PermissionDenied(String),

    #[error("设备访问失败: {0}")]
    DeviceAccessFailed(String),

    #[error("无效的用户输入: {0}")]
    InvalidInput(String),

    #[error("超时")]
    Timeout,

    #[error("I/O 错误: {0}")]
    IoError(#[from] std::io::Error),
}
```

### 错误处理策略

1. **启动阶段**: 错误立即返回
2. **检测阶段**: 记录日志，继续运行
3. **选择阶段**: 错误提示用户，允许重试

---

## 配置验证

### 验证规则

1. 设备路径必须存在
2. 设备必须支持禁用
3. 设备数量 ≤ 10

### 验证流程

```rust
pub fn validate_config(config: &DeviceConfiguration) -> Result<()> {
    // 检查设备数量
    if config.disabled_devices.len() > 10 {
        return Err(DetectionError::InvalidInput(
            "禁用设备数量超过限制（最多10个）".into()
        ));
    }

    // 检查设备引用
    for device_ref in &config.disabled_devices {
        // 验证路径或名称
        if let Some(path) = &device_ref.path {
            if !path.exists() {
                return Err(DetectionError::DeviceAccessFailed(
                    format!("设备不存在: {:?}", path)
                ));
            }
        }
    }

    Ok(())
}
```

---

## 总结

**核心接口**:
1. `DetectionSession` - 会话管理
2. `DeviceMonitor` - 设备监听
3. `DeviceSelector` - 设备选择

**关键约束**:
- 异步支持（tokio）
- 线程安全（Send + Sync）
- 资源清理（Drop trait）

**测试覆盖**:
- 单元测试 >80%
- 集成测试（完整流程）

**下一步**: 创建快速开始指南（quickstart.md）
