#!/bin/bash
# Aegis Input 一键安装脚本
# 通过 curl | bash 安装预编译二进制文件

set -e

# ============================================================================
# 配置和常量
# ============================================================================

REPO="${REPO:-Yu-Xiao-Sheng/aegis-input}"
DOWNLOAD_BASE_URL="${DOWNLOAD_BASE_URL:-https://github.com/${REPO}/releases/download}"
API_BASE_URL="${API_BASE_URL:-https://api.github.com/repos/${REPO}}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
BINARY_NAME="aegis-input"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ============================================================================
# 日志函数
# ============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

abort() {
    log_error "$@"
    exit 1
}

# ============================================================================
# 版本比较函数
# ============================================================================

# 解析版本号
parse_version() {
    local version="$1"
    # 移除 'v' 前缀
    version="${version#v}"
    echo "$version"
}

# 比较版本号
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

# 版本相等
version_eq() {
    [[ "$1" == "$2" ]]
}

# 大于等于
version_ge() {
    version_eq "$1" "$2" || version_gt "$1" "$2"
}

# 小于
version_lt() {
    ! version_ge "$1" "$2"
}

# 获取更新类型
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

# ============================================================================
# 系统检测函数
# ============================================================================

# 检测 CPU 架构
detect_architecture() {
    local uname_machine="$(uname -m)"
    local arch=""

    case "$uname_machine" in
        x86_64|amd64)
            arch="x86_64-unknown-linux-gnu"
            ;;
        aarch64|arm64)
            arch="aarch64-unknown-linux-gnu"
            ;;
        armv7l|armhf)
            arch="armv7-unknown-linux-gnueabihf"
            ;;
        i386|i686)
            abort "不支持的 32 位 x86 架构: $uname_machine"
            ;;
        *)
            abort "不支持的架构: $uname_machine"
            ;;
    esac

    echo "$arch"
}

# 检测操作系统
detect_os() {
    local os="$(uname)"
    local os_id=""

    if [[ "$os" == "Linux" ]]; then
        # 检测发行版
        if [[ -f /etc/os-release ]]; then
            . /etc/os-release
            os_id="$ID"
        fi
        echo "linux:$os_id"
    elif [[ "$os" == "Darwin" ]]; then
        echo "macos"
    else
        abort "不支持的操作系统: $os"
    fi
}

# 检测 libc 类型
detect_libc() {
    if ldd --version 2>&1 | grep -q "musl"; then
        echo "musl"
    elif ldd --version 2>&1 | grep -q "GLIBC"; then
        echo "glibc"
    else
        # 检查动态加载器
        if command -v /lib/ld-musl-*.so.1 >/dev/null 2>&1; then
            echo "musl"
        else
            echo "glibc"
        fi
    fi
}

# 获取系统信息
get_system_info() {
    local os_info=$(detect_os)
    local arch=$(detect_architecture)
    local libc=$(detect_libc)

    echo "os:$os_info arch:$arch libc:$libc"
}

# ============================================================================
# 下载和验证函数
# ============================================================================

# 检查 GitHub API 速率限制
check_github_rate_limit() {
    if ! command -v jq >/dev/null 2>&1; then
        return 0  # 跳过检查（jq 未安装）
    fi

    local rate_limit=$(curl -s "$API_BASE_URL/rate_limit" 2>/dev/null || echo "{}")
    local remaining=$(echo "$rate_limit" | jq -r '.rate.remaining' 2>/dev/null || echo "60")
    local reset=$(echo "$rate_limit" | jq -r '.rate.reset' 2>/dev/null || echo "0")
    local limit=$(echo "$rate_limit" | jq -r '.rate.limit' 2>/dev/null || echo "60")

    if [[ "$remaining" -lt 10 ]]; then
        local reset_time=$(date -d "@$reset" "+%H:%M:%S" 2>/dev/null || date -r "$reset" "+%H:%M:%S" 2>/dev/null || echo "unknown")
        # 输出到 stderr 避免干扰版本检测
        echo "[WARN] GitHub API 速率限制较低（剩余 $remaining / $limit）" >&2
        echo "[WARN] 重置时间: $reset_time" >&2
    fi
}

# 下载文件（带重试）
download_file() {
    local url="$1"
    local output="$2"
    local max_retries="${3:-5}"
    local retry_count=0
    local pause=2

    while [[ $retry_count -lt $max_retries ]]; do
        if curl --proto '=https' --tlsv1.2 -sSf "$url" -o "$output"; then
            return 0
        fi

        retry_count=$((retry_count + 1))
        if [[ $retry_count -lt $max_retries ]]; then
            log_warn "下载失败，${pause}秒后重试 ($retry_count/$max_retries)..."
            sleep "$pause"
            pause=$((pause * 2))  # 指数退避
        fi
    done

    abort "下载失败，已重试 $max_retries 次"
}

# 验证 SHA256 校验和
verify_sha256() {
    local file="$1"
    local expected_checksum="$2"
    local actual_checksum=""

    if [[ ! -f "$file" ]]; then
        abort "文件不存在: $file"
    fi

    actual_checksum=$(sha256sum "$file" | awk '{print $1}')

    if [[ "$actual_checksum" != "$expected_checksum" ]]; then
        abort "SHA256 校验和验证失败！\n期望: $expected_checksum\n实际: $actual_checksum"
    fi

    log_info "SHA256 校验和验证通过"
}

# 下载并验证二进制文件
download_and_verify() {
    local version="$1"
    local arch="$2"
    local tmp_dir="${3:-/tmp}"

    local filename="aegis-input-${arch}.tar.gz"
    local download_url="${DOWNLOAD_BASE_URL}/v${version}/${filename}"
    local checksum_url="${DOWNLOAD_BASE_URL}/v${version}/SHA256SUMS.txt"
    local output_file="${tmp_dir}/${filename}"
    local checksum_file="${tmp_dir}/SHA256SUMS.txt"

    # 下载二进制文件
    log_info "下载 ${filename}..."
    download_file "$download_url" "$output_file"

    # 下载校验和文件
    log_info "下载 SHA256SUMS.txt..."
    download_file "$checksum_url" "$checksum_file"

    # 提取对应架构的校验和
    local expected_checksum=$(grep "$filename" "$checksum_file" | awk '{print $1}')

    if [[ -z "$expected_checksum" ]]; then
        abort "未找到 ${filename} 的校验和"
    fi

    # 验证校验和
    verify_sha256 "$output_file" "$expected_checksum"

    echo "$output_file"
}

# ============================================================================
# 安装功能函数
# ============================================================================

# 获取最新版本
get_latest_version() {
    # 静默检查速率限制（不输出警告）
    if ! command -v jq >/dev/null 2>&1; then
        :  # 跳过检查
    else
        local rate_limit=$(curl -s "$API_BASE_URL/rate_limit" 2>/dev/null || echo "{}")
        local remaining=$(echo "$rate_limit" | jq -r '.rate.remaining' 2>/dev/null || echo "60")
        if [[ "$remaining" -lt 10 ]]; then
            local reset=$(echo "$rate_limit" | jq -r '.rate.reset' 2>/dev/null || echo "0")
            local limit=$(echo "$rate_limit" | jq -r '.rate.limit' 2>/dev/null || echo "60")
            local reset_time=$(date -d "@$reset" "+%H:%M:%S" 2>/dev/null || date -r "$reset" "+%H:%M:%S" 2>/dev/null || echo "unknown")
            # 输出到 stderr
            echo "[WARN] GitHub API 速率限制较低（剩余 $remaining / $limit）" >&2
            echo "[WARN] 重置时间: $reset_time" >&2
        fi
    fi

    local latest_url="${API_BASE_URL}/releases/latest"
    local tag_name=$(curl -s "$latest_url" 2>/dev/null | grep -oP '"tag_name":\s*"\K[^"]*')

    if [[ -z "$tag_name" ]]; then
        abort "无法获取最新版本信息"
    fi

    echo "${tag_name#v}"
}

# 获取当前版本
get_current_version() {
    if [[ -f /var/lib/aegis-input/install.toml ]]; then
        grep "version" /var/lib/aegis-input/install.toml 2>/dev/null | cut -d'"' -f2 || echo ""
    elif command -v aegis-input >/dev/null 2>&1; then
        aegis-input --version 2>/dev/null | grep -oP 'version\s+\K[0-9.]+' || echo ""
    else
        echo ""
    fi
}

# 安装二进制文件
install_binary() {
    local version="$1"
    local arch="$2"
    local tmp_dir="${3:-/tmp}"

    log_info "下载 aegis-input $version ($arch)..."

    # 创建临时目录并设置清理陷阱
    local tmp_binary="${tmp_dir}/aegis-install-$$"
    mkdir -p "$tmp_binary"
    trap 'rm -rf "$tmp_binary"' EXIT

    # 下载并验证
    local binary_file=$(download_and_verify "$version" "$arch" "$tmp_binary")

    # 解压
    log_info "解压二进制文件..."
    tar xzf "$binary_file" -C "$tmp_binary"

    # 检查二进制文件是否存在
    if [[ ! -f "$tmp_binary/aegis-input" ]]; then
        abort "解压后的二进制文件不存在"
    fi

    # 验证二进制文件可执行
    chmod +x "$tmp_binary/aegis-input"

    # 检查是否需要 sudo
    local need_sudo=false
    if [[ ! -w "$INSTALL_DIR" ]]; then
        need_sudo=true
    fi

    # 安装二进制
    log_info "安装到 $INSTALL_DIR..."
    if [[ "$need_sudo" == "true" ]]; then
        sudo mkdir -p "$INSTALL_DIR"
        sudo cp "$tmp_binary/aegis-input" "$INSTALL_DIR/"
        sudo chmod +x "$INSTALL_DIR/aegis-input"
    else
        mkdir -p "$INSTALL_DIR"
        cp "$tmp_binary/aegis-input" "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/aegis-input"
    fi

    # 写入安装状态
    write_install_state "$version" "$arch"

    log_info "安装完成！"

    # 清理（陷阱会自动清理）
    trap - EXIT
    rm -rf "$tmp_binary"
}

# 写入安装状态
write_install_state() {
    local version="$1"
    local arch="$2"
    local install_dir="/var/lib/aegis-input"

    if [[ ! -d "$install_dir" ]]; then
        sudo mkdir -p "$install_dir"
    fi

    local state_file="${install_dir}/install.toml"
    local install_time=$(date -Iseconds 2>/dev/null || date)
    local install_source="${DOWNLOAD_BASE_URL}/install.sh"

    sudo tee "$state_file" > /dev/null << EOF
[installation]
version = "$version"
install_time = "$install_time"
install_method = "one-click"
install_source = "$install_source"
architecture = "$arch"
binary_path = "$INSTALL_DIR/aegis-input"
installed_by = "$USER"
EOF
}

# 创建 systemd 服务
create_systemd_unit() {
    if ! command -v systemctl >/dev/null 2>&1; then
        log_warn "未检测到 systemd，跳过服务安装"
        return 0
    fi

    local service_file="/etc/systemd/system/aegis-input.service"

    # 检查服务是否已存在
    if [[ -f "$service_file" ]]; then
        log_info "systemd 服务已存在"

        # 如果服务正在运行，先停止
        if systemctl is-active --quiet aegis-input; then
            log_info "停止现有服务..."
            sudo systemctl stop aegis-input
        fi
    else
        log_info "创建 systemd 服务..."

        sudo tee "$service_file" > /dev/null << EOF
[Unit]
Description=Aegis Input Service
Documentation=man:aegis-input(8)
After=graphical.target network.target
Wants=graphical.target

[Service]
Type=simple
ExecStart=$INSTALL_DIR/aegis-input
User=root
Group=input
Environment=AEGIS_INPUT_CONFIG=/etc/aegis-input/config.toml
Environment=AEGIS_INPUT_STATUS=/var/lib/aegis-input/status.toml
Restart=on-failure
RestartSec=5s

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/aegis-input /etc/aegis-input

[Install]
WantedBy=multi-user.target
EOF

        sudo chmod 644 "$service_file"
    fi

    # 重载 systemd
    log_info "重载 systemd..."
    sudo systemctl daemon-reload

    # 启用服务
    log_info "启用服务..."
    sudo systemctl enable aegis-input

    # 启动服务
    log_info "启动服务..."
    sudo systemctl start aegis-input

    # 显示状态
    sleep 1
    if systemctl is-active --quiet aegis-input; then
        log_info "服务状态: active (running)"
    else
        log_warn "服务状态: inactive"
    fi
}

# 显示版本信息
show_version_info() {
    local version="$1"
    local arch="$2"

    echo ""
    echo "=========================================="
    echo "安装成功！"
    echo "=========================================="
    echo "版本: $version"
    echo "架构: $arch"
    echo "路径: $INSTALL_DIR/aegis-input"
    echo ""
    echo "使用方法:"
    echo "  查看版本: aegis-input --version"
    echo "  查看状态: systemctl status aegis-input"
    echo "  查看日志: journalctl -u aegis-input -f"
    echo ""
}

# ============================================================================
# 主函数
# ============================================================================

main() {
    local VERSION=""
    local ARCH=""
    local DRY_RUN=false
    local YES=false
    local NO_SYSTEMD=false
    local CHECK_ONLY=false

    # 解析参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            --version)
                VERSION="$2"
                shift 2
                ;;
            -v|--version-check)
                CHECK_ONLY=true
                shift
                ;;
            --target)
                ARCH="$2"
                shift 2
                ;;
            --dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --yes|-y)
                YES=true
                shift
                ;;
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            --no-systemd)
                NO_SYSTEMD=true
                shift
                ;;
            --help|-h)
                echo "用法: $0 [选项]"
                echo ""
                echo "选项:"
                echo "  --version <VER>    指定安装版本"
                echo "  --target <TRIPLE>  指定目标架构"
                echo "  --dir <PATH>       安装目录（默认: /usr/local/bin）"
                echo "  --yes, -y          非交互模式"
                echo "  --dry-run          模拟运行"
                echo "  --no-systemd       跳过 systemd 服务"
                echo "  --version-check, -v 检查新版本"
                echo "  --help, -h         显示帮助"
                exit 0
                ;;
            *)
                abort "未知参数: $1（使用 --help 查看帮助）"
                ;;
        esac
    done

    # 检查版本模式
    if [[ "$CHECK_ONLY" == "true" ]]; then
        local current=$(get_current_version)
        local latest=$(get_latest_version)

        echo "当前版本: ${current:-未安装}"
        echo "最新版本: $latest"

        if version_gt "$latest" "$current"; then
            echo ""
            echo "新版本可用！"
            echo ""
            echo "升级:"
            echo "  curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/${REPO}/main/install/remote/install.sh | bash"
            return 0
        else
            echo "已是最新版本"
            return 1
        fi
    fi

    # 确定版本
    if [[ -z "$VERSION" ]]; then
        VERSION=$(get_latest_version)
    fi

    # 移除可能的 v 前缀
    VERSION="${VERSION#v}"

    log_info "Aegis Input 一键安装脚本"
    log_info "版本: $VERSION"

    # 检测架构
    if [[ -z "$ARCH" ]]; then
        ARCH=$(detect_architecture)
    fi

    log_info "架构: $ARCH"

    # 检测操作系统
    local os_info=$(detect_os)
    log_info "系统: $os_info"

    # 验证操作系统
    if [[ ! "$os_info" =~ ^linux: ]]; then
        abort "当前仅支持 Linux 系统，检测到: $os_info"
    fi

    # 检查权限
    if [[ ! -w "$INSTALL_DIR" ]] && [[ "$DRY_RUN" == "false" ]]; then
        log_warn "需要 sudo 权限来安装到 $INSTALL_DIR"
    fi

    # 模拟模式
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "[模拟] 将下载 aegis-input $VERSION ($ARCH)"
        log_info "[模拟] 将安装到 $INSTALL_DIR"
        return 0
    fi

    # 确认安装（交互模式）
    if [[ "$YES" == "false" ]]; then
        echo ""
        echo "即将安装 aegis-input $VERSION"
        echo "目标架构: $ARCH"
        echo "安装位置: $INSTALL_DIR"
        echo ""
        read -p "继续？ [Y/n] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]] && [[ ! -z "$REPLY" ]]; then
            log_info "安装已取消"
            exit 0
        fi
    fi

    # 安装二进制
    install_binary "$VERSION" "$ARCH"

    # 创建 systemd 服务
    if [[ "$NO_SYSTEMD" == "false" ]]; then
        create_systemd_unit
    fi

    # 显示版本信息
    show_version_info "$VERSION" "$ARCH"

    # 验证安装
    if command -v aegis-input >/dev/null 2>&1; then
        local installed_version=$(aegis-input --version 2>/dev/null || echo "unknown")
        log_info "验证成功: $installed_version"
    fi
}

# 如果直接执行此脚本，运行主函数
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
