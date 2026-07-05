<template>
  <div class="page-shell">
    <div class="toolbar">
      <h2 class="page-title">学员管理</h2>
      <a-button type="primary" @click="openEditor()">新增学员</a-button>
    </div>
    <a-table :columns="columns" :data-source="students" :loading="loading" row-key="id" class="work-panel" :pagination="{ pageSize: 10 }">
      <template #bodyCell="{ column, record }">
        <template v-if="column.key === 'action'">
          <a-space>
            <a @click="openEditor(record)">编辑</a>
            <a-popconfirm title="确认删除该学员？" @confirm="remove(record.id)">
              <a class="danger-link">删除</a>
            </a-popconfirm>
          </a-space>
        </template>
      </template>
    </a-table>

    <a-drawer v-model:open="editorOpen" title="学员信息" width="460" destroy-on-close>
      <a-form layout="vertical" :model="form">
        <a-form-item label="姓名" required>
          <a-input v-model:value="form.name" />
        </a-form-item>
        <a-form-item label="手机号">
          <a-input v-model:value="form.phone" />
        </a-form-item>
        <a-form-item label="年级">
          <a-input v-model:value="form.grade" />
        </a-form-item>
        <a-form-item label="学校">
          <a-input v-model:value="form.school" />
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
import type { Student } from '../api/native'
import { deleteStudent, listStudents, saveStudent } from '../api/native'

const loading = ref(false)
const saving = ref(false)
const editorOpen = ref(false)
const students = ref<Student[]>([])
const form = reactive<Partial<Student>>({
  name: '',
  phone: '',
  grade: '',
  school: '',
  remark: ''
})
const columns = [
  { title: '姓名', dataIndex: 'name' },
  { title: '手机号', dataIndex: 'phone' },
  { title: '年级', dataIndex: 'grade' },
  { title: '学校', dataIndex: 'school' },
  { title: '更新时间', dataIndex: 'updatedAt' },
  { title: '操作', key: 'action', width: 140 }
]

onMounted(load)

async function load() {
  loading.value = true
  try {
    students.value = await listStudents()
  } catch (error) {
    message.error(String(error))
  } finally {
    loading.value = false
  }
}

function openEditor(record?: Student) {
  Object.assign(form, record || { id: undefined, name: '', phone: '', grade: '', school: '', remark: '' })
  editorOpen.value = true
}

async function submit() {
  if (!form.name) {
    message.warning('请填写学员姓名')
    return
  }
  saving.value = true
  try {
    await saveStudent(form)
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
    await deleteStudent(id)
    await load()
  } catch (error) {
    message.error(String(error))
  }
}
</script>
