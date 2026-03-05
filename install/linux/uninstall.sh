#!/bin/bash
# Aegis-Input 卸载脚本
# 用于卸载并清理 aegis-input 服务

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

# 确认卸载
confirm_uninstall() {
    echo "=================================="
    echo "Aegis-Input 卸载脚本"
    echo "=================================="
    echo ""
    echo "此脚本将执行以下操作:"
    echo "  1. 停止 aegis-input 服务"
    echo "  2. 禁用开机自启"
    echo "  3. 删除 systemd 单元文件"
    echo "  4. 删除系统用户"
    echo "  5. 删除配置文件"
    echo "  6. 删除状态文件"
    echo "  7. 删除二进制文件"
    echo ""
    echo "警告：此操作不可逆！"
    echo ""
    read -p "确认要继续卸载吗？(y/N): " -n 1 -r
    echo

    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "卸载已取消"
        exit 0
    fi
}

# 检查是否为 root 用户
check_root() {
    if [[ $EUID -ne 0 ]]; then
        log_error "此脚本必须使用 root 权限运行"
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

# 停止服务
stop_service() {
    if systemctl is-active --quiet aegis-input; then
        log_info "正在停止服务..."
        systemctl stop aegis-input
        log_info "服务已停止"
    fi

    if systemctl is-enabled --quiet aegis-input; then
        log_info "正在禁用开机自启..."
        systemctl disable aegis-input
        log_info "已禁用开机自启"
    fi
}

# 删除 systemd 单元文件
remove_systemd_unit() {
    if [[ -f /etc/systemd/system/aegis-input.service ]]; then
        log_info "正在删除 systemd 单元文件..."
        rm -f /etc/systemd/system/aegis-input.service
        systemctl daemon-reload
        systemctl reset-failed
        log_info "systemd 单元文件已删除"
    else
        log_warn "systemd 单元文件不存在"
    fi
}

# 删除系统用户
remove_system_user() {
    if id -u aegis-input >/dev/null 2>&1; then
        log_info "正在删除系统用户..."
        userdel aegis-input
        log_info "系统用户已删除"
    else
        log_warn "系统用户不存在"
    fi
}

# 删除配置文件
remove_config() {
    if [[ -d /etc/aegis-input ]]; then
        log_info "正在删除配置文件..."
        rm -rf /etc/aegis-input
        log_info "配置文件已删除"
    else
        log_warn "配置文件不存在"
    fi
}

# 删除状态文件
remove_status() {
    if [[ -d /var/lib/aegis-input ]]; then
        log_info "正在删除状态文件..."
        rm -rf /var/lib/aegis-input
        log_info "状态文件已删除"
    else
        log_warn "状态文件不存在"
    fi
}

# 删除二进制文件
remove_binary() {
    if [[ -f /usr/local/bin/aegis-input ]]; then
        log_info "正在删除二进制文件..."
        rm -f /usr/local/bin/aegis-input
        log_info "二进制文件已删除"
    else
        log_warn "二进制文件不存在"
    fi
}

# 检查残留文件
check_residual_files() {
    log_info "检查残留文件..."

    local residual_files=()
    local directories=(
        "/etc/aegis-input"
        "/var/lib/aegis-input"
        "/usr/local/bin/aegis-input"
        "/etc/systemd/system/aegis-input.service"
    )

    for dir in "${directories[@]}"; do
        if [[ -e "$dir" ]]; then
            residual_files+=("$dir")
        fi
    done

    if [[ ${#residual_files[@]} -eq 0 ]]; then
        log_info "所有文件已清理完毕"
    else
        log_warn "发现残留文件:"
        for file in "${residual_files[@]}"; do
            echo "  $file"
        done
        echo ""
        read -p "是否手动清理这些残留文件？(y/N): " -n 1 -r
        echo

        if [[ $REPLY =~ ^[Yy]$ ]]; then
            for file in "${residual_files[@]}"; do
                if [[ -d "$file" ]]; then
                    rm -rf "$file"
                else
                    rm -f "$file"
                fi
                log_info "已删除: $file"
            done
        fi
    fi
}

# 主卸载流程
main() {
    # 检查依赖
    check_root
    check_systemd

    # 确认卸载
    confirm_uninstall

    # 执行卸载步骤
    stop_service
    remove_systemd_unit
    remove_system_user
    remove_config
    remove_status
    remove_binary

    # 检查残留文件
    check_residual_files

    echo ""
    echo "=================================="
    echo log_info "卸载完成！"
    echo ""
    echo "如需重新安装，请运行:"
    echo "  sudo ./install/linux/install.sh"
    echo "=================================="
}

# 运行主流程
main "$@"