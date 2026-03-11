# 安装脚本接口契约

**类型**: Shell 脚本接口
**版本**: 1.0.0
**文件**: `install/remote/install.sh`

本文档定义一键安装脚本的公共接口规范。

---

## 命令行接口

### 基本用法

```bash
curl --proto '=https' --tlsv1.2 -sSf https://.../install.sh | bash [OPTIONS]
```

### 选项

| 选项 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `--version <VER>` | String | `latest` | 指定安装版本（如 `0.3.0`） |
| `--target <TRIPLE>` | String | 自动检测 | 指定目标架构（如 `x86_64-unknown-linux-musl`） |
| `--dir <PATH>` | String | `/usr/local/bin` | 安装目录 |
| `--yes` / `-y` | Flag | `false` | 非交互模式，自动确认 |
| `--dry-run` | Flag | `false` | 模拟运行，不实际安装 |
| `--no-systemd` | Flag | `false` | 跳过 systemd 服务安装 |
| `--help` / `-h` | Flag | - | 显示帮助信息 |
| `--check` | Flag | - | 检查新版本但不安装 |

### 环境变量

| 变量 | 类型 | 描述 |
|------|------|------|
| `AEGIS_INPUT_VERSION` | String | 指定版本（同 `--version`） |
| `AEGIS_INPUT_TARGET` | String | 指定架构（同 `--target`） |
| `AEGIS_INPUT_INSTALL_DIR` | String | 安装目录（同 `--dir`） |
| `AEGIS_INPUT_NO_SYSTEMD` | String | 跳过 systemd（同 `--no-systemd`） |
| `NONINTERACTIVE` | String | 非交互模式（同 `--yes`） |

### 示例

```bash
# 安装最新版本
curl ... | bash

# 安装特定版本
curl ... | bash -s -- --version 0.3.0

# 非交互模式
curl ... | bash -s -- --yes

# 指定安装目录
curl ... | bash -s -- --dir /opt/aegis-input

# 检查更新
curl ... | bash -s -- --check
```

---

## 退出码

| 退出码 | 含义 | 描述 |
|--------|------|------|
| 0 | 成功 | 安装完成 |
| 1 | 一般错误 | 未明确的错误 |
| 2 | 网络错误 | 下载失败 |
| 3 | 校验错误 | SHA256 不匹配 |
| 4 | 权限错误 | 需要 sudo 权限 |
| 5 | 平台错误 | 不支持的系统或架构 |
| 6 | 依赖错误 | 缺少必要依赖 |
| 7 | 版本错误 | 指定的版本不存在 |
| 64 | 使用错误 | 命令行参数错误 |

---

## 输出格式

### 标准输出

脚本在正常运行时输出进度信息：

```
[INFO] 检测系统信息...
[INFO] 系统: Linux (Ubuntu 22.04)
[INFO] 架构: x86_64-unknown-linux-musl
[INFO] 下载 aegis-input v0.3.0...
[INFO] 验证 SHA256 校验和...
[INFO] 安装到 /usr/local/bin...
[INFO] 创建 systemd 服务...
[INFO] 启动服务...
✅ 安装完成！
版本: 0.3.0
状态: active (running)
```

### 标准错误

错误信息输出到 stderr：

```
[ERROR] 下载失败: curl: (7) Failed to connect
[ERROR] 校验和验证失败！文件可能已损坏。
[ERROR] 不支持的架构: armv7l
```

### 日志级别

| 级别 | 前缀 | 用途 |
|------|------|------|
| INFO | `[INFO]` | 正常进度信息 |
| WARN | `[WARN]` | 警告信息（可继续） |
| ERROR | `[ERROR]` | 错误信息（终止运行） |

---

## 安全要求

### 传输安全

- 必须强制使用 HTTPS：`curl --proto '=https'`
- 必须要求 TLS 1.2+：`--tlsv1.2 -sSf`
- 必须验证证书（默认行为）

### 下载验证

- 必须验证 SHA256 校验和
- 校验和不匹配时必须终止安装
- 下载失败时必须清理临时文件

### 权限管理

- 仅在必要时请求 sudo 权限
- 下载过程无需特殊权限
- 必须验证 sudo 权限可用

### 临时文件安全

```bash
# 创建安全的临时文件
tmp_file=$(mktemp "/tmp/aegis-install.XXXXXX")
chmod 600 "$tmp_file"

# 退出时清理
trap 'rm -f "$tmp_file"' EXIT
```

---

## 错误处理

### 网络错误

```bash
[ERROR] 无法连接到 GitHub
[INFO] 请检查网络连接或稍后重试
[INFO] 手动下载: https://github.com/.../releases/download/v0.3.0/...
```

### 权限错误

```bash
[ERROR] 需要 sudo 权限来安装到 /usr/local/bin
[INFO] 请使用: sudo curl ... | bash
[INFO] 或指定用户目录: curl ... | bash -s -- --dir ~/.local/bin
```

### 平台错误

```bash
[ERROR] 不支持的操作系统: macOS
[INFO] 当前仅支持 Linux
[INFO] macOS 支持即将推出
```

### 版本错误

```bash
[ERROR] 版本 9.9.9 不存在
[INFO] 可用版本: 0.2.0, 0.3.0
[INFO] 最新版本: 0.3.0
```

---

## 交互模式

### 确认提示

在交互模式（默认）下，安装前会提示：

```
即将安装 aegis-input v0.3.0
目标架构: x86_64-unknown-linux-musl
安装位置: /usr/local/bin

继续？ [Y/n]
```

### 更新提示

检测到已安装旧版本时：

```
检测到已安装版本: 0.2.0
新版本可用: 0.3.0

变更:
- 新增一键安装功能
- 自动构建多平台二进制

是否升级？ [Y/n]
```

### 非交互模式

```bash
curl ... | bash -s -- --yes
# 或
export NONINTERACTIVE=1
curl ... | bash
```

跳过所有确认提示，自动执行。

---

## 系统检测

### 架构检测

```bash
detect_architecture() {
    case "$(uname -m)" in
        x86_64|amd64)
            ARCH="x86_64-unknown-linux-musl"
            ;;
        aarch64|arm64)
            ARCH="aarch64-unknown-linux-musl"
            ;;
        *)
            abort "不支持的架构: $(uname -m)"
            ;;
    esac
}
```

### 操作系统检测

```bash
detect_os() {
    OS="$(uname)"
    if [[ "$OS" == "Linux" ]]; then
        ON_LINUX=1
        if [[ -f /etc/os-release ]]; then
            . /etc/os-release
            DISTRO="$ID"
        fi
    elif [[ "$OS" == "Darwin" ]]; then
        abort "当前仅支持 Linux，macOS 支持即将推出"
    else
        abort "不支持的操作系统: $OS"
    fi
}
```

### systemd 检测

```bash
if ! command -v systemctl >/dev/null 2>&1; then
    warn "未检测到 systemd，跳过服务安装"
    NO_SYSTEMD=1
fi
```

---

## 安装流程

### 1. 系统检测

```bash
[INFO] 检测系统信息...
[INFO] 系统: Linux (Ubuntu 22.04)
[INFO] 架构: x86_64-unknown-linux-musl
[INFO] systemd: 已检测
```

### 2. 下载二进制

```bash
[INFO] 下载 aegis-input v0.3.0...
[████████████████████] 100% (5.2 MB / 5.2 MB)
```

### 3. 验证校验和

```bash
[INFO] 验证 SHA256 校验和...
[INFO] 校验和验证通过
```

### 4. 安装文件

```bash
[INFO] 安装到 /usr/local/bin...
[INFO] 设置权限...
```

### 5. 配置服务

```bash
[INFO] 创建 systemd 服务...
[INFO] 启动服务...
[INFO] 服务状态: active (running)
```

### 6. 验证安装

```bash
[INFO] 验证安装...
[INFO] 版本: aegis-input 0.3.0
[INFO] 状态: 运行中

✅ 安装完成！
```

---

## 升级流程

### 检测已安装版本

```bash
if [[ -f /var/lib/aegis-input/install.toml ]]; then
    current_version=$(grep "version" /var/lib/aegis-input/install.toml | cut -d'"' -f2)
    [INFO] 检测到已安装版本: $current_version
fi
```

### 版本比较

```bash
if version_gt "$latest_version" "$current_version"; then
    [INFO] 新版本可用: $latest_version
    # 提示升级
fi
```

### 平滑升级

```bash
# 1. 停止现有服务
systemctl stop aegis-input

# 2. 安装新二进制
cp aegis-input /usr/local/bin/

# 3. 更新安装状态
update_install_state

# 4. 重启服务
systemctl daemon-reload
systemctl start aegis-input
```

---

## 卸载支持

虽然安装脚本主要用于安装，但应提供卸载指引：

```bash
[INFO] 卸载 aegis-input
[INFO] 1. 停止服务: sudo systemctl stop aegis-input
[INFO] 2. 禁用服务: sudo systemctl disable aegis-input
[INFO] 3. 删除二进制: sudo rm /usr/local/bin/aegis-input
[INFO] 4. 删除服务: sudo rm /etc/systemd/system/aegis-input.service
[INFO] 5. 删除配置: sudo rm -rf /etc/aegis-input /var/lib/aegis-input
```

或使用现有的卸载脚本：

```bash
sudo /path/to/uninstall.sh
```
