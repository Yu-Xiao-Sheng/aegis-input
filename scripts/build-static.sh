#!/bin/bash
# 静态链接构建脚本
# 用法: ./scripts/build-static.sh [target]

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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

# 默认目标
TARGET=${1:-"x86_64-unknown-linux-musl"}

# 支持的目标列表
VALID_TARGETS=(
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-musl"
)

# 验证目标
if [[ ! " ${VALID_TARGETS[@]} " =~ " ${TARGET} " ]]; then
    log_error "无效的目标: $TARGET"
    log_info "支持的目标: ${VALID_TARGETS[@]}"
    exit 1
fi

log_info "开始构建静态链接二进制"
log_info "目标: $TARGET"

# 检查 rustup 和 target
if ! command -v rustup &> /dev/null; then
    log_error "rustup 未安装"
    exit 1
fi

# 安装 target（如果需要）
if ! rustup target list --installed | grep -q "^${TARGET}$"; then
    log_info "安装 target: $TARGET"
    rustup target add "$TARGET"
fi

# 检查 musl 工具链
case "$TARGET" in
    x86_64-unknown-linux-musl)
        if ! dpkg -l | grep -q musl-dev; then
            log_warn "musl-dev 未安装"
            log_info "安装命令: sudo apt-get install musl-tools musl-dev"
        fi
        ;;
    aarch64-unknown-linux-musl)
        if ! dpkg -l | grep -q gcc-aarch64-linux-gnu; then
            log_warn "gcc-aarch64-linux-gnu 未安装"
            log_info "安装命令: sudo apt-get install gcc-aarch64-linux-gnu"
        fi
        ;;
esac

# 构建
log_info "编译中..."
cargo build --release --target "$TARGET"

BINARY="target/$TARGET/release/aegis-input"

if [ ! -f "$BINARY" ]; then
    log_error "二进制文件未找到: $BINARY"
    exit 1
fi

# 显示二进制信息
log_info "构建成功！"
echo ""
echo "二进制信息:"
echo "  路径: $BINARY"
echo "  大小: $(ls -lh "$BINARY" | awk '{print $5}')"
echo "  架构: $(file "$BINARY" | cut -d: -f2 | xargs)"
echo ""

# 验证静态链接
log_info "验证静态链接..."
if ldd "$BINARY" 2>&1 | grep -q "not a dynamic executable"; then
    echo "  ✓ 完全静态链接"
else
    echo "  ⚠ 部分动态链接"
    echo "  动态依赖:"
    ldd "$BINARY" 2>&1 | grep -v "not a dynamic" | sed 's/^/    /' || true
fi

# 在 Alpine 中测试（如果有 Docker）
if command -v docker &> /dev/null; then
    log_info "在 Alpine 容器中测试..."
    if docker run --rm -v "$(pwd):/app" alpine:latest /app/"$BINARY" --version &> /dev/null; then
        echo "  ✓ Alpine 兼容性测试通过"
    else
        echo "  ⚠ Alpine 兼容性测试失败"
    fi
fi

echo ""
log_info "构建完成！"
log_info "运行: $BINARY"
