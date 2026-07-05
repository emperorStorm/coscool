# SQLite 表说明

首版使用 SQLite 作为本地结构化数据存储。

核心表：

- `teachers`：教师账号、密码、姓名、手机号、备注、状态。
- `students`：学员基础信息。
- `question_categories`：题库分类树。
- `questions`：题目主体信息。
- `question_options`：题目选项。
- `tags`、`question_tags`：标签与题目关系。
- `knowledge_points`、`question_knowledge_points`：知识点与题目关系。
- `assets`：图片、批注板、缩略图、导出文件元信息。
- `app_settings`：本地设置。

默认教师账号为 `yaoyao`，密码为 `123456`。
