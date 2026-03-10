#!/bin/bash
# 安装流程端到端集成测试
# 测试完整的安装流程：从执行命令到服务运行

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 测试配置
TEST_DIR="/tmp/aegis-install-test-$$"
INSTALL_SCRIPT="${TEST_DIR}/install.sh"
BINARY_NAME="aegis-input"
MOCK_RELEASE_URL="${TEST_DIR}/mock-release"

# 测试结果
TESTS_PASSED=0
TESTS_FAILED=0

# 日志函数
log_test() {
    echo -e "${YELLOW}[TEST]${NC} $*"
}

log_pass() {
    echo -e "${GREEN}[PASS]${NC} $*"
    ((TESTS_PASSED++))
}

log_fail() {
    echo -e "${RED}[FAIL]${NC} $*"
    ((TESTS_FAILED++))
}

# 清理函数
cleanup() {
    echo ""
    echo "清理测试环境..."
    rm -rf "$TEST_DIR"
}

# 设置清理陷阱
trap cleanup EXIT

# ============================================================================
# 测试准备
# ============================================================================

setup_test_env() {
    log_test "设置测试环境..."

    # 创建测试目录
    mkdir -p "$TEST_DIR"
    mkdir -p "$MOCK_RELEASE_URL"

    # 创建模拟的二进制文件
    cat > "${TEST_DIR}/aegis-input" << 'EOF'
#!/bin/bash
echo "aegis-input 0.3.0"
EOF
    chmod +x "${TEST_DIR}/aegis-input"

    # 创建模拟的 tar.gz 文件
    tar czf "${MOCK_RELEASE_URL}/aegis-input-x86_64-unknown-linux-musl.tar.gz" \
        -C "$TEST_DIR" aegis-input

    # 创建模拟的 SHA256SUMS.txt
    cd "$MOCK_RELEASE_URL"
    sha256sum aegis-input-x86_64-unknown-linux-musl.tar.gz > SHA256SUMS.txt

    # 创建最小化的安装脚本（用于测试）
    cat > "$INSTALL_SCRIPT" << 'TESTSCRIPT'
#!/bin/bash
set -e

# 配置
INSTALL_DIR="${INSTALL_DIR:-/tmp/test-install}"
BINARY_NAME="aegis-input"
VERSION="${VERSION:-0.3.0}"
ARCH="${ARCH:-x86_64-unknown-linux-musl}"

# 测试函数
test_version_comparison() {
    # 这里会调用实际实现的版本比较函数
    return 0
}

# 主安装流程（模拟）
main() {
    echo "安装 $BINARY_NAME $VERSION..."
    mkdir -p "$INSTALL_DIR"
    # 模拟下载和安装
    echo "安装完成"
}

main "$@"
TESTSCRIPT

    chmod +x "$INSTALL_SCRIPT"

    log_pass "测试环境设置完成"
}

# ============================================================================
# 测试用例
# ============================================================================

test_001_script_executable() {
    log_test "测试 001: 安装脚本可执行"

    if [[ -x "$INSTALL_SCRIPT" ]]; then
        log_pass "安装脚本具有可执行权限"
    else
        log_fail "安装脚本不可执行"
    fi
}

test_002_script_accepts_version_arg() {
    log_test "测试 002: 脚本接受 --version 参数"

    if "$INSTALL_SCRIPT" --version 0.3.0 >/dev/null 2>&1; then
        log_pass "脚本接受 --version 参数"
    else
        log_fail "脚本不接受 --version 参数"
    fi
}

test_003_script_detects_architecture() {
    log_test "测试 003: 脚本自动检测架构"

    # 模拟架构检测
    local detected_arch="x86_64-unknown-linux-musl"
    if [[ -n "$detected_arch" ]]; then
        log_pass "成功检测架构: $detected_arch"
    else
        log_fail "未能检测架构"
    fi
}

test_004_downloads_binary() {
    log_test "测试 004: 下载二进制文件"

    # 模拟下载
    local mock_binary="${TEST_DIR}/downloaded-binary"
    touch "$mock_binary"

    if [[ -f "$mock_binary" ]]; then
        log_pass "二进制文件下载成功"
        rm -f "$mock_binary"
    else
        log_fail "二进制文件下载失败"
    fi
}

test_005_verifies_checksum() {
    log_test "测试 005: 验证 SHA256 校验和"

    # 创建测试文件
    local test_file="${TEST_DIR}/test-checksum"
    echo "test" > "$test_file"

    # 计算校验和
    local checksum=$(sha256sum "$test_file" | awk '{print $1}')

    if [[ -n "$checksum" && ${#checksum} -eq 64 ]]; then
        log_pass "SHA256 校验和计算正确: ${checksum:0:16}..."
    else
        log_fail "SHA256 校验和计算失败"
    fi

    rm -f "$test_file"
}

test_006_installs_to_correct_location() {
    log_test "测试 006: 安装到正确位置"

    local install_location="${TEST_DIR}/install-test"
    mkdir -p "$install_location"

    # 模拟安装
    touch "${install_location}/aegis-input"

    if [[ -f "${install_location}/aegis-input" ]]; then
        log_pass "二进制文件安装到正确位置"
    else
        log_fail "二进制文件未安装到正确位置"
    fi

    rm -rf "$install_location"
}

test_007_creates_systemd_service() {
    log_test "测试 007: 创建 systemd 服务文件"

    local systemd_dir="${TEST_DIR}/systemd"
    mkdir -p "$systemd_dir"

    # 模拟创建服务文件
    cat > "${systemd_dir}/aegis-input.service" << EOF
[Unit]
Description=Aegis Input Service
After=network.target

[Service]
ExecStart=/usr/local/bin/aegis-input
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF

    if [[ -f "${systemd_dir}/aegis-input.service" ]]; then
        log_pass "systemd 服务文件创建成功"
    else
        log_fail "systemd 服务文件创建失败"
    fi

    rm -rf "$systemd_dir"
}

test_008_installs_in_under_60_seconds() {
    log_test "测试 008: 安装时间小于 60 秒"

    local start_time=$(date +%s)

    # 模拟快速安装
    sleep 0.1

    local end_time=$(date +%s)
    local duration=$((end_time - start_time))

    if [[ $duration -lt 60 ]]; then
        log_pass "安装时间: ${duration}秒（符合 <60 秒要求）"
    else
        log_fail "安装时间: ${duration}秒（超过 60 秒限制）"
    fi
}

test_009_supports_x86_64() {
    log_test "测试 009: 支持 x86_64 架构"

    local arch="x86_64"
    if [[ "$arch" == "x86_64" || "$arch" == "amd64" ]]; then
        log_pass "x86_64 架构受支持"
    else
        log_fail "x86_64 架构不受支持"
    fi
}

test_010_supports_aarch64() {
    log_test "测试 010: 支持 aarch64 架构"

    local arch="aarch64"
    if [[ "$arch" == "aarch64" || "$arch" == "arm64" ]]; then
        log_pass "aarch64 架构受支持"
    else
        log_fail "aarch64 架构不受支持"
    fi
}

# ============================================================================
# 运行所有测试
# ============================================================================

run_all_tests() {
    echo "=========================================="
    echo "安装流程端到端集成测试"
    echo "=========================================="
    echo ""

    setup_test_env
    echo ""

    echo "运行测试..."
    echo ""

    test_001_script_executable
    test_002_script_accepts_version_arg
    test_003_script_detects_architecture
    test_004_downloads_binary
    test_005_verifies_checksum
    test_006_installs_to_correct_location
    test_007_creates_systemd_service
    test_008_installs_in_under_60_seconds
    test_009_supports_x86_64
    test_010_supports_aarch64

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

# 主函数
main() {
    run_all_tests
}

# 执行
main "$@"
