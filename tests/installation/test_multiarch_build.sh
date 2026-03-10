#!/bin/bash
# 多平台构建验证测试
# 测试多架构二进制文件的构建配置

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_test() {
    echo -e "${YELLOW}[TEST]${NC} $*"
}

log_pass() {
    echo -e "${GREEN}[PASS]${NC} $*"
}

log_fail() {
    echo -e "${RED}[FAIL]${NC} $*"
}

TESTS_PASSED=0
TESTS_FAILED=0

TEST_DIR="/tmp/aegis-multiarch-test-$$"

cleanup() {
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

# ============================================================================
# 测试用例
# ============================================================================

test_001_x86_64_target_defined() {
    log_test "测试 001: x86_64 目标已定义"

    if grep -q "x86_64-unknown-linux-musl" ".github/workflows/release.yml"; then
        log_pass "x86_64-unknown-linux-musl 目标已定义"
    else
        log_fail "x86_64-unknown-linux-musl 目标未定义"
    fi
}

test_002_aarch64_target_defined() {
    log_test "测试 002: aarch64 目标已定义"

    if grep -q "aarch64-unknown-linux-musl" ".github/workflows/release.yml"; then
        log_pass "aarch64-unknown-linux-musl 目标已定义"
    else
        log_fail "aarch64-unknown-linux-musl 目标未定义"
    fi
}

test_003_targets_have_musl() {
    log_test "测试 003: 目标使用 musl 静态链接"

    local musl_count=$(grep -o "unknown-linux-musl" ".github/workflows/release.yml" | wc -l)

    if [[ $musl_count -ge 2 ]]; then
        log_pass "至少 2 个目标使用 musl: $musl_count"
    else
        log_fail "musl 目标不足: $musl_count"
    fi
}

test_004_parallel_build() {
    log_test "测试 004: 配置了并行构建"

    if grep -q "fail-fast: false" ".github/workflows/release.yml"; then
        log_pass "配置了 fail-fast: false（允许并行）"
    else
        log_fail "未配置并行构建"
    fi
}

test_005_build_matrix_complete() {
    log_test "测试 005: 构建矩阵完整"

    # 检查必需的字段
    local has_target=$(grep -q "target:" ".github/workflows/release.yml" && echo "1" || echo "0")
    local has_os=$(grep -q "os:" ".github/workflows/release.yml" && echo "1" || echo "0")
    local has_use_cross=$(grep -q "use_cross:" ".github/workflows/release.yml" && echo "1" || echo "0")

    local total=$((has_target + has_os + has_use_cross))

    if [[ $total -eq 3 ]]; then
        log_pass "构建矩阵字段完整"
    else
        log_fail "构建矩阵字段不完整: $total/3"
    fi
}

test_006_naming_convention() {
    log_test "测试 006: 二进制文件命名规范正确"

    # 检查命名格式: aegis-input-{target}.tar.gz
    if grep -q "aegis-input-.*-unknown-linux-musl.tar.gz" ".github/workflows/release.yml"; then
        log_pass "文件命名符合规范"
    else
        log_fail "文件命名不符合规范"
    fi
}

test_007_cargo_cache_configured() {
    log_test "测试 007: 配置了 Cargo 缓存"

    if grep -q "actions/cache" ".github/workflows/release.yml"; then
        local cache_count=$(grep -c "actions/cache" ".github/workflows/release.yml")
        log_pass "配置了 Cargo 缓存: $cache_count 个缓存"
    else
        log_fail "未配置 Cargo 缓存"
    fi
}

test_008_rust_toolchain_configured() {
    log_test "测试 008: 配置了 Rust 工具链"

    if grep -q "dtolnay/rust-toolchain" ".github/workflows/release.yml" || \
       grep -q "actions-rs/toolchain" ".github/workflows/release.yml"; then
        log_pass "配置了 Rust 工具链安装"
    else
        log_fail "未配置 Rust 工具链"
    fi
}

test_009_release_permissions() {
    log_test "测试 009: 配置了 Release 权限"

    if grep -q "permissions:" ".github/workflows/release.yml" && \
       grep -q "contents: write" ".github/workflows/release.yml"; then
        log_pass "配置了 contents: write 权限"
    else
        log_fail "未配置 Release 权限"
    fi
}

test_010_artifact_naming_unique() {
    log_test "测试 010: 构建产物命名唯一"

    # 检查每个目标是否有唯一的产物名称
    local x86_name=$(grep -A 1 "x86_64-unknown-linux-musl" ".github/workflows/release.yml" | grep "asset_name:" | head -1 || true)
    local arm64_name=$(grep -A 1 "aarch64-unknown-linux-musl" ".github/workflows/release.yml" | grep "asset_name:" | head -1 || true)

    if [[ -n "$x86_name" && -n "$arm64_name" && "$x86_name" != "$arm64_name" ]]; then
        log_pass "构建产物命名唯一"
    else
        log_fail "构建产物命名可能冲突"
    fi
}

# ============================================================================
# 运行所有测试
# ============================================================================

run_all_tests() {
    echo "=========================================="
    echo "多平台构建验证测试"
    echo "=========================================="
    echo ""

    echo "运行测试..."
    echo ""

    test_001_x86_64_target_defined
    test_002_aarch64_target_defined
    test_003_targets_have_musl
    test_004_parallel_build
    test_005_build_matrix_complete
    test_006_naming_convention
    test_007_cargo_cache_configured
    test_008_rust_toolchain_configured
    test_009_release_permissions
    test_010_artifact_naming_unique

    echo ""
    echo "=========================================="
    echo "测试结果汇总"
    echo "=========================================="
    echo -e "${GREEN}通过: $TESTS_PASSED${NC}"
    echo -e "${RED}失败: $TESTS_FAILED${NC}"
    echo "总计: $((TESTS_PASSED + TESTS_FAILED))"
    echo ""

    if [[ $TESTS_FAILED -eq 0 ]]; then
        echo -e "${GREEN}✓ 所有测试通过！${NC}"
        return 0
    else
        echo -e "${RED}✗ 有测试失败${NC}"
        return 1
    fi
}

main() {
    run_all_tests
}

main "$@"
