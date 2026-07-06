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
  transition: opacity 0.18s ease;
}

.tool-dock:hover::before,
.tool-dock:focus-within::before {
  opacity: 1;
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
  transition:
    transform 0.2s ease,
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

.tool-item {
  top: 47px;
  left: 47px;
  width: 44px;
  height: 44px;
  padding: 0 6px;
  font-size: 12px;
  line-height: 1.2;
  opacity: 0;
  pointer-events: none;
}

.tool-dock:hover .tool-item,
.tool-dock:focus-within .tool-item {
  opacity: 1;
  pointer-events: auto;
}

.tool-item:hover {
  box-shadow: 0 12px 30px rgba(4, 117, 113, 0.24);
}

.tool-item.task {
  transform: translate(-34px, -50px);
}

.tool-item.ai {
  transform: translate(48px, -50px);
}

.tool-item.symbol {
  transform: translate(-62px, 15px);
}

.tool-item.command {
  transform: translate(67px, 15px);
}
</style>
