<template>
  <a-modal
    :open="open"
    :footer="null"
    :width="1280"
    centered
    class="formula-symbol-modal"
    @cancel="emit('update:open', false)"
  >
    <div class="symbol-modal">
      <header class="symbol-header">
        <div class="symbol-title">
          <h2>公式与符号</h2>
          <span>点击项可复制到剪贴板</span>
        </div>
        <div class="symbol-tabs">
          <button
            v-for="tab in tabs"
            :key="tab.key"
            type="button"
            :class="['symbol-tab', { active: activeTab === tab.key }]"
            @click="activeTab = tab.key"
          >
            {{ tab.name }}
          </button>
        </div>
      </header>

      <main class="symbol-body">
        <aside class="symbol-category-pane">
          <template v-if="activeTab === 'formula'">
            <button
              v-for="category in formulaCategories"
              :key="category.key"
              type="button"
              :class="['category-item', { active: activeCategory === category.key }]"
              @click="activeCategory = category.key"
            >
              {{ category.name }}
            </button>
          </template>
          <button v-else type="button" class="category-item active">全部</button>
        </aside>

        <section class="symbol-content">
          <div
            v-for="item in currentItems"
            :key="`${item.label}-${item.latex}`"
            class="formula-card"
            role="button"
            tabindex="0"
            @click="copyFormula(item.latex)"
            @keydown.enter.prevent="copyFormula(item.latex)"
            @keydown.space.prevent="copyFormula(item.latex)"
          >
            <span class="formula-card-label">{{ item.label }}</span>
            <MathText class="formula-preview-text" :content="item.latex" />
          </div>
        </section>
      </main>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { message } from 'ant-design-vue'
import MathText from './MathText.vue'
import { commonSymbols, formulaCategories, phoneticSymbols } from '../constants/formulaSymbols'

const props = defineProps<{
  open: boolean
}>()

const emit = defineEmits<{
  'update:open': [open: boolean]
}>()

const tabs = [
  { key: 'formula', name: '常用公式' },
  { key: 'symbol', name: '常用符号' },
  { key: 'phonetic', name: '英语音标' }
] as const

const activeTab = ref<(typeof tabs)[number]['key']>('formula')
const activeCategory = ref('geometry')
const currentItems = computed(() => {
  if (activeTab.value === 'symbol') return commonSymbols
  if (activeTab.value === 'phonetic') return phoneticSymbols
  return formulaCategories.find((item) => item.key === activeCategory.value)?.items || []
})

async function copyFormula(latex: string) {
  try {
    await copyText(latex)
    message.success('已复制')
  } catch (error) {
    message.error(`复制失败：${String(error)}`)
  }
}

async function copyText(text: string) {
  if (navigator.clipboard?.writeText) {
    await navigator.clipboard.writeText(text)
    return
  }
  const textarea = document.createElement('textarea')
  textarea.value = text
  textarea.setAttribute('readonly', 'true')
  textarea.style.position = 'fixed'
  textarea.style.left = '-9999px'
  document.body.appendChild(textarea)
  textarea.select()
  const copied = document.execCommand('copy')
  document.body.removeChild(textarea)
  if (!copied) throw new Error('当前环境不支持剪贴板')
}
</script>

<style scoped>
.symbol-modal {
  min-height: 720px;
  overflow: hidden;
  background: #ffffff;
  border-radius: 8px;
}

.symbol-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: 86px;
  padding: 0 28px;
  border-bottom: 1px solid #edf1f5;
}

.symbol-title {
  display: flex;
  gap: 18px;
  align-items: center;
}

.symbol-title h2 {
  margin: 0;
  color: #263447;
  font-size: 22px;
  font-weight: 900;
}

.symbol-title span {
  color: #8a98a8;
  font-size: 14px;
  font-weight: 700;
}

.symbol-tabs {
  display: flex;
  gap: 10px;
  padding: 6px;
  background: #f0f4f8;
  border-radius: 10px;
}

.symbol-tab {
  min-width: 112px;
  height: 42px;
  color: #344154;
  font-size: 16px;
  font-weight: 900;
  cursor: pointer;
  background: transparent;
  border: 0;
  border-radius: 8px;
}

.symbol-tab.active {
  color: #ffffff;
  background: #08aaa8;
  box-shadow: 0 4px 10px rgba(8, 170, 168, 0.24);
}

.symbol-body {
  display: grid;
  grid-template-columns: 164px minmax(0, 1fr);
  gap: 20px;
  height: 634px;
  padding: 22px 28px 28px;
}

.symbol-category-pane {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 18px 14px;
  background: #ffffff;
  border: 1px solid #e5ebf2;
  border-radius: 10px;
  box-shadow: 0 6px 16px rgba(38, 52, 71, 0.05);
}

.category-item {
  height: 42px;
  padding: 0 18px;
  color: #42506a;
  font-size: 16px;
  font-weight: 900;
  text-align: left;
  cursor: pointer;
  background: transparent;
  border: 0;
  border-radius: 8px;
}

.category-item.active {
  color: #ffffff;
  background: linear-gradient(135deg, #21c6c6, #00a3a3);
}

.symbol-content {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  grid-auto-rows: auto;
  align-content: start;
  align-items: stretch;
  gap: 14px;
  min-width: 0;
  padding: 18px;
  overflow: auto;
  background: #ffffff;
  border: 1px solid #e5ebf2;
  border-radius: 10px;
  box-shadow: 0 6px 16px rgba(38, 52, 71, 0.05);
}

.formula-card {
  display: flex;
  flex-direction: column;
  gap: 12px;
  justify-content: flex-start;
  min-height: 112px;
  height: fit-content;
  padding: 16px 18px;
  overflow: visible;
  color: #1f2b3d;
  cursor: pointer;
  background: #ffffff;
  border: 1px solid #e7edf4;
  border-radius: 10px;
}

.formula-card:hover {
  border-color: #4ecac8;
  box-shadow: 0 8px 18px rgba(34, 174, 172, 0.12);
}

.formula-card:focus-visible {
  outline: 2px solid #4ecac8;
  outline-offset: 2px;
}

.formula-preview-text {
  display: block;
  width: 100%;
  min-height: 0;
  padding: 4px 0 2px;
  font-size: 18px;
  line-height: 1.8;
  text-align: center;
}

.formula-preview-text :deep(.math-block) {
  display: block;
  margin: 0;
  overflow-x: auto;
  overflow-y: visible;
}

.formula-preview-text :deep(.katex-display) {
  display: block;
  margin: 0;
  overflow: visible;
}

.formula-preview-text :deep(.katex) {
  max-width: 100%;
  white-space: normal;
}

.formula-card-label {
  display: block;
  color: #8390a3;
  font-size: 12px;
  font-weight: 800;
  text-align: center;
}
</style>

<style>
.formula-symbol-modal .ant-modal-content {
  padding: 0;
  overflow: hidden;
  border-radius: 10px;
}
</style>
