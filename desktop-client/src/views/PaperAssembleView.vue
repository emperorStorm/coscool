<template>
  <div class="paper-assemble page-shell">
    <header class="assemble-toolbar">
      <div>
        <h2 class="page-title">组装试卷</h2>
        <span class="muted">试题栏已加入 {{ questions.length }} 道题，可直接在试卷中调整顺序</span>
      </div>
      <a-space wrap>
        <a-button @click="goQuery">继续选题</a-button>
        <a-popconfirm title="确认清空试题栏？" @confirm="clearCart">
          <a-button danger :disabled="questions.length === 0">清空</a-button>
        </a-popconfirm>
        <a-button type="primary" :disabled="questions.length === 0" @click="openGenerateDialog">
          生成试卷
        </a-button>
      </a-space>
    </header>

    <main class="assemble-main">
      <a-spin :spinning="loading">
        <section class="editable-paper">
          <input
            v-model="paperForm.title"
            class="paper-title-input"
            maxlength="80"
            placeholder="请输入试卷标题"
          />

          <a-empty v-if="questions.length === 0" description="暂无已选题目" class="paper-empty" />

          <div v-else class="paper-question-list">
            <article v-for="(question, index) in questions" :key="question.id" class="paper-question">
              <div class="question-title">
                <strong>{{ index + 1 }}.</strong>
                <MathText :content="question.stem || question.title || '暂无题干'" />
              </div>
              <img v-if="questionImageMap[question.id]" class="question-image" :src="questionImageMap[question.id]" alt="题目配图" />
              <p v-else-if="question.imageText" class="image-placeholder">{{ question.imageText }}</p>
              <div v-if="question.options.length && isChoiceQuestion(question.questionType)" class="option-grid">
                <div v-for="option in question.options" :key="option.optionKey" class="option-item">
                  <strong>{{ option.optionKey }}.</strong>
                  <span>
                    <MathText :content="option.content || '-'" />
                    <img
                      v-if="optionImageMap[getOptionImageKey(question.id, option.optionKey)]"
                      class="option-image"
                      :src="optionImageMap[getOptionImageKey(question.id, option.optionKey)]"
                      alt="选项图片"
                    />
                  </span>
                </div>
              </div>
              <div class="question-edit-bar">
                <div class="question-meta">
                  <span>{{ question.questionType || '未设置题型' }}</span>
                  <span>{{ formatDifficultyText(question.difficulty) }}</span>
                  <span v-if="question.year">{{ question.year }}</span>
                </div>
                <a-space :size="8" wrap>
                  <a-button size="small" :disabled="index === 0" @click="moveQuestion(index, -1)">上移</a-button>
                  <a-button size="small" :disabled="index === questions.length - 1" @click="moveQuestion(index, 1)">下移</a-button>
                  <a-button size="small" danger @click="removeQuestion(question.id)">删除</a-button>
                </a-space>
              </div>
            </article>
          </div>
        </section>
      </a-spin>
    </main>

    <a-modal v-model:open="generateDialogOpen" title="生成试卷" :confirm-loading="saving" @ok="generatePaper">
      <a-form layout="vertical">
        <a-form-item label="试卷名称" required>
          <a-input v-model:value="generateForm.title" placeholder="请输入试卷名称" />
        </a-form-item>
        <a-form-item label="备注">
          <a-textarea v-model:value="generateForm.remark" :rows="4" placeholder="请输入备注" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import { message } from 'ant-design-vue'
import { useRouter } from 'vue-router'
import MathText from '../components/MathText.vue'
import type { Question } from '../api/native'
import { getQuestionsByIds, readAssetDataUrl, savePaper } from '../api/native'
import { buildOptionImageMap, buildQuestionImageMap, getOptionImageKey } from '../utils/questionAssets'
import {
  clearPaperCart,
  getPaperCartIds,
  removeQuestionFromPaperCart,
  setPaperCartIds,
} from '../utils/paperCart'

const router = useRouter()
const loading = ref(false)
const saving = ref(false)
const generateDialogOpen = ref(false)
const questions = ref<Question[]>([])
const questionImageMap = ref<Record<number, string>>({})
const optionImageMap = ref<Record<string, string>>({})
const paperForm = reactive({
  title: ''
})
const generateForm = reactive({
  title: '',
  remark: ''
})

onMounted(async () => {
  paperForm.title = `试卷-${formatMinuteTimestamp()}`
  await loadCartQuestions()
})

async function loadCartQuestions() {
  const ids = getPaperCartIds()
  if (!ids.length) {
    questions.value = []
    questionImageMap.value = {}
    optionImageMap.value = {}
    return
  }
  loading.value = true
  try {
    questions.value = await getQuestionsByIds(ids)
    const validIds = questions.value.map((item) => item.id)
    if (validIds.length !== ids.length) {
      setPaperCartIds(validIds)
      message.warning('部分题目已不存在，已从试题栏移除')
    }
    await loadQuestionImages()
  } catch (error) {
    message.error(String(error))
  } finally {
    loading.value = false
  }
}

async function loadQuestionImages() {
  const [questionMap, optionMap] = await Promise.all([
    buildQuestionImageMap(questions.value, readAssetDataUrl),
    buildOptionImageMap(questions.value, readAssetDataUrl)
  ])
  questionImageMap.value = questionMap
  optionImageMap.value = optionMap
}

function moveQuestion(index: number, step: number) {
  const targetIndex = index + step
  if (targetIndex < 0 || targetIndex >= questions.value.length) return
  const next = [...questions.value]
  const [item] = next.splice(index, 1)
  next.splice(targetIndex, 0, item)
  questions.value = next
  setPaperCartIds(next.map((question) => question.id))
}

function removeQuestion(id: number) {
  questions.value = questions.value.filter((question) => question.id !== id)
  const { [id]: _removed, ...nextImageMap } = questionImageMap.value
  questionImageMap.value = nextImageMap
  optionImageMap.value = Object.fromEntries(
    Object.entries(optionImageMap.value).filter(([key]) => !key.startsWith(`${id}:`))
  )
  removeQuestionFromPaperCart(id)
}

function clearCart() {
  questions.value = []
  questionImageMap.value = {}
  optionImageMap.value = {}
  clearPaperCart()
  message.success('试题栏已清空')
}

function openGenerateDialog() {
  generateForm.title = paperForm.title.trim() || `试卷-${formatMinuteTimestamp()}`
  generateForm.remark = ''
  generateDialogOpen.value = true
}

async function generatePaper() {
  if (!generateForm.title.trim()) {
    message.warning('请输入试卷名称')
    return
  }
  if (!questions.value.length) {
    message.warning('请先加入题目')
    return
  }
  saving.value = true
  try {
    await savePaper({
      title: generateForm.title.trim(),
      remark: generateForm.remark.trim(),
      createdBy: getCurrentTeacherName(),
      questions: questions.value.map((question, index) => ({
        questionId: question.id,
        sortOrder: index
      }))
    })
    generateDialogOpen.value = false
    clearPaperCart()
    questions.value = []
    questionImageMap.value = {}
    optionImageMap.value = {}
    message.success('试卷已生成')
    router.push('/app/questions/papers')
  } catch (error) {
    message.error(String(error))
  } finally {
    saving.value = false
  }
}

function getCurrentTeacherName() {
  const raw = localStorage.getItem('coscool_teacher')
  if (!raw) return ''
  try {
    const teacher = JSON.parse(raw)
    return teacher.name || teacher.account || ''
  } catch {
    return ''
  }
}

function goQuery() {
  router.push('/app/questions/query')
}

function formatMinuteTimestamp() {
  const date = new Date()
  const pad = (value: number) => String(value).padStart(2, '0')
  return `${date.getFullYear()}${pad(date.getMonth() + 1)}${pad(date.getDate())}${pad(date.getHours())}${pad(date.getMinutes())}`
}

function isChoiceQuestion(questionType: string) {
  return questionType === '单选题' || questionType === '多选题'
}

function formatDifficultyText(difficulty: number) {
  if (!difficulty || difficulty <= 0) return '未设置难度'
  const score = Math.max(0, Math.min(5, Math.trunc(difficulty)))
  return `${'★'.repeat(score)}${'☆'.repeat(5 - score)}`
}
</script>

<style scoped>
.paper-assemble {
  height: calc(100vh - 58px);
  overflow: auto;
}

.assemble-toolbar {
  display: flex;
  gap: 16px;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 14px;
}

.assemble-toolbar h2 {
  margin-bottom: 4px;
}

.assemble-main {
  min-width: 0;
  padding-bottom: 34px;
}

.editable-paper {
  width: min(1040px, 100%);
  min-height: 1180px;
  margin: 0 auto;
  padding: 46px 58px 64px;
  color: #1f2937;
  background: #ffffff;
  border: 1px solid #dfe8f4;
  border-radius: 8px;
  box-shadow: 0 18px 42px rgba(42, 60, 84, 0.10);
}

.paper-title-input {
  display: block;
  width: 100%;
  margin: 0;
  padding: 0 16px 14px;
  color: #111827;
  font-size: 26px;
  font-weight: 900;
  line-height: 1.4;
  text-align: center;
  background: transparent;
  border: 0;
  border-bottom: 2px solid #0f9187;
  outline: none;
}

.paper-title-input:focus {
  border-bottom-color: #0f9187;
}

.paper-empty {
  margin-top: 70px;
}

.paper-question-list {
  display: grid;
  gap: 28px;
  margin-top: 34px;
}

.paper-question {
  padding: 20px 24px 14px;
  page-break-inside: avoid;
  border: 1px solid transparent;
  border-radius: 8px;
}

.paper-question:hover {
  border-color: #87d8ce;
}

.question-title {
  display: flex;
  gap: 10px;
  align-items: flex-start;
  color: #1f2937;
  font-size: 17px;
  font-weight: 800;
  line-height: 1.9;
}

.question-title > strong {
  flex: 0 0 auto;
  min-width: 24px;
}

.question-title :deep(.math-text) {
  min-width: 0;
}

.question-image {
  display: block;
  max-width: 100%;
  max-height: 280px;
  margin: 14px 0 0 34px;
  object-fit: contain;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
}

.image-placeholder {
  margin: 12px 0 0 34px;
  color: #8792a2;
  font-size: 13px;
}

.option-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px 34px;
  margin: 18px 0 0 34px;
}

.option-item {
  display: flex;
  gap: 8px;
  min-width: 0;
  color: #273244;
  font-size: 15px;
  line-height: 1.7;
}

.option-item strong {
  flex: 0 0 auto;
  font-style: italic;
}

.option-item :deep(.math-text) {
  min-width: 0;
}

.option-item > span {
  min-width: 0;
}

.option-image {
  display: block;
  max-width: 100%;
  max-height: 120px;
  margin-top: 6px;
  object-fit: contain;
  border: 1px solid #e2e9f0;
  border-radius: 6px;
}

.question-edit-bar {
  display: flex;
  gap: 14px;
  align-items: center;
  justify-content: space-between;
  margin: 24px 0 0 34px;
  padding-top: 14px;
  border-top: 1px solid #edf1f5;
}

.question-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
  min-width: 0;
}

.question-meta span {
  display: inline-flex;
  align-items: center;
  min-height: 24px;
  padding: 0 9px;
  color: #0f8f83;
  font-size: 12px;
  font-weight: 800;
  background: #eefaf8;
  border: 1px solid #cdeee8;
  border-radius: 999px;
}

@media (max-width: 900px) {
  .assemble-toolbar,
  .question-edit-bar {
    align-items: stretch;
    flex-direction: column;
  }

  .editable-paper {
    min-height: auto;
    padding: 32px 22px;
  }

  .option-grid {
    grid-template-columns: 1fr;
  }

  .question-image,
  .image-placeholder,
  .option-grid,
  .question-edit-bar {
    margin-left: 0;
  }
}
</style>
