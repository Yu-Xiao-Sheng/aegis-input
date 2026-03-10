# 数据模型: 一键安装与自动化分发

**功能**: 003-one-click-distribution
**创建日期**: 2026-03-10
**状态**: 设计阶段

本文档描述一键安装与自动化分发功能涉及的核心数据实体及其关系。

---

## 核心实体

### 1. Release 元数据（Release Metadata）

表示在 GitHub Releases 中发布的版本信息。

#### 属性

| 属性 | 类型 | 描述 | 示例 |
|------|------|------|------|
| `version` | String | 语义化版本号（不含 v 前缀） | `"0.3.0"` |
| `tag_name` | String | Git tag 名称 | `"v0.3.0"` |
| `published_at` | DateTime | 发布时间戳 | ISO 8601 格式 |
| `release_notes` | String | Release 说明 | Markdown 格式 |
| `artifacts` | Map<Arch, Artifact> | 各架构的二进制文件信息 | 见下方 |

#### Artifact 子结构

| 属性 | 类型 | 描述 | 示例 |
|------|------|------|------|
| `filename` | String | 文件名 | `"aegis-input-x86_64-unknown-linux-musl.tar.gz"` |
| `download_url` | String | GitHub Releases 下载 URL | `"https://github.com/.../releases/download/v0.3.0/..."` |
| `sha256` | String | SHA256 校验和 | `"abc123..."` |
| `size` | Integer | 文件大小（字节） | `5242880` |

#### 验证规则

- `version`: 必须符合 `MAJOR.MINOR.PATCH` 格式
- `tag_name`: 必须以 `v` 开头
- `sha256`: 必须是 64 字符的十六进制字符串
- `artifacts`: 不能为空（至少包含一个架构）

#### 示例

```json
{
  "version": "0.3.0",
  "tag_name": "v0.3.0",
  "published_at": "2026-03-10T12:00:00Z",
  "release_notes": "## 🚀 Release v0.3.0\n\n### 新增\n- 一键安装功能",
  "artifacts": {
    "x86_64-unknown-linux-musl": {
      "filename": "aegis-input-x86_64-unknown-linux-musl.tar.gz",
      "download_url": "https://github.com/user/aegis-input/releases/download/v0.3.0/aegis-input-x86_64-unknown-linux-musl.tar.gz",
      "sha256": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
      "size": 5242880
    },
    "aarch64-unknown-linux-musl": {
      "filename": "aegis-input-aarch64-unknown-linux-musl.tar.gz",
      "download_url": "https://github.com/user/aegis-input/releases/download/v0.3.0/aegis-input-aarch64-unknown-linux-musl.tar.gz",
      "sha256": "486e4645420c8cd46d9cc694a09af05c1bb8b2dd2de4c4e1c6f7f40a4dd8d9f0",
      "size": 4980736
    }
  }
}
```

---

### 2. 安装状态（Installation State）

记录当前系统的安装信息。

#### 属性

| 属性 | 类型 | 描述 | 示例 |
|------|------|------|------|
| `version` | String | 当前安装版本 | `"0.3.0"` |
| `install_time` | DateTime | 安装时间 | ISO 8601 格式 |
| `install_method` | Enum | 安装方式 | `"one-click"` / `"manual"` / `"package"` |
| `install_source` | String | 安装来源 URL 或路径 | `"https://.../install.sh"` |
| `architecture` | String | 安装的架构 | `"x86_64-unknown-linux-musl"` |
| `binary_path` | String | 二进制文件路径 | `"/usr/local/bin/aegis-input"` |
| `installed_by` | String | 执行安装的用户 | `"username"` |

#### 安装方式枚举（InstallMethod）

| 值 | 描述 |
|----|------|
| `one-click` | 一键安装脚本（curl | bash） |
| `manual` | 手动编译安装 |
| `package` | 系统包管理器安装（预留） |

#### 验证规则

- `version`: 必须符合语义化版本格式
- `install_time`: 不能为空
- `install_method`: 必须是预定义值之一
- `binary_path`: 必须是绝对路径

#### 存储位置

```
/var/lib/aegis-input/install.toml
```

#### 示例

```toml
[installation]
version = "0.3.0"
install_time = "2026-03-10T12:00:00Z"
install_method = "one-click"
install_source = "https://raw.githubusercontent.com/user/aegis-input/main/install/remote/install.sh"
architecture = "x86_64-unknown-linux-musl"
binary_path = "/usr/local/bin/aegis-input"
installed_by = "username"
```

---

### 3. 版本信息（Version Info）

表示版本比较和检测结果。

#### 属性

| 属性 | 类型 | 描述 | 示例 |
|------|------|------|------|
| `current_version` | String | 当前安装版本 | `"0.2.0"` |
| `latest_version` | String | 远程最新版本 | `"0.3.0"` |
| `update_available` | Boolean | 是否有更新 | `true` |
| `update_type` | Enum | 更新类型 | `"major"` / `"minor"` / `"patch"` |
| `download_url` | String | 更新下载 URL | `"https://.../v0.3.0/..."` |

#### 更新类型枚举（UpdateType）

| 值 | 描述 | 示例 |
|----|------|------|
| `major` | 主版本更新（可能不兼容） | `0.x → 1.0` |
| `minor` | 次版本更新（新功能） | `0.2 → 0.3` |
| `patch` | 补丁版本更新（修复） | `0.3.0 → 0.3.1` |

#### 版本比较逻辑

```
version_gt(a, b):
  if a.major > b.major: return true
  if a.major == b.major and a.minor > b.minor: return true
  if a.major == b.major and a.minor == b.minor and a.patch > b.patch: return true
  return false
```

#### 示例

```json
{
  "current_version": "0.2.0",
  "latest_version": "0.3.0",
  "update_available": true,
  "update_type": "minor",
  "download_url": "https://github.com/user/aegis-input/releases/download/v0.3.0/aegis-input-x86_64-unknown-linux-musl.tar.gz"
}
```

---

### 4. 系统信息（System Info）

检测到的系统环境信息。

#### 属性

| 属性 | 类型 | 描述 | 示例 |
|------|------|------|------|
| `os` | String | 操作系统 | `"Linux"` |
| `os_id` | String | 发行版标识 | `"ubuntu"` |
| `os_version` | String | 发行版版本 | `"22.04"` |
| `architecture` | String | CPU 架构 | `"x86_64"` |
| `libc` | String | C 库类型 | `"glibc"` / `"musl"` |
| `target_triple` | String | Rust 目标三元组 | `"x86_64-unknown-linux-musl"` |

#### 架构映射

| `uname -m` 输出 | `architecture` | `target_triple` |
|----------------|----------------|-----------------|
| `x86_64`, `amd64` | `x86_64` | `x86_64-unknown-linux-musl` |
| `aarch64`, `arm64` | `aarch64` | `aarch64-unknown-linux-musl` |
| `armv7l`, `armhf` | `armv7` | `armv7-unknown-linux-musleabihf` (预留) |

#### 示例

```json
{
  "os": "Linux",
  "os_id": "ubuntu",
  "os_version": "22.04",
  "architecture": "x86_64",
  "libc": "glibc",
  "target_triple": "x86_64-unknown-linux-musl"
}
```

---

## 实体关系图

```
┌─────────────────┐
│  Release        │
│  Metadata       │
└────────┬────────┘
         │ 1
         │
         │ N
┌────────▼────────┐
│  Artifact       │
│  (per arch)     │
└─────────────────┘

┌─────────────────┐
│  System Info    │
└────────┬────────┘
         │
         │ 检测
         ▼
┌─────────────────┐
│  Install Script │───下载──▶ Artifact
└────────┬────────┘
         │
         │ 安装
         ▼
┌─────────────────┐
│  Installation   │
│  State          │
└─────────────────┘

┌─────────────────┐
│  Version Info   │
└─────────────────┘
```

---

## 数据流

### 安装流程

```
1. 检测系统 → System Info
2. 查询 Release → Release Metadata
3. 下载 Artifact → 校验 SHA256
4. 安装二进制 → 写入 Installation State
```

### 版本检查流程

```
1. 读取 Installation State → current_version
2. 查询 GitHub API → Release Metadata (latest)
3. 比较版本 → Version Info
4. 显示提示（如果 update_available）
```

### 发布流程

```
1. 推送 Git tag
2. GitHub Actions 触发
3. 构建多架构二进制 → Artifact
4. 生成 SHA256SUMS.txt
5. 创建 Release → Release Metadata
```

---

## 存储

### 文件系统

| 文件 | 格式 | 用途 |
|------|------|------|
| `/var/lib/aegis-input/install.toml` | TOML | 安装状态 |
| `/usr/local/bin/aegis-input` | 二进制 | 可执行文件 |
| `/etc/systemd/system/aegis-input.service` | Unit file | systemd 服务 |

### GitHub Releases

| 文件 | 格式 | 用途 |
|------|------|------|
| `aegis-input-{target}.tar.gz` | tar.gz | 二进制归档 |
| `SHA256SUMS.txt` | 文本 | 校验和文件 |
| Release Notes | Markdown | 版本说明 |

### GitHub API

| 端点 | 返回 |
|------|------|
| `/repos/{owner}/{repo}/releases/latest` | Release Metadata (JSON) |
| `/repos/{owner}/{repo}/releases` | 所有 releases |

---

## 验证与约束

### 完整性约束

1. 每个 Release 必须至少包含一个架构的 Artifact
2. Installation State 中的 version 必须与实际二进制版本一致
3. SHA256 校验和必须匹配下载的文件

### 业务规则

1. 仅支持 Linux 系统（macOS/Windows 预留）
2. 仅支持 x86_64 和 aarch64 架构
3. 必须验证下载文件的完整性
4. 安装失败时必须清理临时文件
5. 版本比较遵循语义化版本规范

---

## 未来扩展

### 预留字段

所有实体保留 `metadata` 字段用于未来扩展：

```json
{
  "version": "0.3.0",
  "metadata": {
    "signature": "..."  // 未来添加 GPG 签名
  }
}
```

### 新增实体

- **Package Metadata**: deb/rpm 包信息（未来支持包管理器）
- **Update Config**: 自动更新配置（用户可选）
- **Signature**: GPG 或 sigstore 签名信息
