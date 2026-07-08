<template>
  <a-popover
    v-model:open="popoverOpen"
    trigger="click"
    placement="bottomRight"
    overlay-class-name="notification-popover-overlay"
  >
    <template #content>
      <div class="notification-panel">
        <div class="notification-header">
          <div>
            <strong>通知</strong>
            <span class="notification-count">{{ unreadCount ? `${unreadCount} 条未读` : '全部已读' }}</span>
          </div>
          <a-button type="link" size="small" :disabled="!unreadCount" @click="markAllAsRead">全部已读</a-button>
        </div>

        <div v-if="notifications.length" class="notification-list">
          <button
            v-for="item in notifications"
            :key="item.id"
            type="button"
            class="notification-item"
            :class="{ unread: !item.read }"
            @click="openNotification(item)"
          >
            <span v-if="!item.read" class="unread-dot" />
            <span class="notification-icon">更</span>
            <span class="notification-content">
              <span class="notification-title">{{ item.title }}</span>
              <span class="notification-summary">{{ item.summary }}</span>
              <span class="notification-time">{{ formatTime(item.createdAt) }}</span>
            </span>
          </button>
        </div>

        <a-empty v-else class="notification-empty" description="暂无通知" :image="simpleEmptyImage" />
      </div>
    </template>

    <a-badge :count="unreadCount" :overflow-count="99" size="small">
      <a-button class="notification-trigger" type="text" aria-label="通知">
        <template #icon>
          <Bell class="notification-bell" :size="18" :stroke-width="2.2" />
        </template>
        <span class="sr-only">通知</span>
      </a-button>
    </a-badge>
  </a-popover>
</template>

<script setup lang="ts">
import { computed, h, onMounted, ref } from 'vue'
import { Empty, message, Modal } from 'ant-design-vue'
import { Bell } from 'lucide-vue-next'
import type { UpdateCheckResult } from '../api/native'
import {
  checkAppUpdate,
  formatUpdateError,
  getCurrentVersion,
  installAppUpdate,
  isTauriRuntime
} from '../api/native'

const STORAGE_KEY = 'coscool_notifications'
const UPDATE_NOTIFICATION_PREFIX = 'app-update:'

type AvailableUpdate = NonNullable<UpdateCheckResult['update']>

interface UpdateNotificationPayload {
  currentVersion: string
  latestVersion: string
  body: string
  checkedAt: string
  installed?: boolean
}

interface AppNotification {
  id: string
  type: 'app-update'
  title: string
  summary: string
  createdAt: string
  read: boolean
  updateInfo: UpdateNotificationPayload
}

const simpleEmptyImage = Empty.PRESENTED_IMAGE_SIMPLE
const popoverOpen = ref(false)
const notifications = ref<AppNotification[]>(loadNotifications())
const currentAppVersion = ref('')
const updateCache = new Map<string, AvailableUpdate>()
const unreadCount = computed(() => notifications.value.filter(item => !item.read).length)

onMounted(initUpdateNotifications)

async function initUpdateNotifications() {
  if (!isTauriRuntime()) return
  try {
    currentAppVersion.value = await getCurrentVersion()
    markInstalledNotifications(currentAppVersion.value)
  } catch {
    currentAppVersion.value = ''
  }
  await checkUpdateNotice()
}

async function checkUpdateNotice() {
  if (!isTauriRuntime()) return
  try {
    const result = await checkAppUpdate()
    currentAppVersion.value = result.currentVersion
    markInstalledNotifications(result.currentVersion)
    if (!result.update) return
    const id = `${UPDATE_NOTIFICATION_PREFIX}${result.update.version}`
    updateCache.set(id, result.update)
    upsertUpdateNotification(id, {
      currentVersion: result.currentVersion,
      latestVersion: result.update.version,
      body: result.update.body || '本次更新暂无详细说明。',
      checkedAt: new Date().toISOString()
    })
  } catch (error) {
    message.error(formatUpdateError(error))
  }
}

function upsertUpdateNotification(id: string, updateInfo: UpdateNotificationPayload) {
  const existing = notifications.value.find(item => item.id === id)
  const nextItem: AppNotification = {
    id,
    type: 'app-update',
    title: getNotificationTitle(updateInfo),
    summary: getNotificationSummary(updateInfo),
    createdAt: existing?.createdAt || updateInfo.checkedAt,
    read: existing?.read || false,
    updateInfo
  }

  notifications.value = [
    nextItem,
    ...notifications.value.filter(item => item.id !== id)
  ].sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
  saveNotifications()
}

function openNotification(item: AppNotification) {
  markAsRead(item.id)
  popoverOpen.value = false
  if (item.type === 'app-update') {
    openUpdateModal(item)
  }
}

function markAsRead(id: string) {
  let changed = false
  notifications.value = notifications.value.map(item => {
    if (item.id !== id || item.read) return item
    changed = true
    return { ...item, read: true }
  })
  if (changed) saveNotifications()
}

function markAllAsRead() {
  if (!unreadCount.value) return
  notifications.value = notifications.value.map(item => ({ ...item, read: true }))
  saveNotifications()
}

function markInstalledNotifications(version: string) {
  if (!version) return
  let changed = false
  notifications.value = notifications.value.map(item => {
    if (!isVersionInstalled(version, item.updateInfo.latestVersion) || item.updateInfo.installed) return item
    changed = true
    const updateInfo = { ...item.updateInfo, currentVersion: version, installed: true }
    return {
      ...item,
      title: getNotificationTitle(updateInfo),
      summary: getNotificationSummary(updateInfo),
      updateInfo
    }
  })
  if (changed) saveNotifications()
}

function loadNotifications(): AppNotification[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return []
    const parsed = JSON.parse(raw)
    if (!Array.isArray(parsed)) return []
    return parsed.filter(isNotification).sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
  } catch {
    return []
  }
}

function saveNotifications() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(notifications.value))
}

function isNotification(value: unknown): value is AppNotification {
  const item = value as Partial<AppNotification>
  return Boolean(
    item &&
    typeof item.id === 'string' &&
    item.type === 'app-update' &&
    typeof item.title === 'string' &&
    typeof item.summary === 'string' &&
    typeof item.createdAt === 'string' &&
    typeof item.read === 'boolean' &&
    item.updateInfo &&
    typeof item.updateInfo.currentVersion === 'string' &&
    typeof item.updateInfo.latestVersion === 'string'
  )
}

function formatTime(value: string) {
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return ''
  const pad = (num: number) => String(num).padStart(2, '0')
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())} ${pad(date.getHours())}:${pad(date.getMinutes())}`
}

function openUpdateModal(item: AppNotification) {
  let installing = false
  let progress = 0
  let statusText = ''
  let downloadedBytes = 0
  let totalBytes = 0
  let alreadyInstalled = isNotificationInstalled(item)
  let modalRef: ReturnType<typeof Modal.confirm>

  const renderContent = () => h('div', { class: 'update-notice-modal' }, [
    h('p', `当前版本：${item.updateInfo.currentVersion}`),
    h('p', `最新版本：${item.updateInfo.latestVersion}`),
    alreadyInstalled
      ? h('div', { class: 'update-installed-tip' }, '当前客户端已更新到该版本，无需重复更新。')
      : null,
    h('div', { class: 'update-notice-body-wrap' }, [
      h('p', '更新内容：'),
      h('pre', { class: 'update-notice-body' }, item.updateInfo.body || '本次更新暂无详细说明。')
    ]),
    installing
      ? h('div', { class: 'update-install-state' }, [
          h('span', statusText),
          h('div', { class: 'update-progress-track' }, [
            h('div', { class: 'update-progress-bar', style: { width: `${progress}%` } })
          ])
        ])
      : null
  ])

  const refreshModal = () => {
    modalRef.update({
      content: renderContent(),
      okText: alreadyInstalled ? '已更新' : installing ? '正在更新' : '立即更新',
      okButtonProps: { loading: installing, disabled: alreadyInstalled || installing },
      cancelButtonProps: { disabled: installing }
    })
  }

  modalRef = Modal.confirm({
    title: '发现新版本',
    width: 680,
    content: renderContent(),
    okText: alreadyInstalled ? '已更新' : '立即更新',
    cancelText: alreadyInstalled ? '关闭' : '暂不更新',
    okButtonProps: { disabled: alreadyInstalled },
    onOk: async () => {
      if (alreadyInstalled) return
      if (installing) return Promise.reject()
      installing = true
      progress = 0
      statusText = '正在下载更新'
      refreshModal()

      try {
        const update = await resolveUpdate(item)
        await installAppUpdate(update, event => {
          if (event.event === 'Started') {
            downloadedBytes = 0
            totalBytes = event.data.contentLength || 0
            progress = 0
            statusText = '正在下载更新'
            refreshModal()
          }
          if (event.event === 'Progress') {
            downloadedBytes += event.data.chunkLength
            progress = totalBytes
              ? Math.min(Math.round((downloadedBytes / totalBytes) * 100), 99)
              : Math.max(progress, 1)
            refreshModal()
          }
          if (event.event === 'Finished') {
            progress = 100
            statusText = '更新安装完成，正在重启'
            alreadyInstalled = true
            markUpdateInstalled(item.id)
            refreshModal()
          }
        })
      } catch (error) {
        installing = false
        statusText = downloadedBytes > 0 ? '更新安装失败' : '更新下载失败'
        refreshModal()
        message.error(formatUpdateError(error))
        return Promise.reject(error)
      }
    }
  })
}

async function resolveUpdate(item: AppNotification): Promise<AvailableUpdate> {
  const cached = updateCache.get(item.id)
  if (cached) return cached
  const result = await checkAppUpdate()
  if (!result.update || result.update.version !== item.updateInfo.latestVersion) {
    throw new Error('当前更新信息已失效，请稍后重新检查。')
  }
  updateCache.set(item.id, result.update)
  return result.update
}

function markUpdateInstalled(id: string) {
  let changed = false
  notifications.value = notifications.value.map(item => {
    if (item.id !== id || item.updateInfo.installed) return item
    changed = true
    const updateInfo = { ...item.updateInfo, installed: true }
    return {
      ...item,
      title: getNotificationTitle(updateInfo),
      summary: getNotificationSummary(updateInfo),
      updateInfo
    }
  })
  if (changed) saveNotifications()
}

function isNotificationInstalled(item: AppNotification) {
  return Boolean(item.updateInfo.installed) ||
    isVersionInstalled(currentAppVersion.value || item.updateInfo.currentVersion, item.updateInfo.latestVersion)
}

function isVersionInstalled(currentVersion: string, latestVersion: string) {
  return compareVersions(currentVersion, latestVersion) >= 0
}

function compareVersions(currentVersion: string, latestVersion: string) {
  const currentParts = parseVersionParts(currentVersion)
  const latestParts = parseVersionParts(latestVersion)
  const length = Math.max(currentParts.length, latestParts.length)
  for (let index = 0; index < length; index += 1) {
    const current = currentParts[index] || 0
    const latest = latestParts[index] || 0
    if (current !== latest) return current > latest ? 1 : -1
  }
  return 0
}

function parseVersionParts(version: string) {
  return version
    .replace(/^v/i, '')
    .split(/[^\d]+/)
    .filter(Boolean)
    .map(part => Number(part))
}

function getNotificationTitle(updateInfo: UpdateNotificationPayload) {
  return updateInfo.installed ? `已更新到 ${updateInfo.latestVersion}` : `发现新版本 ${updateInfo.latestVersion}`
}

function getNotificationSummary(updateInfo: UpdateNotificationPayload) {
  if (updateInfo.installed) return `当前客户端已更新到 ${updateInfo.latestVersion}。`
  return `当前版本 ${updateInfo.currentVersion}，点击查看更新内容。`
}
</script>

<style scoped>
.notification-trigger {
  align-items: center;
  border: 1px solid #dce5ed;
  border-radius: 8px;
  color: #3a4a5f;
  display: inline-flex;
  height: 34px;
  justify-content: center;
  width: 34px;
}

.notification-trigger :deep(.ant-btn-icon) {
  align-items: center;
  display: inline-flex;
  justify-content: center;
  line-height: 1;
}

.notification-bell {
  color: currentColor;
  display: block;
  flex: 0 0 auto;
  height: 18px;
  stroke: currentColor;
  width: 18px;
}

.notification-trigger:hover {
  background: #eefaf8;
  border-color: #b9e5df;
  color: #0f8f83;
}

.sr-only {
  clip: rect(0, 0, 0, 0);
  border: 0;
  height: 1px;
  margin: -1px;
  overflow: hidden;
  padding: 0;
  position: absolute;
  white-space: nowrap;
  width: 1px;
}

.notification-panel {
  width: 360px;
}

.notification-header {
  align-items: center;
  border-bottom: 1px solid #e7edf3;
  display: flex;
  justify-content: space-between;
  padding: 4px 4px 10px;
}

.notification-header strong {
  color: #273244;
  display: block;
  font-size: 15px;
}

.notification-count {
  color: #7b8ba0;
  display: block;
  font-size: 12px;
  margin-top: 2px;
}

.notification-list {
  max-height: 420px;
  overflow: auto;
  padding: 8px 2px 2px;
}

.notification-item {
  align-items: flex-start;
  background: #ffffff;
  border: 1px solid transparent;
  border-radius: 8px;
  color: inherit;
  cursor: pointer;
  display: flex;
  gap: 10px;
  min-height: 82px;
  padding: 12px 10px;
  position: relative;
  text-align: left;
  width: 100%;
}

.notification-item:hover {
  background: #f6fbfa;
  border-color: #d7eee9;
}

.notification-item.unread {
  background: #f1fbf9;
}

.unread-dot {
  background: #f08a24;
  border-radius: 50%;
  height: 7px;
  position: absolute;
  right: 10px;
  top: 12px;
  width: 7px;
}

.notification-icon {
  align-items: center;
  background: #0f8f83;
  border-radius: 8px;
  color: #ffffff;
  display: inline-flex;
  flex: 0 0 auto;
  font-size: 13px;
  font-weight: 700;
  height: 30px;
  justify-content: center;
  width: 30px;
}

.notification-content {
  display: flex;
  flex: 1;
  flex-direction: column;
  min-width: 0;
  padding-right: 12px;
}

.notification-title {
  color: #273244;
  font-weight: 700;
  line-height: 20px;
}

.notification-summary {
  color: #5f7084;
  line-height: 18px;
  margin-top: 3px;
}

.notification-time {
  color: #97a3b3;
  font-size: 12px;
  line-height: 18px;
  margin-top: 5px;
}

.notification-empty {
  padding: 26px 0 18px;
}

:global(.notification-popover-overlay .ant-popover-inner) {
  border: 1px solid #dfe8ef;
  border-radius: 8px;
  box-shadow: 0 14px 34px rgb(39 50 68 / 14%);
  padding: 10px;
}

:global(.update-notice-modal p) {
  margin: 0 0 8px;
}

:global(.update-notice-modal) {
  color: #314052;
}

:global(.update-installed-tip) {
  background: #eefaf8;
  border: 1px solid #bfe5df;
  border-radius: 8px;
  color: #0f8f83;
  font-weight: 600;
  margin: 8px 0 10px;
  padding: 9px 12px;
}

:global(.update-notice-body-wrap) {
  margin-top: 10px;
}

:global(.update-notice-body) {
  background: #f5f8fa;
  border: 1px solid #e3eaf1;
  border-radius: 8px;
  color: #314052;
  margin: 6px 0 0;
  max-height: 360px;
  min-height: 160px;
  overflow: auto;
  padding: 12px 14px;
  white-space: pre-wrap;
  word-break: break-word;
}

:global(.update-install-state) {
  color: #607086;
  margin-top: 14px;
}

:global(.update-progress-track) {
  background: #e7edf3;
  border-radius: 999px;
  height: 6px;
  margin-top: 8px;
  overflow: hidden;
}

:global(.update-progress-bar) {
  background: #0f8f83;
  border-radius: inherit;
  height: 100%;
  transition: width 0.2s ease;
}
</style>
