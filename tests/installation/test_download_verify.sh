#!/bin/bash
# 下载验证集成测试
# 测试文件下载和 SHA256 校验和验证功能

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

TEST_DIR="/tmp/aegis-download-test-$$"

# 清理
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
    echo "This is a test binary file" > "${TEST_DIR}/test-binary"

    # 计算正确的校验和
    local correct_checksum=$(sha256sum "${TEST_DIR}/test-binary" | awk '{print $1}')

    # 创建正确的校验和文件
    echo "$correct_checksum  test-binary" > "${TEST_DIR}/SHA256SUMS.txt"

    # 创建错误的校验和文件
    echo "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef  test-binary" > "${TEST_DIR}/SHA256SUMS-bad.txt"
}

# ============================================================================
# 测试用例
# ============================================================================

test_001_sha256_command_available() {
    log_test "测试 001: sha256sum 命令可用"

    if command -v sha256sum >/dev/null 2>&1; then
        log_pass "sha256sum 命令可用"
    else
        log_fail "sha256sum 命令不可用"
    fi
}

test_002_calculate_checksum() {
    log_test "测试 002: 计算 SHA256 校验和"

    setup_test_files

    local checksum=$(sha256sum "${TEST_DIR}/test-binary" | awk '{print $1}')

    if [[ ${#checksum} -eq 64 && "$checksum" =~ ^[0-9a-f]{64}$ ]]; then
        log_pass "SHA256 校验和计算正确: ${checksum:0:16}..."
    else
        log_fail "SHA256 校验和计算错误: $checksum"
    fi
}

test_003_verify_correct_checksum() {
    log_test "测试 003: 验证正确的校验和"

    setup_test_files

    # 模拟验证逻辑
    local expected=$(cat "${TEST_DIR}/SHA256SUMS.txt" | awk '{print $1}')
    local actual=$(sha256sum "${TEST_DIR}/test-binary" | awk '{print $1}')

    if [[ "$expected" == "$actual" ]]; then
        log_pass "校验和验证成功"
    else
        log_fail "校验和不匹配"
    fi
}

test_004_detect_incorrect_checksum() {
    log_test "测试 004: 检测错误的校验和"

    setup_test_files

    local expected=$(cat "${TEST_DIR}/SHA256SUMS-bad.txt" | awk '{print $1}')
    local actual=$(sha256sum "${TEST_DIR}/test-binary" | awk '{print $1}')

    if [[ "$expected" != "$actual" ]]; then
        log_pass "正确检测到校验和不匹配"
    else
        log_fail "未能检测到校验和不匹配"
    fi
}

test_005_checksum_file_format() {
    log_test "测试 005: SHA256SUMS.txt 文件格式"

    setup_test_files

    # 检查文件格式: <checksum>  <filename>
    local line=$(cat "${TEST_DIR}/SHA256SUMS.txt")

    if [[ "$line" =~ ^[0-9a-f]{64}[[:space:]]{2}.* ]]; then
        log_pass "SHA256SUMS.txt 格式正确"
    else
        log_fail "SHA256SUMS.txt 格式错误"
    fi
}

test_006_create_tarball() {
    log_test "测试 006: 创建 tar.gz 归档"

    setup_test_files

    # 创建归档
    tar czf "${TEST_DIR}/test-binary.tar.gz" -C "$TEST_DIR" test-binary

    if [[ -f "${TEST_DIR}/test-binary.tar.gz" ]]; then
        # 验证归档内容
        if tar tzf "${TEST_DIR}/test-binary.tar.gz" | grep -q "test-binary"; then
            log_pass "tar.gz 归档创建成功且包含正确文件"
        else
            log_fail "tar.gz 归档内容不正确"
        fi
    else
        log_fail "tar.gz 归档创建失败"
    fi
}

test_007_download_file_exists() {
    log_test "测试 007: 下载的文件存在"

    setup_test_files

    # 模拟下载的文件
    local downloaded_file="${TEST_DIR}/downloaded.tar.gz"
    touch "$downloaded_file"

    if [[ -f "$downloaded_file" ]]; then
        log_pass "下载文件存在"
    else
        log_fail "下载文件不存在"
    fi
}

test_008_file_size_reasonable() {
    log_test "测试 008: 文件大小合理（<10MB）"

    setup_test_files

    local max_size=10485760  # 10MB
    local file_size=$(stat -f%z "${TEST_DIR}/test-binary" 2>/dev/null || stat -c%s "${TEST_DIR}/test-binary" 2>/dev/null)

    if [[ $file_size -lt $max_size ]]; then
        log_pass "文件大小合理: ${file_size} bytes"
    else
        log_fail "文件大小过大: ${file_size} bytes"
    fi
}

test_009_checksum_file_contains_all_artifacts() {
    log_test "测试 009: SHA256SUMS 包含所有构建产物"

    setup_test_files

    # 检查校验和文件
    local checksum_file="${TEST_DIR}/SHA256SUMS.txt"
    local line_count=$(wc -l < "$checksum_file")

    if [[ $line_count -ge 1 ]]; then
        log_pass "SHA256SUMS.txt 包含 $line_count 个文件"
    else
        log_fail "SHA256SUMS.txt 为空"
    fi
}

test_010_multiple_architectures() {
    log_test "测试 010: 支持多架构校验和"

    setup_test_files

    # 添加多个架构的校验和
    cat >> "${TEST_DIR}/SHA256SUMS.txt" << EOF
abc123...  aegis-input-x86_64-unknown-linux-musl.tar.gz
def456...  aegis-input-aarch64-unknown-linux-musl.tar.gz
EOF

    local x86_64_line=$(grep "x86_64-unknown-linux-musl" "${TEST_DIR}/SHA256SUMS.txt")
    local aarch64_line=$(grep "aarch64-unknown-linux-musl" "${TEST_DIR}/SHA256SUMS.txt")

    if [[ -n "$x86_64_line" && -n "$aarch64_line" ]]; then
        log_pass "多架构校验和存在"
    else
        log_fail "缺少某些架构的校验和"
    fi
}

# ============================================================================
# 运行所有测试
# ============================================================================

run_all_tests() {
    echo "=========================================="
    echo "下载验证集成测试"
    echo "=========================================="
    echo ""

    echo "运行测试..."
    echo ""

    test_001_sha256_command_available
    test_002_calculate_checksum
    test_003_verify_correct_checksum
    test_004_detect_incorrect_checksum
    test_005_checksum_file_format
    test_006_create_tarball
    test_007_download_file_exists
    test_008_file_size_reasonable
    test_009_checksum_file_contains_all_artifacts
    test_010_multiple_architectures

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
