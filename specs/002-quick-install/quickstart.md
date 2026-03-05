# 快速开始（安装版）

## 适用范围

- Linux + systemd
- Debian/Ubuntu/Mint 系列优先

## 安装

1. 进入仓库目录
2. 执行安装脚本
```bash
sudo ./install/linux/install.sh
```

## 启动与关闭

- 启动服务（启用功能）:
```bash
sudo systemctl start aegis-input
```

- 停止服务（关闭功能）:
```bash
sudo systemctl stop aegis-input
```

## 卸载

```bash
sudo ./install/linux/uninstall.sh
```

## 验收要点

- 安装后服务自动运行且功能启用
- 停止服务后 2 秒内功能关闭并恢复内置设备
- 系统重启后服务自动启动
