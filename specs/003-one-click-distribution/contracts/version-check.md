# 版本检测 API 契约

**类型**: REST API + 版本比较逻辑
**版本**: 1.0.0
**用途**: 检测新版本并提供更新提示

本文档定义版本检测机制的接口规范。

---

## GitHub API 端点

### 获取最新 Release

```http
GET /repos/{owner}/{repo}/releases/latest
```

**请求示例**:
```bash
curl -s https://api.github.com/repos/user/aegis-input/releases/latest
```

**响应示例**:
```json
{
  "tag_name": "v0.3.0",
  "name": "Release v0.3.0",
  "published_at": "2026-03-10T12:00:00Z",
  "html_url": "https://github.com/user/aegis-input/releases/tag/v0.3.0",
  "assets": [
    {
      "name": "aegis-input-x86_64-unknown-linux-musl.tar.gz",
      "browser_download_url": "https://github.com/.../aegis-input-x86_64-unknown-linux-musl.tar.gz",
      "size": 5242880
    },
    {
      "name": "aegis-input-aarch64-unknown-linux-musl.tar.gz",
      "browser_download_url": "https://github.com/.../aegis-input-aarch64-unknown-linux-musl.tar.gz",
      "size": 4980736
    },
    {
      "name": "SHA256SUMS.txt",
      "browser_download_url": "https://github.com/.../SHA256SUMS.txt",
      "size": 256
    }
  ]
}
```

### 获取所有 Releases

```http
GET /repos/{owner}/{repo}/releases
```

**请求示例**:
```bash
curl -s https://api.github.com/repos/user/aegis-input/releases
```

**用途**: 获取所有版本历史，用于版本迁移路径分析。

---

## 版本比较逻辑

### 版本解析

```bash
parse_version() {
    local version="$1"
    # 移除 'v' 前缀
    version="${version#v}"
    # 分割为主版本、次版本、补丁版本
    echo "$version" | awk -F. '{print $1, $2, $3}'
}
```

### 版本比较函数

```bash
# version_gt a b: 如果 a > b 返回 0（true），否则返回 1（false）
version_gt() {
    local a="$1"
    local b="$2"

    # 解析版本号
    local a_major=$(echo "$a" | awk -F. '{print $1}')
    local a_minor=$(echo "$a" | awk -F. '{print $2}')
    local a_patch=$(echo "$a" | awk -F. '{print $3}')

    local b_major=$(echo "$b" | awk -F. '{print $1}')
    local b_minor=$(echo "$b" | awk -F. '{print $2}')
    local b_patch=$(echo "$b" | awk -F. '{print $3}')

    # 比较主版本
    if [[ $a_major -gt $b_major ]]; then
        return 0
    elif [[ $a_major -lt $b_major ]]; then
        return 1
    fi

    # 比较次版本
    if [[ $a_minor -gt $b_minor ]]; then
        return 0
    elif [[ $a_minor -lt $b_minor ]]; then
        return 1
    fi

    # 比较补丁版本
    if [[ $a_patch -gt $b_patch ]]; then
        return 0
    fi

    return 1
}

# version_eq a b: 如果 a == b 返回 0
version_eq() {
    [[ "$1" == "$2" ]]
}

# version_ge a b: 如果 a >= b 返回 0
version_ge() {
    version_eq "$1" "$2" || version_gt "$1" "$2"
}

# version_lt a b: 如果 a < b 返回 0
version_lt() {
    ! version_ge "$1" "$2"
}
```

### 更新类型判断

```bash
get_update_type() {
    local current="$1"
    local latest="$2"

    local current_major=$(echo "$current" | awk -F. '{print $1}')
    local latest_major=$(echo "$latest" | awk -F. '{print $1}')

    if [[ $latest_major -gt $current_major ]]; then
        echo "major"
    elif [[ $latest_major -eq $current_major ]]; then
        local current_minor=$(echo "$current" | awk -F. '{print $2}')
        local latest_minor=$(echo "$latest" | awk -F. '{print $2}')
        if [[ $latest_minor -gt $current_minor ]]; then
            echo "minor"
        else
            echo "patch"
        fi
    fi
}
```

### 示例

```bash
# 比较
version_gt "0.3.0" "0.2.0"  # 返回 0（true）
version_gt "0.2.0" "0.3.0"  # 返回 1（false）
version_gt "0.3.0" "0.3.0"  # 返回 1（false，不相等）

# 更新类型
get_update_type "0.2.0" "0.3.0"  # 输出 "minor"
get_update_type "0.2.0" "1.0.0"  # 输出 "major"
get_update_type "0.2.0" "0.2.1"  # 输出 "patch"
```

---

## 版本检测流程

### 1. 获取当前版本

```bash
get_current_version() {
    if [[ -f /var/lib/aegis-input/install.toml ]]; then
        # 从安装状态文件读取
        grep "version" /var/lib/aegis-input/install.toml | \
            cut -d'"' -f2
    elif command -v aegis-input >/dev/null 2>&1; then
        # 从二进制文件读取
        aegis-input --version 2>/dev/null | \
            grep -oP 'version\s+\K[0-9.]+' || \
            echo "unknown"
    else
        echo "not-installed"
    fi
}
```

### 2. 获取最新版本

```bash
get_latest_version() {
    local repo="${REPO:-user/aegis-input}"
    local api_url="https://api.github.com/repos/${repo}/releases/latest"

    # 检查速率限制
    check_github_rate_limit

    # 获取版本
    local tag_name=$(curl -s "$api_url" | \
        grep -oP '"tag_name":\s*"\K[^"]*')

    if [[ -z "$tag_name" ]]; then
        abort "无法获取最新版本信息"
    fi

    # 移除 'v' 前缀
    echo "${tag_name#v}"
}
```

### 3. 检查更新

```bash
check_for_updates() {
    local current_version=$(get_current_version)
    local latest_version=$(get_latest_version)

    if [[ "$current_version" == "not-installed" ]]; then
        log_info "未检测到已安装版本"
        log_info "最新版本: $latest_version"
        return 0
    fi

    if version_gt "$latest_version" "$current_version"; then
        local update_type=$(get_update_type "$current_version" "$latest_version")
        log_info "新版本可用: $latest_version（当前: $current_version）"
        log_info "更新类型: $update_type"

        # 获取下载链接
        local arch=$(detect_architecture)
        local download_url="https://github.com/${repo}/releases/download/v${latest_version}/aegis-input-${arch}.tar.gz"

        log_info "下载链接: $download_url"
        return 0
    else
        log_info "已是最新版本: $current_version"
        return 1
    fi
}
```

---

## CLI 集成

### `--check` 选项

```bash
# 检查更新（不安装）
aegis-input --check
```

**输出示例**:
```
当前版本: 0.2.0
最新版本: 0.3.0

新版本可用！
更新类型: minor

变更:
- 新增一键安装功能
- 自动构建多平台二进制
- 版本检测和更新提示

下载链接: https://github.com/user/aegis-input/releases/download/v0.3.0/aegis-input-x86_64-unknown-linux-musl.tar.gz

安装命令:
curl --proto '=https' --tlsv1.2 -sSf https://.../install.sh | bash
```

### `--version` 选项增强

```bash
# 显示当前版本并检查更新
aegis-input --version
```

**输出示例**:
```
aegis-input 0.2.0

检查更新中...
新版本可用: 0.3.0

使用 --check 查看详情
```

---

## GitHub API 速率限制

### 速率限制检查

```bash
check_github_rate_limit() {
    if ! command -v jq >/dev/null 2>&1; then
        return 0  # 跳过检查（jq 未安装）
    fi

    local rate_limit=$(curl -s https://api.github.com/rate_limit)
    local remaining=$(echo "$rate_limit" | jq -r '.rate.remaining')
    local reset=$(echo "$rate_limit" | jq -r '.rate.reset')
    local limit=$(echo "$rate_limit" | jq -r '.rate.limit')

    if [[ "$remaining" -lt 10 ]]; then
        local reset_time=$(date -d "@$reset" "+%H:%M:%S" 2>/dev/null || \
                           date -r "$reset" "+%H:%M:%S")
        warn "GitHub API 速率限制较低（剩余 $remaining / $limit）"
        warn "重置时间: $reset_time"
    fi
}
```

### 速率限制信息

| 认证状态 | 每小时请求数 |
|----------|--------------|
| 未认证 | 60 |
| 已认证 | 5000 |

### 缓存策略

```bash
# 缓存版本信息 1 小时
CACHE_FILE="/tmp/aegis-input-version-cache.json"
CACHE_TTL=3600

get_cached_version() {
    if [[ -f "$CACHE_FILE" ]]; then
        local cache_time=$(stat -c %Y "$CACHE_FILE")
        local current_time=$(date +%s)
        local cache_age=$((current_time - cache_time))

        if [[ $cache_age -lt $CACHE_TTL ]]; then
            jq -r '.version' "$CACHE_FILE"
            return 0
        fi
    fi
    return 1
}

set_cached_version() {
    local version="$1"
    cat > "$CACHE_FILE" <<EOF
{
  "version": "$version",
  "cached_at": $(date +%s)
}
EOF
}
```

---

## 错误处理

### 网络错误

```bash
[ERROR] 无法连接到 GitHub API
[INFO] 请检查网络连接
[INFO] 或访问: https://github.com/user/aegis-input/releases
```

### 速率限制错误

```bash
[ERROR] GitHub API 速率限制已用尽
[INFO] 请稍后重试或手动访问:
[INFO] https://github.com/user/aegis-input/releases/latest
```

### 解析错误

```bash
[ERROR] 无法解析版本信息
[INFO] 当前版本: unknown
[INFO] 请手动检查更新
```

---

## 退出码

| 退出码 | 含义 |
|--------|------|
| 0 | 有新版本可用 |
| 1 | 无新版本 |
| 2 | 错误（网络、解析等） |
| 3 | 未安装 |

---

## 集成示例

### 安装脚本集成

```bash
# 检查是否有更新
check_for_updates
if [[ $? -eq 0 ]]; then
    echo "检测到新版本，是否升级？ [Y/n]"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        # 执行升级
        install_binary "$latest_version" "$arch"
    fi
fi
```

### systemd 定时器（可选）

```ini
# /etc/systemd/system/aegis-input-check-update.timer
[Unit]
Description=每天检查 aegis-input 更新

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
```

```ini
# /etc/systemd/system/aegis-input-check-update.service
[Unit]
Description=检查 aegis-input 更新

[Service]
Type=oneshot
ExecStart=/usr/local/bin/aegis-input --check
```

---

## 安全考虑

### HTTPS 强制

所有 API 请求必须使用 HTTPS：

```bash
api_url="https://api.github.com/repos/${repo}/releases/latest"
```

### 输入验证

```bash
# 验证版本号格式
validate_version() {
    local version="$1"
    if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        abort "无效的版本号格式: $version"
    fi
}
```

### 证书验证

```bash
# curl 默认验证证书，不要禁用
# ❌ 错误: curl -k
# ✅ 正确: curl --tlsv1.2
```
