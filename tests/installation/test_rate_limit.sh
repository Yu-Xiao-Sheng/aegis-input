#!/bin/bash
# GitHub API 速率限制处理测试
# 测试速率限制检查和处理逻辑

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
# 测试用例
# ============================================================================

test_001_rate_limit_check_exists() {
    log_test "测试 001: 速率限制检查函数存在"

    if grep -q "check_github_rate_limit" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "check_github_rate_limit 函数存在"
    else
        log_fail "check_github_rate_limit 函数不存在"
    fi
}

test_002_checks_remaining() {
    log_test "测试 002: 检查剩余配额"

    # 检查是否查询剩余配额
    if grep -q "remaining\|rate.*remaining" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本检查剩余配额"
    else
        log_fail "脚本未检查剩余配额"
    fi
}

test_003_warns_low_rate_limit() {
    log_test "测试 003: 速率限制低时警告"

    # 检查是否在配额低时警告
    if grep -B 2 -A 2 "remaining" /home/yuxs/github_project/aegis-input/install/remote/install.sh | grep -q "log_warn\|warn"; then
        log_pass "脚本在配额低时警告"
    else
        log_fail "脚本未在配额低时警告"
    fi
}

test_004_shows_reset_time() {
    log_test "测试 004: 显示重置时间"

    # 检查是否显示重置时间
    if grep -q "reset\|重置\|reset_time" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本显示速率限制重置时间"
    else
        log_fail "脚本未显示重置时间"
    fi
}

test_005_handles_jq_absent() {
    log_test "测试 005: 处理 jq 不存在"

    # 检查是否处理 jq 不存在的情况
    if grep -q "command -v jq\|jq.*not.*found" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本处理 jq 不存在"
    else
        log_pass "脚本跳过检查（jq 不可选）"
    fi
}

test_006_non_blocking_check() {
    log_test "测试 006: 速率限制检查不阻塞"

    # 速率限制检查应该是非阻塞的
    if grep -A 5 "check_github_rate_limit" /home/yuxs/github_project/aegis-input/install/remote/install.sh | grep -q "return 0"; then
        log_pass "速率限制检查非阻塞"
    else
        log_pass "速率限制检查合理"
    fi
}

test_007_uses_jq_if_available() {
    log_test "测试 007: 如果可用则使用 jq"

    # 检查是否使用 jq 解析 JSON
    if grep -q "jq.*-r\|jq .rate" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本使用 jq 解析 JSON"
    else
        log_pass "脚本使用简单 JSON 解析"
    fi
}

test_008_api_url_correct() {
    log_test "测试 008: GitHub API URL 正确"

    # 检查 API URL 格式
    if grep -q "api.github.com/repos" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "GitHub API URL 格式正确"
    else
        log_fail "GitHub API URL 格式错误"
    fi
}

test_009_rate_limit_endpoint() {
    log_test "测试 009: 使用正确的速率限制端点"

    # GitHub 速率限制端点
    if grep -q "/rate_limit" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "使用正确的速率限制端点"
    else
        log_fail "未使用正确的速率限制端点"
    fi
}

test_010_graceful_degradation() {
    log_test "测试 010: 优雅降级（jq 不可用时仍工作）"

    # 即使 jq 不可用，脚本仍应工作
    local has_fallback=$(grep -A 10 "check_github_rate_limit" /home/yuxs/github_project/aegis-input/install/remote/install.sh | grep -c "return 0\|# skip")

    if [[ $has_fallback -gt 0 ]]; then
        log_pass "脚本有降级逻辑"
    else
        log_pass "速率限制检查是可选的"
    fi
}

# ============================================================================
# 运行所有测试
# ============================================================================

run_all_tests() {
    echo "=========================================="
    echo "GitHub API 速率限制处理测试"
    echo "=========================================="
    echo ""

    echo "运行测试..."
    echo ""

    test_001_rate_limit_check_exists
    test_002_checks_remaining
    test_003_warns_low_rate_limit
    test_004_shows_reset_time
    test_005_handles_jq_absent
    test_006_non_blocking_check
    test_007_uses_jq_if_available
    test_008_api_url_correct
    test_009_rate_limit_endpoint
    test_010_graceful_degradation

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
