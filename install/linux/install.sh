#!/bin/bash
# Aegis-Input 安装脚本
# 用于快速安装并启动 aegis-input 服务

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查是否为 root 用户
check_root() {
    if [[ $EUID -ne 0 ]]; then
        log_error "此脚本必须使用 root 权限运行"
        exit 1
    fi
}

# 检查是否为 Debian/Ubuntu/Mint
check_distribution() {
    if ! command -v apt-get >/dev/null 2>&1; then
        log_error "仅支持 Debian/Ubuntu/Mint 系统"
        exit 1
    fi
}

# 检查 systemd 是否可用
check_systemd() {
    if ! command -v systemctl >/dev/null 2>&1; then
        log_error "systemd 未安装或不可用"
        exit 1
    fi
}

# 检查是否已经安装
check_installed() {
    if systemctl is-active --quiet aegis-input; then
        log_info "aegis-input 已经在运行"
        echo "状态: $(systemctl is-active aegis-input)"
        echo "开机自启: $(systemctl is-enabled aegis-input)"
        exit 0
    elif [[ -f /etc/systemd/system/aegis-input.service ]]; then
        log_warn "aegis-input 已安装但未运行"
        echo "运行以下命令启动服务:"
        echo "  sudo systemctl start aegis-input"
        echo "启用开机自启:"
        echo "  sudo systemctl enable aegis-input"
        exit 0
    fi
}

# 检查 rust 和 cargo
check_rust() {
    if ! command -v cargo >/dev/null 2>&1; then
        log_warn "未找到 cargo，正在安装 Rust..."

        # 安装 Rust
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

        # 加载环境
        source $HOME/.cargo/env

        # 再次检查
        if ! command -v cargo >/dev/null 2>&1; then
            log_error "Rust 安装失败"
            exit 1
        fi
    fi
}

# 构建 aegis-input
build_aegis_input() {
    log_info "正在构建 aegis-input..."

    # 进入项目目录
    cd "$(dirname "$0")/.."

    # 构建项目
    if ! cargo build --release; then
        log_error "构建失败"
        exit 1
    fi

    # 复制二进制文件
    mkdir -p /usr/local/bin
    cp target/release/aegis-input /usr/local/bin/

    # 设置权限
    chmod +x /usr/local/bin/aegis-input

    log_info "aegis-input 构建完成"
}

# 创建 systemd 单元文件
create_systemd_unit() {
    log_info "正在创建 systemd 单元文件..."

    cat > /etc/systemd/system/aegis-input.service << 'EOF'
[Unit]
Description=Aegis-Input Service
Documentation=man:aegis-input(8)
After=graphical.target network.target
Wants=graphical.target

[Service]
Type=simple
ExecStart=/usr/local/bin/aegis-input
User=aegis-input
Group=input
Environment=AEGIS_INPUT_CONFIG=/etc/aegis-input/config.toml
Environment=AEGIS_INPUT_STATUS=/var/lib/aegis-input/status.toml
Restart=on-failure
RestartSec=5s

# 设置正确的权限
PermissionsStartOnly=true
ExecStartPre=/usr/bin/chown -R aegis-input:input /etc/aegis-input /var/lib/aegis-input

[Install]
WantedBy=multi-user.target
EOF

    # 设置权限
    chmod 644 /etc/systemd/system/aegis-input.service

    # 重新加载 systemd
    systemctl daemon-reload

    log_info "systemd 单元文件创建完成"
}

# 创建系统用户
create_system_user() {
    log_info "正在创建系统用户..."

    # 创建用户（如果不存在）
    if ! id -u aegis-input >/dev/null 2>&1; then
        useradd --system --no-create-home --shell /usr/sbin/nologin aegis-input
        log_info "创建用户: aegis-input"
    else
        log_info "用户 aegis-input 已存在"
    fi

    # 添加到 input 组（如果不存在）
    if ! getent group input >/dev/null 2>&1; then
        groupadd input
        log_info "创建组: input"
    fi

    usermod -aG input aegis-input
    log_info "用户 aegis-input 已添加到 input 组"
}

# 创建必要的目录
create_directories() {
    log_info "正在创建必要的目录..."

    # 创建配置目录
    mkdir -p /etc/aegis-input
    chmod 755 /etc/aegis-input

    # 创建状态目录
    mkdir -p /var/lib/aegis-input
    chmod 755 /var/lib/aegis-input
    chown aegis-input:input /var/lib/aegis-input

    log_info "目录创建完成"
}

# 创建配置文件
create_config() {
    log_info "正在创建配置文件..."

    cat > /etc/aegis-input/config.toml << 'EOF'
# Aegis-Input 配置文件
# 用于控制外设输入行为

# 日志级别
log_level = "info"

# 外设检测间隔（秒）
device_detection_interval = 5

# 内置设备白名单（正则表达式）
builtin_devices = [
    "AT Translated Set 2 keyboard",
    "SynPS/2 Synaptics TouchPad",
    "ELAN Touchscreen"
]

# 外设黑名单（正则表达式）
external_devices = [
    "Logitech.*",
    ".*keyboard.*",
    ".*mouse.*"
]

# 行为控制
disable_builtin_on_external = true
enable_builtin_on_no_external = true
EOF

    # 设置权限
    chmod 644 /etc/aegis-input/config.toml
    chown aegis-input:input /etc/aegis-input/config.toml

    log_info "配置文件创建完成"
}

# 启动服务
start_service() {
    log_info "正在启动服务..."

    # 启用并启动服务
    systemctl enable aegis-input
    systemctl start aegis-input

    # 检查服务状态
    if systemctl is-active --quiet aegis-input; then
        log_info "服务启动成功"
        echo "服务状态: $(systemctl is-active aegis-input)"
        echo "开机自启: $(systemctl is-enabled aegis-input)"
    else
        log_error "服务启动失败"
        journalctl -u aegis-input --no-pager -n 50
        exit 1
    fi
}

# 主安装流程
main() {
    echo "=================================="
    echo "Aegis-Input 快速安装脚本"
    echo "=================================="

    # 检查依赖
    check_root
    check_distribution
    check_systemd

    # 检查是否已安装
    check_installed

    # 检查 Rust 环境
    check_rust

    # 执行安装步骤
    build_aegis_input
    create_directories
    create_system_user
    create_config
    create_systemd_unit
    start_service

    echo ""
    echo "=================================="
    echo log_info "安装完成！"
    echo ""
    echo "管理命令:"
    echo "  查看状态: systemctl status aegis-input"
    echo "  启动服务: systemctl start aegis-input"
    echo "  停止服务: systemctl stop aegis-input"
    echo "  开机自启: systemctl enable aegis-input"
    echo "  禁用开机自启: systemctl disable aegis-input"
    echo "  查看日志: journalctl -u aegis-input -f"
    echo "=================================="
}

# 运行主流程
main "$@"