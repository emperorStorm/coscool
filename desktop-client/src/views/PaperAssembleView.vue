<template>
  <div class="paper-assemble page-shell">
    <header class="assemble-toolbar">
      <div>
        <h2 class="page-title">组装试卷</h2>
        <span class="muted">已加入 {{ questions.length }} 道题</span>
      </div>
      <a-space>
        <a-button @click="goQuery">继续选题</a-button>
        <a-popconfirm title="确认清空购物车？" @confirm="clearCart">
          <a-button danger :disabled="questions.length === 0">清空</a-button>
        </a-popconfirm>
        <a-button type="primary" :disabled="questions.length === 0" @click="openGenerateDialog">生成试卷</a-button>
      </a-space>
    </header>

    <main class="assemble-main">
      <section class="selected-panel work-panel">
        <div class="panel-head">
          <strong>已选题目</strong>
          <span class="muted">可调整顺序或删除</span>
        </div>
        <a-spin :spinning="loading">
          <a-empty v-if="questions.length === 0" description="暂无已选题目" />
          <div v-else class="selected-list">
            <article v-for="(question, index) in questions" :key="question.id" class="selected-item">
              <span class="question-index">{{ index + 1 }}</span>
              <div class="question-summary">
                <strong>{{ question.title || '未命名题目' }}</strong>
                <span>{{ question.questionType || '未设置题型' }} / {{ question.year || '未设置年份' }}</span>
              </div>
              <a-space>
                <a-button size="small" :disabled="index === 0" @click="moveQuestion(index, -1)">上移</a-button>
                <a-button size="small" :disabled="index === questions.length - 1" @click="moveQuestion(index, 1)">下移</a-button>
                <a-button size="small" danger @click="removeQuestion(question.id)">删除</a-button>
              </a-space>
            </article>
          </div>
        </a-spin>
      </section>

      <section class="preview-panel">
        <div class="panel-head preview-head">
          <strong>试卷预览</strong>
          <span class="muted">预览不包含答案和解析</span>
        </div>
        <PaperPreview :title="previewTitle" :questions="questions" :image-map="questionImageMap" />
      </section>
    </main>

    <a-modal v-model:open="generateDialogOpen" title="生成试卷" :confirm-loading="saving" @ok="generatePaper">
      <a-form layout="vertical">
        <a-form-item label="试卷名称" required>
          <a-input v-model:value="paperForm.title" placeholder="请输入试卷名称" />
        </a-form-item>
        <a-form-item label="备注">
          <a-textarea v-model:value="paperForm.remark" :rows="4" placeholder="请输入备注" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { message } from 'ant-design-vue'
import { useRouter } from 'vue-router'
import PaperPreview from '../components/PaperPreview.vue'
import type { Question } from '../api/native'
import { getQuestionsByIds, readAssetDataUrl, savePaper } from '../api/native'
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
const paperForm = reactive({
  title: '',
  remark: ''
})
const previewTitle = computed(() => paperForm.title || '课思库试卷')

onMounted(loadCartQuestions)

async function loadCartQuestions() {
  const ids = getPaperCartIds()
  if (!ids.length) {
    questions.value = []
    questionImageMap.value = {}
    return
  }
  loading.value = true
  try {
    questions.value = await getQuestionsByIds(ids)
    const validIds = questions.value.map((item) => item.id)
    if (validIds.length !== ids.length) {
      setPaperCartIds(validIds)
      message.warning('部分题目已不存在，已从购物车移除')
    }
    await loadQuestionImages()
  } catch (error) {
    message.error(String(error))
  } finally {
    loading.value = false
  }
}

async function loadQuestionImages() {
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
  removeQuestionFromPaperCart(id)
}

function clearCart() {
  questions.value = []
  questionImageMap.value = {}
  clearPaperCart()
  message.success('购物车已清空')
}

function openGenerateDialog() {
  paperForm.title = `试卷-${formatMinuteTimestamp()}`
  paperForm.remark = ''
  generateDialogOpen.value = true
}

async function generatePaper() {
  if (!paperForm.title.trim()) {
    message.warning('请输入试卷名称')
    return
  }
  saving.value = true
  try {
    await savePaper({
      title: paperForm.title.trim(),
      remark: paperForm.remark.trim(),
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
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  gap: 14px;
}

.selected-panel {
  padding: 16px;
}

.panel-head {
  display: flex;
  gap: 12px;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 14px;
}

.panel-head strong {
  color: #263447;
  font-size: 16px;
}

.selected-list {
  display: grid;
  gap: 10px;
}

.selected-item {
  display: grid;
  grid-template-columns: 34px minmax(0, 1fr) auto;
  gap: 12px;
  align-items: center;
  padding: 12px;
  background: #f8fafc;
  border: 1px solid #e4ebf2;
  border-radius: 8px;
}

.question-index {
  display: grid;
  width: 30px;
  height: 30px;
  color: #0f9187;
  font-weight: 900;
  place-items: center;
  background: #e9fbf8;
  border: 1px solid #b9e8e1;
  border-radius: 6px;
}

.question-summary {
  min-width: 0;
}

.question-summary strong,
.question-summary span {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.question-summary strong {
  color: #263447;
}

.question-summary span {
  margin-top: 4px;
  color: #8190a2;
  font-size: 13px;
}

.preview-panel {
  min-width: 0;
  padding-bottom: 30px;
}

.preview-head {
  max-width: 800px;
  margin-right: auto;
  margin-left: auto;
}

@media (max-width: 860px) {
  .assemble-toolbar,
  .selected-item {
    align-items: stretch;
    grid-template-columns: 1fr;
    flex-direction: column;
  }
}
</style>
