# 课思库

课思库是面向本地个人使用的题库管理桌面客户端。首版只实现 Windows 与 macOS 桌面端，数据存储使用 SQLite，图片、批注板、导出文件等资源统一放入用户指定的数据仓库目录。

## 目录结构

```text
desktop-client/   Vue 3 + TypeScript 桌面前端
tauri-desktop/    Tauri 2 桌面壳、Rust 命令、SQLite 与文件仓库
mobile-client/    后续移动端预留目录
web-admin/        后续 Web 管理端预留目录
server/           后续服务端预留目录
docs/             数据仓库、数据库、开发说明
```

## 开发启动

```bash
cd desktop-client
npm install

cd ../tauri-desktop
npm install
npm run tauri:dev
```

默认登录账号：

```text
账号：yaoyao
密码：123456
```

首次进入题库管理时，系统会提示选择本地基础数据仓库目录。
