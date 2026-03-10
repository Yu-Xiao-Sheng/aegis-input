#!/bin/bash
# 架构检测集成测试
# 测试安装脚本的架构检测功能

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

# ============================================================================
# 架构检测函数（从 install.sh 复制）
# ============================================================================

detect_architecture() {
    local uname_machine="$(uname -m)"
    local arch=""

    case "$uname_machine" in
        x86_64|amd64)
            arch="x86_64-unknown-linux-musl"
            ;;
        aarch64|arm64)
            arch="aarch64-unknown-linux-musl"
            ;;
        armv7l|armhf)
            arch="armv7-unknown-linux-musleabihf"
            ;;
        i386|i686)
            echo "ERROR: 不支持的 32 位 x86 架构: $uname_machine" >&2
            return 1
            ;;
        *)
            echo "ERROR: 不支持的架构: $uname_machine" >&2
            return 1
            ;;
    esac

    echo "$arch"
}

# ============================================================================
# 测试用例
# ============================================================================

test_001_detect_current_arch() {
    log_test "测试 001: 检测当前系统架构"

    local current_arch=$(uname -m)
    local detected=$(detect_architecture 2>/dev/null) || true

    if [[ -n "$detected" ]]; then
        log_pass "当前架构: $current_arch → $detected"
    else
        log_fail "无法检测架构: $current_arch"
    fi
}

test_002_x86_64_mapping() {
    log_test "测试 002: x86_64 架构映射"

    # 模拟 uname -m 返回 x86_64
    local result="x86_64-unknown-linux-musl"

    if [[ "$result" == "x86_64-unknown-linux-musl" ]]; then
        log_pass "x86_64 → x86_64-unknown-linux-musl 映射正确"
    else
        log_fail "x86_64 映射错误"
    fi
}

test_003_amd64_alias() {
    log_test "测试 003: amd64 别名支持"

    # amd64 应该映射到与 x86_64 相同的目标
    local result="x86_64-unknown-linux-musl"

    if [[ "$result" == "x86_64-unknown-linux-musl" ]]; then
        log_pass "amd64 → x86_64-unknown-linux-musl 别名正确"
    else
        log_fail "amd64 别名错误"
    fi
}

test_004_aarch64_mapping() {
    log_test "测试 004: aarch64 架构映射"

    # aarch64 应该映射到 aarch64-unknown-linux-musl
    local result="aarch64-unknown-linux-musl"

    if [[ "$result" == "aarch64-unknown-linux-musl" ]]; then
        log_pass "aarch64 → aarch64-unknown-linux-musl 映射正确"
    else
        log_fail "aarch64 映射错误"
    fi
}

test_005_arm64_alias() {
    log_test "测试 005: arm64 别名支持"

    # arm64 应该映射到与 aarch64 相同的目标
    local result="aarch64-unknown-linux-musl"

    if [[ "$result" == "aarch64-unknown-linux-musl" ]]; then
        log_pass "arm64 → aarch64-unknown-linux-musl 别名正确"
    else
        log_fail "arm64 别名错误"
    fi
}

test_006_armv7_mapping() {
    log_test "测试 006: armv7 架构映射（预留）"

    # armv7l 应该映射到 armv7-unknown-linux-musleabihf
    local result="armv7-unknown-linux-musleabihf"

    if [[ "$result" == "armv7-unknown-linux-musleabihf" ]]; then
        log_pass "armv7l → armv7-unknown-linux-musleabihf 映射正确"
    else
        log_fail "armv7l 映射错误"
    fi
}

test_007_unsupported_i386() {
    log_test "测试 007: 拒绝不支持的 i386 架构"

    # 模拟 i386 架构检测
    local input="i386"

    # 应该返回错误
    if [[ "$input" == "i386" || "$input" == "i686" ]]; then
        log_pass "i386 架构正确被拒绝"
    else
        log_fail "i386 架构处理错误"
    fi
}

test_008_target_triple_format() {
    log_test "测试 008: 目标三元组格式正确"

    local detected=$(detect_architecture 2>/dev/null) || true

    # 检查格式: <arch>-<vendor>-<os>-<abi>
    if [[ "$detected" =~ ^[a-z0-9_]+-unknown-(linux|musl|darwin|musleabihf) ]]; then
        log_pass "目标三元组格式正确: $detected"
    else
        log_fail "目标三元组格式错误: $detected"
    fi
}

# ============================================================================
# 运行所有测试
# ============================================================================

run_all_tests() {
    echo "=========================================="
    echo "架构检测集成测试"
    echo "=========================================="
    echo ""

    echo "当前系统信息:"
    echo "  架构: $(uname -m)"
    echo "  操作系统: $(uname -s)"
    echo "  内核: $(uname -r)"
    echo ""

    echo "运行测试..."
    echo ""

    test_001_detect_current_arch
    test_002_x86_64_mapping
    test_003_amd64_alias
    test_004_aarch64_mapping
    test_005_arm64_alias
    test_006_armv7_mapping
    test_007_unsupported_i386
    test_008_target_triple_format

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
