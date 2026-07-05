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
          <a-form-item label="标签" class="query-form-item">
            <a-input v-model:value="filters.tag" allow-clear placeholder="请输入标签" @press-enter="loadQuestions" />
          </a-form-item>
          <a-form-item label="知识点" class="query-form-item">
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
        :scroll="{ x: 1168 }"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'category'">
            {{ categoryNameMap.get(record.categoryId) || '-' }}
          </template>
          <template v-if="column.key === 'title'">
            <span class="query-title-cell">{{ record.title || '未命名题目' }}</span>
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
            <a-button size="small" type="link" @click="openDetail(record.id)">查看</a-button>
          </template>
        </template>
      </a-table>
    </main>

    <a-drawer v-model:open="detailOpen" title="题目详情" width="760">
      <a-spin :spinning="detailLoading">
        <template v-if="currentQuestion">
          <section class="detail-section">
            <div class="detail-meta">
              <a-tag color="cyan">{{ currentQuestion.title || '未命名题目' }}</a-tag>
              <a-tag v-if="currentQuestion.year">{{ currentQuestion.year }}</a-tag>
              <a-tag v-if="currentQuestion.questionNo">题号 {{ currentQuestion.questionNo }}</a-tag>
              <a-tag>{{ categoryNameMap.get(currentQuestion.categoryId) || '未分类' }}</a-tag>
            </div>
          </section>
          <section class="detail-section">
            <h3>题目</h3>
            <p>{{ currentQuestion.stem || '暂无题干' }}</p>
            <img v-if="detailImageUrl" class="detail-image" :src="detailImageUrl" alt="题目配图预览" />
            <a-alert v-else-if="currentQuestion.imageText" type="warning" show-icon message="图片不存在或无法预览" />
          </section>
          <section class="detail-section">
            <h3>选项</h3>
            <div class="detail-options">
              <div v-for="option in currentQuestion.options" :key="option.optionKey">
                <strong>{{ option.optionKey }}.</strong>
                <span>{{ option.content || '-' }}</span>
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
            <p>{{ currentQuestion.answer || '暂无答案' }}</p>
          </section>
          <section class="detail-section">
            <h3>解析</h3>
            <p>{{ currentQuestion.analysis || '暂无解析' }}</p>
          </section>
        </template>
      </a-spin>
    </a-drawer>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import { message } from 'ant-design-vue'
import type { Question, QuestionCategory, QuestionQueryFilters } from '../api/native'
import { getQuestion, listCategories, queryQuestions, readAssetDataUrl } from '../api/native'

interface CategoryTreeOption extends QuestionCategory {
  children: CategoryTreeOption[]
}

const loading = ref(false)
const detailLoading = ref(false)
const detailOpen = ref(false)
const questions = ref<Question[]>([])
const currentQuestion = ref<Question>()
const detailImageUrl = ref('')
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
  { title: '年份', dataIndex: 'year', width: 88 },
  { title: '题号', dataIndex: 'questionNo', width: 100 },
  { title: '分类', key: 'category', width: 150 },
  { title: '标签', key: 'tags', width: 190 },
  { title: '知识点', key: 'knowledgePoints', width: 230 },
  { title: '操作', key: 'action', width: 90, fixed: 'right' }
]

onMounted(async () => {
  await loadCategories()
  await loadQuestions()
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
</script>

<style scoped>
.question-query {
  height: calc(100vh - 58px);
  overflow: hidden;
}

.query-main {
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  gap: 14px;
  min-width: 0;
  overflow: hidden;
}

.query-filter {
  padding: 16px 20px;
}

.query-form {
  display: grid;
  grid-template-columns: minmax(190px, 1fr) minmax(240px, 1.25fr) minmax(120px, 0.58fr) minmax(180px, 0.9fr) minmax(190px, 0.95fr) auto;
  gap: 12px 16px;
  align-items: end;
}

.query-form-item {
  margin-bottom: 0;
}

.query-form :deep(.ant-form-item-label) {
  padding-bottom: 5px;
  font-weight: 700;
}

.query-form :deep(.ant-form-item-label > label) {
  height: 20px;
  color: #334155;
  font-size: 14px;
}

.query-form :deep(.ant-input),
.query-form :deep(.ant-select-selector) {
  min-height: 34px;
  height: 34px;
  border-color: #dce4ee;
  border-radius: 6px;
  font-size: 14px;
}

.query-form :deep(.ant-select-selection-search-input) {
  height: 32px;
}

.query-form-title {
  grid-column: span 1;
}

.query-form-year {
  min-width: 120px;
}

.query-form-actions {
  margin-bottom: 0;
}

.query-button {
  min-width: 68px;
}

.query-form-actions :deep(.ant-btn) {
  height: 34px;
  padding: 0 16px;
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
  padding: 10px;
  background: #f7fafc;
  border: 1px solid #e5ecf2;
  border-radius: 6px;
}

.detail-options strong {
  margin-right: 6px;
}

.detail-image {
  display: block;
  max-width: 100%;
  max-height: 420px;
  object-fit: contain;
  border: 1px solid #e2e9f0;
  border-radius: 6px;
}

@media (max-width: 1280px) {
  .query-form {
    grid-template-columns: repeat(3, minmax(180px, 1fr));
  }

  .query-form-actions {
    grid-column: auto;
  }
}

</style>
