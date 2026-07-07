<template>
  <div class="paper-export-page">
    <header class="export-toolbar">
      <div class="toolbar-title">
        <button class="back-button" type="button" title="返回" @click="goBack">
          <ChevronLeft :size="19" :stroke-width="2.4" />
        </button>
        <div>
          <strong>课思库试卷导出</strong>
          <span>{{ paperDetail?.paper.title || '未命名试卷' }}</span>
        </div>
      </div>

      <div class="toolbar-actions">
        <button
          v-for="item in fieldButtons"
          :key="item.key"
          class="toggle-button"
          :class="{ active: visibleFields[item.key] }"
          type="button"
          @click="visibleFields[item.key] = !visibleFields[item.key]"
        >
          {{ item.label }}
        </button>
        <button
          class="toggle-button screen-button"
          :class="{ active: fullScreen }"
          type="button"
          @click="fullScreen = !fullScreen"
        >
          占满屏幕
        </button>
        <button class="export-button pdf-button" type="button" @click="pdfDialogOpen = true">
          导出PDF
        </button>
      </div>
    </header>

    <main class="export-workspace" :class="{ 'full-layout': fullScreen }">
      <a-spin :spinning="loading">
        <PaperPreview
          v-if="paperDetail"
          ref="exportPaperRef"
          class="export-paper"
          :class="{ 'paper-wide': fullScreen }"
          :title="paperDetail.paper.title"
          :questions="paperDetail.questions"
          :image-map="questionImageMap"
          :show-answer="visibleFields.answer"
          :show-analysis="visibleFields.analysis"
          :show-knowledge-points="visibleFields.knowledgePoints"
          :show-tags="visibleFields.tags"
        />
        <a-empty v-else-if="!loading" description="未找到试卷" class="export-empty" />
      </a-spin>
    </main>

    <a-modal v-model:open="pdfDialogOpen" width="1060px" :footer="null" centered>
      <template #title>
        <div class="modal-title">导出配置</div>
      </template>

      <div class="pdf-config">
        <section class="config-panel">
          <h3>页眉设置</h3>
          <div class="config-divider" />

          <label class="config-label">
            <span>左侧内容</span>
            <a-switch v-model:checked="pdfConfig.headerLeftEnabled" checked-children="启用" un-checked-children="禁用" />
          </label>
          <a-input v-model:value="pdfConfig.headerLeft" :disabled="!pdfConfig.headerLeftEnabled" placeholder="请输入左侧页眉内容" />

          <label class="config-label">
            <span>中间内容</span>
            <a-switch v-model:checked="pdfConfig.headerCenterEnabled" checked-children="启用" un-checked-children="禁用" />
          </label>
          <a-input v-model:value="pdfConfig.headerCenter" :disabled="!pdfConfig.headerCenterEnabled" placeholder="请输入中间页眉内容" />

          <label class="config-label">
            <span>右侧内容</span>
            <a-switch v-model:checked="pdfConfig.headerRightEnabled" checked-children="启用" un-checked-children="禁用" />
          </label>
          <a-input v-model:value="pdfConfig.headerRight" :disabled="!pdfConfig.headerRightEnabled" placeholder="请输入右侧页眉内容" />
        </section>

        <section class="config-panel">
          <h3>水印设置</h3>
          <div class="config-divider" />

          <label class="config-label">
            <span>文本</span>
            <a-switch v-model:checked="pdfConfig.watermarkTextEnabled" checked-children="开启" un-checked-children="关闭" />
          </label>
          <a-input v-model:value="pdfConfig.watermarkText" :disabled="!pdfConfig.watermarkTextEnabled" placeholder="请输入水印文本" />

          <label class="config-label">
            <span>图片</span>
            <a-switch v-model:checked="pdfConfig.watermarkImageEnabled" checked-children="开启" un-checked-children="关闭" />
          </label>
          <a-input v-model:value="pdfConfig.watermarkImage" :disabled="!pdfConfig.watermarkImageEnabled" placeholder="请输入图片链接或 dataURL" />

          <div class="watermark-grid">
            <label>
              <span>图片宽度 (%)</span>
              <a-input-number v-model:value="pdfConfig.watermarkImageWidth" :min="10" :max="100" :disabled="!pdfConfig.watermarkImageEnabled" />
            </label>
            <label>
              <span>旋转</span>
              <a-input-number v-model:value="pdfConfig.watermarkRotate" :min="-90" :max="90" />
            </label>
            <label>
              <span>透明</span>
              <a-input-number v-model:value="pdfConfig.watermarkOpacity" :min="0.1" :max="1" :step="0.1" />
            </label>
          </div>
        </section>

        <section class="config-panel answer-page-panel">
          <h3>答案部分分页设置</h3>
          <div class="config-divider" />
          <label class="config-label compact">
            <span>
              <strong>允许分页</strong>
              <em>答案、解析等附加内容可以跨页显示，避免大片空白</em>
            </span>
            <a-switch v-model:checked="pdfConfig.allowAnswerPageBreak" checked-children="允许" un-checked-children="禁用" />
          </label>
        </section>
      </div>

      <div class="modal-actions">
        <a-button @click="pdfDialogOpen = false">取消</a-button>
        <a-button type="primary" :loading="exportingPdf" @click="exportPdf">导出</a-button>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, reactive, ref } from 'vue'
import { message } from 'ant-design-vue'
import { save as saveFile } from '@tauri-apps/plugin-dialog'
import { useRoute, useRouter } from 'vue-router'
import html2canvas from 'html2canvas'
import { jsPDF } from 'jspdf'
import { ChevronLeft } from 'lucide-vue-next'
import PaperPreview from '../components/PaperPreview.vue'
import type { PaperDetail, Question } from '../api/native'
import { getPaper, readAssetDataUrl, saveExportFile } from '../api/native'

type FieldKey = 'answer' | 'analysis' | 'knowledgePoints' | 'tags'

const route = useRoute()
const router = useRouter()
const loading = ref(false)
const exportingPdf = ref(false)
const fullScreen = ref(false)
const pdfDialogOpen = ref(false)
const exportPaperRef = ref<InstanceType<typeof PaperPreview>>()
const paperDetail = ref<PaperDetail>()
const questionImageMap = ref<Record<number, string>>({})
const visibleFields = reactive<Record<FieldKey, boolean>>({
  answer: false,
  analysis: false,
  knowledgePoints: false,
  tags: false
})
const pdfConfig = reactive({
  headerLeftEnabled: true,
  headerLeft: '课思库 试卷',
  headerCenterEnabled: false,
  headerCenter: '',
  headerRightEnabled: false,
  headerRight: '',
  watermarkTextEnabled: false,
  watermarkText: '',
  watermarkImageEnabled: false,
  watermarkImage: '',
  watermarkImageWidth: 60,
  watermarkRotate: 0,
  watermarkOpacity: 0.5,
  allowAnswerPageBreak: true
})
const fieldButtons: { key: FieldKey; label: string }[] = [
  { key: 'answer', label: '答案' },
  { key: 'analysis', label: '解析' },
  { key: 'knowledgePoints', label: '知识点' },
  { key: 'tags', label: '标签' }
]
const paperId = computed(() => Number(Array.isArray(route.params.id) ? route.params.id[0] : route.params.id) || 0)

onMounted(loadPaper)

async function loadPaper() {
  if (!paperId.value) {
    message.warning('试卷编号不存在')
    return
  }
  loading.value = true
  try {
    paperDetail.value = await getPaper(paperId.value)
    await loadQuestionImages(paperDetail.value.questions)
  } catch (error) {
    message.error(String(error))
  } finally {
    loading.value = false
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

function goBack() {
  router.replace('/app/questions/papers')
}

async function exportPdf() {
  exportingPdf.value = true
  try {
    const targetPath = await selectExportPath()
    if (!targetPath) return
    const canvas = await renderPaperCanvas()
    const watermarkImage = pdfConfig.watermarkImageEnabled && pdfConfig.watermarkImage
      ? await loadImageDataUrl(pdfConfig.watermarkImage)
      : ''
    const pdf = new jsPDF({ orientation: 'portrait', unit: 'mm', format: 'a4' })
    const pageWidth = pdf.internal.pageSize.getWidth()
    const pageHeight = pdf.internal.pageSize.getHeight()
    const marginX = 10
    const marginBottom = 10
    const headerHeight = hasHeader() ? 18 : 10
    const contentWidth = fullScreen.value ? pageWidth - marginX * 2 : Math.min(pageWidth - marginX * 2, 190)
    const contentX = (pageWidth - contentWidth) / 2
    const contentHeight = pageHeight - headerHeight - marginBottom

    if (!pdfConfig.allowAnswerPageBreak) {
      drawPdfDecorations(pdf, pageWidth, pageHeight, watermarkImage)
      const singlePageWidth = Math.min(contentWidth, contentHeight * canvas.width / canvas.height)
      const singlePageHeight = canvas.height * singlePageWidth / canvas.width
      pdf.addImage(
        canvas.toDataURL('image/jpeg', 0.95),
        'JPEG',
        (pageWidth - singlePageWidth) / 2,
        headerHeight,
        singlePageWidth,
        singlePageHeight
      )
      const savedPath = await saveExportFile(targetPath, pdf.output('datauristring'))
      pdfDialogOpen.value = false
      message.success(`PDF 已保存：${savedPath}`)
      return
    }

    const sliceHeight = Math.floor(contentHeight * canvas.width / contentWidth)
    let sourceY = 0
    let pageIndex = 0
    while (sourceY < canvas.height) {
      if (pageIndex > 0) pdf.addPage()
      drawPdfDecorations(pdf, pageWidth, pageHeight, watermarkImage)

      const currentSliceHeight = Math.min(sliceHeight, canvas.height - sourceY)
      const pageCanvas = document.createElement('canvas')
      pageCanvas.width = canvas.width
      pageCanvas.height = currentSliceHeight
      const context = pageCanvas.getContext('2d')
      if (!context) throw new Error('无法创建 PDF 画布')
      context.drawImage(canvas, 0, sourceY, canvas.width, currentSliceHeight, 0, 0, canvas.width, currentSliceHeight)

      const imageHeight = currentSliceHeight * contentWidth / canvas.width
      pdf.addImage(pageCanvas.toDataURL('image/jpeg', 0.95), 'JPEG', contentX, headerHeight, contentWidth, imageHeight)
      sourceY += currentSliceHeight
      pageIndex += 1
    }

    const savedPath = await saveExportFile(targetPath, pdf.output('datauristring'))
    pdfDialogOpen.value = false
    message.success(`PDF 已保存：${savedPath}`)
  } catch (error) {
    message.error(`导出PDF失败：${String(error)}`)
  } finally {
    exportingPdf.value = false
  }
}

async function renderPaperCanvas() {
  await nextTick()
  const target = exportPaperRef.value?.$el as HTMLElement | undefined
  if (!target) throw new Error('导出区域不存在')
  return html2canvas(target, {
    backgroundColor: '#ffffff',
    scale: 2,
    useCORS: true,
    logging: false
  })
}

function drawPdfDecorations(pdf: jsPDF, pageWidth: number, pageHeight: number, watermarkImage: string) {
  pdf.setFont('helvetica', 'normal')
  pdf.setFontSize(9)
  pdf.setTextColor(67, 80, 98)
  if (pdfConfig.headerLeftEnabled && pdfConfig.headerLeft) {
    pdf.text(pdfConfig.headerLeft, 12, 10)
  }
  if (pdfConfig.headerCenterEnabled && pdfConfig.headerCenter) {
    pdf.text(pdfConfig.headerCenter, pageWidth / 2, 10, { align: 'center' })
  }
  if (pdfConfig.headerRightEnabled && pdfConfig.headerRight) {
    pdf.text(pdfConfig.headerRight, pageWidth - 12, 10, { align: 'right' })
  }

  if (pdfConfig.watermarkTextEnabled && pdfConfig.watermarkText) {
    setPdfOpacity(pdf, pdfConfig.watermarkOpacity)
    pdf.setFontSize(36)
    pdf.setTextColor(198, 207, 219)
    pdf.text(pdfConfig.watermarkText, pageWidth / 2, pageHeight / 2, {
      align: 'center',
      angle: pdfConfig.watermarkRotate
    })
    setPdfOpacity(pdf, 1)
  }

  if (watermarkImage) {
    setPdfOpacity(pdf, pdfConfig.watermarkOpacity)
    const width = pageWidth * pdfConfig.watermarkImageWidth / 100
    const height = width * 0.5
    pdf.addImage(watermarkImage, getImageFormat(watermarkImage), (pageWidth - width) / 2, (pageHeight - height) / 2, width, height, undefined, 'FAST', pdfConfig.watermarkRotate)
    setPdfOpacity(pdf, 1)
  }
}

function setPdfOpacity(pdf: jsPDF, opacity: number) {
  const pdfWithState = pdf as jsPDF & {
    GState?: new (options: { opacity: number }) => unknown
    setGState?: (state: unknown) => void
  }
  if (!pdfWithState.GState || !pdfWithState.setGState) return
  pdfWithState.setGState(new pdfWithState.GState({ opacity }))
}

function hasHeader() {
  return Boolean(
    (pdfConfig.headerLeftEnabled && pdfConfig.headerLeft)
    || (pdfConfig.headerCenterEnabled && pdfConfig.headerCenter)
    || (pdfConfig.headerRightEnabled && pdfConfig.headerRight)
  )
}

async function selectExportPath() {
  const selectedPath = await saveFile({
    title: '保存导出PDF',
    defaultPath: `paper-${paperId.value}-${formatTimestamp()}.pdf`,
    filters: [{ name: 'PDF 文件', extensions: ['pdf'] }]
  })
  return selectedPath ? ensureExtension(selectedPath, 'pdf') : ''
}

function ensureExtension(path: string, extension: string) {
  return path.toLowerCase().endsWith(`.${extension}`) ? path : `${path}.${extension}`
}

async function loadImageDataUrl(source: string) {
  if (source.startsWith('data:image/')) return source
  const response = await fetch(source)
  if (!response.ok) throw new Error('水印图片无法读取')
  const blob = await response.blob()
  return new Promise<string>((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => resolve(String(reader.result))
    reader.onerror = () => reject(new Error('水印图片转换失败'))
    reader.readAsDataURL(blob)
  })
}

function getImageFormat(dataUrl: string) {
  if (dataUrl.includes('image/jpeg') || dataUrl.includes('image/jpg')) return 'JPEG'
  if (dataUrl.includes('image/webp')) return 'WEBP'
  return 'PNG'
}

function formatTimestamp() {
  const date = new Date()
  const pad = (value: number) => String(value).padStart(2, '0')
  return `${date.getFullYear()}${pad(date.getMonth() + 1)}${pad(date.getDate())}${pad(date.getHours())}${pad(date.getMinutes())}${pad(date.getSeconds())}`
}
</script>

<style scoped>
.paper-export-page {
  display: grid;
  grid-template-rows: 64px minmax(0, 1fr);
  min-height: 100vh;
  color: #253142;
  background: #edf1f4;
}

.export-toolbar {
  position: sticky;
  top: 0;
  z-index: 20;
  display: flex;
  gap: 18px;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  background: rgba(255, 255, 255, 0.96);
  border-bottom: 1px solid #dde6ef;
  box-shadow: 0 10px 24px rgba(35, 49, 68, 0.08);
}

.toolbar-title {
  display: flex;
  gap: 12px;
  align-items: center;
  min-width: 220px;
}

.toolbar-title strong,
.toolbar-title span {
  display: block;
  max-width: 320px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.toolbar-title strong {
  color: #1f2b3a;
  font-size: 17px;
  font-weight: 900;
}

.toolbar-title span {
  margin-top: 3px;
  color: #7a8797;
  font-size: 12px;
}

.back-button {
  display: grid;
  width: 34px;
  height: 34px;
  color: #607085;
  cursor: pointer;
  place-items: center;
  background: #f7f9fb;
  border: 1px solid #dbe4ee;
  border-radius: 8px;
}

.back-button:hover {
  color: #0f9187;
  background: #effaf8;
  border-color: #bfe8e3;
}

.toolbar-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  align-items: center;
  justify-content: flex-end;
}

.toggle-button,
.export-button {
  min-width: 72px;
  height: 34px;
  padding: 0 14px;
  color: #556272;
  font-size: 14px;
  font-weight: 800;
  cursor: pointer;
  background: #eef2f6;
  border: 1px solid #d9e2ec;
  border-radius: 8px;
}

.toggle-button.active {
  color: #ffffff;
  background: #4b6fd8;
  border-color: #4b6fd8;
  box-shadow: 0 5px 12px rgba(75, 111, 216, 0.24);
}

.screen-button.active {
  background: #73808c;
  border-color: #73808c;
}

.export-button {
  color: #ffffff;
  border: 0;
}

.pdf-button {
  background: #f36d99;
  box-shadow: 0 5px 12px rgba(243, 109, 153, 0.22);
}

.export-workspace {
  min-height: 0;
  padding: 32px 20px 56px;
  overflow: auto;
}

.export-workspace :deep(.ant-spin-nested-loading),
.export-workspace :deep(.ant-spin-container) {
  min-height: 100%;
}

.export-paper {
  width: min(800px, calc(100vw - 72px));
  min-height: calc(100vh - 150px);
}

.full-layout {
  padding-right: 28px;
  padding-left: 28px;
}

.paper-wide {
  width: min(1280px, calc(100vw - 56px));
}

.export-empty {
  padding-top: 120px;
}

.modal-title {
  color: #223047;
  font-size: 18px;
  font-weight: 900;
}

.pdf-config {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 18px;
  padding: 8px 8px 0;
}

.config-panel {
  padding: 22px 24px;
  border: 1px solid #e4e9ef;
  border-radius: 8px;
}

.config-panel h3 {
  margin: 0;
  color: #0f9a9b;
  font-size: 17px;
  font-weight: 900;
}

.config-divider {
  height: 1px;
  margin: 14px 0 18px;
  border-top: 1px dashed #d8e0e8;
}

.config-label {
  display: flex;
  gap: 12px;
  align-items: center;
  justify-content: space-between;
  margin: 18px 0 8px;
  color: #303946;
  font-size: 15px;
  font-weight: 900;
}

.config-label:first-of-type {
  margin-top: 0;
}

.config-label.compact {
  margin: 0;
}

.config-label em {
  display: block;
  margin-top: 6px;
  color: #8a96a5;
  font-size: 13px;
  font-style: normal;
  font-weight: 600;
}

.watermark-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 12px;
  margin-top: 18px;
}

.watermark-grid label {
  display: grid;
  gap: 8px;
  color: #303946;
  font-size: 14px;
  font-weight: 900;
}

.watermark-grid :deep(.ant-input-number) {
  width: 100%;
}

.answer-page-panel {
  grid-column: 1 / 2;
}

.modal-actions {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
  padding: 22px 8px 0;
}

@media (max-width: 1180px) {
  .paper-export-page {
    grid-template-rows: auto minmax(0, 1fr);
  }

  .export-toolbar {
    position: relative;
    align-items: flex-start;
    flex-direction: column;
    padding: 14px 18px;
  }

  .toolbar-actions {
    justify-content: flex-start;
  }

  .pdf-config {
    grid-template-columns: 1fr;
  }

  .answer-page-panel {
    grid-column: auto;
  }
}

@media (max-width: 720px) {
  .export-workspace {
    padding: 18px 12px 36px;
  }

  .export-paper,
  .paper-wide {
    width: 100%;
  }

  .watermark-grid {
    grid-template-columns: 1fr;
  }
}
</style>
