<template>
  <div class="paper-maintenance page-shell">
    <header class="toolbar">
      <div>
        <h2 class="page-title">试卷维护</h2>
        <span class="muted">管理已生成的本地试卷</span>
      </div>
      <a-space>
        <a-button @click="goAssemble">组装试卷</a-button>
        <a-button type="primary" @click="loadPapers">刷新</a-button>
      </a-space>
    </header>

    <a-table
      class="work-panel"
      :columns="columns"
      :data-source="papers"
      :loading="loading"
      row-key="id"
      :pagination="{ pageSize: 10 }"
    >
      <template #bodyCell="{ column, record }">
        <template v-if="column.key === 'questionCount'">
          {{ record.questionCount }} 道
        </template>
        <template v-if="column.key === 'remark'">
          <span class="remark-cell">{{ record.remark || '-' }}</span>
        </template>
        <template v-if="column.key === 'action'">
          <a-space>
            <a-button size="small" type="link" @click="openDetail(record.id)">查看</a-button>
            <a-button size="small" type="link" @click="copyToCart(record.id)">复制</a-button>
            <a-button size="small" type="link" @click="openExportPage(record.id)">导出PDF</a-button>
            <a-popconfirm title="确认删除该试卷？" @confirm="removePaper(record.id)">
              <a-button size="small" type="link" danger>删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </template>
    </a-table>

    <a-drawer v-model:open="detailOpen" width="960" title="试卷详情">
      <a-spin :spinning="detailLoading">
        <template v-if="currentDetail">
          <section class="detail-meta">
            <a-tag color="cyan">{{ currentDetail.paper.title }}</a-tag>
            <a-tag>{{ currentDetail.paper.questionCount }} 道题</a-tag>
            <a-tag v-if="currentDetail.paper.createdBy">{{ currentDetail.paper.createdBy }}</a-tag>
            <span class="muted">{{ currentDetail.paper.createdAt }}</span>
          </section>
          <p v-if="currentDetail.paper.remark" class="paper-remark">{{ currentDetail.paper.remark }}</p>
          <PaperPreview
            :title="currentDetail.paper.title"
            :questions="currentDetail.questions"
            :image-map="questionImageMap"
          />
        </template>
      </a-spin>
    </a-drawer>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { message } from 'ant-design-vue'
import { useRouter } from 'vue-router'
import PaperPreview from '../components/PaperPreview.vue'
import type { Paper, PaperDetail, Question } from '../api/native'
import { deletePaper, getPaper, listPapers, readAssetDataUrl } from '../api/native'
import { setPaperCartIds } from '../utils/paperCart'

const router = useRouter()
const loading = ref(false)
const detailLoading = ref(false)
const detailOpen = ref(false)
const papers = ref<Paper[]>([])
const currentDetail = ref<PaperDetail>()
const questionImageMap = ref<Record<number, string>>({})
const columns = [
  { title: '试卷名称', dataIndex: 'title', key: 'title', width: 260 },
  { title: '题目数', key: 'questionCount', width: 100 },
  { title: '创建人', dataIndex: 'createdBy', width: 120 },
  { title: '备注', key: 'remark' },
  { title: '创建时间', dataIndex: 'createdAt', width: 170 },
  { title: '更新时间', dataIndex: 'updatedAt', width: 170 },
  { title: '操作', key: 'action', width: 260, fixed: 'right' }
]

onMounted(loadPapers)

async function loadPapers() {
  loading.value = true
  try {
    papers.value = await listPapers()
  } catch (error) {
    message.error(String(error))
  } finally {
    loading.value = false
  }
}

async function openDetail(id: number) {
  detailOpen.value = true
  detailLoading.value = true
  try {
    currentDetail.value = await getPaper(id)
    await loadQuestionImages(currentDetail.value.questions)
  } catch (error) {
    message.error(String(error))
  } finally {
    detailLoading.value = false
  }
}

async function copyToCart(id: number) {
  try {
    const detail = await getPaper(id)
    setPaperCartIds(detail.questions.map((question) => question.id))
    message.success('已复制到组卷购物车')
    router.push('/app/questions/paper-assemble')
  } catch (error) {
    message.error(String(error))
  }
}

function openExportPage(id: number) {
  router.push(`/paper-export/${id}`)
}

async function removePaper(id: number) {
  try {
    await deletePaper(id)
    await loadPapers()
    message.success('试卷已删除')
  } catch (error) {
    message.error(String(error))
  }
}

async function loadQuestionImages(questions: Question[]) {
  const imageMap: Record<number, string> = {}
  await Promise.all(
    questions.map(async (question) => {
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

function goAssemble() {
  router.push('/app/questions/paper-assemble')
}
</script>

<style scoped>
.paper-maintenance {
  height: calc(100vh - 58px);
  overflow: auto;
}

.paper-maintenance .toolbar {
  margin-bottom: 14px;
}

.paper-maintenance .toolbar h2 {
  margin-bottom: 4px;
}

.paper-maintenance :deep(.ant-table-thead > tr > th) {
  color: #334155;
  font-weight: 800;
  background: #ffffff;
}

.remark-cell {
  display: block;
  max-width: 360px;
  overflow: hidden;
  color: #526174;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.detail-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
  margin-bottom: 12px;
}

.paper-remark {
  margin: 0 0 16px;
  padding: 12px;
  color: #526174;
  background: #f7fafc;
  border: 1px solid #e4ebf2;
  border-radius: 8px;
}

</style>
