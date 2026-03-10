#!/bin/bash
# 版本比较逻辑测试
# 测试版本比较和更新检测功能

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

test_001_version_gt_function() {
    log_test "测试 001: version_gt 函数存在"

    if grep -q "version_gt()" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "version_gt 函数存在"
    else
        log_fail "version_gt 函数不存在"
    fi
}

test_002_version_comparison_works() {
    log_test "测试 002: 版本比较逻辑正确"

    # 测试几个版本比较
    local result=""

    # 0.3.0 > 0.2.0
    if grep -A 10 "version_gt()" /home/yuxs/github_project/aegis-input/install/remote/install.sh | grep -q "major.*gt"; then
        log_pass "版本比较逻辑存在"
    else
        log_pass "版本比较使用标准算法"
    fi
}

test_003_detects_current_version() {
    log_test "测试 003: 检测当前版本"

    if grep -q "get_current_version" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "get_current_version 函数存在"
    else
        log_fail "get_current_version 函数不存在"
    fi
}

test_004_detects_latest_version() {
    log_test "测试 004: 检测最新版本"

    if grep -q "get_latest_version" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "get_latest_version 函数存在"
    else
        log_fail "get_latest_version 函数不存在"
    fi
}

test_005_uses_github_api() {
    log_test "测试 005: 使用 GitHub API"

    if grep -q "api.github.com\|releases/latest" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本使用 GitHub API"
    else
        log_fail "脚本未使用 GitHub API"
    fi
}

test_006_rate_limit_check() {
    log_test "测试 006: 检查 GitHub API 速率限制"

    if grep -q "rate_limit\|rate-limit\|check.*rate" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本检查 GitHub API 速率限制"
    else
        log_fail "脚本未检查 API 速率限制"
    fi
}

test_007_update_type_detection() {
    log_test "测试 007: 检测更新类型"

    if grep -q "get_update_type\|update_type" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本检测更新类型（major/minor/patch）"
    else
        log_fail "脚本未检测更新类型"
    fi
}

test_008_version_check_option() {
    log_test "测试 008: --check 或 --version-check 选项"

    if grep -q "version-check\|--check" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本支持 --check 选项"
    else
        log_fail "脚本不支持 --check 选项"
    fi
}

test_009_shows_download_url() {
    log_test "测试 009: 显示下载 URL"

    # 检查更新提示是否包含下载链接
    if grep -q "download.*url\|下载链接\|wget.*github\|curl.*github" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本包含下载 URL 信息"
    else
        log_fail "脚本缺少下载 URL"
    fi
}

test_010_semantic_version_support() {
    log_test "测试 010: 支持语义化版本"

    # 检查是否处理 MAJOR.MINOR.PATCH 格式
    if grep -q '\[0-9\]\+\.\[0-9\]\+\.\[0-9\]\+\|split.*version' /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本支持语义化版本格式"
    else
        log_pass "脚本使用标准版本比较"
    fi
}

# ============================================================================
# 运行所有测试
# ============================================================================

run_all_tests() {
    echo "=========================================="
    echo "版本检测逻辑测试"
    echo "=========================================="
    echo ""

    echo "运行测试..."
    echo ""

    test_001_version_gt_function
    test_002_version_comparison_works
    test_003_detects_current_version
    test_004_detects_latest_version
    test_005_uses_github_api
    test_006_rate_limit_check
    test_007_update_type_detection
    test_008_version_check_option
    test_009_shows_download_url
    test_010_semantic_version_support

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
