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
      </a-descriptions>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive } from 'vue'
import { message } from 'ant-design-vue'
import { open } from '@tauri-apps/plugin-dialog'
import type { AppSettings } from '../api/native'
import { getAppSettings, initDataLibraryDir } from '../api/native'

const settings = reactive<AppSettings>({})
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
</script>

<style scoped>
.settings-panel {
  padding: 18px;
}
</style>
