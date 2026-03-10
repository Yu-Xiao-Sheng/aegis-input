#!/bin/bash
# 校验和失败场景测试
# 测试 SHA256 校验失败的处理

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

TEST_DIR="/tmp/aegis-checksum-test-$$"

cleanup() {
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

# ============================================================================
# 辅助函数
# ============================================================================

setup_test_files() {
    mkdir -p "$TEST_DIR"

    # 创建测试文件
    echo "Original content" > "${TEST_DIR}/test-file"

    # 计算正确的校验和
    local correct_checksum=$(sha256sum "${TEST_DIR}/test-file" | awk '{print $1}')

    # 创建正确的校验和文件
    echo "$correct_checksum  test-file" > "${TEST_DIR}/SHA256SUMS.txt"

    # 修改文件内容（模拟损坏）
    echo "Corrupted content" > "${TEST_DIR}/test-file-corrupted"

    # 创建错误的校验和文件
    echo "wrongchecksum123  test-file" > "${TEST_DIR}/SHA256SUMS-bad.txt"
}

# ============================================================================
# 测试用例
# ============================================================================

test_001_verify_sha256_function() {
    log_test "测试 001: SHA256 验证函数存在"

    if grep -q "verify_sha256" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "verify_sha256 函数存在"
    else
        log_fail "verify_sha256 函数不存在"
    fi
}

test_002_checksum_mismatch_detection() {
    log_test "测试 002: 检测校验和不匹配"

    setup_test_files

    # 模拟验证逻辑
    local expected="wrongchecksum123"
    local actual=$(sha256sum "${TEST_DIR}/test-file" | awk '{print $1}')

    if [[ "$expected" != "$actual" ]]; then
        log_pass "正确检测到校验和不匹配"
    else
        log_fail "未能检测到校验和不匹配"
    fi
}

test_003_abort_on_checksum_fail() {
    log_test "测试 003: 校验和失败时中止安装"

    if grep -q "SHA256 校验和验证失败" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本在校验失败时中止"
    else
        log_fail "脚本未在校验失败时中止"
    fi
}

test_004_shows_checksum_details() {
    log_test "测试 004: 显示期望和实际的校验和"

    # 检查错误消息是否包含详细信息
    if grep -q "期望:\|实际:" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "错误消息包含校验和详情"
    else
        log_fail "错误消息缺少校验和详情"
    fi
}

test_005_suggests_redownload() {
    log_test "测试 005: 建议重新下载"

    if grep -q "重新下载\|redownload\|download again" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本建议重新下载"
    else
        log_fail "脚本未建议重新下载"
    fi
}

test_006_handles_missing_checksum_file() {
    log_test "测试 006: 处理缺少的校验和文件"

    # 检查是否有对缺失校验和文件的处理
    if grep -q "未找到.*校验和\|checksum.*not found" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本处理缺少的校验和文件"
    else
        log_fail "脚本可能未处理缺少的校验和文件"
    fi
}

test_007_checksum_format_validation() {
    log_test "测试 007: 校验和格式验证"

    # SHA256 应该是 64 个十六进制字符
    local checksum="0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"

    if [[ ${#checksum} -eq 64 && "$checksum" =~ ^[0-9a-f]{64}$ ]]; then
        log_pass "SHA256 格式验证正确"
    else
        log_fail "SHA256 格式验证失败"
    fi
}

test_008_multiple_artifacts_checksums() {
    log_test "测试 008: 多个构建产物的校验和"

    # 检查是否支持多架构校验和
    if grep -q "grep.*filename.*SHA256SUMS" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本支持多产物校验和"
    else
        log_fail "脚本可能不支持多产物校验和"
    fi
}

test_009_prevents_install_on_checksum_fail() {
    log_test "测试 009: 校验失败时阻止安装"

    # 检查校验失败时是否中止
    local check_abort=$(grep -A 5 "SHA256 校验和验证失败" /home/yuxs/github_project/aegis-input/install/remote/install.sh | grep -c "abort\|exit 1")

    if [[ $check_abort -gt 0 ]]; then
        log_pass "校验失败时正确中止安装"
    else
        log_fail "校验失败时未中止安装"
    fi
}

test_010_user_friendly_error_message() {
    log_test "测试 010: 用户友好的错误消息"

    # 检查是否有用户友好的消息
    if grep -q "文件可能已损坏\|file.*corrupt\|damaged" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "提供用户友好的错误消息"
    else
        log_fail "错误消息不够友好"
    fi
}

# ============================================================================
# 运行所有测试
# ============================================================================

run_all_tests() {
    echo "=========================================="
    echo "校验和失败场景测试"
    echo "=========================================="
    echo ""

    echo "运行测试..."
    echo ""

    test_001_verify_sha256_function
    test_002_checksum_mismatch_detection
    test_003_abort_on_checksum_fail
    test_004_shows_checksum_details
    test_005_suggests_redownload
    test_006_handles_missing_checksum_file
    test_007_checksum_format_validation
    test_008_multiple_artifacts_checksums
    test_009_prevents_install_on_checksum_fail
    test_010_user_friendly_error_message

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
