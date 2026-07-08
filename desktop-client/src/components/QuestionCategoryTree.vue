<template>
  <section class="category-tree-panel">
    <div class="tree-title-row">
      <div class="tree-title">{{ title }}</div>
      <div class="tree-actions">
        <a-button shape="circle" size="small" title="导出题库备份" :loading="exportingBackup" @click="handleExportBackup">
          <Download v-if="!exportingBackup" class="tree-action-icon" :size="14" :stroke-width="2.4" />
        </a-button>
        <a-button shape="circle" size="small" title="备份还原" :loading="restoringBackup" @click="selectBackupFile">
          <Upload v-if="!restoringBackup" class="tree-action-icon" :size="14" :stroke-width="2.4" />
        </a-button>
        <a-button type="primary" shape="circle" size="small" title="新增根分类" @click="openEditor('root')">+</a-button>
      </div>
    </div>
    <a-input-search v-model:value="categoryKeyword" placeholder="搜索分类" allow-clear class="category-search" @change="filterCategoryTree" />
    <a-tree
      v-if="filteredTreeData.length"
      :tree-data="filteredTreeData"
      :field-names="{ children: 'children', title: 'name', key: 'id' }"
      :expanded-keys="expandedKeys"
      :selected-keys="selectedKeys"
      show-line
      block-node
      @expand="onExpand"
      @select="onSelectCategory"
    >
      <template #title="node">
        <div class="category-node">
          <span class="category-node-name">
            <span v-for="(part, index) in renderCategoryName(node.name)" :key="index" :class="{ 'category-keyword': part.match }">
              {{ part.text }}
            </span>
          </span>
          <a-dropdown :trigger="['click']" placement="bottomRight">
            <button class="category-node-more" title="分类操作" @click.stop="activeMenuNode = node">...</button>
            <template #overlay>
              <a-menu @click="handleCategoryMenuClick">
                <a-menu-item key="sibling">新增同级</a-menu-item>
                <a-menu-item key="child">新增子级</a-menu-item>
                <a-menu-item key="edit">重命名</a-menu-item>
                <a-menu-item key="delete" danger>删除</a-menu-item>
              </a-menu>
            </template>
          </a-dropdown>
        </div>
      </template>
    </a-tree>
    <a-empty v-else description="暂无分类" class="category-empty">
      <a-button size="small" type="primary" @click="openEditor('root')">新增根分类</a-button>
    </a-empty>

    <a-modal v-model:open="editorOpen" :title="editorTitle" @ok="saveEditor" @cancel="closeEditor">
      <a-form layout="vertical" :model="editorForm">
        <a-form-item label="分类名称" required>
          <a-input v-model:value="editorForm.name" placeholder="请输入分类名称" />
        </a-form-item>
      </a-form>
    </a-modal>
  </section>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { message, Modal } from 'ant-design-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { Download, Upload } from 'lucide-vue-next'
import type { QuestionCategory } from '../api/native'
import { deleteCategory, exportDataBackup, listCategories, restoreDataBackup, saveCategory } from '../api/native'

export interface CategoryTreeNode extends QuestionCategory {
  children: CategoryTreeNode[]
}

const props = withDefaults(
  defineProps<{
    title?: string
    selectedId?: number
    allowAll?: boolean
  }>(),
  {
    title: '题库分类',
    selectedId: undefined,
    allowAll: false
  }
)

const emit = defineEmits<{
  select: [id?: number, node?: CategoryTreeNode]
  loaded: [categories: QuestionCategory[], tree: CategoryTreeNode[]]
  restored: []
}>()

const categories = ref<QuestionCategory[]>([])
const treeData = ref<CategoryTreeNode[]>([])
const filteredTreeData = ref<CategoryTreeNode[]>([])
const flatCategoryList = ref<CategoryTreeNode[]>([])
const expandedKeys = ref<number[]>([])
const cachedExpandedKeys = ref<number[]>([])
const selectedKeys = ref<number[]>([])
const categoryKeyword = ref('')
const editorOpen = ref(false)
const editorMode = ref('root')
const editorTarget = ref<CategoryTreeNode | undefined>()
const activeMenuNode = ref<CategoryTreeNode | undefined>()
const exportingBackup = ref(false)
const restoringBackup = ref(false)
const editorForm = reactive<Partial<QuestionCategory>>({
  id: undefined,
  parentId: undefined,
  name: '',
  sortOrder: 0
})

const editorTitle = computed(() => {
  if (editorMode.value === 'edit') return '修改分类'
  if (editorMode.value === 'child') return '新增子级分类'
  return '新增分类'
})

watch(
  () => props.selectedId,
  (id) => {
    selectedKeys.value = id ? [id] : []
  }
)

onMounted(load)

async function load(selectedId?: number) {
  categories.value = await listCategories()
  treeData.value = buildTree(categories.value)
  flatCategoryList.value = flattenTree(treeData.value)
  filteredTreeData.value = categoryKeyword.value ? filterTree(treeData.value, categoryKeyword.value) : treeData.value
  const nextSelectedId = selectedId || props.selectedId
  selectedKeys.value = nextSelectedId ? [nextSelectedId] : []
  expandedKeys.value = flatCategoryList.value.map((item) => item.id)
  cachedExpandedKeys.value = [...expandedKeys.value]
  emit('loaded', categories.value, treeData.value)
}

function buildTree(list: QuestionCategory[]) {
  const nodeMap = new Map<number, CategoryTreeNode>()
  list.forEach((item) => nodeMap.set(item.id, { ...item, children: [] }))
  const roots: CategoryTreeNode[] = []
  nodeMap.forEach((node) => {
    if (node.parentId && nodeMap.has(node.parentId)) {
      nodeMap.get(node.parentId)?.children.push(node)
    } else {
      roots.push(node)
    }
  })
  return roots
}

function flattenTree(tree: CategoryTreeNode[]) {
  return tree.reduce<CategoryTreeNode[]>((list, item) => {
    list.push(item)
    if (item.children.length) list.push(...flattenTree(item.children))
    return list
  }, [])
}

function filterCategoryTree() {
  const keyword = categoryKeyword.value.trim()
  filteredTreeData.value = keyword ? filterTree(treeData.value, keyword) : treeData.value
  if (keyword) {
    expandedKeys.value = flattenTree(filteredTreeData.value).map((item) => item.id)
  } else if (cachedExpandedKeys.value.length) {
    expandedKeys.value = [...cachedExpandedKeys.value]
  }
}

function filterTree(tree: CategoryTreeNode[], keyword: string): CategoryTreeNode[] {
  return tree
    .map((item) => {
      const children: CategoryTreeNode[] = filterTree(item.children, keyword)
      if (item.name.includes(keyword) || children.length) {
        return { ...item, children }
      }
      return undefined
    })
    .filter(Boolean) as CategoryTreeNode[]
}

function onExpand(keys: (string | number)[]) {
  expandedKeys.value = keys.map(Number)
  if (!categoryKeyword.value) cachedExpandedKeys.value = [...expandedKeys.value]
}

function onSelectCategory(keys: (string | number)[]) {
  const selectedId = keys.length ? Number(keys[0]) : undefined
  selectedKeys.value = selectedId ? [selectedId] : []
  emit('select', selectedId, flatCategoryList.value.find((item) => item.id === selectedId))
}

function onCategoryMenuClick(key: string, node: CategoryTreeNode) {
  if (key === 'delete') {
    confirmDeleteCategory(node)
    return
  }
  openEditor(key, node)
}

function handleCategoryMenuClick(event: { key: string | number }) {
  if (!activeMenuNode.value) return
  onCategoryMenuClick(String(event.key), activeMenuNode.value)
}

function openEditor(mode: string, target?: CategoryTreeNode) {
  editorMode.value = mode
  editorTarget.value = target
  if (mode === 'edit' && target) {
    Object.assign(editorForm, target)
  } else {
    Object.assign(editorForm, {
      id: undefined,
      name: '',
      parentId: resolveParentId(mode, target),
      sortOrder: getNextSortOrder(resolveParentId(mode, target))
    })
  }
  editorOpen.value = true
}

function resolveParentId(mode: string, target?: CategoryTreeNode) {
  if (mode === 'child' && target) return target.id
  if (mode === 'sibling' && target) return target.parentId
  return undefined
}

function getNextSortOrder(parentId?: number) {
  return categories.value.filter((item) => item.parentId === parentId).length + 1
}

function closeEditor() {
  editorOpen.value = false
  Object.assign(editorForm, { id: undefined, parentId: undefined, name: '', sortOrder: 0 })
}

async function saveEditor() {
  if (!editorForm.name?.trim()) {
    message.warning('请输入分类名称')
    return
  }
  try {
    const saved = await saveCategory(editorForm)
    closeEditor()
    await load(saved.id)
    selectedKeys.value = [saved.id]
    emit('select', saved.id, flatCategoryList.value.find((item) => item.id === saved.id))
    message.success('保存成功')
  } catch (error) {
    message.error(String(error))
  }
}

async function handleExportBackup() {
  if (exportingBackup.value) return
  exportingBackup.value = true
  try {
    const result = await exportDataBackup()
    message.success({
      content: `导出成功：${result.filePath}`,
      duration: 6
    })
  } catch (error) {
    message.error(String(error))
  } finally {
    exportingBackup.value = false
  }
}

async function selectBackupFile() {
  if (restoringBackup.value) return
  const selected = await open({
    directory: false,
    multiple: false,
    title: '选择课思库备份文件',
    filters: [{ name: 'Zip 备份文件', extensions: ['zip'] }]
  })
  const path = Array.isArray(selected) ? selected[0] : selected
  if (!path) return
  Modal.confirm({
    title: '确认覆盖还原数据？',
    content: '还原后当前题库分类、题目和资源会被备份文件覆盖，请确认已选择正确的 zip 备份文件。',
    okText: '覆盖还原',
    okType: 'danger',
    cancelText: '取消',
    async onOk() {
      restoringBackup.value = true
      try {
        const result = await restoreDataBackup(path)
        selectedKeys.value = []
        await load()
        emit('restored')
        message.success({
          content: `还原成功：${result.filePath}`,
          duration: 6
        })
      } catch (error) {
        message.error(String(error))
      } finally {
        restoringBackup.value = false
      }
    }
  })
}

function confirmDeleteCategory(node: CategoryTreeNode) {
  Modal.confirm({
    title: '确认删除分类？',
    content: `分类“${node.name}”删除后不可恢复，已关联题目会取消分类归属。`,
    okText: '删除',
    okType: 'danger',
    cancelText: '取消',
    async onOk() {
      await deleteCategory(node.id)
      if (selectedKeys.value.includes(node.id)) {
        selectedKeys.value = []
        emit('select', undefined, undefined)
      }
      await load()
      message.success('删除成功')
    }
  })
}

function renderCategoryName(name: string) {
  const keyword = categoryKeyword.value.trim()
  if (!keyword) return [{ text: name || '-', match: false }]
  const parts: Array<{ text: string; match: boolean }> = []
  let startIndex = 0
  let matchIndex = name.indexOf(keyword)
  while (matchIndex > -1) {
    if (matchIndex > startIndex) parts.push({ text: name.slice(startIndex, matchIndex), match: false })
    parts.push({ text: name.slice(matchIndex, matchIndex + keyword.length), match: true })
    startIndex = matchIndex + keyword.length
    matchIndex = name.indexOf(keyword, startIndex)
  }
  if (startIndex < name.length) parts.push({ text: name.slice(startIndex), match: false })
  return parts.length ? parts : [{ text: name || '-', match: false }]
}

defineExpose({ load })
</script>

<style scoped>
.category-tree-panel {
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

.tree-actions {
  display: inline-flex;
  gap: 6px;
  align-items: center;
  flex: 0 0 auto;
}

.tree-actions :deep(.ant-btn) {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.tree-action-icon {
  color: currentcolor;
}

.category-search {
  margin-bottom: 12px;
}

.category-node {
  display: flex;
  gap: 8px;
  align-items: center;
  justify-content: space-between;
  min-width: 0;
  width: 100%;
  padding-right: 4px;
}

.category-node-name {
  min-width: 0;
  overflow: hidden;
  color: #263f5f;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.category-node-more {
  width: 24px;
  height: 24px;
  color: #8190a2;
  cursor: pointer;
  background: transparent;
  border: 0;
  border-radius: 4px;
  opacity: 0;
}

.category-node:hover .category-node-more {
  opacity: 1;
}

.category-node-more:hover {
  color: #0f8f83;
  background: #e9f7f5;
}

.category-keyword {
  color: #d97706;
  font-weight: 700;
}

.category-empty {
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
