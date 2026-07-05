<template>
  <div class="page-shell">
    <div class="toolbar">
      <h2 class="page-title">教师管理</h2>
      <a-button type="primary" @click="openEditor()">新增教师</a-button>
    </div>
    <a-table :columns="columns" :data-source="teachers" :loading="loading" row-key="id" class="work-panel" :pagination="{ pageSize: 10 }">
      <template #bodyCell="{ column, record }">
        <template v-if="column.key === 'status'">
          <a-tag :color="record.status === 'enabled' ? 'green' : 'default'">{{ record.status === 'enabled' ? '启用' : '停用' }}</a-tag>
        </template>
        <template v-if="column.key === 'action'">
          <a-space>
            <a @click="openEditor(record)">编辑</a>
            <a-popconfirm title="确认删除该教师？" @confirm="remove(record.id)">
              <a class="danger-link">删除</a>
            </a-popconfirm>
          </a-space>
        </template>
      </template>
    </a-table>

    <a-drawer v-model:open="editorOpen" title="教师信息" width="460" destroy-on-close>
      <a-form layout="vertical" :model="form">
        <a-form-item label="登录账号" required>
          <a-input v-model:value="form.account" />
        </a-form-item>
        <a-form-item label="登录密码">
          <a-input-password v-model:value="form.password" placeholder="不修改可留空" />
        </a-form-item>
        <a-form-item label="教师姓名" required>
          <a-input v-model:value="form.name" />
        </a-form-item>
        <a-form-item label="手机号">
          <a-input v-model:value="form.phone" />
        </a-form-item>
        <a-form-item label="状态">
          <a-segmented v-model:value="form.status" :options="statusOptions" />
        </a-form-item>
        <a-form-item label="备注">
          <a-textarea v-model:value="form.remark" :rows="4" />
        </a-form-item>
      </a-form>
      <template #footer>
        <a-space>
          <a-button @click="editorOpen = false">取消</a-button>
          <a-button type="primary" :loading="saving" @click="submit">保存</a-button>
        </a-space>
      </template>
    </a-drawer>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import { message } from 'ant-design-vue'
import type { Teacher } from '../api/native'
import { deleteTeacher, listTeachers, saveTeacher } from '../api/native'

const loading = ref(false)
const saving = ref(false)
const editorOpen = ref(false)
const teachers = ref<Teacher[]>([])
const statusOptions = [
  { label: '启用', value: 'enabled' },
  { label: '停用', value: 'disabled' }
]
const form = reactive<Partial<Teacher>>({
  account: '',
  password: '',
  name: '',
  phone: '',
  remark: '',
  status: 'enabled'
})
const columns = [
  { title: '账号', dataIndex: 'account' },
  { title: '姓名', dataIndex: 'name' },
  { title: '手机号', dataIndex: 'phone' },
  { title: '状态', key: 'status' },
  { title: '更新时间', dataIndex: 'updatedAt' },
  { title: '操作', key: 'action', width: 140 }
]

onMounted(load)

async function load() {
  loading.value = true
  try {
    teachers.value = await listTeachers()
  } catch (error) {
    message.error(String(error))
  } finally {
    loading.value = false
  }
}

function openEditor(record?: Teacher) {
  Object.assign(form, record || { id: undefined, account: '', password: '', name: '', phone: '', remark: '', status: 'enabled' })
  form.password = ''
  editorOpen.value = true
}

async function submit() {
  if (!form.account || !form.name) {
    message.warning('请填写账号和教师姓名')
    return
  }
  saving.value = true
  try {
    await saveTeacher(form)
    editorOpen.value = false
    await load()
  } catch (error) {
    message.error(String(error))
  } finally {
    saving.value = false
  }
}

async function remove(id: number) {
  try {
    await deleteTeacher(id)
    await load()
  } catch (error) {
    message.error(String(error))
  }
}
</script>
