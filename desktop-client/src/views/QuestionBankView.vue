<template>
  <div class="question-bank page-shell">
    <aside class="category-pane">
      <QuestionCategoryTree ref="categoryTreeRef" :selected-id="selectedCategoryId" title="题库分类" @select="selectCategory" />
    </aside>

    <main class="question-main">
      <div class="entry-toolbar">
        <div class="entry-search">
          <a-input v-model:value="keyword" placeholder="搜索题目" class="search-input" @press-enter="loadQuestions" />
          <button class="entry-search-button" title="搜索" @click="loadQuestions">
            <Search :size="16" :stroke-width="2.4" />
          </button>
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
              <span v-if="question.questionType" class="meta-chip">{{ question.questionType }}</span>
              <span v-if="question.difficulty > 0" class="meta-chip difficulty-chip">{{ formatDifficulty(question.difficulty) }}</span>
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
            <MathText class="question-text" :content="question.stem || '暂无题干'" />
            <img v-if="questionImageMap[question.id]" class="card-image" :src="questionImageMap[question.id]" alt="题目配图预览" />
            <p v-else-if="question.imageText" class="muted">{{ question.imageText }}</p>
            <div class="option-row">
              <div v-for="option in question.options" :key="option.optionKey" class="option-item">
                <strong>{{ option.optionKey }}.</strong>
                <MathText :content="option.content || '-'" />
              </div>
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

  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { message } from 'ant-design-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useRouter } from 'vue-router'
import { Search } from 'lucide-vue-next'
import MathText from '../components/MathText.vue'
import QuestionCategoryTree from '../components/QuestionCategoryTree.vue'
import type { Question } from '../api/native'
import {
  deleteQuestion,
  getAppSettings,
  initDataLibraryDir,
  listQuestions,
  readAssetDataUrl,
} from '../api/native'

const loading = ref(false)
const categoryTreeRef = ref<InstanceType<typeof QuestionCategoryTree>>()
const router = useRouter()
const selectedCategoryId = ref<number | undefined>()
const keyword = ref('')
const questionImageMap = ref<Record<number, string>>({})
const questions = ref<Question[]>([])

onMounted(async () => {
  await ensureLibrary()
  await loadQuestions()
})

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

function openQuestionEditor(id?: number) {
  if (id) {
    router.push(`/question-editor/${id}`)
    return
  }
  router.push({
    path: '/question-editor',
    query: selectedCategoryId.value ? { categoryId: selectedCategoryId.value } : undefined
  })
}

async function removeQuestion(id: number) {
  try {
    await deleteQuestion(id)
    await loadQuestions()
  } catch (error) {
    message.error(String(error))
  }
}

function handleExport(id: number) {
  router.push(`/question-export/${id}`)
}

function formatDifficulty(difficulty: number) {
  const score = Math.max(0, Math.min(5, Math.trunc(difficulty || 0)))
  return `${'★'.repeat(score)}${'☆'.repeat(5 - score)}`
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
  padding-bottom: 22px;
  background: #edf1f4;
}

.entry-search {
  display: grid;
  grid-template-columns: minmax(220px, 360px) 40px;
  height: 34px;
}

.entry-search :deep(.ant-input) {
  height: 34px;
  padding: 0 12px;
  color: #263447;
  font-size: 14px;
  font-weight: 400;
  background: #ffffff;
  border-color: #d7e0ea;
  border-radius: 8px 0 0 8px;
}

.entry-search :deep(.ant-input::placeholder) {
  color: #c0c4ca;
}

.entry-search-button {
  display: grid;
  color: #687584;
  cursor: pointer;
  place-items: center;
  background: #ffffff;
  border: 1px solid #d7e0ea;
  border-left: 0;
  border-radius: 0 8px 8px 0;
  box-shadow: 0 1px 2px rgba(49, 63, 82, 0.06);
  line-height: 1;
}

.entry-search-button:hover {
  color: #0f9187;
}

.entry-create-button {
  min-width: 96px;
  height: 34px;
  padding: 0 16px;
  color: #ffffff;
  font-size: 14px;
  font-weight: 700;
  cursor: pointer;
  background: #0f9187;
  border: 0;
  border-radius: 6px;
  box-shadow: 0 2px 4px rgba(10, 93, 86, 0.14);
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

.meta-chip {
  display: inline-grid;
  min-height: 26px;
  padding: 0 9px;
  color: #607085;
  font-size: 13px;
  font-weight: 800;
  place-items: center;
  background: #f3f6fa;
  border: 1px solid #e1e8f0;
  border-radius: 999px;
  white-space: nowrap;
}

.difficulty-chip {
  color: #d97a00;
  background: #fff8ec;
  border-color: #ffe2ad;
}

.question-box {
  margin: 14px 0;
  padding: 18px 22px;
  border: 1px dashed #cfe1f0;
  border-radius: 8px;
}

.question-text {
  color: #243144;
  font-size: 15px;
  font-weight: 700;
  line-height: 1.7;
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

.option-item,
.preview-option-item {
  display: flex;
  gap: 6px;
  min-width: 0;
}

.option-item strong,
.preview-option-item strong {
  flex: 0 0 auto;
}

.option-item :deep(.math-text),
.preview-option-item :deep(.math-text) {
  min-width: 0;
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

@media (max-width: 1280px) {
  .question-bank {
    grid-template-columns: 260px minmax(0, 1fr);
  }

  .question-footer {
    align-items: flex-start;
    flex-direction: column;
  }

  .question-tags {
    justify-content: flex-start;
  }
}
</style>
