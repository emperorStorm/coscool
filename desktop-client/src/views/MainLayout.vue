<template>
  <a-layout class="main-layout">
    <a-layout-sider width="220" class="side-bar">
      <div class="side-logo">
        <img src="../assets/brand/coscool-icon.svg" alt="课思库" />
        <strong>课思库</strong>
      </div>
      <a-menu v-model:selectedKeys="selectedKeys" v-model:openKeys="openKeys" mode="inline" @click="handleMenuClick">
        <a-sub-menu key="questions">
          <template #title>题库管理</template>
          <a-menu-item key="/app/questions/entry">题库录入</a-menu-item>
          <a-menu-item key="/app/questions/query">题库查询</a-menu-item>
          <a-menu-item key="/app/questions/papers">试卷维护</a-menu-item>
        </a-sub-menu>
        <a-sub-menu key="basic">
          <template #title>基础管理</template>
          <a-menu-item key="/app/basic/knowledge-points">知识点维护</a-menu-item>
        </a-sub-menu>
        <a-menu-item key="/app/teachers">教师管理</a-menu-item>
        <a-menu-item key="/app/students">学员管理</a-menu-item>
        <a-menu-item key="/app/settings">系统设置</a-menu-item>
      </a-menu>
    </a-layout-sider>
    <a-layout>
      <a-layout-header class="top-bar">
        <div>
          <strong>{{ routeTitle }}</strong>
          <span class="muted"> 本地个人版</span>
        </div>
        <a-space>
          <AppNotificationCenter />
          <span class="muted">{{ teacherName }}</span>
          <a-button @click="logout">退出</a-button>
        </a-space>
      </a-layout-header>
      <a-layout-content class="content-area">
        <router-view />
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AppNotificationCenter from '../components/AppNotificationCenter.vue'

const route = useRoute()
const router = useRouter()
const selectedKeys = ref<string[]>([])
const openKeys = ref<string[]>(['questions'])

const teacherName = computed(() => {
  const raw = localStorage.getItem('coscool_teacher')
  if (!raw) return '未登录'
  try {
    const teacher = JSON.parse(raw)
    return teacher.name || teacher.account || '教师'
  } catch {
    return '教师'
  }
})

const routeTitle = computed(() => {
  if (route.path.includes('/questions/entry')) return '题库录入'
  if (route.path.includes('/questions/query')) return '题库查询'
  if (route.path.includes('/questions/papers')) return '试卷维护'
  if (route.path.includes('/questions/paper-assemble')) return '组装试卷'
  if (route.path.includes('/basic/knowledge-points')) return '知识点维护'
  if (route.path.includes('teachers')) return '教师管理'
  if (route.path.includes('students')) return '学员管理'
  if (route.path.includes('settings')) return '系统设置'
  return '题库管理'
})

watch(
  () => route.path,
  (path) => {
    selectedKeys.value = [path]
    const activeSubMenu = resolveActiveSubMenu(path)
    if (activeSubMenu && !openKeys.value.includes(activeSubMenu)) {
      openKeys.value = [...openKeys.value, activeSubMenu]
    }
  },
  { immediate: true }
)

function resolveActiveSubMenu(path: string) {
  if (path.includes('/questions')) return 'questions'
  if (path.includes('/basic')) return 'basic'
  return ''
}

function handleMenuClick(event: { key: string }) {
  router.push(event.key)
}

function logout() {
  localStorage.removeItem('coscool_teacher')
  router.replace('/login')
}
</script>

<style scoped>
.main-layout {
  min-height: 100vh;
}

.side-bar {
  background: #f3f6f8;
  border-right: 1px solid #dde5ed;
}

.side-logo {
  display: flex;
  gap: 10px;
  align-items: center;
  height: 64px;
  padding: 0 18px;
}

.side-logo img {
  width: 34px;
  height: 34px;
  display: block;
  border-radius: 6px;
}

.side-logo strong {
  font-size: 18px;
}

.top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 58px;
  padding: 0 18px;
  background: #ffffff;
  border-bottom: 1px solid #e0e7ef;
}

.content-area {
  min-height: calc(100vh - 58px);
}
</style>
