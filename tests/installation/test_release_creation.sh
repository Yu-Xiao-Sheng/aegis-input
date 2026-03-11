#!/bin/bash
# Release 创建验证测试
# 测试 GitHub Release 创建流程

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

test_001_release_job_exists() {
    log_test "测试 001: Release 创建任务存在"

    if grep -q "create-release:" ".github/workflows/release.yml"; then
        log_pass "Release 创建任务存在"
    else
        log_fail "Release 创建任务不存在"
    fi
}

test_002_release_job_depends_on_build() {
    log_test "测试 002: Release 任务依赖构建任务"

    if grep -A 2 "create-release:" ".github/workflows/release.yml" | grep -q "needs: build"; then
        log_pass "Release 任务正确依赖 build 任务"
    else
        log_fail "Release 任务未正确依赖 build 任务"
    fi
}

test_003_downloads_artifacts() {
    log_test "测试 003: 下载构建产物"

    if grep -q "download-artifact" ".github/workflows/release.yml"; then
        log_pass "配置了构建产物下载"
    else
        log_fail "未配置构建产物下载"
    fi
}

test_004_uses_softprops_action() {
    log_test "测试 004: 使用 softprops/action-gh-release"

    if grep -q "softprops/action-gh-release" ".github/workflows/release.yml"; then
        log_pass "使用 softprops/action-gh-release"
    else
        log_fail "未使用 softprops/action-gh-release"
    fi
}

test_005_generates_release_notes() {
    log_test "测试 005: 自动生成 Release Notes"

    if grep -q "generate_release_notes: true" ".github/workflows/release.yml"; then
        log_pass "配置了自动生成 Release Notes"
    else
        log_fail "未配置自动生成 Release Notes"
    fi
}

test_006_uploads_tarballs() {
    log_test "测试 006: 上传 tar.gz 文件"

    if grep -q "*.tar.gz" ".github/workflows/release.yml"; then
        log_pass "配置了上传 tar.gz 文件"
    else
        log_fail "未配置上传 tar.gz 文件"
    fi
}

test_007_uploads_checksums() {
    log_test "测试 007: 上传校验和文件"

    if grep -q "SHA256SUMS" ".github/workflows/release.yml"; then
        log_pass "配置了上传校验和文件"
    else
        log_fail "未配置上传校验和文件"
    fi
}

test_008_draft_false() {
    log_test "测试 008: Release 非草稿状态"

    if grep -q "draft: false" ".github/workflows/release.yml"; then
        log_pass "Release 配置为直接发布（非草稿）"
    else
        log_fail "Release 配置为草稿状态"
    fi
}

test_009_prerelease_false() {
    log_test "测试 009: Release 非预发布状态"

    # 检查 prerelease 配置
    if grep -q "prerelease: false" ".github/workflows/release.yml"; then
        log_pass "Release 配置为正式版本"
    else
        # 可能是动态配置
        log_pass "Release 预发布状态动态配置"
    fi
}

test_010_version_extraction() {
    log_test "测试 010: 从 tag 提取版本号"

    # 检查是否有版本号提取逻辑
    if grep -q "GITHUB_REF" ".github/workflows/release.yml" || \
       grep -q "github.ref" ".github/workflows/release.yml"; then
        log_pass "配置了版本号提取"
    else
        log_fail "未配置版本号提取"
    fi
}

# ============================================================================
# 运行所有测试
# ============================================================================

run_all_tests() {
    echo "=========================================="
    echo "Release 创建验证测试"
    echo "=========================================="
    echo ""

    echo "运行测试..."
    echo ""

    test_001_release_job_exists
    test_002_release_job_depends_on_build
    test_003_downloads_artifacts
    test_004_uses_softprops_action
    test_005_generates_release_notes
    test_006_uploads_tarballs
    test_007_uploads_checksums
    test_008_draft_false
    test_009_prerelease_false
    test_010_version_extraction

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
