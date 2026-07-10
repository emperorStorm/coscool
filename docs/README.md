# 课思库文档目录

`docs/` 是课思库的项目文档和 GitHub Pages 发布目录。README 只保留项目总入口，原型和细分说明统一从这里继续进入。

## 在线入口

- [系统交互原型](https://emperorStorm.github.io/coscool/prototype/)
- [本仓库中文说明](../README.zh-CN.md)
- [本仓库英文说明](../README.md)

## 文档清单

- [开发说明](development.md)：本地启动、构建命令和开发技术栈。
- [SQLite 表说明](database.md)：本地结构化数据表和核心字段说明。
- [本地数据仓库规划](data-library.md)：`CoscoolData` 目录、资源存储和备份迁移原则。
- [系统交互原型](prototype/)：用于产品走查的静态交互页面，不连接真实 Tauri、Rust 或 SQLite 服务。

## GitHub Pages 设置

仓库 Pages 建议使用以下发布源：

```text
Source: Deploy from a branch
Branch: main
Folder: /docs
```

设置完成后，原型访问地址为：

```text
https://emperorstorm.github.io/coscool/prototype/
```
