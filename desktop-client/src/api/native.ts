import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import { getVersion } from '@tauri-apps/api/app'
import { relaunch } from '@tauri-apps/plugin-process'
import { check, type DownloadEvent, type Update } from '@tauri-apps/plugin-updater'

const UPDATE_CHECK_TIMEOUT_MS = 15000
const UPDATE_DOWNLOAD_TIMEOUT_MS = 120000

export interface AppSettings {
  dataLibraryPath?: string
  currentTeacherAccount?: string
}

export interface Teacher {
  id: number
  account: string
  password?: string
  name: string
  phone: string
  remark: string
  status: string
  createdAt: string
  updatedAt: string
}

export interface Student {
  id: number
  name: string
  phone: string
  grade: string
  school: string
  remark: string
  createdAt: string
  updatedAt: string
}

export interface QuestionCategory {
  id: number
  parentId?: number
  name: string
  sortOrder: number
  createdAt: string
  updatedAt: string
}

export interface KnowledgePoint {
  id: number
  parentId?: number
  name: string
  sortOrder: number
  createdAt: string
  updatedAt: string
}

export interface KnowledgePointReorderItem {
  id: number
  parentId?: number
  sortOrder: number
}

export interface QuestionOption {
  id?: number
  optionKey: string
  content: string
  imageText: string
  sortOrder: number
}

export interface Question {
  id: number
  categoryId?: number
  title: string
  stem: string
  imageText: string
  year: string
  questionNo: string
  questionType: string
  difficulty: number
  answer: string
  analysis: string
  createdBy: string
  createdAt: string
  updatedAt: string
  options: QuestionOption[]
  tags: string[]
  knowledgePoints: string[]
  knowledgePointIds: number[]
}

export interface Paper {
  id: number
  title: string
  remark: string
  questionCount: number
  createdBy: string
  createdAt: string
  updatedAt: string
}

export interface PaperQuestionPayload {
  questionId: number
  sortOrder: number
}

export interface PaperPayload {
  title: string
  remark: string
  createdBy: string
  questions: PaperQuestionPayload[]
}

export interface PaperDetail {
  paper: Paper
  questions: Question[]
}

export interface QuestionPayload {
  id?: number
  categoryId?: number
  title: string
  stem: string
  imageText: string
  year: string
  questionNo: string
  questionType: string
  difficulty: number
  answer: string
  analysis: string
  createdBy: string
  options: QuestionOption[]
  tags: string[]
  knowledgePoints: string[]
  knowledgePointIds: number[]
}

export interface AssetRecord {
  id: number
  questionId?: number
  assetType: string
  filePath: string
  thumbPath: string
  mimeType: string
  hashValue: string
  width: number
  height: number
  size: number
  createdAt: string
}

export interface QuestionQueryFilters {
  categoryId?: number
  includeChildren: boolean
  title: string
  year: string
  tag: string
  knowledgePoint: string
}

export interface AssetDataUrl {
  dataUrl: string
  mimeType: string
}

export interface BackupResult {
  filePath: string
  createdAt: string
}

export interface UpdateCheckResult {
  currentVersion: string
  update?: Update
}

function invoke<T>(command: string, args?: Parameters<typeof tauriInvoke>[1]) {
  if (!isTauriRuntime()) {
    return Promise.reject(new Error('当前页面需要在课思库桌面客户端中运行，请通过 Tauri 开发模式或已安装的桌面应用打开。'))
  }
  return tauriInvoke<T>(command, args)
}

export function isTauriRuntime() {
  return typeof window !== 'undefined' && Boolean((window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__)
}

export function getCurrentVersion() {
  return getVersion()
}

export function formatUpdateError(error: unknown) {
  const text = String(error)
  if (text.includes('error sending request') || text.includes('timed out') || text.includes('timeout')) {
    return '连接更新服务失败，请稍后重试或检查当前网络。'
  }
  return text
}

export async function checkAppUpdate(): Promise<UpdateCheckResult> {
  const update = await check({ timeout: UPDATE_CHECK_TIMEOUT_MS })
  return {
    currentVersion: update?.currentVersion || await getVersion(),
    update: update || undefined
  }
}

export async function installAppUpdate(
  update: Update,
  onEvent: (event: DownloadEvent) => void
) {
  await update.downloadAndInstall(onEvent, { timeout: UPDATE_DOWNLOAD_TIMEOUT_MS })
  await relaunch()
}

export function getAppSettings() {
  return invoke<AppSettings>('get_app_settings')
}

export function saveAppSettings(settings: AppSettings) {
  return invoke<AppSettings>('save_app_settings', { settings })
}

export function initDataLibraryDir(path: string) {
  return invoke<AppSettings>('init_data_library_dir', { path })
}

export function login(account: string, password: string) {
  return invoke<Teacher>('login', { account, password })
}

export function listTeachers() {
  return invoke<Teacher[]>('teacher_list')
}

export function saveTeacher(teacher: Partial<Teacher>) {
  return invoke<Teacher>('teacher_save', { teacher })
}

export function deleteTeacher(id: number) {
  return invoke<boolean>('teacher_delete', { id })
}

export function listStudents() {
  return invoke<Student[]>('student_list')
}

export function saveStudent(student: Partial<Student>) {
  return invoke<Student>('student_save', { student })
}

export function deleteStudent(id: number) {
  return invoke<boolean>('student_delete', { id })
}

export function listCategories() {
  return invoke<QuestionCategory[]>('category_list')
}

export function saveCategory(category: Partial<QuestionCategory>) {
  return invoke<QuestionCategory>('category_save', { category })
}

export function deleteCategory(id: number) {
  return invoke<boolean>('category_delete', { id })
}

export function listKnowledgePoints() {
  return invoke<KnowledgePoint[]>('knowledge_point_list')
}

export function saveKnowledgePoint(knowledgePoint: Partial<KnowledgePoint>) {
  return invoke<KnowledgePoint>('knowledge_point_save', { knowledgePoint })
}

export function deleteKnowledgePoint(id: number) {
  return invoke<boolean>('knowledge_point_delete', { id })
}

export function reorderKnowledgePoints(updates: KnowledgePointReorderItem[]) {
  return invoke<boolean>('knowledge_point_reorder', { updates })
}

export function listQuestions(categoryId?: number, keyword?: string) {
  return invoke<Question[]>('question_list', { categoryId, keyword })
}

export function queryQuestions(filters: QuestionQueryFilters) {
  return invoke<Question[]>('question_query', { filters })
}

export function getQuestion(id: number) {
  return invoke<Question>('question_get', { id })
}

export function getQuestionsByIds(ids: number[]) {
  return invoke<Question[]>('question_batch_get', { ids })
}

export function saveQuestion(question: QuestionPayload) {
  return invoke<Question>('question_save', { question })
}

export function deleteQuestion(id: number) {
  return invoke<boolean>('question_delete', { id })
}

export function importAsset(sourcePath: string, questionId?: number) {
  return invoke<AssetRecord>('import_asset', { sourcePath, questionId })
}

export function saveBoard(questionId: number | undefined, boardJson: string, previewDataUrl: string) {
  return invoke<AssetRecord>('save_board', { questionId, boardJson, previewDataUrl })
}

export function exportQuestion(id: number) {
  return invoke<string>('export_question', { id })
}

export function exportDataBackup() {
  return invoke<BackupResult>('export_data_backup')
}

export function restoreDataBackup(sourcePath: string) {
  return invoke<BackupResult>('restore_data_backup', { sourcePath })
}

export function saveExportFile(targetPath: string, dataUrl: string) {
  return invoke<string>('save_export_file', { targetPath, dataUrl })
}

export function readAssetDataUrl(relativePath: string) {
  return invoke<AssetDataUrl>('read_asset_data_url', { relativePath })
}

export function listPapers() {
  return invoke<Paper[]>('paper_list')
}

export function getPaper(id: number) {
  return invoke<PaperDetail>('paper_get', { id })
}

export function savePaper(paper: PaperPayload) {
  return invoke<PaperDetail>('paper_save', { paper })
}

export function deletePaper(id: number) {
  return invoke<boolean>('paper_delete', { id })
}
