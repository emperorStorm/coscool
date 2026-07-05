<template>
  <div class="page-shell">
    <div class="toolbar">
      <h2 class="page-title">系统设置</h2>
      <a-button type="primary" @click="chooseLibrary">选择数据仓库</a-button>
    </div>
    <section class="work-panel settings-panel">
      <a-descriptions bordered :column="1" size="middle">
        <a-descriptions-item label="数据仓库目录">
          <span v-if="settings.dataLibraryPath">{{ settings.dataLibraryPath }}</span>
          <span v-else class="muted">尚未选择</span>
        </a-descriptions-item>
        <a-descriptions-item label="当前登录账号">
          {{ settings.currentTeacherAccount || currentTeacher || '未记录' }}
        </a-descriptions-item>
        <a-descriptions-item label="资源存储">
          SQLite 存结构化数据，图片、批注板和导出文件存用户指定目录。
        </a-descriptions-item>
        <a-descriptions-item label="当前版本">
          <span>{{ appVersion || '读取中' }}</span>
        </a-descriptions-item>
        <a-descriptions-item label="软件更新">
          <div class="update-row">
            <div>
              <div>{{ updateStatus }}</div>
              <a-progress
                v-if="installing"
                class="update-progress"
                :percent="updateProgress"
                :show-info="false"
                size="small"
              />
            </div>
            <a-button
              type="primary"
              :loading="checking || installing"
              :disabled="installing"
              @click="checkUpdate"
            >
              检查更新
            </a-button>
          </div>
        </a-descriptions-item>
      </a-descriptions>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, h, onMounted, reactive, ref } from 'vue'
import { message, Modal } from 'ant-design-vue'
import { open } from '@tauri-apps/plugin-dialog'
import type { AppSettings, UpdateCheckResult } from '../api/native'
import {
  checkAppUpdate,
  formatUpdateError,
  getAppSettings,
  getCurrentVersion,
  initDataLibraryDir,
  installAppUpdate
} from '../api/native'

const settings = reactive<AppSettings>({})
const appVersion = ref('')
const checking = ref(false)
const installing = ref(false)
const updateProgress = ref(0)
const updateStatus = ref('可手动检查 GitHub Releases 中的新版本')
const currentTeacher = computed(() => {
  const raw = localStorage.getItem('coscool_teacher')
  if (!raw) return ''
  try {
    return JSON.parse(raw).account || ''
  } catch {
    return ''
  }
})

onMounted(load)

async function load() {
  try {
    appVersion.value = await getCurrentVersion()
    Object.assign(settings, await getAppSettings())
  } catch (error) {
    message.error(String(error))
  }
}

async function chooseLibrary() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择课思库基础数据仓库目录'
    })
    const path = Array.isArray(selected) ? selected[0] : selected
    if (!path) return
    Object.assign(settings, await initDataLibraryDir(path))
    message.success('数据仓库已设置')
  } catch (error) {
    message.error(String(error))
  }
}

async function checkUpdate() {
  if (checking.value || installing.value) return
  checking.value = true
  updateStatus.value = '正在检查新版本'
  try {
    const result = await checkAppUpdate()
    appVersion.value = result.currentVersion
    if (!result.update) {
      updateStatus.value = '已是最新版本'
      message.success('已是最新版本')
      return
    }
    updateStatus.value = `发现新版本 ${result.update.version}`
    Modal.confirm({
      title: '发现新版本',
      content: h('div', { class: 'update-confirm' }, [
        h('p', `当前版本：${result.currentVersion}`),
        h('p', `最新版本：${result.update.version}`),
        result.update.body
          ? h('div', [
              h('p', '更新内容：'),
              h('pre', { class: 'update-body' }, result.update.body)
            ])
          : null
      ]),
      okText: '立即更新',
      cancelText: '暂不更新',
      onOk: () => installUpdate(result.update!)
    })
  } catch (error) {
    updateStatus.value = '检查更新失败'
    message.error(formatUpdateError(error))
  } finally {
    checking.value = false
  }
}

async function installUpdate(update: NonNullable<UpdateCheckResult['update']>) {
  installing.value = true
  updateProgress.value = 0
  updateStatus.value = '正在下载更新'
  let downloadedBytes = 0
  let totalBytes = 0
  try {
    await installAppUpdate(update, event => {
      if (event.event === 'Started') {
        downloadedBytes = 0
        totalBytes = event.data.contentLength || 0
        updateProgress.value = 0
      }
      if (event.event === 'Progress') {
        downloadedBytes += event.data.chunkLength
        updateProgress.value = totalBytes
          ? Math.min(Math.round((downloadedBytes / totalBytes) * 100), 99)
          : Math.max(updateProgress.value, 1)
      }
      if (event.event === 'Finished') {
        updateProgress.value = 100
        updateStatus.value = '更新安装完成，正在重启'
      }
    })
  } catch (error) {
    installing.value = false
    updateStatus.value = downloadedBytes > 0 ? '更新安装失败' : '更新下载失败'
    message.error(formatUpdateError(error))
  }
}
</script>

<style scoped>
.settings-panel {
  padding: 18px;
}

.update-row {
  align-items: center;
  display: flex;
  gap: 16px;
  justify-content: space-between;
}

.update-progress {
  margin-top: 8px;
  max-width: 240px;
}

:global(.update-body) {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
}
</style>
