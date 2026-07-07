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

      <a-table
        class="work-panel query-table"
        :columns="columns"
        :data-source="questions"
        :loading="loading"
        row-key="id"
        :pagination="{ pageSize: 10 }"
        :scroll="{ x: 1394 }"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'category'">
            {{ categoryNameMap.get(record.categoryId) || '-' }}
          </template>
          <template v-if="column.key === 'title'">
            <span class="query-title-cell">{{ record.title || '未命名题目' }}</span>
          </template>
          <template v-if="column.key === 'questionType'">
            {{ record.questionType || '-' }}
          </template>
          <template v-if="column.key === 'difficulty'">
            <span v-if="record.difficulty > 0" class="difficulty-text">{{ formatDifficulty(record.difficulty) }}</span>
            <span v-else>-</span>
          </template>
          <template v-if="column.key === 'tags'">
            <a-space wrap>
              <a-tag v-for="tag in record.tags" :key="tag" color="cyan">{{ tag }}</a-tag>
            </a-space>
          </template>
          <template v-if="column.key === 'knowledgePoints'">
            <a-space wrap>
              <a-tag v-for="item in record.knowledgePoints" :key="item">{{ item }}</a-tag>
            </a-space>
          </template>
          <template v-if="column.key === 'action'">
            <a-space :size="4">
              <a-button size="small" type="link" @click="openDetail(record.id)">查看</a-button>
              <a-button size="small" type="link" :disabled="cartIdSet.has(record.id)" @click="addToPaperCart(record.id)">
                {{ cartIdSet.has(record.id) ? '已加入' : '加入试卷' }}
              </a-button>
            </a-space>
          </template>
        </template>
      </a-table>
    </main>

    <button class="paper-cart-button" type="button" title="组装试卷" @click="openPaperAssemble">
      <ShoppingCart :size="24" :stroke-width="2.4" />
      <span v-if="cartCount > 0">{{ cartCount }}</span>
    </button>

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
          <section class="detail-section">
            <h3>选项</h3>
            <div class="detail-options">
              <div v-for="option in currentQuestion.options" :key="option.optionKey">
                <strong>{{ option.optionKey }}.</strong>
                <MathText :content="option.content || '-'" />
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
import { ShoppingCart } from 'lucide-vue-next'
import MathText from '../components/MathText.vue'
import type { Question, QuestionCategory, QuestionQueryFilters } from '../api/native'
import { getQuestion, listCategories, queryQuestions, readAssetDataUrl } from '../api/native'
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
const columns = [
  { title: '标题', dataIndex: 'title', key: 'title', width: 320 },
  { title: '题型', key: 'questionType', width: 96 },
  { title: '难度', key: 'difficulty', width: 130 },
  { title: '年份', dataIndex: 'year', width: 88 },
  { title: '题号', dataIndex: 'questionNo', width: 100 },
  { title: '分类', key: 'category', width: 150 },
  { title: '标签', key: 'tags', width: 190 },
  { title: '知识点', key: 'knowledgePoints', width: 230 },
  { title: '操作', key: 'action', width: 160, fixed: 'right' }
]
const cartIdSet = computed(() => new Set(cartIds.value))
const cartCount = computed(() => cartIds.value.length)

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
  message.success('已加入试卷')
}

function openPaperAssemble() {
  router.push('/app/questions/paper-assemble')
}

async function openDetail(id: number) {
  detailOpen.value = true
  detailLoading.value = true
  detailImageUrl.value = ''
  try {
    currentQuestion.value = await getQuestion(id)
    await loadDetailImage()
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
  grid-template-columns: 300px 300px 140px;
  width: fit-content;
  max-width: 100%;
  gap: 10px 14px;
  align-items: end;
  justify-content: start;
}

.query-form-item {
  margin-bottom: 0;
  width: 300px;
  min-width: 0;
}

.query-form-year {
  width: 140px;
}

.query-form-tag,
.query-form-knowledge {
  width: 300px;
}

.query-form-actions {
  width: auto;
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

.query-form-title {
  grid-column: span 1;
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

.query-table {
  min-height: 0;
  overflow: auto;
}

.query-table :deep(.ant-table) {
  color: #273244;
}

.query-table :deep(.ant-table-thead > tr > th) {
  color: #334155;
  font-weight: 800;
  background: #ffffff;
}

.query-table :deep(.ant-table-thead > tr > th),
.query-table :deep(.ant-table-tbody > tr > td) {
  padding: 12px 14px;
  font-size: 14px;
}

.query-table :deep(.ant-table-tbody > tr > td) {
  vertical-align: middle;
}

.query-title-cell {
  display: block;
  max-width: 300px;
  overflow: hidden;
  color: #253142;
  font-weight: 700;
  line-height: 1.6;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.difficulty-text {
  color: #d97a00;
  font-weight: 800;
  white-space: nowrap;
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

.paper-cart-button {
  position: fixed;
  right: 34px;
  bottom: 34px;
  z-index: 30;
  display: grid;
  width: 58px;
  height: 58px;
  color: #ffffff;
  cursor: pointer;
  place-items: center;
  background: #0f9187;
  border: 0;
  border-radius: 50%;
  box-shadow: 0 12px 28px rgba(15, 145, 135, 0.28);
}

.paper-cart-button:hover {
  background: #0b8279;
}

.paper-cart-button span {
  position: absolute;
  top: -6px;
  right: -4px;
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
}

@media (max-width: 1180px) {
  .query-form {
    grid-template-columns: 1fr 1fr 140px;
    width: 100%;
  }

  .query-form-item,
  .query-form-tag,
  .query-form-knowledge {
    width: 100%;
  }

  .query-form-year {
    width: 140px;
  }
}

@media (max-width: 900px) {
  .query-form {
    grid-template-columns: minmax(0, 1fr);
    width: 100%;
  }

  .query-form-item,
  .query-form-year {
    width: 100%;
  }
}

</style>
