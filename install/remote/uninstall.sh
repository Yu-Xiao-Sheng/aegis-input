#!/bin/bash
# Aegis Input 卸载脚本
# 用于完全卸载 Aegis Input 及其相关文件

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ============================================================================
# 辅助函数
# ============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

abort() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
    exit 1
}

# 检查是否以 root 权限运行
check_root() {
    if [[ $EUID -ne 0 ]]; then
        abort "此脚本需要 root 权限运行。请使用 sudo。"
    fi
}

# ============================================================================
# 卸载函数
# ============================================================================

stop_service() {
    log_info "停止 aegis-input 服务..."

    if systemctl is-active --quiet aegis-input 2>/dev/null; then
        systemctl stop aegis-input
        log_success "服务已停止"
    else
        log_warning "服务未运行"
    fi
}

disable_service() {
    log_info "禁用 aegis-input 服务..."

    if systemctl is-enabled --quiet aegis-input 2>/dev/null; then
        systemctl disable aegis-input
        log_success "服务已禁用"
    else
        log_warning "服务未启用"
    fi
}

remove_binary() {
    log_info "删除二进制文件..."

    if [[ -f /usr/local/bin/aegis-input ]]; then
        rm -f /usr/local/bin/aegis-input
        log_success "二进制文件已删除"
    else
        log_warning "二进制文件不存在"
    fi
}

remove_service_file() {
    log_info "删除 systemd 服务文件..."

    if [[ -f /etc/systemd/system/aegis-input.service ]]; then
        rm -f /etc/systemd/system/aegis-input.service
        log_success "服务文件已删除"
    else
        log_warning "服务文件不存在"
    fi
}

remove_config_and_data() {
    log_info "删除配置和数据文件..."

    local removed=false

    if [[ -d /etc/aegis-input ]]; then
        rm -rf /etc/aegis-input
        log_success "配置目录已删除: /etc/aegis-input"
        removed=true
    fi

    if [[ -d /var/lib/aegis-input ]]; then
        rm -rf /var/lib/aegis-input
        log_success "数据目录已删除: /var/lib/aegis-input"
        removed=true
    fi

    if [[ "$removed" == false ]]; then
        log_warning "没有找到配置和数据文件"
    fi
}

reload_systemd() {
    log_info "重载 systemd..."

    systemctl daemon-reload
    log_success "systemd 已重载"
}

remove_user() {
    log_info "删除 aegis-input 系统用户..."

    if id "aegis-input" &>/dev/null; then
        userdel aegis-input 2>/dev/null || true
        log_success "系统用户已删除"
    else
        log_warning "系统用户不存在"
    fi
}

remove_group() {
    log_info "删除 aegis-input 系统组..."

    if getent group "aegis-input" &>/dev/null; then
        groupdel aegis-input 2>/dev/null || true
        log_success "系统组已删除"
    else
        log_warning "系统组不存在"
    fi
}

show_summary() {
    echo ""
    echo "=========================================="
    echo "卸载完成！"
    echo "=========================================="
    echo ""
    echo "已删除："
    echo "  - 二进制文件: /usr/local/bin/aegis-input"
    echo "  - 服务文件: /etc/systemd/system/aegis-input.service"
    echo "  - 配置目录: /etc/aegis-input"
    echo "  - 数据目录: /var/lib/aegis-input"
    echo "  - 系统用户: aegis-input (如果存在)"
    echo "  - 系统组: aegis-input (如果存在)"
    echo ""
    echo "感谢您使用 Aegis Input！"
    echo ""
}

# ============================================================================
# 主函数
# ============================================================================

main() {
    echo "=========================================="
    echo "Aegis Input 卸载脚本"
    echo "=========================================="
    echo ""
    echo "此脚本将完全卸载 Aegis Input 及其所有相关文件。"
    echo ""

    # 交互式确认
    if [[ "$1" != "--yes" ]] && [[ "$1" != "-y" ]]; then
        echo -n "确定要继续吗? (y/N) "
        read -r response
        if [[ ! "$response" =~ ^[Yy]$ ]]; then
            echo "卸载已取消"
            exit 0
        fi
    fi

    echo ""
    log_info "开始卸载..."

    # 检查 root 权限
    check_root

    # 执行卸载步骤
    stop_service
    disable_service
    remove_binary
    remove_service_file
    remove_config_and_data
    reload_systemd
    remove_user
    remove_group

    # 显示摘要
    show_summary
}

# ============================================================================
# 执行主函数
# ============================================================================

main "$@"
