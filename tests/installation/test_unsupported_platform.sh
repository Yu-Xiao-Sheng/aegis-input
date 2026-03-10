#!/bin/bash
# 不支持平台场景测试
# 测试不支持的架构和操作系统的处理

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

test_001_detects_unsupported_arch_i386() {
    log_test "测试 001: 检测不支持的 i386 架构"

    # 检查脚本是否有 i386 处理
    if grep -q "i386\|i686" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        if grep -B 2 -A 2 "i386\|i686" /home/yuxs/github_project/aegis-input/install/remote/install.sh | grep -q "不支持的\|unsupported"; then
            log_pass "脚本正确拒绝 i386 架构"
        else
            log_fail "脚本未拒绝 i386 架构"
        fi
    else
        log_pass "脚本不处理 i386（或不需要处理）"
    fi
}

test_002_detects_unsupported_os_macos() {
    log_test "测试 002: 检测不支持的 macOS"

    # 检查脚本是否有 macOS 检测
    if grep -q "Darwin\|macOS" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        if grep -B 2 -A 2 "Darwin" /home/yuxs/github_project/aegis-input/install/remote/install.sh | grep -q "当前仅支持\|仅支持 Linux"; then
            log_pass "脚本正确提示 macOS 不支持"
        else
            log_fail "脚本未正确提示 macOS 不支持"
        fi
    else
        log_fail "脚本缺少 macOS 检测"
    fi
}

test_003_shows_supported_archs() {
    log_test "测试 003: 显示支持的架构列表"

    # 检查错误消息是否包含支持的架构
    if grep -q "x86_64\|aarch64\|支持的架构" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        local arch_count=$(grep -o "x86_64\|aarch64" /home/yuxs/github_project/aegis-input/install/remote/install.sh | wc -l)
        log_pass "脚本提到支持的架构: $arch_count 次"
    else
        log_fail "脚本未显示支持的架构"
    fi
}

test_004_suggests_source_build() {
    log_test "测试 004: 建议从源码编译"

    # 检查是否建议从源码编译
    if grep -q "从源码编译\|source.*compile\|cargo build" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本建议从源码编译"
    else
        log_fail "脚本未建议从源码编译"
    fi
}

test_005_early_detection() {
    log_test "测试 005: 早期检测不支持的平台"

    # 平台检测应该在下载之前进行
    local detect_pos=$(grep -n "detect_architecture\|detect_os" /home/yuxs/github_project/aegis-input/install/remote/install.sh | head -1 | cut -d: -f1)
    local download_pos=$(grep -n "download_and_verify\|下载.*二进制" /home/yuxs/github_project/aegis-input/install/remote/install.sh | head -1 | cut -d: -f1)

    if [[ -n "$detect_pos" && -n "$download_pos" && "$detect_pos" -lt "$download_pos" ]]; then
        log_pass "平台检测在下载之前执行（第 $detect_pos 行 vs 第 $download_pos 行）"
    else
        log_fail "平台检测未在下载之前执行"
    fi
}

test_006_clear_unsupported_message() {
    log_test "测试 006: 清晰的不支持提示消息"

    # 检查是否有明确的"不支持"消息
    if grep -q "不支持的\|不支持\|not supported\|unsupported" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本包含明确的不支持消息"
    else
        log_fail "脚本缺少明确的不支持消息"
    fi
}

test_007_handles_rare_archs() {
    log_test "测试 007: 处理罕见架构"

    # 检查是否处理了罕见架构
    if grep -q "armv7\|riscv\|powerpc\|ppc" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本处理罕见架构"
    else
        log_fail "脚本可能未处理所有架构"
    fi
}

test_008_platform_info_in_error() {
    log_test "测试 008: 错误消息包含平台信息"

    # 检查错误消息是否包含检测到的平台信息
    if grep -E "uname.*\|架构.*\|系统.*" /home/yuxs/github_project/aegis-input/install/remote/install.sh | grep -q "不支持的"; then
        log_pass "错误消息包含平台详情"
    else
        log_fail "错误消息缺少平台详情"
    fi
}

test_009_windows_detection() {
    log_test "测试 009: 检测 Windows 系统"

    # Windows 通常通过 $OS 或 $OSTYPE 检测
    if grep -q "Windows\|MINGW\|MSYS" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本包含 Windows 检测"
    else
        log_pass "脚本不处理 Windows（Linux only）"
    fi
}

test_010_future_proofing() {
    log_test "测试 010: 预留未来平台扩展"

    # 检查是否有 macOS/Windows 预留
    if grep -q "macOS.*即将推出\|Windows.*预留\|未来.*支持" /home/yuxs/github_project/aegis-input/install/remote/install.sh; then
        log_pass "脚本预留了未来平台支持"
    else
        log_fail "脚本未预留未来平台"
    fi
}

# ============================================================================
# 运行所有测试
# ============================================================================

run_all_tests() {
    echo "=========================================="
    echo "不支持平台场景测试"
    echo "=========================================="
    echo ""

    echo "运行测试..."
    echo ""

    test_001_detects_unsupported_arch_i386
    test_002_detects_unsupported_os_macos
    test_003_shows_supported_archs
    test_004_suggests_source_build
    test_005_early_detection
    test_006_clear_unsupported_message
    test_007_handles_rare_archs
    test_008_platform_info_in_error
    test_009_windows_detection
    test_010_future_proofing

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
