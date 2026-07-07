<template>
  <div class="question-editor-page">
    <aside class="editor-category">
      <QuestionCategoryTree
        ref="categoryTreeRef"
        :selected-id="selectedCategoryId"
        title="题库分类"
        @select="selectCategory"
      />
    </aside>

    <main class="editor-workspace">
      <header class="editor-header">
        <div class="editor-topbar">
          <a-select v-model:value="questionForm.year" placeholder="年份" allow-clear class="year-select">
            <a-select-option value="2026">2026</a-select-option>
            <a-select-option value="2025">2025</a-select-option>
            <a-select-option value="2024">2024</a-select-option>
          </a-select>
          <a-input v-model:value="questionForm.questionNo" placeholder="题号" class="question-no-input" />
          <a-button>智能识别</a-button>
          <a-button>快速录入</a-button>
          <a-button @click="addImage">配图</a-button>
          <a-button @click="boardVisible = !boardVisible">画板</a-button>
        </div>
        <div class="editor-actions">
          <button class="save-button" type="button" :disabled="savingQuestion" @click="submitQuestion">
            {{ savingQuestion ? '保存中' : '保存' }}
          </button>
          <button class="return-button" type="button" @click="goBack">返回</button>
        </div>
      </header>

      <section class="editor-content">
        <section class="editor-form-pane">
          <div class="entry-card meta-card">
            <div class="meta-field">
              <label>题型</label>
              <div class="question-type-buttons" role="radiogroup" aria-label="题型">
                <button
                  v-for="item in questionTypeOptions"
                  :key="item"
                  class="question-type-button"
                  :class="{ active: questionForm.questionType === item }"
                  type="button"
                  role="radio"
                  :aria-checked="questionForm.questionType === item"
                  @click="questionForm.questionType = item"
                >
                  {{ item }}
                </button>
              </div>
            </div>

            <div class="meta-field difficulty-field">
              <label>难度</label>
              <div class="difficulty-stars" aria-label="难度">
                <button
                  v-for="star in difficultyStars"
                  :key="star"
                  class="difficulty-star"
                  :class="{ active: star <= questionForm.difficulty }"
                  type="button"
                  :aria-label="`选择 ${star} 星难度`"
                  :title="`${star} 星难度`"
                  @click="setDifficulty(star)"
                >
                  {{ star <= questionForm.difficulty ? '★' : '☆' }}
                </button>
              </div>
            </div>
          </div>

          <div class="entry-card title-card">
            <label>标题</label>
            <a-input v-model:value="questionForm.title" placeholder="请输入..." />
          </div>

          <div class="entry-card stem-card">
            <div class="card-head">
              <label>题目</label>
              <a-space>
                <a-button size="small">替换</a-button>
                <a-button size="small">选择并解析选项</a-button>
              </a-space>
            </div>
            <a-textarea v-model:value="questionForm.stem" placeholder="请输入..." :rows="7" />
          </div>

          <div class="entry-card image-card">
            <div class="card-head">
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

          <div v-if="isChoiceQuestion" class="option-section">
            <div
              v-for="option in questionForm.options"
              :key="option.optionKey"
              class="entry-card option-card"
            >
              <label>选项 {{ option.optionKey }}</label>
              <a-textarea v-model:value="option.content" placeholder="..." :rows="3" />
            </div>
            <div class="option-tools">
              <button type="button" class="active">一行</button>
              <button type="button">一列</button>
              <button type="button">两列</button>
              <button type="button">模型</button>
              <button type="button" @click="addOption">添加</button>
            </div>
          </div>

          <div class="entry-card">
            <div class="card-head">
              <label>知识点</label>
              <a-button size="small">历史数据</a-button>
            </div>
            <a-select
              v-model:value="questionForm.knowledgePoints"
              mode="tags"
              placeholder="知识点，可输入多个"
              :token-separators="tagSeparators"
            />
          </div>

          <div class="entry-card">
            <div class="card-head">
              <label>标签</label>
              <a-button size="small">历史数据</a-button>
            </div>
            <a-select
              v-model:value="questionForm.tags"
              mode="tags"
              placeholder="标签，可输入多个"
              :token-separators="tagSeparators"
            />
          </div>

          <div class="entry-card">
            <div class="card-head">
              <label>答案（结果）</label>
              <a-space>
                <a-button size="small">替换</a-button>
                <a-button v-for="option in choiceOptions" :key="option.optionKey" size="small">
                  {{ option.optionKey }}
                </a-button>
              </a-space>
            </div>
            <a-textarea v-model:value="questionForm.answer" placeholder="请输入..." :rows="4" />
          </div>

          <div class="entry-card">
            <label>解析（解答过程）</label>
            <a-textarea v-model:value="questionForm.analysis" placeholder="请输入..." :rows="5" />
          </div>
        </section>

        <aside class="preview-pane">
          <div class="preview-paper">
            <span class="preview-label">题目</span>
            <h3>{{ questionForm.title || '未命名题目' }}</h3>
            <div v-if="questionForm.questionType || questionForm.difficulty > 0" class="preview-meta">
              <span v-if="questionForm.questionType">{{ questionForm.questionType }}</span>
              <span v-if="questionForm.difficulty > 0">{{ formatDifficulty(questionForm.difficulty) }}</span>
            </div>
            <MathText class="preview-question-text" :content="questionForm.stem || '题目内容预览'" />
            <img v-if="questionImageUrl" class="preview-image" :src="questionImageUrl" alt="题目配图预览" />
            <p v-else-if="questionForm.imageText" class="muted">{{ questionForm.imageText }}</p>
            <div v-if="isChoiceQuestion" class="preview-options">
              <div v-for="option in choiceOptions" :key="option.optionKey" class="preview-option-item">
                <strong>{{ option.optionKey }}.</strong>
                <MathText :content="option.content || '未填写'" />
              </div>
            </div>
          </div>

          <div class="preview-paper">
            <span class="preview-label">知识点</span>
            <div class="pill-list">
              <span v-for="item in questionForm.knowledgePoints" :key="item" class="pill">{{ item }}</span>
              <span v-if="!questionForm.knowledgePoints.length" class="muted">暂无</span>
            </div>
          </div>

          <div class="preview-paper">
            <span class="preview-label">标签</span>
            <div class="pill-list">
              <span v-for="item in questionForm.tags" :key="item" class="pill">{{ item }}</span>
              <span v-if="!questionForm.tags.length" class="muted">暂无</span>
            </div>
          </div>

          <div class="preview-paper">
            <span class="preview-label">答案</span>
            <MathText :content="questionForm.answer || '暂无答案'" />
          </div>

          <div class="preview-paper">
            <span class="preview-label">解析</span>
            <MathText :content="questionForm.analysis || '暂无解析'" />
          </div>
        </aside>
      </section>
    </main>

    <FloatingToolDock />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { message } from 'ant-design-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useRoute, useRouter } from 'vue-router'
import BoardEditor from '../components/BoardEditor.vue'
import FloatingToolDock from '../components/FloatingToolDock.vue'
import MathText from '../components/MathText.vue'
import QuestionCategoryTree from '../components/QuestionCategoryTree.vue'
import type { QuestionPayload } from '../api/native'
import {
  getAppSettings,
  getQuestion,
  importAsset,
  initDataLibraryDir,
  readAssetDataUrl,
  saveBoard,
  saveQuestion
} from '../api/native'

const route = useRoute()
const router = useRouter()
const savingQuestion = ref(false)
const boardVisible = ref(false)
const categoryTreeRef = ref<InstanceType<typeof QuestionCategoryTree>>()
const selectedCategoryId = ref<number | undefined>()
const questionImageUrl = ref('')
const questionForm = reactive<QuestionPayload>(createEmptyQuestion())
const questionTypeOptions = ['单选题', '多选题', '填空题', '证明题', '计算题', '解答题']
const choiceQuestionTypes = ['单选题', '多选题']
const difficultyStars = [1, 2, 3, 4, 5]
const tagSeparators = [',', '，', ';', '；', '\n']
const isChoiceQuestion = computed(() => choiceQuestionTypes.includes(questionForm.questionType))
const choiceOptions = computed(() => (isChoiceQuestion.value ? questionForm.options : []))

onMounted(async () => {
  await ensureLibrary()
  await loadQuestion()
})

watch(
  () => questionForm.imageText,
  () => loadQuestionImage()
)

watch(
  () => questionForm.questionType,
  () => {
    if (isChoiceQuestion.value && questionForm.options.length === 0) {
      questionForm.options = createDefaultOptions()
    }
  }
)

function createEmptyQuestion(): QuestionPayload {
  return {
    categoryId: undefined,
    title: '',
    stem: '',
    imageText: '',
    year: '',
    questionNo: '',
    questionType: '',
    difficulty: 0,
    answer: '',
    analysis: '',
    createdBy: 'yaoyao',
    options: createDefaultOptions(),
    tags: [],
    knowledgePoints: []
  }
}

function createDefaultOptions() {
  return [
    { optionKey: 'A', content: '', sortOrder: 1 },
    { optionKey: 'B', content: '', sortOrder: 2 },
    { optionKey: 'C', content: '', sortOrder: 3 },
    { optionKey: 'D', content: '', sortOrder: 4 }
  ]
}

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

async function loadQuestion() {
  resetQuestion()
  const id = getRouteQuestionId()
  if (id) {
    Object.assign(questionForm, await getQuestion(id))
    selectedCategoryId.value = questionForm.categoryId
  } else {
    const categoryId = Number(route.query.categoryId) || undefined
    questionForm.categoryId = categoryId
    selectedCategoryId.value = categoryId
  }
  await loadQuestionImage()
}

function getRouteQuestionId() {
  const id = Array.isArray(route.params.id) ? route.params.id[0] : route.params.id
  return Number(id) || undefined
}

function resetQuestion() {
  Object.assign(questionForm, createEmptyQuestion())
  questionImageUrl.value = ''
}

function selectCategory(id?: number) {
  selectedCategoryId.value = id
  questionForm.categoryId = id
}

function addOption() {
  const nextIndex = questionForm.options.length
  const optionKey = String.fromCharCode(65 + nextIndex)
  questionForm.options.push({
    optionKey,
    content: '',
    sortOrder: nextIndex + 1
  })
}

function setDifficulty(star: number) {
  questionForm.difficulty = questionForm.difficulty === star ? 0 : star
}

function formatDifficulty(difficulty: number) {
  const score = Math.max(0, Math.min(5, Math.trunc(difficulty || 0)))
  return `${'★'.repeat(score)}${'☆'.repeat(5 - score)}`
}

async function submitQuestion() {
  if (!questionForm.title || !questionForm.stem) {
    message.warning('请填写标题和题目')
    return
  }
  savingQuestion.value = true
  try {
    await saveQuestion(buildQuestionPayload())
    await categoryTreeRef.value?.load()
    message.success('题目已保存')
    await router.replace('/app/questions/entry')
  } catch (error) {
    message.error(String(error))
  } finally {
    savingQuestion.value = false
  }
}

function goBack() {
  router.replace('/app/questions/entry')
}

function buildQuestionPayload(): QuestionPayload {
  return {
    ...questionForm,
    options: isChoiceQuestion.value ? questionForm.options : [],
    tags: normalizeNameList(questionForm.tags),
    knowledgePoints: normalizeNameList(questionForm.knowledgePoints)
  }
}

function normalizeNameList(items: string[]) {
  const nameSet = new Set<string>()
  items.forEach((item) => {
    item
      .split(/[,，;；\n]/)
      .map((name) => name.trim())
      .filter(Boolean)
      .forEach((name) => nameSet.add(name))
  })
  return Array.from(nameSet)
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
.question-editor-page {
  display: grid;
  grid-template-columns: 300px minmax(0, 1fr);
  height: 100vh;
  overflow: hidden;
  background: #eef1f4;
}

.editor-category {
  min-height: 0;
  padding: 16px 12px 16px 18px;
  overflow: hidden;
  background: #f4f6f8;
  border-right: 1px solid #dde4ed;
}

.editor-workspace {
  display: grid;
  grid-template-rows: 58px minmax(0, 1fr);
  min-width: 0;
  overflow: hidden;
}

.editor-header {
  display: flex;
  gap: 16px;
  align-items: center;
  justify-content: space-between;
  padding: 10px 16px 8px;
  background: #eef1f4;
  border-bottom: 1px solid #e1e7ee;
}

.editor-topbar {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
  min-width: 0;
}

.year-select {
  width: 120px;
}

.question-no-input {
  width: 120px;
}

.editor-actions {
  display: flex;
  flex: 0 0 auto;
  gap: 8px;
}

.save-button,
.return-button {
  height: 34px;
  min-width: 68px;
  padding: 0 16px;
  color: #ffffff;
  font-size: 14px;
  font-weight: 900;
  cursor: pointer;
  border: 0;
  border-radius: 6px;
}

.save-button {
  background: #18b7b2;
}

.save-button:disabled {
  cursor: not-allowed;
  opacity: 0.65;
}

.return-button {
  color: #354153;
  background: #ffffff;
  border: 1px solid #d7e0ea;
}

.editor-content {
  display: grid;
  grid-template-columns: minmax(520px, 1fr) minmax(420px, 1fr);
  gap: 12px;
  min-height: 0;
  padding: 10px 14px 14px;
  overflow: hidden;
}

.editor-form-pane,
.preview-pane {
  min-width: 0;
  overflow: auto;
}

.editor-form-pane {
  padding-right: 4px;
}

.entry-card,
.preview-paper {
  margin-bottom: 10px;
  padding: 10px;
  background: #ffffff;
  border: 1px solid #e5ebf1;
  border-radius: 8px;
}

.entry-card label {
  display: inline-block;
  margin-bottom: 8px;
  color: #7a3ff2;
  font-size: 14px;
  font-weight: 900;
}

.meta-card {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 220px;
  gap: 14px;
  align-items: start;
}

.meta-field {
  min-width: 0;
}

.question-type-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.question-type-button {
  height: 32px;
  padding: 0 13px;
  color: #526174;
  font-size: 13px;
  font-weight: 800;
  cursor: pointer;
  background: #f8fafc;
  border: 1px solid #dce5ef;
  border-radius: 6px;
}

.question-type-button.active,
.question-type-button:hover {
  color: #ffffff;
  background: #3aa4ff;
  border-color: #3aa4ff;
}

.difficulty-stars {
  display: flex;
  gap: 4px;
  align-items: center;
  min-height: 32px;
}

.difficulty-star {
  display: grid;
  width: 28px;
  height: 28px;
  padding: 0;
  color: #aeb8c4;
  font-size: 22px;
  line-height: 1;
  cursor: pointer;
  place-items: center;
  background: transparent;
  border: 0;
}

.difficulty-star.active,
.difficulty-star:hover {
  color: #ff9f1c;
}

.card-head {
  display: flex;
  gap: 8px;
  align-items: center;
  justify-content: space-between;
}

.option-section {
  position: relative;
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 8px;
  padding-right: 54px;
}

.option-tools {
  position: absolute;
  top: 0;
  right: 0;
  display: grid;
  gap: 6px;
  width: 44px;
}

.option-tools button {
  height: 28px;
  color: #58677b;
  font-size: 13px;
  font-weight: 900;
  cursor: pointer;
  background: #ffffff;
  border: 1px solid #dbe5ee;
  border-radius: 6px;
}

.option-tools button.active,
.option-tools button:hover {
  color: #ffffff;
  background: #3aa4ff;
  border-color: #3aa4ff;
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

.board-section {
  margin-top: 10px;
}

.preview-pane {
  padding-left: 10px;
  border-left: 3px solid #d4dae2;
}

.preview-paper {
  padding: 18px;
}

.preview-paper h3 {
  margin: 0 0 12px;
  color: #263447;
  font-size: 18px;
}

.preview-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin: -2px 0 12px;
}

.preview-meta span {
  min-height: 24px;
  padding: 2px 9px;
  color: #607085;
  font-size: 13px;
  font-weight: 800;
  background: #f3f6fa;
  border: 1px solid #e1e8f0;
  border-radius: 999px;
}

.preview-label {
  display: block;
  margin-bottom: 12px;
  color: #98a5b3;
  font-weight: 800;
}

.preview-question-text {
  color: #243144;
  font-size: 16px;
  font-weight: 800;
  line-height: 1.8;
}

.preview-image {
  margin: 12px 0;
}

.preview-options {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
  margin-top: 18px;
}

.preview-option-item {
  display: flex;
  gap: 6px;
  min-width: 0;
  color: #263447;
  font-size: 15px;
  line-height: 1.7;
}

.preview-option-item strong {
  flex: 0 0 auto;
}

.preview-option-item :deep(.math-text) {
  min-width: 0;
}

@media (max-width: 1280px) {
  .question-editor-page {
    grid-template-columns: 260px minmax(0, 1fr);
  }

  .editor-content {
    grid-template-columns: 1fr;
    overflow: auto;
  }

  .editor-form-pane,
  .preview-pane {
    overflow: visible;
  }

  .preview-pane {
    padding-left: 0;
    border-left: 0;
  }

  .meta-card {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
