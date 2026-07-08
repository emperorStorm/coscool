<template>
  <div class="knowledge-page page-shell">
    <section class="knowledge-panel work-panel">
      <div class="page-head">
        <div>
          <h2>知识点维护</h2>
          <p class="muted">维护题库录入时可选择的树形知识点数据</p>
        </div>
        <a-button type="primary" @click="treeRef?.load()">刷新</a-button>
      </div>
      <KnowledgePointTree ref="treeRef" title="知识点维护" @loaded="handleLoaded" @select="handleSelect" />
    </section>

    <aside class="summary-panel work-panel">
      <h3>数据概览</h3>
      <div class="summary-grid">
        <div>
          <strong>{{ totalCount }}</strong>
          <span>知识点总数</span>
        </div>
        <div>
          <strong>{{ rootCount }}</strong>
          <span>一级知识点</span>
        </div>
        <div>
          <strong>{{ childCount }}</strong>
          <span>子级知识点</span>
        </div>
      </div>
      <section class="selected-card">
        <span class="muted">当前选择</span>
        <strong>{{ selectedNode?.name || '未选择' }}</strong>
        <p v-if="selectedNode" class="muted">可通过节点右侧操作新增同级、子级、重命名或删除。</p>
        <p v-else class="muted">选择左侧节点后，可在这里确认当前维护对象。</p>
      </section>
    </aside>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import KnowledgePointTree, { type KnowledgePointNode } from '../components/KnowledgePointTree.vue'
import type { KnowledgePoint } from '../api/native'

const treeRef = ref<InstanceType<typeof KnowledgePointTree>>()
const knowledgePoints = ref<KnowledgePoint[]>([])
const selectedNode = ref<KnowledgePointNode>()

const totalCount = computed(() => knowledgePoints.value.length)
const rootCount = computed(() => knowledgePoints.value.filter((item) => !item.parentId).length)
const childCount = computed(() => Math.max(0, totalCount.value - rootCount.value))

function handleLoaded(items: KnowledgePoint[]) {
  knowledgePoints.value = items
}

function handleSelect(_id?: number, node?: KnowledgePointNode) {
  selectedNode.value = node
}
</script>

<style scoped>
.knowledge-page {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 320px;
  gap: 14px;
  height: calc(100vh - 58px);
  overflow: hidden;
}

.knowledge-panel,
.summary-panel {
  min-height: 0;
  overflow: hidden;
}

.knowledge-panel {
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  gap: 12px;
  padding: 16px;
}

.page-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.page-head h2 {
  margin: 0 0 4px;
  color: #172033;
  font-size: 20px;
}

.summary-panel {
  padding: 18px;
}

.summary-panel h3 {
  margin: 0 0 14px;
  color: #172033;
  font-size: 18px;
}

.summary-grid {
  display: grid;
  gap: 10px;
}

.summary-grid div,
.selected-card {
  padding: 14px;
  background: #ffffff;
  border: 1px solid #e1e9f2;
  border-radius: 8px;
}

.summary-grid strong {
  display: block;
  color: #0f8f83;
  font-size: 24px;
  line-height: 1.2;
}

.summary-grid span,
.selected-card p {
  margin: 4px 0 0;
}

.selected-card {
  margin-top: 14px;
}

.selected-card strong {
  display: block;
  margin-top: 8px;
  color: #263447;
  font-size: 16px;
}
</style>
