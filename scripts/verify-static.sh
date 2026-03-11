#!/bin/bash
# 静态链接验证脚本
# 用法: ./scripts/verify-static.sh <binary-path>

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

log_section "静态链接验证"
echo ""

# 1. 检查文件类型
log_section "1. 文件类型"
file_output=$(file "$BINARY")
echo "$file_output"
echo ""

# 2. 检查架构
log_section "2. 目标架构"
if echo "$file_output" | grep -q "x86-64"; then
    echo "  架构: x86_64 (AMD64)"
elif echo "$file_output" | grep -q "aarch64"; then
    echo "  架构: aarch64 (ARM64)"
elif echo "$file_output" | grep -q "ARM"; then
    echo "  架构: ARM (32-bit)"
else
    echo "  架构: 未知"
fi
echo ""

# 3. 检查静态链接
log_section "3. 静态链接检查"
if ldd "$BINARY" 2>&1 | grep -q "not a dynamic executable"; then
    echo "  ✓ 完全静态链接（无动态依赖）"
else
    echo "  ⚠ 存在动态依赖"
    echo ""
    echo "  动态依赖列表:"
    ldd "$BINARY" 2>&1 | grep -v "not a dynamic" | sed 's/^/    /' || true
fi
echo ""

# 4. 检查 strip 状态
log_section "4. 符号表检查"
if file "$BINARY" | grep -q "stripped"; then
    echo "  ✓ 符号表已移除（strip）"
else
    echo "  ⚠ 符号表未移除"
    echo "  建议: strip --strip-all $BINARY"
fi
echo ""

# 5. 检查是否可执行
log_section "5. 可执行性检查"
if [ -x "$BINARY" ]; then
    echo "  ✓ 可执行权限已设置"
else
    echo "  ⚠ 缺少可执行权限"
    echo "  建议: chmod +x $BINARY"
fi
echo ""

# 6. 二进制大小
log_section "6. 二进制大小"
size=$(ls -lh "$BINARY" | awk '{print $5}')
size_bytes=$(stat -f%z "$BINARY" 2>/dev/null || stat -c%s "$BINARY" 2>/dev/null)
echo "  大小: $size ($size_bytes bytes)"

# 大小评估
size_mb=$(echo "scale=2; $size_bytes / 1024 / 1024" | bc)
if (( $(echo "$size_mb < 2" | bc -l) )); then
    echo "  评级: ✓ 优秀（小于 2MB）"
elif (( $(echo "$size_mb < 5" | bc -l) )); then
    echo "  评级: ✓ 良好（小于 5MB）"
elif (( $(echo "$size_mb < 10" | bc -l) )); then
    echo "  评级: ⚠ 一般（小于 10MB）"
else
    echo "  评级: ⚠ 较大（超过 10MB）"
    echo "  建议: 使用 UPX 压缩或检查依赖"
fi
echo ""

# 7. 在 Alpine 中测试
if command -v docker &> /dev/null; then
    log_section "7. Alpine 兼容性测试"
    echo "  在 Alpine 容器中运行..."

    # 测试 --version
    if docker run --rm -v "$(dirname "$BINARY"):/app" alpine:latest \
        /app/$(basename "$BINARY") --version &> /dev/null; then
        echo "  ✓ 基本功能测试通过"
    else
        echo "  ⚠ 基本功能测试失败"
    fi

    # 再次检查 ldd
    echo "  在 Alpine 中检查动态依赖..."
    if docker run --rm -v "$(dirname "$BINARY"):/app" alpine:latest \
        sh -c "ldd /app/$(basename "$BINARY") 2>&1 | grep -q 'not a dynamic'"; then
        echo "  ✓ Alpine 确认为静态链接"
    else
        echo "  ⚠ Alpine 检测到动态依赖"
    fi
else
    log_section "7. Alpine 兼容性测试"
    echo "  Docker 不可用，跳过测试"
    echo "  安装 Docker 以验证兼容性"
fi
echo ""

# 8. 优化建议
log_section "8. 优化建议"

# 检查是否已 strip
if ! file "$BINARY" | grep -q "stripped"; then
    echo "  • 移除符号表: strip --strip-all $BINARY"
fi

# 检查大小
if (( $(echo "$size_mb > 2" | bc -l) )); then
    echo "  • 使用 UPX 压缩: upx --best --lzma $BINARY"
fi

# 检查是否为 musl
if ! echo "$file_output" | grep -q "musl"; then
    echo "  • 考虑使用 musl 目标以获得更好的静态链接支持"
fi

echo ""

log_section "验证完成"
