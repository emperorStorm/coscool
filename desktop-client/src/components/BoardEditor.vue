<template>
  <div class="board-editor">
    <div class="board-toolbar">
      <a-segmented v-model:value="tool" :options="toolOptions" />
      <a-button size="small" @click="undo">撤销</a-button>
      <a-button size="small" @click="clear">清空</a-button>
      <a-button size="small" type="primary" @click="emitSave">保存批注板</a-button>
    </div>
    <canvas
      ref="canvasRef"
      width="760"
      height="360"
      class="board-canvas"
      @mousedown="startDraw"
      @mousemove="drawing"
      @mouseup="endDraw"
      @mouseleave="endDraw"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'

interface Stroke {
  tool: string
  color: string
  points: Array<{ x: number; y: number }>
  text?: string
}

const emit = defineEmits<{
  save: [payload: { json: string; previewDataUrl: string }]
}>()

const canvasRef = ref<HTMLCanvasElement>()
const tool = ref('pen')
const drawingNow = ref(false)
const strokes = ref<Stroke[]>([])
const activeStroke = ref<Stroke | null>(null)
const toolOptions = [
  { label: '画笔', value: 'pen' },
  { label: '矩形', value: 'rect' },
  { label: '箭头', value: 'arrow' },
  { label: '文字', value: 'text' }
]

onMounted(render)

function getPoint(event: MouseEvent) {
  const canvas = canvasRef.value
  if (!canvas) return { x: 0, y: 0 }
  const rect = canvas.getBoundingClientRect()
  return {
    x: (event.clientX - rect.left) * (canvas.width / rect.width),
    y: (event.clientY - rect.top) * (canvas.height / rect.height)
  }
}

function startDraw(event: MouseEvent) {
  const point = getPoint(event)
  if (tool.value === 'text') {
    const text = window.prompt('请输入标注文字')
    if (!text) return
    strokes.value.push({ tool: 'text', color: '#0f8f83', points: [point], text })
    render()
    return
  }
  drawingNow.value = true
  activeStroke.value = { tool: tool.value, color: '#0f8f83', points: [point] }
}

function drawing(event: MouseEvent) {
  if (!drawingNow.value || !activeStroke.value) return
  const point = getPoint(event)
  if (activeStroke.value.tool === 'pen') {
    activeStroke.value.points.push(point)
  } else {
    activeStroke.value.points = [activeStroke.value.points[0], point]
  }
  render()
  drawStroke(activeStroke.value)
}

function endDraw() {
  if (!drawingNow.value || !activeStroke.value) return
  strokes.value.push(activeStroke.value)
  activeStroke.value = null
  drawingNow.value = false
  render()
}

function undo() {
  strokes.value.pop()
  render()
}

function clear() {
  strokes.value = []
  render()
}

function emitSave() {
  const canvas = canvasRef.value
  if (!canvas) return
  emit('save', {
    json: JSON.stringify({ width: canvas.width, height: canvas.height, strokes: strokes.value }),
    previewDataUrl: canvas.toDataURL('image/png')
  })
}

function render() {
  const canvas = canvasRef.value
  if (!canvas) return
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  ctx.fillStyle = '#ffffff'
  ctx.fillRect(0, 0, canvas.width, canvas.height)
  ctx.strokeStyle = '#dce7ef'
  ctx.setLineDash([8, 6])
  ctx.strokeRect(1, 1, canvas.width - 2, canvas.height - 2)
  ctx.setLineDash([])
  strokes.value.forEach(drawStroke)
}

function drawStroke(stroke: Stroke) {
  const canvas = canvasRef.value
  const ctx = canvas?.getContext('2d')
  if (!ctx || stroke.points.length === 0) return
  ctx.strokeStyle = stroke.color
  ctx.fillStyle = stroke.color
  ctx.lineWidth = 3
  ctx.lineCap = 'round'
  ctx.lineJoin = 'round'
  if (stroke.tool === 'text') {
    const point = stroke.points[0]
    ctx.font = '18px sans-serif'
    ctx.fillText(stroke.text || '', point.x, point.y)
    return
  }
  if (stroke.tool === 'rect') {
    const [start, end] = stroke.points
    if (!end) return
    ctx.strokeRect(start.x, start.y, end.x - start.x, end.y - start.y)
    return
  }
  if (stroke.tool === 'arrow') {
    const [start, end] = stroke.points
    if (!end) return
    drawArrow(ctx, start.x, start.y, end.x, end.y)
    return
  }
  ctx.beginPath()
  stroke.points.forEach((point, index) => {
    if (index === 0) ctx.moveTo(point.x, point.y)
    else ctx.lineTo(point.x, point.y)
  })
  ctx.stroke()
}

function drawArrow(ctx: CanvasRenderingContext2D, fromX: number, fromY: number, toX: number, toY: number) {
  const angle = Math.atan2(toY - fromY, toX - fromX)
  const headLength = 12
  ctx.beginPath()
  ctx.moveTo(fromX, fromY)
  ctx.lineTo(toX, toY)
  ctx.lineTo(toX - headLength * Math.cos(angle - Math.PI / 6), toY - headLength * Math.sin(angle - Math.PI / 6))
  ctx.moveTo(toX, toY)
  ctx.lineTo(toX - headLength * Math.cos(angle + Math.PI / 6), toY - headLength * Math.sin(angle + Math.PI / 6))
  ctx.stroke()
}
</script>

<style scoped>
.board-editor {
  display: grid;
  gap: 10px;
}

.board-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}

.board-canvas {
  width: 100%;
  max-width: 760px;
  aspect-ratio: 19 / 9;
  background: #ffffff;
  border: 1px solid #e2e9f0;
  border-radius: 6px;
}
</style>
