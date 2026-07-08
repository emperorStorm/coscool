<template>
  <section class="knowledge-tree-panel">
    <div class="tree-title-row">
      <div class="tree-title">{{ title }}</div>
      <div class="tree-actions">
        <a-button type="primary" shape="circle" size="small" title="新增根知识点" @click="openEditor('root')">+</a-button>
      </div>
    </div>
    <a-input-search v-model:value="keyword" placeholder="搜索知识点" allow-clear class="tree-search" @change="filterTreeData" />
    <a-tree
      v-if="filteredTreeData.length"
      :tree-data="filteredTreeData"
      :field-names="{ children: 'children', title: 'name', key: 'id' }"
      :expanded-keys="expandedKeys"
      :selected-keys="selectedKeys"
      show-line
      block-node
      @expand="onExpand"
      @select="onSelect"
    >
      <template #title="node">
        <div
          class="knowledge-node"
          :class="{
            dragging: draggingId === node.id,
            'drag-over-before': dragOverId === node.id && dragDropPosition === 'before',
            'drag-over-inside': dragOverId === node.id && dragDropPosition === 'inside',
            'drag-over-after': dragOverId === node.id && dragDropPosition === 'after'
          }"
          :draggable="!keyword.trim()"
          @dragstart.stop="onNativeDragStart($event, node)"
          @dragover.prevent.stop="onNativeDragOver($event, node)"
          @dragleave.stop="onNativeDragLeave(node)"
          @drop.prevent.stop="onNativeDrop($event, node)"
          @dragend.stop="onNativeDragEnd"
        >
          <span class="knowledge-node-name">
            <span v-for="(part, index) in renderName(node.name)" :key="index" :class="{ 'keyword-hit': part.match }">
              {{ part.text }}
            </span>
          </span>
          <span @dragstart.stop.prevent>
            <a-dropdown :trigger="['click']" placement="bottomRight">
              <button class="node-more" title="知识点操作" @click.stop="activeMenuNode = node">...</button>
              <template #overlay>
                <a-menu @click="handleMenuClick">
                  <a-menu-item key="sibling">新增同级</a-menu-item>
                  <a-menu-item key="child">新增子级</a-menu-item>
                  <a-menu-item key="edit">重命名</a-menu-item>
                  <a-menu-item key="delete" danger>删除</a-menu-item>
                </a-menu>
              </template>
            </a-dropdown>
          </span>
        </div>
      </template>
    </a-tree>
    <a-empty v-else description="暂无知识点" class="tree-empty">
      <a-button size="small" type="primary" @click="openEditor('root')">新增根知识点</a-button>
    </a-empty>

    <a-modal v-model:open="editorOpen" :title="editorTitle" @ok="saveEditor" @cancel="closeEditor">
      <a-form layout="vertical" :model="editorForm">
        <a-form-item label="知识点名称" required>
          <a-input v-model:value="editorForm.name" placeholder="请输入知识点名称" />
        </a-form-item>
      </a-form>
    </a-modal>
  </section>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { message, Modal } from 'ant-design-vue'
import type { KnowledgePoint, KnowledgePointReorderItem } from '../api/native'
import { deleteKnowledgePoint, listKnowledgePoints, reorderKnowledgePoints, saveKnowledgePoint } from '../api/native'

export interface KnowledgePointNode extends KnowledgePoint {
  children: KnowledgePointNode[]
}

const props = withDefaults(
  defineProps<{
    title?: string
    selectedId?: number
  }>(),
  {
    title: '知识点维护',
    selectedId: undefined
  }
)

const emit = defineEmits<{
  select: [id?: number, node?: KnowledgePointNode]
  loaded: [items: KnowledgePoint[], tree: KnowledgePointNode[]]
}>()

const items = ref<KnowledgePoint[]>([])
const treeData = ref<KnowledgePointNode[]>([])
const filteredTreeData = ref<KnowledgePointNode[]>([])
const flatList = ref<KnowledgePointNode[]>([])
const expandedKeys = ref<number[]>([])
const cachedExpandedKeys = ref<number[]>([])
const selectedKeys = ref<number[]>([])
const keyword = ref('')
const draggingId = ref<number>()
const dragOverId = ref<number>()
const dragDropPosition = ref<NativeDropPosition>()
const editorOpen = ref(false)
const editorMode = ref('root')
const editorTarget = ref<KnowledgePointNode | undefined>()
const activeMenuNode = ref<KnowledgePointNode | undefined>()
const editorForm = reactive<Partial<KnowledgePoint>>({
  id: undefined,
  parentId: undefined,
  name: '',
  sortOrder: 0
})

const editorTitle = computed(() => {
  if (editorMode.value === 'edit') return '修改知识点'
  if (editorMode.value === 'child') return '新增子级知识点'
  return '新增知识点'
})

watch(
  () => props.selectedId,
  (id) => {
    selectedKeys.value = id ? [id] : []
  }
)

onMounted(load)

async function load(selectedId?: number) {
  items.value = await listKnowledgePoints()
  treeData.value = buildTree(items.value)
  flatList.value = flattenTree(treeData.value)
  filteredTreeData.value = keyword.value ? filterTree(treeData.value, keyword.value) : treeData.value
  const nextSelectedId = selectedId || props.selectedId
  selectedKeys.value = nextSelectedId ? [nextSelectedId] : []
  expandedKeys.value = treeData.value.map((item) => item.id)
  cachedExpandedKeys.value = [...expandedKeys.value]
  emit('loaded', items.value, treeData.value)
}

function buildTree(list: KnowledgePoint[]) {
  const nodeMap = new Map<number, KnowledgePointNode>()
  list.forEach((item) => nodeMap.set(item.id, { ...item, children: [] }))
  const roots: KnowledgePointNode[] = []
  nodeMap.forEach((node) => {
    if (node.parentId && nodeMap.has(node.parentId)) {
      nodeMap.get(node.parentId)?.children.push(node)
    } else {
      roots.push(node)
    }
  })
  return roots
}

function flattenTree(tree: KnowledgePointNode[]) {
  return tree.reduce<KnowledgePointNode[]>((list, item) => {
    list.push(item)
    if (item.children.length) list.push(...flattenTree(item.children))
    return list
  }, [])
}

function filterTreeData() {
  const text = keyword.value.trim()
  filteredTreeData.value = text ? filterTree(treeData.value, text) : treeData.value
  if (text) {
    expandedKeys.value = flattenTree(filteredTreeData.value).map((item) => item.id)
  } else {
    expandedKeys.value = [...cachedExpandedKeys.value]
  }
}

function filterTree(tree: KnowledgePointNode[], text: string): KnowledgePointNode[] {
  return tree
    .map((item) => {
      const children = filterTree(item.children, text)
      if (item.name.includes(text) || children.length) {
        return { ...item, children }
      }
      return undefined
    })
    .filter(Boolean) as KnowledgePointNode[]
}

function onExpand(keys: (string | number)[]) {
  expandedKeys.value = keys.map(Number)
  if (!keyword.value) cachedExpandedKeys.value = [...expandedKeys.value]
}

function onSelect(keys: (string | number)[]) {
  const selectedId = keys.length ? Number(keys[0]) : undefined
  selectedKeys.value = selectedId ? [selectedId] : []
  emit('select', selectedId, flatList.value.find((item) => item.id === selectedId))
}

function onNativeDragStart(event: DragEvent, node: KnowledgePointNode) {
  if (keyword.value.trim()) {
    event.preventDefault()
    return
  }
  draggingId.value = node.id
  event.dataTransfer?.setData('text/plain', String(node.id))
  if (event.dataTransfer) event.dataTransfer.effectAllowed = 'move'
}

function onNativeDragOver(event: DragEvent, node: KnowledgePointNode) {
  const dragId = getDraggingId(event)
  if (!dragId || dragId === node.id || keyword.value.trim()) {
    clearDragOverState()
    if (event.dataTransfer) event.dataTransfer.dropEffect = 'none'
    return
  }
  const draggingNode = findNode(treeData.value, dragId)
  if (!draggingNode || isDescendant(draggingNode, node.id)) {
    clearDragOverState()
    if (event.dataTransfer) event.dataTransfer.dropEffect = 'none'
    return
  }
  dragOverId.value = node.id
  dragDropPosition.value = getNativeDropPosition(event)
  if (event.dataTransfer) event.dataTransfer.dropEffect = 'move'
}

function onNativeDragLeave(node: KnowledgePointNode) {
  if (dragOverId.value === node.id) clearDragOverState()
}

async function onNativeDrop(event: DragEvent, node: KnowledgePointNode) {
  const dragId = getDraggingId(event)
  const position = dragDropPosition.value || getNativeDropPosition(event)
  clearDragOverState()
  if (!dragId || dragId === node.id || keyword.value.trim()) return
  await applyTreeDrop(dragId, node.id, position)
}

function onNativeDragEnd() {
  draggingId.value = undefined
  clearDragOverState()
}

async function applyTreeDrop(dragId: number, targetId: number, position: NativeDropPosition) {
  const nextTree = cloneTree(treeData.value)
  const movingNode = removeNode(nextTree, dragId)
  if (!movingNode) return
  if (isDescendant(movingNode, targetId)) {
    message.warning('不能拖到自身下级')
    return
  }

  const targetNode = findNode(nextTree, targetId)
  if (!targetNode) {
    await load(selectedKeys.value[0])
    return
  }

  if (position === 'inside') {
    movingNode.parentId = targetNode.id
    targetNode.children.push(movingNode)
    expandNode(targetNode.id)
  } else {
    insertNearTarget(nextTree, movingNode, targetId, position)
  }

  await saveTreeOrder(nextTree, '排序已保存')
}

async function saveTreeOrder(nextTree: KnowledgePointNode[], successText: string) {
  normalizeTreeOrder(nextTree, undefined)
  try {
    await reorderKnowledgePoints(buildReorderPayload(nextTree))
    applyTreeState(nextTree)
    message.success(successText)
  } catch (error) {
    message.error(String(error))
    await load(selectedKeys.value[0])
  }
}

function applyTreeState(nextTree: KnowledgePointNode[]) {
  const text = keyword.value.trim()
  treeData.value = nextTree
  flatList.value = flattenTree(nextTree)
  filteredTreeData.value = text ? filterTree(nextTree, text) : nextTree
  items.value = flatList.value.map(({ children, ...item }) => item)
  cachedExpandedKeys.value = [...expandedKeys.value]
  emit('loaded', items.value, treeData.value)
}

function getDraggingId(event: DragEvent) {
  const rawId = draggingId.value || event.dataTransfer?.getData('text/plain')
  const id = Number(rawId)
  return Number.isFinite(id) ? id : undefined
}

function getNativeDropPosition(event: DragEvent): NativeDropPosition {
  const element = event.currentTarget as HTMLElement | null
  if (!element) return 'inside'
  const rect = element.getBoundingClientRect()
  const offsetY = event.clientY - rect.top
  if (offsetY < rect.height * 0.28) return 'before'
  if (offsetY > rect.height * 0.72) return 'after'
  return 'inside'
}

function clearDragOverState() {
  dragOverId.value = undefined
  dragDropPosition.value = undefined
}

function handleMenuClick(event: { key: string | number }) {
  if (!activeMenuNode.value) return
  const key = String(event.key)
  if (key === 'delete') {
    confirmDelete(activeMenuNode.value)
    return
  }
  openEditor(key, activeMenuNode.value)
}

function openEditor(mode: string, target?: KnowledgePointNode) {
  editorMode.value = mode
  editorTarget.value = target
  if (mode === 'edit' && target) {
    Object.assign(editorForm, target)
  } else {
    const parentId = resolveParentId(mode, target)
    Object.assign(editorForm, {
      id: undefined,
      parentId,
      name: '',
      sortOrder: getNextSortOrder(parentId)
    })
  }
  editorOpen.value = true
}

function resolveParentId(mode: string, target?: KnowledgePointNode) {
  if (mode === 'child' && target) return target.id
  if (mode === 'sibling' && target) return target.parentId
  return undefined
}

function getNextSortOrder(parentId?: number) {
  return items.value.filter((item) => item.parentId === parentId).length + 1
}

function closeEditor() {
  editorOpen.value = false
  Object.assign(editorForm, { id: undefined, parentId: undefined, name: '', sortOrder: 0 })
}

async function saveEditor() {
  if (!editorForm.name?.trim()) {
    message.warning('请输入知识点名称')
    return
  }
  try {
    const saved = await saveKnowledgePoint(editorForm)
    closeEditor()
    await load(saved.id)
    selectedKeys.value = [saved.id]
    emit('select', saved.id, flatList.value.find((item) => item.id === saved.id))
    message.success('保存成功')
  } catch (error) {
    message.error(String(error))
  }
}

function confirmDelete(node: KnowledgePointNode) {
  Modal.confirm({
    title: '确认删除知识点？',
    content: `知识点“${node.name}”及其子级删除后不可恢复，已关联题目会取消该知识点关系。`,
    okText: '删除',
    okType: 'danger',
    cancelText: '取消',
    async onOk() {
      await deleteKnowledgePoint(node.id)
      if (selectedKeys.value.includes(node.id)) {
        selectedKeys.value = []
        emit('select', undefined, undefined)
      }
      await load()
      message.success('删除成功')
    }
  })
}

function renderName(name: string) {
  const text = keyword.value.trim()
  if (!text) return [{ text: name || '-', match: false }]
  const parts: Array<{ text: string; match: boolean }> = []
  let startIndex = 0
  let matchIndex = name.indexOf(text)
  while (matchIndex > -1) {
    if (matchIndex > startIndex) parts.push({ text: name.slice(startIndex, matchIndex), match: false })
    parts.push({ text: name.slice(matchIndex, matchIndex + text.length), match: true })
    startIndex = matchIndex + text.length
    matchIndex = name.indexOf(text, startIndex)
  }
  if (startIndex < name.length) parts.push({ text: name.slice(startIndex), match: false })
  return parts.length ? parts : [{ text: name || '-', match: false }]
}

type NativeDropPosition = 'before' | 'inside' | 'after'

function cloneTree(tree: KnowledgePointNode[]): KnowledgePointNode[] {
  return tree.map((item) => ({
    ...item,
    children: cloneTree(item.children)
  }))
}

function removeNode(tree: KnowledgePointNode[], id: number): KnowledgePointNode | undefined {
  const index = tree.findIndex((item) => item.id === id)
  if (index > -1) {
    return tree.splice(index, 1)[0]
  }
  for (const item of tree) {
    const removed = removeNode(item.children, id)
    if (removed) return removed
  }
  return undefined
}

function findNode(tree: KnowledgePointNode[], id: number): KnowledgePointNode | undefined {
  for (const item of tree) {
    if (item.id === id) return item
    const child = findNode(item.children, id)
    if (child) return child
  }
  return undefined
}

function isDescendant(node: KnowledgePointNode, targetId: number): boolean {
  return node.children.some((child) => child.id === targetId || isDescendant(child, targetId))
}

function insertNearTarget(
  tree: KnowledgePointNode[],
  movingNode: KnowledgePointNode,
  targetId: number,
  position: 'before' | 'after'
) {
  const inserted = insertNearTargetInList(tree, movingNode, targetId, position, undefined)
  if (!inserted) tree.push(movingNode)
}

function insertNearTargetInList(
  list: KnowledgePointNode[],
  movingNode: KnowledgePointNode,
  targetId: number,
  position: Exclude<NativeDropPosition, 'inside'>,
  parentId?: number
): boolean {
  const index = list.findIndex((item) => item.id === targetId)
  if (index > -1) {
    movingNode.parentId = parentId
    list.splice(position === 'before' ? index : index + 1, 0, movingNode)
    return true
  }
  return list.some((item) => insertNearTargetInList(item.children, movingNode, targetId, position, item.id))
}

function normalizeTreeOrder(tree: KnowledgePointNode[], parentId?: number) {
  tree.forEach((item, index) => {
    item.parentId = parentId
    item.sortOrder = index + 1
    normalizeTreeOrder(item.children, item.id)
  })
}

function buildReorderPayload(tree: KnowledgePointNode[]) {
  return flattenTree(tree).map<KnowledgePointReorderItem>((item) => ({
    id: item.id,
    parentId: item.parentId,
    sortOrder: item.sortOrder
  }))
}

function expandNode(id: number) {
  if (!expandedKeys.value.includes(id)) {
    expandedKeys.value = [...expandedKeys.value, id]
  }
}

defineExpose({ load })
</script>

<style scoped>
.knowledge-tree-panel {
  height: 100%;
  padding: 14px;
  overflow: hidden;
  background: linear-gradient(180deg, #ffffff 0%, #fafdff 100%);
  border: 1px solid #dcecff;
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(43, 96, 155, 0.08);
}

.tree-title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 2px 0 12px;
  margin-bottom: 12px;
  border-bottom: 1px solid #e6f0fb;
}

.tree-title {
  position: relative;
  padding-left: 10px;
  color: #16365c;
  font-size: 16px;
  font-weight: 700;
  line-height: 24px;
}

.tree-title::before {
  position: absolute;
  top: 5px;
  left: 0;
  width: 3px;
  height: 14px;
  content: '';
  background: #0f8f83;
  border-radius: 3px;
}

.tree-search {
  margin-bottom: 12px;
}

.knowledge-node {
  position: relative;
  display: flex;
  gap: 8px;
  align-items: center;
  justify-content: space-between;
  min-width: 0;
  width: 100%;
  padding: 2px 4px;
  border-radius: 4px;
}

.knowledge-node[draggable='true'] {
  cursor: move;
}

.knowledge-node.dragging {
  opacity: 0.55;
}

.knowledge-node.drag-over-inside {
  background: #e6f7ff;
  outline: 1px dashed #0f8f83;
}

.knowledge-node.drag-over-before::before,
.knowledge-node.drag-over-after::after {
  position: absolute;
  right: 4px;
  left: 4px;
  height: 2px;
  content: '';
  background: #0f8f83;
  border-radius: 2px;
}

.knowledge-node.drag-over-before::before {
  top: 0;
}

.knowledge-node.drag-over-after::after {
  bottom: 0;
}

.knowledge-node-name {
  min-width: 0;
  overflow: hidden;
  color: #263f5f;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.node-more {
  width: 24px;
  height: 24px;
  color: #8190a2;
  cursor: pointer;
  background: transparent;
  border: 0;
  border-radius: 4px;
  opacity: 0;
}

.knowledge-node:hover .node-more {
  opacity: 1;
}

.node-more:hover {
  color: #0f8f83;
  background: #e9f7f5;
}

.keyword-hit {
  color: #d97706;
  font-weight: 700;
}

.tree-empty {
  padding-top: 70px;
}

:deep(.ant-tree) {
  max-height: calc(100vh - 245px);
  overflow: auto;
  padding: 4px 2px 10px;
}

:deep(.ant-tree-node-content-wrapper) {
  min-width: 0;
  width: calc(100% - 24px);
  border-radius: 6px;
}

:deep(.ant-tree-node-selected) {
  background: #e8f7f4 !important;
}
</style>
