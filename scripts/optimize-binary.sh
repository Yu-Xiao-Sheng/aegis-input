#!/bin/bash
# 二进制优化脚本
# 用法: ./scripts/optimize-binary.sh <binary-path>

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_section() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

BINARY="$1"

if [ -z "$BINARY" ]; then
    log_error "请提供二进制文件路径"
    echo "用法: $0 <binary-path>"
    exit 1
fi

if [ ! -f "$BINARY" ]; then
    log_error "文件不存在: $BINARY"
    exit 1
fi

# 备份原始文件
BACKUP="${BINARY}.backup"
cp "$BINARY" "$BACKUP"

log_section "二进制优化"
echo ""

# 记录原始大小
ORIGINAL_SIZE=$(stat -f%z "$BINARY" 2>/dev/null || stat -c%s "$BINARY" 2>/dev/null)
ORIGINAL_SIZE_MB=$(echo "scale=2; $ORIGINAL_SIZE / 1024 / 1024" | bc)

log_info "原始大小: $ORIGINAL_SIZE_MB MB"
echo ""

# Step 1: Strip 符号
log_section "Step 1: 移除符号表"
if command -v strip &> /dev/null; then
    if file "$BINARY" | grep -q "stripped"; then
        log_info "符号表已移除，跳过"
    else
        log_info "移除符号表..."
        strip --strip-all "$BINARY"
        STRIPPED_SIZE=$(stat -f%z "$BINARY" 2>/dev/null || stat -c%s "$BINARY" 2>/dev/null)
        STRIPPED_SIZE_MB=$(echo "scale=2; $STRIPPED_SIZE / 1024 / 1024" | bc)
        REDUCTION=$(echo "scale=1; ($ORIGINAL_SIZE - $STRIPPED_SIZE) / $ORIGINAL_SIZE * 100" | bc)
        log_info "完成! 大小: $STRIPPED_SIZE_MB MB (减少 ${REDUCTION}%)"
    fi
else
    log_warn "strip 命令不可用"
fi
echo ""

# Step 2: UPX 压缩
log_section "Step 2: UPX 压缩"
if command -v upx &> /dev/null; then
    log_info "使用 UPX 压缩..."

    # 检查是否已压缩
    if file "$BINARY" | grep -q "UPX"; then
        log_info "文件已使用 UPX 压缩，跳过"
    else
        # 使用最佳压缩
        upx --best --lzma "$BINARY" 2>&1 | sed 's/^/  /'

        COMPRESSED_SIZE=$(stat -f%z "$BINARY" 2>/dev/null || stat -c%s "$BINARY" 2>/dev/null)
        COMPRESSED_SIZE_MB=$(echo "scale=2; $COMPRESSED_SIZE / 1024 / 1024" | bc)
        TOTAL_REDUCTION=$(echo "scale=1; ($ORIGINAL_SIZE - $COMPRESSED_SIZE) / $ORIGINAL_SIZE * 100" | bc)

        log_info "压缩完成! 大小: $COMPRESSED_SIZE_MB MB (总计减少 ${TOTAL_REDUCTION}%)"
    fi
else
    log_warn "UPX 不可用"
    log_info "安装命令:"
    echo "  Ubuntu/Debian: sudo apt-get install upx"
    echo "  Arch: sudo pacman -S upx"
    echo "  macOS: brew install upx"
fi
echo ""

# Step 3: 测试压缩后的二进制
log_section "Step 3: 功能验证"
if [ -x "$BINARY" ]; then
    log_info "测试基本功能..."

    # 尝试运行 --version
    if "$BINARY" --version &> /dev/null; then
        log_info "✓ 功能正常"
    else
        log_warn "⚠ --version 测试失败（可能没有 --version 选项）"
    fi
else
    log_warn "⚠ 文件不可执行"
fi
echo ""

# 最终报告
log_section "优化报告"
echo ""
echo "  原始大小: $ORIGINAL_SIZE_MB MB"
if [ -n "${STRIPPED_SIZE_MB:-}" ]; then
    echo "  Strip 后: $STRIPPED_SIZE_MB MB"
fi
if [ -n "${COMPRESSED_SIZE_MB:-}" ]; then
    echo "  最终大小: $COMPRESSED_SIZE_MB MB"
    echo "  总减少: ${TOTAL_REDUCTION}%"
fi
echo ""
echo "  备份位置: $BACKUP"
echo ""

log_info "优化完成!"
echo ""
log_info "恢复原始文件:"
echo "  mv $BACKUP $BINARY"
echo ""
