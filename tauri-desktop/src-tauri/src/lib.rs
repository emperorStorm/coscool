use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Local;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Component;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

const SETTING_DATA_LIBRARY_PATH: &str = "data_library_path";
const SETTING_CURRENT_TEACHER: &str = "current_teacher_account";
const QUESTION_TYPES: [&str; 6] = ["单选题", "多选题", "填空题", "证明题", "计算题", "解答题"];

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct AppSettings {
    data_library_path: Option<String>,
    current_teacher_account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default, rename_all = "camelCase")]
struct Teacher {
    id: i64,
    account: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    password: String,
    name: String,
    phone: String,
    remark: String,
    status: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default, rename_all = "camelCase")]
struct Student {
    id: i64,
    name: String,
    phone: String,
    grade: String,
    school: String,
    remark: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default, rename_all = "camelCase")]
struct QuestionCategory {
    id: i64,
    parent_id: Option<i64>,
    name: String,
    sort_order: i64,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct QuestionOption {
    id: Option<i64>,
    option_key: String,
    content: String,
    sort_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Question {
    id: i64,
    category_id: Option<i64>,
    title: String,
    stem: String,
    image_text: String,
    year: String,
    question_no: String,
    question_type: String,
    difficulty: i64,
    answer: String,
    analysis: String,
    created_by: String,
    created_at: String,
    updated_at: String,
    options: Vec<QuestionOption>,
    tags: Vec<String>,
    knowledge_points: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Paper {
    id: i64,
    title: String,
    remark: String,
    question_count: i64,
    created_by: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PaperQuestionPayload {
    question_id: i64,
    sort_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PaperPayload {
    title: String,
    remark: String,
    created_by: String,
    questions: Vec<PaperQuestionPayload>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PaperDetail {
    paper: Paper,
    questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct QuestionPayload {
    id: Option<i64>,
    category_id: Option<i64>,
    title: String,
    stem: String,
    image_text: String,
    year: String,
    question_no: String,
    #[serde(default)]
    question_type: String,
    #[serde(default)]
    difficulty: i64,
    answer: String,
    analysis: String,
    created_by: String,
    options: Vec<QuestionOption>,
    tags: Vec<String>,
    knowledge_points: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AssetRecord {
    id: i64,
    question_id: Option<i64>,
    asset_type: String,
    file_path: String,
    thumb_path: String,
    mime_type: String,
    hash_value: String,
    width: i64,
    height: i64,
    size: i64,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default, rename_all = "camelCase")]
struct QuestionQueryFilters {
    category_id: Option<i64>,
    include_children: bool,
    title: String,
    year: String,
    tag: String,
    knowledge_point: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AssetDataUrl {
    data_url: String,
    mime_type: String,
}

fn now_text() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn normalize_difficulty(difficulty: i64) -> i64 {
    difficulty.clamp(0, 5)
}

fn normalize_question_type(question_type: &str) -> Result<String, String> {
    let text = question_type.trim();
    if text.is_empty() || QUESTION_TYPES.contains(&text) {
        return Ok(text.to_string());
    }
    Err("题型不合法".to_string())
}

fn app_config_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_config_dir().map_err(|error| error.to_string())?;
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    Ok(dir)
}

fn config_db_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_config_dir(app)?.join("coscool-config.sqlite"))
}

fn open_config_db(app: &AppHandle) -> Result<Connection, String> {
    let conn = Connection::open(config_db_path(app)?).map_err(|error| error.to_string())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )
    .map_err(|error| error.to_string())?;
    Ok(conn)
}

fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    conn.query_row("SELECT value FROM app_settings WHERE key = ?1", [key], |row| row.get(0))
        .optional()
        .map_err(|error| error.to_string())
}

fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO app_settings (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params![key, value, now_text()],
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

fn read_settings(app: &AppHandle) -> Result<AppSettings, String> {
    let conn = open_config_db(app)?;
    Ok(AppSettings {
        data_library_path: get_setting(&conn, SETTING_DATA_LIBRARY_PATH)?,
        current_teacher_account: get_setting(&conn, SETTING_CURRENT_TEACHER)?,
    })
}

fn data_library_root(app: &AppHandle) -> Result<PathBuf, String> {
    let settings = read_settings(app)?;
    let path = settings
        .data_library_path
        .ok_or_else(|| "请先选择基础数据仓库目录".to_string())?;
    let root = PathBuf::from(path);
    if !root.exists() {
        return Err("数据仓库目录不存在，请重新选择".to_string());
    }
    Ok(root)
}

fn init_library(root: &Path) -> Result<(), String> {
    fs::create_dir_all(root).map_err(|error| error.to_string())?;
    for sub_dir in [
        "assets/images",
        "assets/boards",
        "assets/thumbs",
        "assets/exports",
        "backups",
        "imports",
    ] {
        fs::create_dir_all(root.join(sub_dir)).map_err(|error| error.to_string())?;
    }
    let conn = Connection::open(root.join("coscool.sqlite")).map_err(|error| error.to_string())?;
    init_schema(&conn)?;
    seed_default_teacher(&conn)?;
    Ok(())
}

fn open_library_db(app: &AppHandle) -> Result<Connection, String> {
    let root = data_library_root(app)?;
    init_library(&root)?;
    Connection::open(root.join("coscool.sqlite")).map_err(|error| error.to_string())
}

fn init_schema(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS teachers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            name TEXT NOT NULL,
            phone TEXT NOT NULL DEFAULT '',
            remark TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT 'enabled',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS students (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            phone TEXT NOT NULL DEFAULT '',
            grade TEXT NOT NULL DEFAULT '',
            school TEXT NOT NULL DEFAULT '',
            remark TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS question_categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            parent_id INTEGER,
            name TEXT NOT NULL,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS questions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category_id INTEGER,
            title TEXT NOT NULL,
            stem TEXT NOT NULL,
            image_text TEXT NOT NULL DEFAULT '',
            year TEXT NOT NULL DEFAULT '',
            question_no TEXT NOT NULL DEFAULT '',
            question_type TEXT NOT NULL DEFAULT '',
            difficulty INTEGER NOT NULL DEFAULT 0,
            answer TEXT NOT NULL DEFAULT '',
            analysis TEXT NOT NULL DEFAULT '',
            created_by TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS question_options (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            question_id INTEGER NOT NULL,
            option_key TEXT NOT NULL,
            content TEXT NOT NULL DEFAULT '',
            sort_order INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS question_tags (
            question_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (question_id, tag_id)
        );
        CREATE TABLE IF NOT EXISTS knowledge_points (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS question_knowledge_points (
            question_id INTEGER NOT NULL,
            knowledge_point_id INTEGER NOT NULL,
            PRIMARY KEY (question_id, knowledge_point_id)
        );
        CREATE TABLE IF NOT EXISTS assets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            question_id INTEGER,
            asset_type TEXT NOT NULL,
            file_path TEXT NOT NULL,
            thumb_path TEXT NOT NULL DEFAULT '',
            mime_type TEXT NOT NULL DEFAULT '',
            hash_value TEXT NOT NULL DEFAULT '',
            width INTEGER NOT NULL DEFAULT 0,
            height INTEGER NOT NULL DEFAULT 0,
            size INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS papers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            remark TEXT NOT NULL DEFAULT '',
            question_count INTEGER NOT NULL DEFAULT 0,
            created_by TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS paper_questions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            paper_id INTEGER NOT NULL,
            question_id INTEGER NOT NULL,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_questions_category ON questions(category_id);
        CREATE INDEX IF NOT EXISTS idx_questions_year ON questions(year);
        CREATE INDEX IF NOT EXISTS idx_paper_questions_paper ON paper_questions(paper_id);
        CREATE INDEX IF NOT EXISTS idx_papers_updated_at ON papers(updated_at);
        ",
    )
    .map_err(|error| error.to_string())?;
    migrate_question_schema(conn)?;
    Ok(())
}

fn migrate_question_schema(conn: &Connection) -> Result<(), String> {
    let mut stmt = conn.prepare("PRAGMA table_info(questions)").map_err(|error| error.to_string())?;
    let rows = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|error| error.to_string())?;
    let columns = rows.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?;
    if !columns.iter().any(|item| item == "question_type") {
        conn.execute("ALTER TABLE questions ADD COLUMN question_type TEXT NOT NULL DEFAULT ''", [])
            .map_err(|error| error.to_string())?;
    }
    if !columns.iter().any(|item| item == "difficulty") {
        conn.execute("ALTER TABLE questions ADD COLUMN difficulty INTEGER NOT NULL DEFAULT 0", [])
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn seed_default_teacher(conn: &Connection) -> Result<(), String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM teachers WHERE account = 'yaoyao'", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    if count == 0 {
        let now = now_text();
        conn.execute(
            "INSERT INTO teachers (account, password, name, phone, remark, status, created_at, updated_at)
             VALUES ('yaoyao', '123456', '姚瑶', '', '系统默认教师账号', 'enabled', ?1, ?1)",
            [now],
        )
        .map_err(|error| error.to_string())?;
    } else {
        conn.execute(
            "UPDATE teachers SET name = '姚瑶', updated_at = ?1 WHERE account = 'yaoyao' AND (name = '' OR name = '瑶瑶老师')",
            [now_text()],
        )
        .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn row_to_teacher(row: &rusqlite::Row<'_>) -> rusqlite::Result<Teacher> {
    Ok(Teacher {
        id: row.get(0)?,
        account: row.get(1)?,
        password: String::new(),
        name: row.get(2)?,
        phone: row.get(3)?,
        remark: row.get(4)?,
        status: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

fn row_to_student(row: &rusqlite::Row<'_>) -> rusqlite::Result<Student> {
    Ok(Student {
        id: row.get(0)?,
        name: row.get(1)?,
        phone: row.get(2)?,
        grade: row.get(3)?,
        school: row.get(4)?,
        remark: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

fn row_to_category(row: &rusqlite::Row<'_>) -> rusqlite::Result<QuestionCategory> {
    Ok(QuestionCategory {
        id: row.get(0)?,
        parent_id: row.get(1)?,
        name: row.get(2)?,
        sort_order: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
    })
}

fn query_options(conn: &Connection, question_id: i64) -> Result<Vec<QuestionOption>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, option_key, content, sort_order
             FROM question_options
             WHERE question_id = ?1
             ORDER BY sort_order ASC, id ASC",
        )
        .map_err(|error| error.to_string())?;
    let rows = stmt
        .query_map([question_id], |row| {
            Ok(QuestionOption {
                id: row.get(0)?,
                option_key: row.get(1)?,
                content: row.get(2)?,
                sort_order: row.get(3)?,
            })
        })
        .map_err(|error| error.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

fn query_names(conn: &Connection, table: &str, link_table: &str, link_column: &str, question_id: i64) -> Result<Vec<String>, String> {
    let sql = format!(
        "SELECT t.name FROM {table} t
         INNER JOIN {link_table} l ON l.{link_column} = t.id
         WHERE l.question_id = ?1
         ORDER BY t.name ASC"
    );
    let mut stmt = conn.prepare(&sql).map_err(|error| error.to_string())?;
    let rows = stmt.query_map([question_id], |row| row.get(0)).map_err(|error| error.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

fn get_or_create_name(conn: &Connection, table: &str, name: &str) -> Result<i64, String> {
    conn.execute(&format!("INSERT OR IGNORE INTO {table} (name) VALUES (?1)"), [name])
        .map_err(|error| error.to_string())?;
    conn.query_row(&format!("SELECT id FROM {table} WHERE name = ?1"), [name], |row| row.get(0))
        .map_err(|error| error.to_string())
}

fn save_name_links(conn: &Connection, question_id: i64, table: &str, link_table: &str, link_column: &str, names: &[String]) -> Result<(), String> {
    conn.execute(&format!("DELETE FROM {link_table} WHERE question_id = ?1"), [question_id])
        .map_err(|error| error.to_string())?;
    for name in names.iter().map(|item| item.trim()).filter(|item| !item.is_empty()) {
        let id = get_or_create_name(conn, table, name)?;
        conn.execute(
            &format!("INSERT OR IGNORE INTO {link_table} (question_id, {link_column}) VALUES (?1, ?2)"),
            params![question_id, id],
        )
        .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn fetch_question(conn: &Connection, id: i64) -> Result<Question, String> {
    let mut question = conn
        .query_row(
            "SELECT id, category_id, title, stem, image_text, year, question_no, question_type, difficulty,
                    answer, analysis, created_by, created_at, updated_at
             FROM questions WHERE id = ?1",
            [id],
            |row| {
                Ok(Question {
                    id: row.get(0)?,
                    category_id: row.get(1)?,
                    title: row.get(2)?,
                    stem: row.get(3)?,
                    image_text: row.get(4)?,
                    year: row.get(5)?,
                    question_no: row.get(6)?,
                    question_type: row.get(7)?,
                    difficulty: row.get(8)?,
                    answer: row.get(9)?,
                    analysis: row.get(10)?,
                    created_by: row.get(11)?,
                    created_at: row.get(12)?,
                    updated_at: row.get(13)?,
                    options: Vec::new(),
                    tags: Vec::new(),
                    knowledge_points: Vec::new(),
                })
            },
        )
        .map_err(|error| error.to_string())?;
    question.options = query_options(conn, id)?;
    question.tags = query_names(conn, "tags", "question_tags", "tag_id", id)?;
    question.knowledge_points = query_names(conn, "knowledge_points", "question_knowledge_points", "knowledge_point_id", id)?;
    Ok(question)
}

fn row_to_paper(row: &rusqlite::Row<'_>) -> rusqlite::Result<Paper> {
    Ok(Paper {
        id: row.get(0)?,
        title: row.get(1)?,
        remark: row.get(2)?,
        question_count: row.get(3)?,
        created_by: row.get(4)?,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

fn fetch_paper(conn: &Connection, id: i64) -> Result<Paper, String> {
    conn.query_row(
        "SELECT id, title, remark, question_count, created_by, created_at, updated_at FROM papers WHERE id = ?1",
        [id],
        row_to_paper,
    )
    .map_err(|error| error.to_string())
}

fn category_descendant_ids(conn: &Connection, category_id: i64) -> Result<Vec<i64>, String> {
    let mut ids = vec![category_id];
    let mut index = 0;
    while index < ids.len() {
        let parent_id = ids[index];
        let mut stmt = conn
            .prepare("SELECT id FROM question_categories WHERE parent_id = ?1 ORDER BY sort_order ASC, id ASC")
            .map_err(|error| error.to_string())?;
        let rows = stmt.query_map([parent_id], |row| row.get::<_, i64>(0)).map_err(|error| error.to_string())?;
        for row in rows {
            let id = row.map_err(|error| error.to_string())?;
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
        index += 1;
    }
    Ok(ids)
}

fn question_matches_filters(question: &Question, filters: &QuestionQueryFilters, category_ids: &[i64]) -> bool {
    if !category_ids.is_empty() {
        match question.category_id {
            Some(id) if category_ids.contains(&id) => {}
            _ => return false,
        }
    }
    if !filters.title.trim().is_empty() && !question.title.contains(filters.title.trim()) {
        return false;
    }
    if !filters.year.trim().is_empty() && question.year != filters.year.trim() {
        return false;
    }
    if !filters.tag.trim().is_empty() && !question.tags.iter().any(|item| item.contains(filters.tag.trim())) {
        return false;
    }
    if !filters.knowledge_point.trim().is_empty()
        && !question.knowledge_points.iter().any(|item| item.contains(filters.knowledge_point.trim()))
    {
        return false;
    }
    true
}

#[tauri::command]
fn get_app_settings(app: AppHandle) -> Result<AppSettings, String> {
    read_settings(&app)
}

#[tauri::command]
fn save_app_settings(app: AppHandle, settings: AppSettings) -> Result<AppSettings, String> {
    let conn = open_config_db(&app)?;
    if let Some(path) = settings.data_library_path.as_deref() {
        init_library(Path::new(path))?;
        set_setting(&conn, SETTING_DATA_LIBRARY_PATH, path)?;
    }
    if let Some(account) = settings.current_teacher_account.as_deref() {
        set_setting(&conn, SETTING_CURRENT_TEACHER, account)?;
    }
    read_settings(&app)
}

#[tauri::command]
fn select_data_library_dir(app: AppHandle) -> Result<AppSettings, String> {
    let default_root = app_config_dir(&app)?.join("CoscoolData");
    save_data_library_path(&app, default_root)
}

#[tauri::command]
fn init_data_library_dir(app: AppHandle, path: String) -> Result<AppSettings, String> {
    if path.trim().is_empty() {
        return Err("目录路径不能为空".to_string());
    }
    save_data_library_path(&app, PathBuf::from(path))
}

fn save_data_library_path(app: &AppHandle, root: PathBuf) -> Result<AppSettings, String> {
    let data_root = if root.file_name().and_then(|name| name.to_str()) == Some("CoscoolData") {
        root
    } else {
        root.join("CoscoolData")
    };
    init_library(&data_root)?;
    let conn = open_config_db(app)?;
    set_setting(&conn, SETTING_DATA_LIBRARY_PATH, &data_root.to_string_lossy())?;
    read_settings(app)
}

#[tauri::command]
fn login(app: AppHandle, account: String, password: String) -> Result<Teacher, String> {
    let settings = read_settings(&app)?;
    if settings.data_library_path.is_none() {
        let conn = open_config_db(&app)?;
        if account == "yaoyao" && password == "123456" {
            set_setting(&conn, SETTING_CURRENT_TEACHER, &account)?;
            return Ok(Teacher {
                id: 0,
                account,
                password: String::new(),
                name: "姚瑶".to_string(),
                phone: String::new(),
                remark: "系统默认教师账号".to_string(),
                status: "enabled".to_string(),
                created_at: now_text(),
                updated_at: now_text(),
            });
        }
        return Err("首次使用请用默认账号 yaoyao / 123456 登录".to_string());
    }
    let conn = open_library_db(&app)?;
    let teacher = conn
        .query_row(
            "SELECT id, account, name, phone, remark, status, created_at, updated_at
             FROM teachers
             WHERE account = ?1 AND password = ?2 AND status = 'enabled'",
            params![account, password],
            row_to_teacher,
        )
        .optional()
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "账号或密码错误".to_string())?;
    let config = open_config_db(&app)?;
    set_setting(&config, SETTING_CURRENT_TEACHER, &teacher.account)?;
    Ok(teacher)
}

#[tauri::command]
fn teacher_list(app: AppHandle) -> Result<Vec<Teacher>, String> {
    let conn = open_library_db(&app)?;
    let mut stmt = conn
        .prepare("SELECT id, account, name, phone, remark, status, created_at, updated_at FROM teachers ORDER BY id DESC")
        .map_err(|error| error.to_string())?;
    let rows = stmt.query_map([], row_to_teacher).map_err(|error| error.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

#[tauri::command]
fn teacher_save(app: AppHandle, teacher: Teacher) -> Result<Teacher, String> {
    if teacher.account.trim().is_empty() || teacher.name.trim().is_empty() {
        return Err("账号和姓名不能为空".to_string());
    }
    let conn = open_library_db(&app)?;
    let now = now_text();
    if teacher.id > 0 {
        if teacher.password.trim().is_empty() {
            conn.execute(
                "UPDATE teachers SET account = ?1, name = ?2, phone = ?3, remark = ?4, status = ?5, updated_at = ?6 WHERE id = ?7",
                params![teacher.account, teacher.name, teacher.phone, teacher.remark, teacher.status, now, teacher.id],
            )
            .map_err(|error| error.to_string())?;
        } else {
            conn.execute(
                "UPDATE teachers SET account = ?1, password = ?2, name = ?3, phone = ?4, remark = ?5, status = ?6, updated_at = ?7 WHERE id = ?8",
                params![teacher.account, teacher.password, teacher.name, teacher.phone, teacher.remark, teacher.status, now, teacher.id],
            )
            .map_err(|error| error.to_string())?;
        }
        fetch_teacher(&conn, teacher.id)
    } else {
        let password = if teacher.password.trim().is_empty() { "123456".to_string() } else { teacher.password };
        conn.execute(
            "INSERT INTO teachers (account, password, name, phone, remark, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)",
            params![teacher.account, password, teacher.name, teacher.phone, teacher.remark, teacher.status, now],
        )
        .map_err(|error| error.to_string())?;
        fetch_teacher(&conn, conn.last_insert_rowid())
    }
}

fn fetch_teacher(conn: &Connection, id: i64) -> Result<Teacher, String> {
    conn.query_row(
        "SELECT id, account, name, phone, remark, status, created_at, updated_at FROM teachers WHERE id = ?1",
        [id],
        row_to_teacher,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
fn teacher_delete(app: AppHandle, id: i64) -> Result<bool, String> {
    let conn = open_library_db(&app)?;
    conn.execute("DELETE FROM teachers WHERE id = ?1 AND account <> 'yaoyao'", [id])
        .map_err(|error| error.to_string())?;
    Ok(true)
}

#[tauri::command]
fn student_list(app: AppHandle) -> Result<Vec<Student>, String> {
    let conn = open_library_db(&app)?;
    let mut stmt = conn
        .prepare("SELECT id, name, phone, grade, school, remark, created_at, updated_at FROM students ORDER BY id DESC")
        .map_err(|error| error.to_string())?;
    let rows = stmt.query_map([], row_to_student).map_err(|error| error.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

#[tauri::command]
fn student_save(app: AppHandle, student: Student) -> Result<Student, String> {
    if student.name.trim().is_empty() {
        return Err("学员姓名不能为空".to_string());
    }
    let conn = open_library_db(&app)?;
    let now = now_text();
    if student.id > 0 {
        conn.execute(
            "UPDATE students SET name = ?1, phone = ?2, grade = ?3, school = ?4, remark = ?5, updated_at = ?6 WHERE id = ?7",
            params![student.name, student.phone, student.grade, student.school, student.remark, now, student.id],
        )
        .map_err(|error| error.to_string())?;
        fetch_student(&conn, student.id)
    } else {
        conn.execute(
            "INSERT INTO students (name, phone, grade, school, remark, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)",
            params![student.name, student.phone, student.grade, student.school, student.remark, now],
        )
        .map_err(|error| error.to_string())?;
        fetch_student(&conn, conn.last_insert_rowid())
    }
}

fn fetch_student(conn: &Connection, id: i64) -> Result<Student, String> {
    conn.query_row(
        "SELECT id, name, phone, grade, school, remark, created_at, updated_at FROM students WHERE id = ?1",
        [id],
        row_to_student,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
fn student_delete(app: AppHandle, id: i64) -> Result<bool, String> {
    let conn = open_library_db(&app)?;
    conn.execute("DELETE FROM students WHERE id = ?1", [id]).map_err(|error| error.to_string())?;
    Ok(true)
}

#[tauri::command]
fn category_list(app: AppHandle) -> Result<Vec<QuestionCategory>, String> {
    let conn = open_library_db(&app)?;
    let mut stmt = conn
        .prepare("SELECT id, parent_id, name, sort_order, created_at, updated_at FROM question_categories ORDER BY sort_order ASC, id ASC")
        .map_err(|error| error.to_string())?;
    let rows = stmt.query_map([], row_to_category).map_err(|error| error.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

#[tauri::command]
fn category_save(app: AppHandle, category: QuestionCategory) -> Result<QuestionCategory, String> {
    if category.name.trim().is_empty() {
        return Err("分类名称不能为空".to_string());
    }
    let conn = open_library_db(&app)?;
    let now = now_text();
    if category.id > 0 {
        conn.execute(
            "UPDATE question_categories SET parent_id = ?1, name = ?2, sort_order = ?3, updated_at = ?4 WHERE id = ?5",
            params![category.parent_id, category.name, category.sort_order, now, category.id],
        )
        .map_err(|error| error.to_string())?;
        fetch_category(&conn, category.id)
    } else {
        conn.execute(
            "INSERT INTO question_categories (parent_id, name, sort_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?4)",
            params![category.parent_id, category.name, category.sort_order, now],
        )
        .map_err(|error| error.to_string())?;
        fetch_category(&conn, conn.last_insert_rowid())
    }
}

fn fetch_category(conn: &Connection, id: i64) -> Result<QuestionCategory, String> {
    conn.query_row(
        "SELECT id, parent_id, name, sort_order, created_at, updated_at FROM question_categories WHERE id = ?1",
        [id],
        row_to_category,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
fn category_delete(app: AppHandle, id: i64) -> Result<bool, String> {
    let conn = open_library_db(&app)?;
    conn.execute("UPDATE questions SET category_id = NULL WHERE category_id = ?1", [id])
        .map_err(|error| error.to_string())?;
    conn.execute("DELETE FROM question_categories WHERE id = ?1", [id])
        .map_err(|error| error.to_string())?;
    Ok(true)
}

#[tauri::command]
fn question_list(app: AppHandle, category_id: Option<i64>, keyword: Option<String>) -> Result<Vec<Question>, String> {
    let conn = open_library_db(&app)?;
    let keyword_text = keyword.unwrap_or_default();
    let like_keyword = format!("%{}%", keyword_text.trim());
    let mut questions = Vec::new();
    if let Some(id) = category_id {
        let mut stmt = conn
            .prepare(
            "SELECT DISTINCT q.id
             FROM questions q
             LEFT JOIN question_tags qt ON qt.question_id = q.id
             LEFT JOIN tags t ON t.id = qt.tag_id
             LEFT JOIN question_knowledge_points qk ON qk.question_id = q.id
             LEFT JOIN knowledge_points k ON k.id = qk.knowledge_point_id
             WHERE q.category_id = ?1
               AND (?2 = '%%' OR q.title LIKE ?2 OR q.stem LIKE ?2 OR q.year LIKE ?2 OR t.name LIKE ?2 OR k.name LIKE ?2)
             ORDER BY q.id DESC",
            )
            .map_err(|error| error.to_string())?;
        let ids = stmt
            .query_map(params![id, like_keyword], |row| row.get::<_, i64>(0))
            .map_err(|error| error.to_string())?;
        for row_id in ids {
            questions.push(fetch_question(&conn, row_id.map_err(|error| error.to_string())?)?);
        }
    } else {
        let mut stmt = conn
            .prepare(
            "SELECT DISTINCT q.id
             FROM questions q
             LEFT JOIN question_tags qt ON qt.question_id = q.id
             LEFT JOIN tags t ON t.id = qt.tag_id
             LEFT JOIN question_knowledge_points qk ON qk.question_id = q.id
             LEFT JOIN knowledge_points k ON k.id = qk.knowledge_point_id
             WHERE ?1 = '%%' OR q.title LIKE ?1 OR q.stem LIKE ?1 OR q.year LIKE ?1 OR t.name LIKE ?1 OR k.name LIKE ?1
             ORDER BY q.id DESC",
            )
            .map_err(|error| error.to_string())?;
        let ids = stmt
            .query_map([like_keyword], |row| row.get::<_, i64>(0))
            .map_err(|error| error.to_string())?;
        for row_id in ids {
            questions.push(fetch_question(&conn, row_id.map_err(|error| error.to_string())?)?);
        }
    }
    Ok(questions)
}

#[tauri::command]
fn question_get(app: AppHandle, id: i64) -> Result<Question, String> {
    let conn = open_library_db(&app)?;
    fetch_question(&conn, id)
}

#[tauri::command]
fn question_batch_get(app: AppHandle, ids: Vec<i64>) -> Result<Vec<Question>, String> {
    let conn = open_library_db(&app)?;
    let mut questions = Vec::new();
    for id in ids {
        if id <= 0 || questions.iter().any(|item: &Question| item.id == id) {
            continue;
        }
        if let Ok(question) = fetch_question(&conn, id) {
            questions.push(question);
        }
    }
    Ok(questions)
}

#[tauri::command]
fn question_query(app: AppHandle, filters: QuestionQueryFilters) -> Result<Vec<Question>, String> {
    let conn = open_library_db(&app)?;
    let category_ids = if let Some(category_id) = filters.category_id {
        if filters.include_children {
            category_descendant_ids(&conn, category_id)?
        } else {
            vec![category_id]
        }
    } else {
        Vec::new()
    };
    let mut stmt = conn
        .prepare("SELECT id FROM questions ORDER BY id DESC")
        .map_err(|error| error.to_string())?;
    let rows = stmt.query_map([], |row| row.get::<_, i64>(0)).map_err(|error| error.to_string())?;
    let mut questions = Vec::new();
    for row in rows {
        let question = fetch_question(&conn, row.map_err(|error| error.to_string())?)?;
        if question_matches_filters(&question, &filters, &category_ids) {
            questions.push(question);
        }
    }
    Ok(questions)
}

#[tauri::command]
fn question_save(app: AppHandle, question: QuestionPayload) -> Result<Question, String> {
    if question.title.trim().is_empty() || question.stem.trim().is_empty() {
        return Err("标题和题目不能为空".to_string());
    }
    let question_type = normalize_question_type(&question.question_type)?;
    let difficulty = normalize_difficulty(question.difficulty);
    let mut conn = open_library_db(&app)?;
    let tx = conn.transaction().map_err(|error| error.to_string())?;
    let now = now_text();
    let question_id = if let Some(id) = question.id {
        tx.execute(
            "UPDATE questions SET category_id = ?1, title = ?2, stem = ?3, image_text = ?4, year = ?5,
             question_no = ?6, question_type = ?7, difficulty = ?8, answer = ?9, analysis = ?10,
             created_by = ?11, updated_at = ?12 WHERE id = ?13",
            params![
                question.category_id,
                question.title,
                question.stem,
                question.image_text,
                question.year,
                question.question_no,
                question_type,
                difficulty,
                question.answer,
                question.analysis,
                question.created_by,
                now,
                id
            ],
        )
        .map_err(|error| error.to_string())?;
        id
    } else {
        tx.execute(
            "INSERT INTO questions (category_id, title, stem, image_text, year, question_no, question_type, difficulty,
                                    answer, analysis, created_by, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12)",
            params![
                question.category_id,
                question.title,
                question.stem,
                question.image_text,
                question.year,
                question.question_no,
                question_type,
                difficulty,
                question.answer,
                question.analysis,
                question.created_by,
                now
            ],
        )
        .map_err(|error| error.to_string())?;
        tx.last_insert_rowid()
    };
    tx.execute("DELETE FROM question_options WHERE question_id = ?1", [question_id])
        .map_err(|error| error.to_string())?;
    for option in question.options {
        tx.execute(
            "INSERT INTO question_options (question_id, option_key, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            params![question_id, option.option_key, option.content, option.sort_order],
        )
        .map_err(|error| error.to_string())?;
    }
    save_name_links(&tx, question_id, "tags", "question_tags", "tag_id", &question.tags)?;
    save_name_links(
        &tx,
        question_id,
        "knowledge_points",
        "question_knowledge_points",
        "knowledge_point_id",
        &question.knowledge_points,
    )?;
    tx.commit().map_err(|error| error.to_string())?;
    let conn = open_library_db(&app)?;
    fetch_question(&conn, question_id)
}

#[tauri::command]
fn question_delete(app: AppHandle, id: i64) -> Result<bool, String> {
    let conn = open_library_db(&app)?;
    conn.execute("DELETE FROM question_options WHERE question_id = ?1", [id])
        .map_err(|error| error.to_string())?;
    conn.execute("DELETE FROM question_tags WHERE question_id = ?1", [id])
        .map_err(|error| error.to_string())?;
    conn.execute("DELETE FROM question_knowledge_points WHERE question_id = ?1", [id])
        .map_err(|error| error.to_string())?;
    conn.execute("DELETE FROM questions WHERE id = ?1", [id]).map_err(|error| error.to_string())?;
    Ok(true)
}

#[tauri::command]
fn paper_list(app: AppHandle) -> Result<Vec<Paper>, String> {
    let conn = open_library_db(&app)?;
    let mut stmt = conn
        .prepare(
            "SELECT id, title, remark, question_count, created_by, created_at, updated_at
             FROM papers
             ORDER BY updated_at DESC, id DESC",
        )
        .map_err(|error| error.to_string())?;
    let rows = stmt.query_map([], row_to_paper).map_err(|error| error.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

#[tauri::command]
fn paper_get(app: AppHandle, id: i64) -> Result<PaperDetail, String> {
    let conn = open_library_db(&app)?;
    let paper = fetch_paper(&conn, id)?;
    let mut stmt = conn
        .prepare(
            "SELECT question_id FROM paper_questions
             WHERE paper_id = ?1
             ORDER BY sort_order ASC, id ASC",
        )
        .map_err(|error| error.to_string())?;
    let rows = stmt.query_map([id], |row| row.get::<_, i64>(0)).map_err(|error| error.to_string())?;
    let mut questions = Vec::new();
    for row in rows {
        if let Ok(question) = fetch_question(&conn, row.map_err(|error| error.to_string())?) {
            questions.push(question);
        }
    }
    Ok(PaperDetail { paper, questions })
}

#[tauri::command]
fn paper_save(app: AppHandle, paper: PaperPayload) -> Result<PaperDetail, String> {
    if paper.title.trim().is_empty() {
        return Err("试卷名称不能为空".to_string());
    }
    if paper.questions.is_empty() {
        return Err("请先加入题目再生成试卷".to_string());
    }
    let mut conn = open_library_db(&app)?;
    let tx = conn.transaction().map_err(|error| error.to_string())?;
    let now = now_text();
    let mut question_ids = Vec::new();
    for item in paper.questions {
        if item.question_id <= 0 || question_ids.contains(&item.question_id) {
            continue;
        }
        if fetch_question(&tx, item.question_id).is_ok() {
            question_ids.push(item.question_id);
        }
    }
    if question_ids.is_empty() {
        return Err("试卷题目不存在，请重新选择题目".to_string());
    }
    tx.execute(
        "INSERT INTO papers (title, remark, question_count, created_by, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
        params![paper.title.trim(), paper.remark.trim(), question_ids.len() as i64, paper.created_by.trim(), now],
    )
    .map_err(|error| error.to_string())?;
    let paper_id = tx.last_insert_rowid();
    for (index, question_id) in question_ids.iter().enumerate() {
        tx.execute(
            "INSERT INTO paper_questions (paper_id, question_id, sort_order, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![paper_id, question_id, index as i64, now],
        )
        .map_err(|error| error.to_string())?;
    }
    tx.commit().map_err(|error| error.to_string())?;
    let conn = open_library_db(&app)?;
    let paper = fetch_paper(&conn, paper_id)?;
    let mut questions = Vec::new();
    for question_id in question_ids {
        questions.push(fetch_question(&conn, question_id)?);
    }
    Ok(PaperDetail { paper, questions })
}

#[tauri::command]
fn paper_delete(app: AppHandle, id: i64) -> Result<bool, String> {
    let conn = open_library_db(&app)?;
    conn.execute("DELETE FROM paper_questions WHERE paper_id = ?1", [id])
        .map_err(|error| error.to_string())?;
    conn.execute("DELETE FROM papers WHERE id = ?1", [id]).map_err(|error| error.to_string())?;
    Ok(true)
}

#[tauri::command]
fn import_asset(app: AppHandle, source_path: String, question_id: Option<i64>) -> Result<AssetRecord, String> {
    let source = PathBuf::from(source_path);
    if !source.exists() || !source.is_file() {
        return Err("图片文件不存在".to_string());
    }
    let root = data_library_root(&app)?;
    let bytes = fs::read(&source).map_err(|error| error.to_string())?;
    let hash = format!("{:x}", Sha256::digest(&bytes));
    let ext = source.extension().and_then(|item| item.to_str()).unwrap_or("png");
    let file_name = format!("{}-{}.{}", Local::now().format("%Y%m%d%H%M%S"), &hash[..12], ext);
    let relative_path = format!("assets/images/{file_name}");
    let target = root.join(&relative_path);
    fs::copy(&source, &target).map_err(|error| error.to_string())?;
    let thumb_path = format!("assets/thumbs/{file_name}");
    fs::copy(&source, root.join(&thumb_path)).map_err(|error| error.to_string())?;
    insert_asset(&app, question_id, "image", &relative_path, &thumb_path, mime_from_ext(ext), &hash, bytes.len() as i64)
}

#[tauri::command]
fn save_board(app: AppHandle, question_id: Option<i64>, board_json: String, preview_data_url: String) -> Result<AssetRecord, String> {
    let root = data_library_root(&app)?;
    let id = Uuid::new_v4().to_string();
    let board_relative = format!("assets/boards/{id}.json");
    let preview_relative = format!("assets/thumbs/{id}.png");
    fs::write(root.join(&board_relative), board_json.as_bytes()).map_err(|error| error.to_string())?;
    let preview_bytes = decode_data_url(&preview_data_url)?;
    fs::write(root.join(&preview_relative), &preview_bytes).map_err(|error| error.to_string())?;
    let hash = format!("{:x}", Sha256::digest(board_json.as_bytes()));
    insert_asset(&app, question_id, "canvas", &board_relative, &preview_relative, "application/json", &hash, board_json.len() as i64)
}

#[tauri::command]
fn export_question(app: AppHandle, id: i64) -> Result<String, String> {
    let root = data_library_root(&app)?;
    let conn = open_library_db(&app)?;
    let question = fetch_question(&conn, id)?;
    let file_name = format!("question-{}-{}.json", id, Local::now().format("%Y%m%d%H%M%S"));
    let relative_path = format!("assets/exports/{file_name}");
    let json = serde_json::to_string_pretty(&question).map_err(|error| error.to_string())?;
    fs::write(root.join(&relative_path), json).map_err(|error| error.to_string())?;
    Ok(root.join(relative_path).to_string_lossy().to_string())
}

#[tauri::command]
fn read_asset_data_url(app: AppHandle, relative_path: String) -> Result<AssetDataUrl, String> {
    let path_text = relative_path.trim();
    if path_text.is_empty() {
        return Err("资源路径不能为空".to_string());
    }
    let relative = Path::new(path_text);
    if relative.is_absolute() || relative.components().any(|part| !matches!(part, Component::Normal(_))) {
        return Err("资源路径不合法".to_string());
    }
    let root = data_library_root(&app)?;
    let target = root.join(relative);
    if !target.exists() || !target.is_file() {
        return Err("图片不存在".to_string());
    }
    let bytes = fs::read(&target).map_err(|error| error.to_string())?;
    let ext = target.extension().and_then(|item| item.to_str()).unwrap_or("png");
    let mime_type = mime_from_ext(ext).to_string();
    Ok(AssetDataUrl {
        data_url: format!("data:{};base64,{}", mime_type, STANDARD.encode(bytes)),
        mime_type,
    })
}

#[tauri::command]
fn save_export_file(target_path: String, data_url: String) -> Result<String, String> {
    let path_text = target_path.trim();
    if path_text.is_empty() {
        return Err("保存路径不能为空".to_string());
    }
    let target = PathBuf::from(path_text);
    if target.file_name().is_none() {
        return Err("保存路径不合法".to_string());
    }
    if let Some(parent) = target.parent() {
        if !parent.exists() {
            return Err("保存目录不存在".to_string());
        }
    }
    let bytes = decode_data_url(&data_url)?;
    fs::write(&target, bytes).map_err(|error| error.to_string())?;
    Ok(target.to_string_lossy().to_string())
}

fn insert_asset(
    app: &AppHandle,
    question_id: Option<i64>,
    asset_type: &str,
    file_path: &str,
    thumb_path: &str,
    mime_type: &str,
    hash_value: &str,
    size: i64,
) -> Result<AssetRecord, String> {
    let conn = open_library_db(app)?;
    let now = now_text();
    conn.execute(
        "INSERT INTO assets (question_id, asset_type, file_path, thumb_path, mime_type, hash_value, width, height, size, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, 0, ?7, ?8)",
        params![question_id, asset_type, file_path, thumb_path, mime_type, hash_value, size, now],
    )
    .map_err(|error| error.to_string())?;
    fetch_asset(&conn, conn.last_insert_rowid())
}

fn fetch_asset(conn: &Connection, id: i64) -> Result<AssetRecord, String> {
    conn.query_row(
        "SELECT id, question_id, asset_type, file_path, thumb_path, mime_type, hash_value, width, height, size, created_at
         FROM assets WHERE id = ?1",
        [id],
        |row| {
            Ok(AssetRecord {
                id: row.get(0)?,
                question_id: row.get(1)?,
                asset_type: row.get(2)?,
                file_path: row.get(3)?,
                thumb_path: row.get(4)?,
                mime_type: row.get(5)?,
                hash_value: row.get(6)?,
                width: row.get(7)?,
                height: row.get(8)?,
                size: row.get(9)?,
                created_at: row.get(10)?,
            })
        },
    )
    .map_err(|error| error.to_string())
}

fn mime_from_ext(ext: &str) -> &'static str {
    match ext.to_ascii_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "image/png",
    }
}

fn decode_data_url(data_url: &str) -> Result<Vec<u8>, String> {
    let payload = data_url.split_once(',').map(|(_, data)| data).unwrap_or(data_url);
    STANDARD.decode(payload).map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            get_app_settings,
            save_app_settings,
            select_data_library_dir,
            init_data_library_dir,
            login,
            teacher_list,
            teacher_save,
            teacher_delete,
            student_list,
            student_save,
            student_delete,
            category_list,
            category_save,
            category_delete,
            question_list,
            question_batch_get,
            question_query,
            question_get,
            question_save,
            question_delete,
            paper_list,
            paper_get,
            paper_save,
            paper_delete,
            import_asset,
            save_board,
            export_question,
            save_export_file,
            read_asset_data_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running coscool desktop");
}
