<template>
  <div class="tool-dock">
    <button class="tool-button main-tool" type="button" title="工具">
      <MoreVertical :size="22" :stroke-width="3" />
    </button>
    <button
      v-for="item in tools"
      :key="item.key"
      type="button"
      :class="['tool-button', 'tool-item', item.key]"
      :title="item.enabled ? item.label : `${item.label}暂未开放`"
      @click="handleTool(item)"
    >
      {{ item.label }}
    </button>
    <FormulaSymbolModal v-model:open="symbolOpen" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { message } from 'ant-design-vue'
import { MoreVertical } from 'lucide-vue-next'
import FormulaSymbolModal from './FormulaSymbolModal.vue'

interface ToolItem {
  key: string
  label: string
  enabled: boolean
}

const symbolOpen = ref(false)
const tools: ToolItem[] = [
  { key: 'task', label: '任务', enabled: false },
  { key: 'ai', label: 'AI工具', enabled: false },
  { key: 'symbol', label: '符号', enabled: true },
  { key: 'command', label: '命令说明', enabled: false }
]

function handleTool(item: ToolItem) {
  if (!item.enabled) {
    message.info('暂未开放')
    return
  }
  if (item.key === 'symbol') {
    symbolOpen.value = true
  }
}
</script>

<style scoped>
.tool-dock {
  position: fixed;
  bottom: 32px;
  left: 38px;
  z-index: 40;
  width: 138px;
  height: 138px;
}

.tool-dock::before {
  position: absolute;
  inset: 7px;
  content: "";
  pointer-events: none;
  border: 7px solid rgba(36, 45, 58, 0.08);
  border-radius: 50%;
  opacity: 0;
  transform: scale(0.86);
  transition:
    opacity 0.18s ease,
    transform 0.24s cubic-bezier(0.2, 0.8, 0.2, 1);
}

.tool-dock:hover::before,
.tool-dock:focus-within::before {
  opacity: 1;
  transform: scale(1);
}

.tool-button {
  position: absolute;
  display: grid;
  place-items: center;
  color: #007f7e;
  font-weight: 900;
  cursor: pointer;
  background: radial-gradient(circle at 35% 30%, #efffff, #c9f2f0);
  border: 1px solid #a8e3df;
  border-radius: 50%;
  box-shadow: 0 10px 28px rgba(4, 117, 113, 0.16);
  outline: none;
  transition:
    color 0.18s ease,
    background 0.18s ease,
    border-color 0.18s ease,
    transform 0.2s cubic-bezier(0.2, 0.8, 0.2, 1),
    opacity 0.18s ease,
    box-shadow 0.18s ease;
}

.main-tool {
  top: 45px;
  left: 45px;
  z-index: 3;
  width: 48px;
  height: 48px;
  color: #ffffff;
  background: linear-gradient(145deg, #25c8c6, #008f8f);
  border: 4px solid rgba(255, 255, 255, 0.8);
}

.main-tool:hover,
.main-tool:focus-visible {
  background: linear-gradient(145deg, #39dddd, #008382);
  box-shadow:
    0 14px 34px rgba(0, 132, 130, 0.3),
    0 0 0 5px rgba(37, 200, 198, 0.18);
  transform: translateY(-2px) scale(1.04);
}

.tool-item {
  --x: 0;
  --y: 0;
  --delay: 0ms;

  top: 47px;
  left: 47px;
  width: 44px;
  height: 44px;
  padding: 0 6px;
  font-size: 12px;
  line-height: 1.2;
  opacity: 0;
  pointer-events: none;
  transform: translate(0, 0) scale(0.55);
  transition-duration: 0.16s;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
}

.tool-dock:hover .tool-item,
.tool-dock:focus-within .tool-item {
  opacity: 1;
  pointer-events: auto;
  transform: translate(var(--x), var(--y)) scale(1);
  transition-delay: var(--delay);
  transition-duration: 0.28s;
  transition-timing-function: cubic-bezier(0.18, 0.9, 0.28, 1.18);
}

.tool-item:hover,
.tool-item:focus-visible {
  color: #006d6c;
  background: radial-gradient(circle at 35% 28%, #ffffff, #b7ece8);
  border-color: #78d8d1;
  box-shadow:
    0 12px 30px rgba(4, 117, 113, 0.24),
    0 0 0 4px rgba(0, 143, 143, 0.1);
}

.tool-item.task {
  --x: -34px;
  --y: -50px;
  --delay: 35ms;
}

.tool-item.ai {
  --x: 48px;
  --y: -50px;
  --delay: 70ms;
}

.tool-item.symbol {
  --x: -62px;
  --y: 15px;
  --delay: 105ms;
}

.tool-item.command {
  --x: 67px;
  --y: 15px;
  --delay: 140ms;
}
</style>
