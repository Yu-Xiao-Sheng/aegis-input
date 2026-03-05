# 数据模型: 快速安装与服务化运行

## 实体与字段

### 安装元数据（InstallMetadata）

- **version**: 安装版本
- **platform**: 平台标识（linux/windows/macos）
- **install_path**: 二进制安装路径
- **unit_path**: systemd unit 文件路径
- **installed_at**: 安装时间

### 服务状态（ServiceState）

- **running**: 是否运行
- **enabled**: 是否开机自启
- **last_changed_at**: 最近变更时间

### 安装配置（InstallConfig）

- **config_path**: 配置文件路径
- **status_path**: 状态文件路径
- **user**: 运行用户
- **group**: 运行组（需包含 input）

## 关系

- 一个 **InstallMetadata** 对应一个 **InstallConfig**
- **ServiceState** 反映当前 systemd 服务状态

## 规则与校验

- 安装完成后必须写入 InstallMetadata
- 卸载必须清除 unit 文件与 InstallMetadata
- `running=true` 即功能启用，`running=false` 即功能关闭

## 状态迁移

### ServiceState

- `stopped -> running`: 功能启用
- `running -> stopped`: 功能关闭并恢复内置设备

### InstallMetadata

- `absent -> present`: 安装完成
- `present -> absent`: 卸载完成
