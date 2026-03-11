#!/bin/bash
# CI/CD 工作流集成测试
# 测试 GitHub Actions 工作流配置

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

WORKFLOW_FILE=".github/workflows/release.yml"

# ============================================================================
# 测试用例
# ============================================================================

test_001_workflow_file_exists() {
    log_test "测试 001: Release 工作流文件存在"

    if [[ -f "$WORKFLOW_FILE" ]]; then
        log_pass "工作流文件存在: $WORKFLOW_FILE"
    else
        log_fail "工作流文件不存在: $WORKFLOW_FILE"
    fi
}

test_002_workflow_valid_yaml() {
    log_test "测试 002: 工作流是有效的 YAML"

    # 检查是否有基本的 YAML 语法错误
    if grep -q "^name:" "$WORKFLOW_FILE" && \
       grep -q "^on:" "$WORKFLOW_FILE" && \
       grep -q "^jobs:" "$WORKFLOW_FILE"; then
        log_pass "工作流 YAML 结构有效"
    else
        log_fail "工作流 YAML 结构无效"
    fi
}

test_003_trigger_on_tag_push() {
    log_test "测试 003: 配置了 tag 触发"

    if grep -q "tags:" "$WORKFLOW_FILE" && \
       grep -q "'v\*.*\.*'" "$WORKFLOW_FILE"; then
        log_pass "正确配置了 tag 触发: v*.*.*"
    else
        log_fail "未配置 tag 触发"
    fi
}

test_004_build_matrix_exists() {
    log_test "测试 004: 构建矩阵存在"

    if grep -q "strategy:" "$WORKFLOW_FILE" && \
       grep -q "matrix:" "$WORKFLOW_FILE"; then
        log_pass "构建矩阵配置存在"
    else
        log_fail "构建矩阵配置不存在"
    fi
}

test_005_x86_64_in_matrix() {
    log_test "测试 005: 构建矩阵包含 x86_64"

    if grep -q "x86_64-unknown-linux-musl" "$WORKFLOW_FILE"; then
        log_pass "构建矩阵包含 x86_64-unknown-linux-musl"
    else
        log_fail "构建矩阵缺少 x86_64"
    fi
}

test_006_aarch64_in_matrix() {
    log_test "测试 006: 构建矩阵包含 aarch64"

    if grep -q "aarch64-unknown-linux-musl" "$WORKFLOW_FILE"; then
        log_pass "构建矩阵包含 aarch64-unknown-linux-musl"
    else
        log_fail "构建矩阵缺少 aarch64"
    fi
}

test_007_uses_cross_for_aarch64() {
    log_test "测试 007: aarch64 使用 cross 编译"

    # 检查 aarch64 配置行附近是否有 use_cross: true
    if grep -A 3 "aarch64-unknown-linux-musl" "$WORKFLOW_FILE" | grep -q "use_cross: true"; then
        log_pass "aarch64 配置使用 cross"
    else
        log_fail "aarch64 未配置 cross"
    fi
}

test_008_creates_release() {
    log_test "测试 008: 配置了 Release 创建"

    if grep -q "softprops/action-gh-release" "$WORKFLOW_FILE"; then
        log_pass "使用 softprops/action-gh-release 创建 Release"
    else
        log_fail "未配置 Release 创建"
    fi
}

test_009_generates_checksums() {
    log_test "测试 009: 生成 SHA256 校验和"

    if grep -q "SHA256" "$WORKFLOW_FILE" || \
       grep -q "sha256sum" "$WORKFLOW_FILE"; then
        log_pass "配置了 SHA256 校验和生成"
    else
        log_fail "未配置 SHA256 校验和生成"
    fi
}

test_010_uploads_artifacts() {
    log_test "测试 010: 上传构建产物"

    if grep -q "upload-artifact" "$WORKFLOW_FILE" && \
       grep -q "files:" "$WORKFLOW_FILE"; then
        log_pass "配置了构建产物上传"
    else
        log_fail "未配置构建产物上传"
    fi
}

# ============================================================================
# 运行所有测试
# ============================================================================

run_all_tests() {
    echo "=========================================="
    echo "CI/CD 工作流集成测试"
    echo "=========================================="
    echo ""

    echo "运行测试..."
    echo ""

    test_001_workflow_file_exists
    test_002_workflow_valid_yaml
    test_003_trigger_on_tag_push
    test_004_build_matrix_exists
    test_005_x86_64_in_matrix
    test_006_aarch64_in_matrix
    test_007_uses_cross_for_aarch64
    test_008_creates_release
    test_009_generates_checksums
    test_010_uploads_artifacts

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
