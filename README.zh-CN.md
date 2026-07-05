# 课思库

[English](README.md)

课思库是一个本地优先的桌面题库管理工具。首版面向个人离线使用，主要支持 Windows 与 macOS 桌面端。结构化数据使用 SQLite 存储，图片、批注板、缩略图、导出文件、备份和导入文件统一保存在用户选择的本地数据仓库目录中。

## 目录

```text
Coscool/
├── desktop-client/   # Vue 3 + TypeScript 桌面前端
├── tauri-desktop/    # Tauri 2 桌面壳、Rust 命令、SQLite 与本地文件仓库
├── mobile-client/    # 移动端预留目录
├── web-admin/        # Web 管理端预留目录
├── server/           # 服务端预留目录
├── docs/             # 数据仓库、数据库和开发说明
└── assets/           # 公共品牌资源
```

## 本地启动

安装前端依赖：

```bash
cd desktop-client
npm install
```

安装 Tauri 依赖并启动桌面端：

```bash
cd ../tauri-desktop
npm install
npm run tauri:dev
```

构建桌面端安装包：

```bash
cd tauri-desktop
npm run tauri:build
```

`tauri:dev` 会自动启动 Vite 前端服务。开发环境下前端服务地址为 `http://127.0.0.1:5173`，Tauri 窗口会加载这个地址。

默认登录账号：

```text
账号：yaoyao
密码：123456
```

首次进入题库页面时，系统会提示用户选择本地基础数据仓库目录。

## 当前能力

- 本地教师登录，内置默认账号 `yaoyao`。
- 教师管理，支持账号、密码、姓名、手机号、备注和状态。
- 学员管理，支持姓名、手机号、年级、学校和备注。
- 题库分类树，用于组织个人题库。
- 题目录入，支持标题、题干、年份、题号、选项、答案、解析、标签和知识点。
- 题目图片导入和本地图片预览。
- 基于 Konva 的批注板，批注板 JSON 和预览资源保存在本地。
- 题目列表、关键词搜索、分类筛选，以及按年份、标签、知识点进行高级查询。
- 单题导出到本地数据仓库。
- 设置页用于查看或更新当前本地数据仓库目录。

## 数据仓库

用户选择的数据仓库目录会初始化为以下结构：

```text
CoscoolData/
├── coscool.sqlite
├── assets/
│   ├── images/
│   ├── boards/
│   ├── thumbs/
│   └── exports/
├── backups/
└── imports/
```

SQLite 只保存结构化记录和资源相对路径。图片、批注板 JSON、缩略图和导出文件保存在本地文件系统中。后续备份或迁移时，可以直接复制整个 `CoscoolData` 目录。

## 技术栈

- Tauri 2
- Rust
- SQLite / `rusqlite`
- Vue 3
- TypeScript
- Vite
- Ant Design Vue
- Konva
- Lucide Vue Next

## 当前边界

- 首版是本地个人桌面应用，不连接远程服务器。
- `mobile-client`、`web-admin` 和 `server` 是预留目录，当前没有正式业务代码。
- 数据保存在当前电脑上，首版不包含跨设备同步、多人协作和云端备份。
- 默认账号用于本地个人使用，暂未引入生产级多用户权限模型。
- 题目删除时首版不会立即物理删除资源文件，避免误删复用资源。

## 平台说明

- Windows 和 macOS 是首版通过 Tauri 支持的桌面目标。
- Tauri 打包目标配置为 `all`，但每个平台的安装包仍需要在对应操作系统上单独验证。
- Windows 打包需要重点验证安装器行为、文件权限、未签名应用提示和本地资源路径。
- macOS 打包需要重点验证 DMG 安装、应用权限，以及私有环境外分发时的签名或公证。
