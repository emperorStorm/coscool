<template>
  <div class="question-bank page-shell">
    <aside class="category-pane">
      <QuestionCategoryTree ref="categoryTreeRef" :selected-id="selectedCategoryId" title="题库分类" @select="selectCategory" />
    </aside>

    <main class="question-main">
      <div class="entry-toolbar">
        <div class="entry-search">
          <a-input v-model:value="keyword" placeholder="搜索题目" class="search-input" @press-enter="loadQuestions" />
          <button class="entry-search-button" title="搜索" @click="loadQuestions">⌕</button>
        </div>
        <button class="entry-create-button" @click="openQuestionEditor()">新增题目</button>
      </div>

      <a-empty v-if="questions.length === 0 && !loading" description="暂无题目" class="empty-panel" />
      <a-spin :spinning="loading">
        <article v-for="(question, index) in questions" :key="question.id" class="question-card">
          <div class="question-header">
            <div class="question-title-group">
              <span class="index-badge">{{ index + 1 }}</span>
              <span class="title-badge">{{ question.title || '未命名题目' }}</span>
            </div>
            <div class="question-actions">
              <button class="ghost-action" @click="openQuestionEditor(question.id)">编辑</button>
              <button class="ghost-action" @click="handleExport(question.id)">导出</button>
              <a-popconfirm title="确认删除该题目？" @confirm="removeQuestion(question.id)">
                <button class="danger-action">删除</button>
              </a-popconfirm>
            </div>
          </div>
          <div class="question-box">
            <p>{{ question.stem || '暂无题干' }}</p>
            <img v-if="questionImageMap[question.id]" class="card-image" :src="questionImageMap[question.id]" alt="题目配图预览" />
            <p v-else-if="question.imageText" class="muted">{{ question.imageText }}</p>
            <div class="option-row">
              <span v-for="option in question.options" :key="option.optionKey">{{ option.optionKey }}. {{ option.content }}</span>
            </div>
          </div>
          <div class="question-footer">
            <div class="pill-list">
              <span class="meta-label">知识点</span>
              <span v-for="item in question.knowledgePoints" :key="item" class="pill">{{ item }}</span>
              <span v-if="!question.knowledgePoints.length" class="muted meta-empty">暂无</span>
            </div>
            <div class="pill-list question-tags">
              <span class="meta-label">标签</span>
              <span v-for="item in question.tags" :key="item" class="pill">{{ item }}</span>
              <span v-if="!question.tags.length" class="muted meta-empty">暂无</span>
            </div>
          </div>
        </article>
      </a-spin>
    </main>

    <a-drawer v-model:open="questionOpen" title="题目录入" width="92%" destroy-on-close>
      <div class="editor-layout">
        <section class="editor-form">
          <div class="editor-topbar">
            <a-select v-model:value="questionForm.year" placeholder="年份" allow-clear>
              <a-select-option value="2026">2026</a-select-option>
              <a-select-option value="2025">2025</a-select-option>
              <a-select-option value="2024">2024</a-select-option>
            </a-select>
            <a-input v-model:value="questionForm.questionNo" placeholder="题号" />
            <a-button>智能识别</a-button>
            <a-button>AI 助手</a-button>
            <a-button>快速录入</a-button>
          </div>
          <div class="entry-block">
            <label>标题</label>
            <a-input v-model:value="questionForm.title" />
          </div>
          <div class="entry-block">
            <div class="block-head">
              <label>题目</label>
              <a-space>
                <a-button size="small">替换</a-button>
                <a-button size="small">选择并解析选项</a-button>
              </a-space>
            </div>
            <a-textarea v-model:value="questionForm.stem" :rows="5" />
          </div>
          <div class="entry-block">
            <div class="block-head">
              <label>题目配图</label>
              <a-space>
                <a-button size="small" @click="addImage">导入图片</a-button>
                <a-button size="small" @click="boardVisible = !boardVisible">批注板</a-button>
              </a-space>
            </div>
            <a-input v-model:value="questionForm.imageText" placeholder="图片说明或配图描述" />
            <div v-if="questionImageUrl" class="entry-image-preview">
              <img :src="questionImageUrl" alt="题目配图预览" />
            </div>
            <BoardEditor v-if="boardVisible" class="board-section" @save="handleBoardSave" />
          </div>
          <div class="option-grid">
            <div v-for="option in questionForm.options" :key="option.optionKey" class="entry-block option-block">
              <label>选项 {{ option.optionKey }}</label>
              <a-textarea v-model:value="option.content" :rows="3" />
            </div>
          </div>
          <div class="entry-block">
            <div class="block-head">
              <label>知识点</label>
              <a-button size="small">历史数据</a-button>
            </div>
            <a-select v-model:value="questionForm.knowledgePoints" mode="tags" placeholder="输入知识点后回车" />
          </div>
          <div class="entry-block">
            <div class="block-head">
              <label>标签</label>
              <a-button size="small">历史数据</a-button>
            </div>
            <a-select v-model:value="questionForm.tags" mode="tags" placeholder="输入标签后回车" />
          </div>
          <div class="entry-block">
            <label>答案（结果）</label>
            <a-textarea v-model:value="questionForm.answer" :rows="3" />
          </div>
          <div class="entry-block">
            <label>解析</label>
            <a-textarea v-model:value="questionForm.analysis" :rows="4" />
          </div>
        </section>

        <section class="preview-pane">
          <div class="preview-card">
            <span class="preview-label">题目</span>
            <h3>{{ questionForm.stem || '题目内容预览' }}</h3>
            <img v-if="questionImageUrl" class="preview-image" :src="questionImageUrl" alt="题目配图预览" />
            <p v-else-if="questionForm.imageText" class="muted">{{ questionForm.imageText }}</p>
            <div class="preview-options">
              <span v-for="option in questionForm.options" :key="option.optionKey">{{ option.optionKey }}. {{ option.content || '未填写' }}</span>
            </div>
          </div>
          <div class="preview-card">
            <span class="preview-label">知识点</span>
            <div class="pill-list">
              <span v-for="item in questionForm.knowledgePoints" :key="item" class="pill">{{ item }}</span>
            </div>
          </div>
          <div class="preview-card">
            <span class="preview-label">标签</span>
            <div class="pill-list">
              <span v-for="item in questionForm.tags" :key="item" class="pill">{{ item }}</span>
            </div>
          </div>
          <div class="preview-card">
            <span class="preview-label">答案</span>
            <p>{{ questionForm.answer || '暂无答案' }}</p>
          </div>
          <div class="preview-card">
            <span class="preview-label">解析</span>
            <p>{{ questionForm.analysis || '暂无解析' }}</p>
          </div>
        </section>
      </div>
      <template #footer>
        <a-space>
          <a-button @click="questionOpen = false">返回</a-button>
          <a-button type="primary" :loading="savingQuestion" @click="submitQuestion">保存</a-button>
          <a-button type="primary" :loading="savingQuestion" @click="submitAndNext">录下一题</a-button>
        </a-space>
      </template>
    </a-drawer>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref, watch } from 'vue'
import { message } from 'ant-design-vue'
import { open } from '@tauri-apps/plugin-dialog'
import BoardEditor from '../components/BoardEditor.vue'
import QuestionCategoryTree from '../components/QuestionCategoryTree.vue'
import type { Question, QuestionPayload } from '../api/native'
import {
  deleteQuestion,
  exportQuestion,
  getAppSettings,
  getQuestion,
  initDataLibraryDir,
  importAsset,
  listQuestions,
  readAssetDataUrl,
  saveBoard,
  saveQuestion,
} from '../api/native'

const loading = ref(false)
const savingQuestion = ref(false)
const questionOpen = ref(false)
const boardVisible = ref(false)
const categoryTreeRef = ref<InstanceType<typeof QuestionCategoryTree>>()
const selectedCategoryId = ref<number | undefined>()
const keyword = ref('')
const questionImageUrl = ref('')
const questionImageMap = ref<Record<number, string>>({})
const questions = ref<Question[]>([])
const questionForm = reactive<QuestionPayload>({
  categoryId: undefined,
  title: '',
  stem: '',
  imageText: '',
  year: '',
  questionNo: '',
  answer: '',
  analysis: '',
  createdBy: 'yaoyao',
  options: [
    { optionKey: 'A', content: '', sortOrder: 1 },
    { optionKey: 'B', content: '', sortOrder: 2 },
    { optionKey: 'C', content: '', sortOrder: 3 },
    { optionKey: 'D', content: '', sortOrder: 4 }
  ],
  tags: [],
  knowledgePoints: []
})

onMounted(async () => {
  await ensureLibrary()
  await loadQuestions()
})

watch(
  () => questionForm.imageText,
  () => loadQuestionImage()
)

async function ensureLibrary() {
  const settings = await getAppSettings()
  if (!settings.dataLibraryPath) {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择课思库基础数据仓库目录'
    })
    const path = Array.isArray(selected) ? selected[0] : selected
    if (!path) {
      message.warning('需要先选择基础数据仓库目录')
      return
    }
    await initDataLibraryDir(path)
  }
}

async function loadQuestions() {
  loading.value = true
  try {
    questions.value = await listQuestions(selectedCategoryId.value, keyword.value)
    await loadQuestionListImages()
  } catch (error) {
    message.error(String(error))
  } finally {
    loading.value = false
  }
}

async function loadQuestionListImages() {
  const imageMap: Record<number, string> = {}
  await Promise.all(
    questions.value.map(async (question) => {
      if (!question.imageText || !question.imageText.startsWith('assets/')) return
      try {
        const asset = await readAssetDataUrl(question.imageText)
        imageMap[question.id] = asset.dataUrl
      } catch {
        imageMap[question.id] = ''
      }
    })
  )
  questionImageMap.value = imageMap
}

function selectCategory(id?: number) {
  selectedCategoryId.value = id
  loadQuestions()
}

async function openQuestionEditor(id?: number) {
  resetQuestion()
  if (id) {
    Object.assign(questionForm, await getQuestion(id))
  } else {
    questionForm.categoryId = selectedCategoryId.value
  }
  questionOpen.value = true
  await loadQuestionImage()
}

function resetQuestion() {
  Object.assign(questionForm, {
    id: undefined,
    categoryId: selectedCategoryId.value,
    title: '',
    stem: '',
    imageText: '',
    year: '',
    questionNo: '',
    answer: '',
    analysis: '',
    createdBy: 'yaoyao',
    options: [
      { optionKey: 'A', content: '', sortOrder: 1 },
      { optionKey: 'B', content: '', sortOrder: 2 },
      { optionKey: 'C', content: '', sortOrder: 3 },
      { optionKey: 'D', content: '', sortOrder: 4 }
    ],
    tags: [],
    knowledgePoints: []
  })
}

async function submitQuestion() {
  if (!questionForm.title || !questionForm.stem) {
    message.warning('请填写标题和题目')
    return false
  }
  savingQuestion.value = true
  try {
    const saved = await saveQuestion(questionForm)
    Object.assign(questionForm, saved)
    questionOpen.value = false
    await categoryTreeRef.value?.load()
    await loadQuestions()
    return true
  } catch (error) {
    message.error(String(error))
    return false
  } finally {
    savingQuestion.value = false
  }
}

async function submitAndNext() {
  const saved = await submitQuestion()
  if (!saved) return
  resetQuestion()
  questionOpen.value = true
}

async function removeQuestion(id: number) {
  try {
    await deleteQuestion(id)
    await loadQuestions()
  } catch (error) {
    message.error(String(error))
  }
}

async function handleExport(id: number) {
  try {
    const path = await exportQuestion(id)
    message.success(`已导出：${path}`)
  } catch (error) {
    message.error(String(error))
  }
}

async function addImage() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: '图片',
        extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif']
      }
    ]
  })
  const sourcePath = Array.isArray(selected) ? selected[0] : selected
  if (!sourcePath) return
  try {
    const asset = await importAsset(sourcePath, questionForm.id)
    questionForm.imageText = asset.filePath
    await loadQuestionImage()
    message.success('图片已导入资源目录')
  } catch (error) {
    message.error(String(error))
  }
}

async function handleBoardSave(payload: { json: string; previewDataUrl: string }) {
  try {
    await saveBoard(questionForm.id, payload.json, payload.previewDataUrl)
    message.success('批注板已保存')
  } catch (error) {
    message.error(String(error))
  }
}

async function loadQuestionImage() {
  questionImageUrl.value = ''
  if (!questionForm.imageText || !questionForm.imageText.startsWith('assets/')) return
  try {
    const asset = await readAssetDataUrl(questionForm.imageText)
    questionImageUrl.value = asset.dataUrl
  } catch {
    questionImageUrl.value = ''
  }
}
</script>

<style scoped>
.question-bank {
  display: grid;
  grid-template-columns: 300px minmax(0, 1fr);
  gap: 14px;
  height: calc(100vh - 58px);
  overflow: hidden;
}

.category-pane {
  min-height: 0;
  overflow: hidden;
}

.search-input {
  min-width: 220px;
}

.question-main {
  min-width: 0;
  padding: 34px 44px;
  overflow: auto;
}

.entry-toolbar {
  position: sticky;
  top: 0;
  z-index: 5;
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: 980px;
  margin: 0 auto;
  padding-bottom: 42px;
  background: #edf1f4;
}

.entry-search {
  display: grid;
  grid-template-columns: minmax(240px, 390px) 54px;
  height: 46px;
}

.entry-search :deep(.ant-input) {
  height: 46px;
  padding: 0 16px;
  color: #263447;
  font-size: 18px;
  font-weight: 700;
  background: #ffffff;
  border-color: #d7e0ea;
  border-radius: 8px 0 0 8px;
}

.entry-search :deep(.ant-input::placeholder) {
  color: #c0c4ca;
}

.entry-search-button {
  display: grid;
  color: #777f88;
  cursor: pointer;
  place-items: center;
  background: #ffffff;
  border: 1px solid #d7e0ea;
  border-left: 0;
  border-radius: 0 8px 8px 0;
  box-shadow: 0 2px 4px rgba(49, 63, 82, 0.08);
  font-size: 30px;
  line-height: 1;
}

.entry-create-button {
  min-width: 136px;
  height: 50px;
  padding: 0 22px;
  color: #ffffff;
  font-size: 18px;
  font-weight: 800;
  cursor: pointer;
  background: #0f9187;
  border: 0;
  border-radius: 8px;
  box-shadow: 0 5px 0 rgba(10, 93, 86, 0.22);
}

.entry-create-button:hover {
  background: #0b8279;
}

.empty-panel {
  padding: 80px 0;
  background: #ffffff;
  border-radius: 8px;
}

.question-header {
  display: flex;
  gap: 12px;
  align-items: center;
  justify-content: space-between;
  min-width: 0;
}

.question-card {
  max-width: 980px;
  margin-right: auto;
  margin-left: auto;
  padding: 16px 18px;
}

.question-title-group {
  display: flex;
  gap: 10px;
  align-items: center;
  min-width: 0;
}

.question-actions {
  display: flex;
  flex: 0 0 auto;
  gap: 8px;
  align-items: center;
}

.index-badge {
  display: inline-grid;
  min-width: 26px;
  height: 26px;
  padding: 0 7px;
  color: #253142;
  font-size: 14px;
  place-items: center;
  background: #f8fbff;
  border: 1px solid #dbe3ee;
  border-radius: 6px;
}

.title-badge {
  display: inline-grid;
  min-width: 0;
  min-height: 26px;
  padding: 0 10px;
  overflow: hidden;
  color: #009b94;
  font-size: 14px;
  font-weight: 800;
  place-items: center;
  background: #e9fbf8;
  border: 1px solid #77ddd4;
  border-radius: 6px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.question-box {
  margin: 14px 0;
  padding: 18px 22px;
  border: 1px dashed #cfe1f0;
  border-radius: 8px;
}

.question-box p {
  color: #243144;
  font-size: 15px;
  font-weight: 700;
  line-height: 1.7;
  margin: 0;
}

.option-row {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 12px;
  margin-top: 18px;
  color: #263447;
  font-size: 15px;
  line-height: 1.6;
}

.card-image {
  display: block;
  max-width: 100%;
  max-height: 170px;
  margin: 12px 0;
  object-fit: contain;
  border: 1px solid #e2e9f0;
  border-radius: 6px;
}

.meta-label {
  color: #8a98a8;
  font-size: 13px;
  font-weight: 700;
}

.question-footer {
  display: flex;
  gap: 14px;
  align-items: center;
  justify-content: space-between;
}

.ghost-action,
.danger-action {
  min-width: 52px;
  height: 28px;
  padding: 0 10px;
  font-size: 13px;
  font-weight: 800;
  cursor: pointer;
  background: #ffffff;
  border-radius: 6px;
  box-shadow: 0 2px 0 rgba(37, 49, 66, 0.08);
}

.ghost-action {
  color: #263447;
  border: 1px solid #d8e1ec;
}

.danger-action {
  color: #ff2d2d;
  border: 1px solid #ff3b3b;
}

.meta-empty {
  font-size: 13px;
}

.question-tags {
  justify-content: flex-end;
}

.pill {
  min-height: 26px;
  padding: 0 12px;
  color: #009b94;
  font-size: 13px;
  font-weight: 800;
  background: #e9fbf8;
  border-color: #c6eee8;
}

.editor-layout {
  display: grid;
  grid-template-columns: minmax(520px, 1fr) minmax(420px, 1fr);
  gap: 14px;
  min-height: calc(100vh - 170px);
  background: #eef1f4;
}

.editor-form,
.preview-pane {
  min-width: 0;
  max-height: calc(100vh - 170px);
  overflow: auto;
}

.editor-topbar {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  padding: 10px 0;
}

.entry-block,
.preview-card {
  margin-bottom: 12px;
  padding: 12px;
  background: #ffffff;
  border: 1px solid #e6ebf0;
  border-radius: 8px;
}

.entry-block label {
  display: inline-block;
  margin-bottom: 8px;
  color: #7a3ff2;
  font-weight: 700;
}

.block-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.option-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
}

.board-section {
  margin-top: 10px;
}

.entry-image-preview {
  margin-top: 10px;
}

.entry-image-preview img,
.preview-image {
  display: block;
  max-width: 100%;
  max-height: 260px;
  object-fit: contain;
  border: 1px solid #e2e9f0;
  border-radius: 6px;
}

.preview-image {
  margin: 12px 0;
}

.preview-pane {
  padding-left: 10px;
  border-left: 3px solid #d3d9e0;
}

.preview-card {
  padding: 20px;
}

.preview-label {
  display: block;
  margin-bottom: 14px;
  color: #98a5b3;
  font-weight: 700;
}

.preview-options {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 18px;
  margin-top: 22px;
  font-size: 16px;
}

@media (max-width: 1280px) {
  .question-bank {
    grid-template-columns: 260px minmax(0, 1fr);
  }

  .editor-layout {
    grid-template-columns: 1fr;
  }

  .question-footer {
    align-items: flex-start;
    flex-direction: column;
  }

  .question-tags {
    justify-content: flex-start;
  }

  .preview-pane {
    padding-left: 0;
    border-left: 0;
  }
}
</style>
