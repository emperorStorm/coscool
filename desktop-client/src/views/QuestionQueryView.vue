<template>
  <div class="question-query page-shell">
    <main class="query-main">
      <section class="query-filter work-panel">
        <a-form class="query-form" layout="vertical" :model="filters">
          <a-form-item label="题库分类" class="query-form-item">
            <a-tree-select
              v-model:value="filters.categoryId"
              class="category-select"
              allow-clear
              show-search
              tree-default-expand-all
              placeholder="请选择分类"
              :tree-data="categoryTreeOptions"
              :field-names="{ children: 'children', label: 'name', value: 'id' }"
              @change="loadQuestions"
            />
          </a-form-item>
          <a-form-item label="标题" class="query-form-item query-form-title">
            <a-input v-model:value="filters.title" allow-clear placeholder="请输入标题" @press-enter="loadQuestions" />
          </a-form-item>
          <a-form-item label="年份" class="query-form-item query-form-year">
            <a-input v-model:value="filters.year" allow-clear placeholder="如 2026" @press-enter="loadQuestions" />
          </a-form-item>
          <a-form-item label="标签" class="query-form-item query-form-tag">
            <a-input v-model:value="filters.tag" allow-clear placeholder="请输入标签" @press-enter="loadQuestions" />
          </a-form-item>
          <a-form-item label="知识点" class="query-form-item query-form-knowledge">
            <a-input v-model:value="filters.knowledgePoint" allow-clear placeholder="请输入知识点" @press-enter="loadQuestions" />
          </a-form-item>
          <a-form-item class="query-form-actions">
            <a-space :size="10">
              <a-button type="primary" class="query-button" @click="loadQuestions">查询</a-button>
              <a-button @click="resetFilters">重置</a-button>
            </a-space>
          </a-form-item>
        </a-form>
      </section>

      <section class="query-result work-panel">
        <div class="result-head">
          <strong>题库查询</strong>
          <span class="muted">共 {{ questions.length }} 道题，已加入试题栏 {{ cartCount }} 道</span>
        </div>
        <a-spin :spinning="loading">
          <a-empty v-if="!questions.length" description="暂无匹配题目" />
          <div v-else class="question-list">
            <article
              v-for="(question, index) in questions"
              :key="question.id"
              class="question-result-card"
              :class="{ selected: cartIdSet.has(question.id) }"
            >
              <div class="question-card-main">
                <div class="question-order">{{ index + 1 }}</div>
                <div class="question-content">
                  <div class="question-title-row">
                    <span v-if="question.year" class="source-badge">{{ question.year }}</span>
                    <strong>{{ question.title || '未命名题目' }}</strong>
                  </div>
                  <div class="question-stem">
                    <MathText :content="question.stem || '暂无题干'" />
                  </div>
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
                </div>
              </div>

              <div class="question-card-foot">
                <div class="question-meta">
                  <span>更新：{{ formatDate(question.updatedAt || question.createdAt) }}</span>
                  <span>题型：{{ question.questionType || '未设置' }}</span>
                  <span>难度：{{ formatDifficultyText(question.difficulty) }}</span>
                  <span>分类：{{ categoryNameMap.get(question.categoryId) || '未分类' }}</span>
                  <span v-if="question.questionNo">题号：{{ question.questionNo }}</span>
                </div>
                <div class="question-tags">
                  <span v-for="item in question.knowledgePoints" :key="`knowledge-${item}`">{{ item }}</span>
                  <span v-for="tag in question.tags" :key="`tag-${tag}`" class="tag-cyan">{{ tag }}</span>
                </div>
                <div class="question-actions">
                  <a-button size="small" type="link" @click="openDetail(question.id)">查看</a-button>
                  <a-button size="small" type="link" @click="openDetail(question.id)">解析</a-button>
                  <a-button size="small" type="link" @click="openQuestionExport(question.id)">下载</a-button>
                  <a-button
                    size="small"
                    class="cart-add-button"
                    :class="{ added: cartIdSet.has(question.id) }"
                    :disabled="cartIdSet.has(question.id)"
                    @click="addToPaperCart(question.id)"
                  >
                    {{ cartIdSet.has(question.id) ? '已加入' : '+试题栏' }}
                  </a-button>
                </div>
              </div>
            </article>
          </div>
        </a-spin>
      </section>
    </main>

    <div class="paper-cart-float">
      <div class="cart-analysis-panel">
        <div class="analysis-title">
          <strong>试题栏分析</strong>
          <span>题量 {{ cartCount }}</span>
        </div>
        <a-empty v-if="cartCount === 0" description="暂无试题" />
        <template v-else>
          <div class="analysis-summary">
            <div>
              <strong>{{ cartCount }}</strong>
              <span>总题量</span>
            </div>
            <div>
              <strong>{{ cartKnownCount }}</strong>
              <span>当前可分析</span>
            </div>
            <div>
              <strong>{{ cartUnknownCount }}</strong>
              <span>未在当前列表</span>
            </div>
          </div>
          <div class="analysis-section">
            <h3>题型分布</h3>
            <div v-for="item in cartTypeStats" :key="item.name" class="stat-row">
              <span>{{ item.name }}</span>
              <div>
                <i :style="{ width: `${item.percent}%` }"></i>
              </div>
              <em>{{ item.count }}</em>
            </div>
          </div>
          <div class="analysis-section">
            <h3>难度分布</h3>
            <div v-for="item in cartDifficultyStats" :key="item.name" class="stat-row">
              <span>{{ item.name }}</span>
              <div>
                <i :style="{ width: `${item.percent}%` }"></i>
              </div>
              <em>{{ item.count }}</em>
            </div>
          </div>
        </template>
      </div>
      <button class="paper-cart-button" type="button" title="进入试题栏" @click="openPaperAssemble">
        <span class="basket-icon-shell">
          <svg class="basket-3d-icon" viewBox="0 0 64 58" aria-hidden="true">
            <defs>
              <linearGradient id="basketBodyGradient" x1="16" x2="48" y1="18" y2="54" gradientUnits="userSpaceOnUse">
                <stop offset="0" stop-color="#45cbbd" />
                <stop offset="0.58" stop-color="#109b90" />
                <stop offset="1" stop-color="#0a746e" />
              </linearGradient>
              <linearGradient id="basketHandleGradient" x1="18" x2="46" y1="7" y2="25" gradientUnits="userSpaceOnUse">
                <stop offset="0" stop-color="#c8f8ef" />
                <stop offset="1" stop-color="#1caa9d" />
              </linearGradient>
              <linearGradient id="basketFrontLight" x1="18" x2="46" y1="28" y2="46" gradientUnits="userSpaceOnUse">
                <stop offset="0" stop-color="rgba(255,255,255,0.72)" />
                <stop offset="1" stop-color="rgba(255,255,255,0.08)" />
              </linearGradient>
            </defs>
            <path class="basket-shadow" d="M14 48c5 5 31 7 38 0 1-1 2-3 1-5H11c0 2 1 4 3 5Z" />
            <path class="basket-handle" d="M19 25C20 10 44 10 45 25" />
            <path class="basket-rim" d="M12 23h40l-4 7H16l-4-7Z" />
            <path class="basket-body" d="M16 29h32l-4 20H20l-4-20Z" />
            <path class="basket-highlight" d="M20 32h23l-2 6H21l-1-6Z" />
            <path class="basket-weave" d="M24 29l3 20M32 29v20M40 29l-3 20M18 37h28M19 44h26" />
          </svg>
        </span>
        <strong>试题栏</strong>
        <span v-if="cartCount > 0" class="cart-count-badge">{{ cartCount }}</span>
      </button>
    </div>

    <a-drawer v-model:open="detailOpen" title="题目详情" width="760">
      <a-spin :spinning="detailLoading">
        <template v-if="currentQuestion">
          <section class="detail-section">
            <div class="detail-meta">
              <a-tag color="cyan">{{ currentQuestion.title || '未命名题目' }}</a-tag>
              <a-tag v-if="currentQuestion.year">{{ currentQuestion.year }}</a-tag>
              <a-tag v-if="currentQuestion.questionNo">题号 {{ currentQuestion.questionNo }}</a-tag>
              <a-tag v-if="currentQuestion.questionType" color="blue">{{ currentQuestion.questionType }}</a-tag>
              <a-tag v-if="currentQuestion.difficulty > 0" color="gold">{{ formatDifficulty(currentQuestion.difficulty) }}</a-tag>
              <a-tag>{{ categoryNameMap.get(currentQuestion.categoryId) || '未分类' }}</a-tag>
            </div>
          </section>
          <section class="detail-section">
            <h3>题目</h3>
            <MathText class="detail-math-text" :content="currentQuestion.stem || '暂无题干'" />
            <img v-if="detailImageUrl" class="detail-image" :src="detailImageUrl" alt="题目配图预览" />
            <a-alert v-else-if="currentQuestion.imageText" type="warning" show-icon message="图片不存在或无法预览" />
          </section>
          <section v-if="currentQuestion.options.length && isChoiceQuestion(currentQuestion.questionType)" class="detail-section">
            <h3>选项</h3>
            <div class="detail-options">
              <div v-for="option in currentQuestion.options" :key="option.optionKey">
                <strong>{{ option.optionKey }}.</strong>
                <span>
                  <MathText :content="option.content || '-'" />
                  <img
                    v-if="detailOptionImageMap[getOptionImageKey(currentQuestion.id, option.optionKey)]"
                    class="detail-option-image"
                    :src="detailOptionImageMap[getOptionImageKey(currentQuestion.id, option.optionKey)]"
                    alt="选项图片"
                  />
                </span>
              </div>
            </div>
          </section>
          <section class="detail-section">
            <h3>知识点</h3>
            <a-space wrap>
              <a-tag v-for="item in currentQuestion.knowledgePoints" :key="item">{{ item }}</a-tag>
              <span v-if="!currentQuestion.knowledgePoints.length" class="muted">暂无</span>
            </a-space>
          </section>
          <section class="detail-section">
            <h3>标签</h3>
            <a-space wrap>
              <a-tag v-for="item in currentQuestion.tags" :key="item" color="cyan">{{ item }}</a-tag>
              <span v-if="!currentQuestion.tags.length" class="muted">暂无</span>
            </a-space>
          </section>
          <section class="detail-section">
            <h3>答案</h3>
            <MathText class="detail-math-text" :content="currentQuestion.answer || '暂无答案'" />
          </section>
          <section class="detail-section">
            <h3>解析</h3>
            <MathText class="detail-math-text" :content="currentQuestion.analysis || '暂无解析'" />
          </section>
        </template>
      </a-spin>
    </a-drawer>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, reactive, ref } from 'vue'
import { message } from 'ant-design-vue'
import { useRouter } from 'vue-router'
import { ShoppingBasket } from 'lucide-vue-next'
import MathText from '../components/MathText.vue'
import type { Question, QuestionCategory, QuestionQueryFilters } from '../api/native'
import { getQuestion, listCategories, queryQuestions, readAssetDataUrl } from '../api/native'
import { buildOptionImageMap, getOptionImageKey } from '../utils/questionAssets'
import { addQuestionToPaperCart, getPaperCartIds } from '../utils/paperCart'

interface CategoryTreeOption extends QuestionCategory {
  children: CategoryTreeOption[]
}

const loading = ref(false)
const detailLoading = ref(false)
const detailOpen = ref(false)
const questions = ref<Question[]>([])
const currentQuestion = ref<Question>()
const detailImageUrl = ref('')
const optionImageMap = ref<Record<string, string>>({})
const detailOptionImageMap = ref<Record<string, string>>({})
const cartIds = ref<number[]>([])
const router = useRouter()
const categoryNameMap = ref(new Map<number | undefined, string>())
const categoryTreeOptions = ref<CategoryTreeOption[]>([])
const filters = reactive<QuestionQueryFilters>({
  categoryId: undefined,
  includeChildren: true,
  title: '',
  year: '',
  tag: '',
  knowledgePoint: ''
})
const cartIdSet = computed(() => new Set(cartIds.value))
const cartCount = computed(() => cartIds.value.length)
const cartQuestions = computed(() => questions.value.filter((question) => cartIdSet.value.has(question.id)))
const cartKnownCount = computed(() => cartQuestions.value.length)
const cartUnknownCount = computed(() => Math.max(0, cartCount.value - cartKnownCount.value))
const cartTypeStats = computed(() => buildStats(cartQuestions.value.map((question) => question.questionType || '未设置')))
const cartDifficultyStats = computed(() => buildStats(cartQuestions.value.map((question) => formatDifficultyText(question.difficulty))))

onMounted(async () => {
  refreshPaperCart()
  window.addEventListener('coscool-paper-cart-change', refreshPaperCart)
  await loadCategories()
  await loadQuestions()
})

onUnmounted(() => {
  window.removeEventListener('coscool-paper-cart-change', refreshPaperCart)
})

async function loadCategories() {
  const categories = await listCategories()
  categoryNameMap.value = new Map(categories.map((item) => [item.id, item.name]))
  categoryTreeOptions.value = buildCategoryTree(categories)
}

function buildCategoryTree(categories: QuestionCategory[]) {
  const nodeMap = new Map<number, CategoryTreeOption>()
  categories.forEach((item) => nodeMap.set(item.id, { ...item, children: [] }))
  const roots: CategoryTreeOption[] = []
  nodeMap.forEach((node) => {
    if (node.parentId && nodeMap.has(node.parentId)) {
      nodeMap.get(node.parentId)?.children.push(node)
    } else {
      roots.push(node)
    }
  })
  return roots
}

async function loadQuestions() {
  loading.value = true
  try {
    questions.value = await queryQuestions(filters)
    optionImageMap.value = await buildOptionImageMap(questions.value, readAssetDataUrl)
  } catch (error) {
    message.error(String(error))
  } finally {
    loading.value = false
  }
}

function resetFilters() {
  Object.assign(filters, {
    categoryId: undefined,
    includeChildren: true,
    title: '',
    year: '',
    tag: '',
    knowledgePoint: ''
  })
  loadQuestions()
}

function refreshPaperCart() {
  cartIds.value = getPaperCartIds()
}

function addToPaperCart(id: number) {
  addQuestionToPaperCart(id)
  refreshPaperCart()
  message.success('已加入试题栏')
}

function openPaperAssemble() {
  router.push('/app/questions/paper-assemble')
}

function openQuestionExport(id: number) {
  router.push(`/question-export/${id}`)
}

async function openDetail(id: number) {
  detailOpen.value = true
  detailLoading.value = true
  detailImageUrl.value = ''
  detailOptionImageMap.value = {}
  try {
    currentQuestion.value = await getQuestion(id)
    await loadDetailImage()
    detailOptionImageMap.value = await buildOptionImageMap([currentQuestion.value], readAssetDataUrl)
  } catch (error) {
    message.error(String(error))
  } finally {
    detailLoading.value = false
  }
}

async function loadDetailImage() {
  if (!currentQuestion.value?.imageText || !currentQuestion.value.imageText.startsWith('assets/')) return
  try {
    const asset = await readAssetDataUrl(currentQuestion.value.imageText)
    detailImageUrl.value = asset.dataUrl
  } catch {
    detailImageUrl.value = ''
  }
}

function formatDifficulty(difficulty: number) {
  const score = Math.max(0, Math.min(5, Math.trunc(difficulty || 0)))
  return `${'★'.repeat(score)}${'☆'.repeat(5 - score)}`
}

function formatDifficultyText(difficulty: number) {
  if (!difficulty || difficulty <= 0) return '未设置'
  return formatDifficulty(difficulty)
}

function formatDate(value: string) {
  if (!value) return '-'
  return value.slice(0, 10)
}

function buildStats(names: string[]) {
  const countMap = new Map<string, number>()
  names.forEach((name) => countMap.set(name, (countMap.get(name) || 0) + 1))
  return Array.from(countMap.entries()).map(([name, count]) => ({
    name,
    count,
    percent: names.length ? Math.round((count / names.length) * 100) : 0
  }))
}

function isChoiceQuestion(questionType: string) {
  return questionType === '单选题' || questionType === '多选题'
}
</script>

<style scoped>
.question-query {
  position: relative;
  height: calc(100vh - 58px);
  overflow: hidden;
}

.query-main {
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  gap: 10px;
  min-width: 0;
  overflow: hidden;
}

.query-filter {
  padding: 12px 14px;
  overflow: hidden;
}

.query-form {
  display: grid;
  grid-template-columns: repeat(16, minmax(0, 1fr));
  width: 100%;
  max-width: 100%;
  gap: 10px 14px;
  align-items: end;
}

.query-form-item {
  margin-bottom: 0;
  width: auto;
  min-width: 0;
  grid-column: span 3;
}

.query-form-title {
  grid-column: span 5;
}

.query-form-year {
  grid-column: span 2;
}

.query-form-tag,
.query-form-knowledge {
  grid-column: span 3;
}

.query-form-actions {
  grid-column: span 3;
  margin-bottom: 0;
}

.query-form :deep(.ant-form-item-row),
.query-form :deep(.ant-form-item-control),
.query-form :deep(.ant-form-item-control-input),
.query-form :deep(.ant-form-item-control-input-content) {
  width: 100%;
  min-width: 0;
}

.query-form :deep(.ant-form-item-label) {
  padding-bottom: 3px;
  font-weight: 700;
  line-height: 18px;
}

.query-form :deep(.ant-form-item-label > label) {
  height: 18px;
  color: #334155;
  font-size: 13px;
  line-height: 18px;
}

.query-form :deep(.ant-input-affix-wrapper),
.query-form :deep(.ant-select-selector) {
  width: 100%;
  min-width: 0;
  min-height: 32px;
  height: 32px;
  border-color: #dce4ee;
  border-radius: 6px;
  font-size: 13px;
  line-height: 30px;
}

.query-form :deep(.ant-input-affix-wrapper) {
  padding: 0 10px;
}

.query-form :deep(.ant-input-affix-wrapper .ant-input) {
  height: 30px;
  padding: 0;
  background: transparent;
  border: 0;
  border-radius: 0;
  box-shadow: none;
  line-height: 30px;
}

.query-form :deep(.ant-select-single) {
  width: 100%;
}

.query-form :deep(.ant-select-single .ant-select-selector .ant-select-selection-search),
.query-form :deep(.ant-select-single .ant-select-selector .ant-select-selection-item),
.query-form :deep(.ant-select-single .ant-select-selector .ant-select-selection-placeholder) {
  line-height: 30px;
}

.query-form :deep(.ant-select-selection-search-input) {
  height: 30px;
}

.query-button {
  min-width: 56px;
}

.query-form-actions :deep(.ant-btn) {
  height: 32px;
  padding: 0 12px;
  font-size: 13px;
  border-radius: 6px;
}

.category-select {
  width: 100%;
}

.query-result {
  min-height: 0;
  padding: 14px;
  overflow: auto;
}

.result-head {
  display: flex;
  gap: 12px;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.result-head strong {
  color: #263447;
  font-size: 16px;
}

.question-list {
  display: grid;
  gap: 14px;
}

.question-result-card {
  display: grid;
  gap: 16px;
  padding: 22px 26px 18px;
  background: #ffffff;
  border: 1px solid #dfe5ec;
  border-radius: 8px;
  box-shadow: 0 8px 20px rgba(39, 50, 68, 0.04);
  transition: border-color 0.18s ease, background 0.18s ease, box-shadow 0.18s ease;
}

.question-result-card.selected {
  background: #f2fbf9;
  border-color: #87d8ce;
  box-shadow: 0 10px 24px rgba(15, 145, 135, 0.10);
}

.question-card-main {
  display: grid;
  grid-template-columns: 34px minmax(0, 1fr);
  gap: 14px;
}

.question-order {
  color: #263447;
  font-size: 17px;
  font-weight: 800;
  line-height: 1.9;
  text-align: right;
}

.question-content {
  min-width: 0;
}

.question-title-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
  margin-bottom: 8px;
}

.question-title-row strong {
  min-width: 0;
  color: #2388e8;
  font-size: 17px;
  font-weight: 800;
  line-height: 1.6;
}

.source-badge {
  flex: 0 0 auto;
  color: #2388e8;
  font-size: 16px;
  font-weight: 800;
}

.question-stem {
  color: #273244;
  font-size: 16px;
  font-weight: 700;
  line-height: 1.9;
}

.question-stem :deep(.math-text) {
  min-width: 0;
}

.option-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(120px, 1fr));
  gap: 12px 24px;
  margin-top: 20px;
}

.option-item {
  display: flex;
  gap: 8px;
  min-width: 0;
  color: #303b4d;
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

.option-image,
.detail-option-image {
  display: block;
  max-width: 100%;
  max-height: 110px;
  margin-top: 6px;
  object-fit: contain;
  border: 1px solid #e2e9f0;
  border-radius: 6px;
}

.question-card-foot {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 10px 14px;
  padding-top: 12px;
  border-top: 1px solid #e8edf3;
}

.question-meta,
.question-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px 14px;
  align-items: center;
  min-width: 0;
  color: #8792a2;
  font-size: 13px;
  font-weight: 700;
}

.question-tags {
  grid-column: 1;
  gap: 8px;
}

.question-tags span {
  display: inline-flex;
  align-items: center;
  min-height: 24px;
  padding: 0 9px;
  color: #385dd6;
  background: #eef1ff;
  border: 1px solid #cbd6ff;
  border-radius: 999px;
}

.question-tags .tag-cyan {
  color: #0f8f83;
  background: #eefaf8;
  border-color: #cdeee8;
}

.question-actions {
  grid-row: 1 / span 2;
  grid-column: 2;
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  align-items: end;
  justify-content: flex-end;
}

.question-actions :deep(.ant-btn-link) {
  color: #2388e8;
  font-weight: 800;
}

.cart-add-button {
  min-width: 78px;
  color: #ffffff;
  font-weight: 900;
  background: #0f9187;
  border-color: #0f9187;
  border-radius: 6px;
  box-shadow: 0 6px 14px rgba(15, 145, 135, 0.18);
}

.cart-add-button:not(:disabled):hover {
  color: #ffffff;
  background: #0b8279;
  border-color: #0b8279;
}

.cart-add-button.added,
.cart-add-button.added:disabled {
  color: #0f6f68;
  background: #dff8f4;
  border-color: #9ddfd7;
  box-shadow: none;
  opacity: 1;
}

.detail-section {
  margin-bottom: 18px;
  padding-bottom: 16px;
  border-bottom: 1px solid #edf1f5;
}

.detail-section h3 {
  margin: 0 0 10px;
  font-size: 15px;
}

.detail-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.detail-options {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
}

.detail-options div {
  display: flex;
  gap: 6px;
  min-width: 0;
  padding: 10px;
  background: #f7fafc;
  border: 1px solid #e5ecf2;
  border-radius: 6px;
}

.detail-options strong {
  flex: 0 0 auto;
}

.detail-options :deep(.math-text) {
  min-width: 0;
}

.detail-options span {
  min-width: 0;
}

.detail-math-text {
  color: #253142;
  font-size: 15px;
  line-height: 1.8;
}

.detail-image {
  display: block;
  max-width: 100%;
  max-height: 420px;
  object-fit: contain;
  border: 1px solid #e2e9f0;
  border-radius: 6px;
}

.paper-cart-float {
  position: fixed;
  right: 34px;
  bottom: 34px;
  z-index: 30;
}

.cart-analysis-panel {
  position: absolute;
  right: 78px;
  bottom: 0;
  width: 340px;
  padding: 20px;
  pointer-events: none;
  visibility: hidden;
  background: #ffffff;
  border: 1px solid #dcefeb;
  border-radius: 8px;
  box-shadow: 0 18px 44px rgba(15, 145, 135, 0.16);
  opacity: 0;
  transform: translateX(10px) scale(0.98);
  transition: opacity 0.18s ease, transform 0.18s ease, visibility 0.18s ease;
}

.paper-cart-float:hover .cart-analysis-panel {
  pointer-events: auto;
  visibility: visible;
  opacity: 1;
  transform: translateX(0) scale(1);
}

.analysis-title {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-bottom: 12px;
  border-bottom: 3px solid #0f9187;
}

.analysis-title strong {
  color: #0f9187;
  font-size: 17px;
}

.analysis-title span {
  color: #263447;
  font-weight: 800;
}

.analysis-summary {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
  margin: 16px 0;
}

.analysis-summary div {
  display: grid;
  gap: 4px;
  min-width: 0;
  padding: 10px;
  text-align: center;
  background: #f5fbfa;
  border: 1px solid #dcefeb;
  border-radius: 8px;
}

.analysis-summary strong {
  color: #0f9187;
  font-size: 22px;
  line-height: 1;
}

.analysis-summary span {
  color: #6b7788;
  font-size: 12px;
  font-weight: 800;
}

.analysis-section {
  display: grid;
  gap: 8px;
  margin-top: 14px;
}

.analysis-section h3 {
  margin: 0;
  color: #263447;
  font-size: 14px;
}

.stat-row {
  display: grid;
  grid-template-columns: 86px minmax(0, 1fr) 28px;
  gap: 10px;
  align-items: center;
  color: #536173;
  font-size: 13px;
  font-weight: 800;
}

.stat-row > span {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.stat-row div {
  height: 8px;
  overflow: hidden;
  background: #edf2f7;
  border-radius: 999px;
}

.stat-row i {
  display: block;
  height: 100%;
  background: linear-gradient(90deg, #35b8aa, #0f9187);
  border-radius: inherit;
}

.stat-row em {
  color: #263447;
  font-style: normal;
  text-align: right;
}

.paper-cart-button {
  position: relative;
  display: grid;
  width: 76px;
  min-height: 88px;
  padding: 10px 7px 9px;
  color: #0f9187;
  cursor: pointer;
  gap: 5px;
  place-items: center;
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.96), rgba(237, 251, 248, 0.96)),
    radial-gradient(circle at 50% 0, rgba(132, 224, 211, 0.32), transparent 58%);
  border: 1px solid #cfebe6;
  border-radius: 8px;
  box-shadow:
    0 18px 34px rgba(15, 145, 135, 0.20),
    inset 0 1px 0 rgba(255, 255, 255, 0.9),
    inset 0 -8px 18px rgba(15, 145, 135, 0.08);
  transition: color 0.18s ease, background 0.18s ease, box-shadow 0.18s ease, transform 0.18s ease;
}

.paper-cart-button::before {
  position: absolute;
  inset: 8px 10px auto;
  height: 22px;
  pointer-events: none;
  content: "";
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.75), rgba(255, 255, 255, 0));
  border-radius: 999px;
}

.paper-cart-button:hover {
  color: #0b8279;
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.98), rgba(225, 249, 245, 0.98)),
    radial-gradient(circle at 50% 0, rgba(107, 213, 198, 0.38), transparent 60%);
  box-shadow:
    0 22px 42px rgba(15, 145, 135, 0.25),
    inset 0 1px 0 rgba(255, 255, 255, 0.95),
    inset 0 -9px 20px rgba(15, 145, 135, 0.10);
  transform: translateY(-4px);
}

.paper-cart-button:active {
  transform: translateY(-1px) scale(0.98);
}

.basket-icon-shell {
  display: grid;
  width: 46px;
  height: 42px;
  place-items: center;
  background:
    radial-gradient(circle at 50% 18%, rgba(255, 255, 255, 0.9), rgba(255, 255, 255, 0) 36%),
    linear-gradient(180deg, #e8fbf7, #c4f0e9);
  border: 1px solid #a4e1d9;
  border-radius: 14px;
  box-shadow:
    0 10px 16px rgba(15, 145, 135, 0.18),
    inset 0 1px 0 rgba(255, 255, 255, 0.92),
    inset 0 -7px 13px rgba(15, 145, 135, 0.10);
  transition: transform 0.18s ease, box-shadow 0.18s ease;
}

.paper-cart-button:hover .basket-icon-shell {
  box-shadow:
    0 13px 20px rgba(15, 145, 135, 0.22),
    inset 0 1px 0 rgba(255, 255, 255, 0.95),
    inset 0 -8px 15px rgba(15, 145, 135, 0.12);
  transform: translateY(-2px) rotate(-2deg) scale(1.05);
}

.basket-3d-icon {
  display: block;
  width: 38px;
  height: 34px;
  overflow: visible;
}

.basket-shadow {
  fill: rgba(4, 92, 86, 0.22);
  filter: blur(0.2px);
}

.basket-handle {
  fill: none;
  stroke: url("#basketHandleGradient");
  stroke-width: 6;
  stroke-linecap: round;
}

.basket-rim {
  fill: #31bdb0;
  stroke: #06766f;
  stroke-width: 2;
  stroke-linejoin: round;
}

.basket-body {
  fill: url("#basketBodyGradient");
  stroke: #06766f;
  stroke-width: 2;
  stroke-linejoin: round;
}

.basket-highlight {
  fill: url("#basketFrontLight");
}

.basket-weave {
  fill: none;
  stroke: rgba(232, 255, 251, 0.72);
  stroke-width: 2.2;
  stroke-linecap: round;
}

.paper-cart-button:hover .basket-3d-icon {
  animation: basket-breathe 0.72s ease both;
}

.paper-cart-button strong {
  color: #536173;
  font-size: 14px;
  font-weight: 900;
  line-height: 1.2;
}

.cart-count-badge {
  position: absolute;
  top: -8px;
  right: -8px;
  display: grid;
  min-width: 24px;
  height: 24px;
  padding: 0 7px;
  color: #ffffff;
  font-size: 13px;
  font-weight: 900;
  place-items: center;
  background: #e34b63;
  border: 2px solid #ffffff;
  border-radius: 999px;
  box-shadow: 0 6px 12px rgba(227, 75, 99, 0.22);
}

@keyframes basket-breathe {
  0% {
    transform: translateY(0) scale(1);
  }

  45% {
    transform: translateY(-2px) scale(1.06);
  }

  100% {
    transform: translateY(0) scale(1);
  }
}

@media (max-width: 1180px) {
  .query-form {
    grid-template-columns: repeat(8, minmax(0, 1fr));
    width: 100%;
  }

  .query-form-item {
    grid-column: span 4;
  }

  .query-form-title {
    grid-column: span 5;
  }

  .query-form-year {
    grid-column: span 3;
  }

  .query-form-actions {
    grid-column: span 3;
  }

  .option-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 900px) {
  .query-form {
    grid-template-columns: minmax(0, 1fr);
    width: 100%;
  }

  .query-form-item,
  .query-form-year,
  .query-form-actions {
    grid-column: 1;
    width: 100%;
  }

  .question-card-foot,
  .question-actions {
    grid-template-columns: 1fr;
    grid-column: 1;
    grid-row: auto;
    justify-content: flex-start;
  }

  .cart-analysis-panel {
    right: 0;
    bottom: 92px;
    width: min(340px, calc(100vw - 48px));
  }
}

@media (max-width: 640px) {
  .question-result-card {
    padding: 16px;
  }

  .question-card-main {
    grid-template-columns: 1fr;
  }

  .question-order {
    text-align: left;
  }

  .option-grid {
    grid-template-columns: 1fr;
  }
}

</style>
