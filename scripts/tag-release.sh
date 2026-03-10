#!/bin/bash
# 发布脚本 - 自动打标签并触发 GitHub Actions 构建

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查是否提供了版本号
if [ $# -eq 0 ]; then
    echo -e "${RED}错误: 未提供版本号${NC}"
    echo "用法: $0 <version> [remote]"
    echo "示例: $0 1.0.0 origin"
    exit 1
fi

VERSION=$1
REMOTE=${2:-origin}

# 验证版本号格式
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}错误: 版本号格式不正确${NC}"
    echo "版本号应为语义化版本格式，如: 1.0.0"
    exit 1
fi

TAG_NAME="v$VERSION"

echo -e "${YELLOW}准备发布版本: $TAG_NAME${NC}"

# 检查是否有未提交的更改
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${RED}错误: 存在未提交的更改${NC}"
    git status
    exit 1
fi

# 检查标签是否已存在
if git rev-parse "$TAG_NAME" >/dev/null 2>&1; then
    echo -e "${RED}错误: 标签 $TAG_NAME 已存在${NC}"
    exit 1
fi

# 更新 Cargo.toml 中的版本号
echo -e "${YELLOW}更新 Cargo.toml 版本号...${NC}"
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# 提交版本更新
echo -e "${YELLOW}提交版本更新...${NC}"
git add Cargo.toml
git commit -m "chore: bump version to $VERSION"

# 创建标签
echo -e "${YELLOW}创建标签 $TAG_NAME...${NC}"
git tag -a "$TAG_NAME" -m "Release $TAG_NAME"

# 推送提交和标签
echo -e "${YELLOW}推送到 $REMOTE...${NC}"
git push "$REMOTE" main
git push "$REMOTE" "$TAG_NAME"

echo -e "${GREEN}✓ 发布成功!${NC}"
echo -e "${GREEN}GitHub Actions 将自动构建并创建 Release${NC}"
echo -e "${YELLOW}查看构建状态: ${NC}https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/actions"
