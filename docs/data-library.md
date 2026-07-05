# 本地数据仓库规划

用户首次进入题库管理前，需要选择一个基础数据仓库目录。系统会在该目录下创建统一结构：

```text
CoscoolData/
  coscool.sqlite
  assets/
    images/
    boards/
    thumbs/
    exports/
  backups/
  imports/
```

## 存储原则

- SQLite 只存结构化数据和资源相对路径。
- 图片、批注板 JSON、缩略图、导出文件存文件系统。
- 题目删除时首版不立即物理删除资源，避免误删复用文件。
- 后续迁移备份时，直接打包整个 `CoscoolData` 目录即可。
