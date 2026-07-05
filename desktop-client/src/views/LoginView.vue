<template>
  <main class="login-page">
    <section class="login-card">
      <img class="brand-mark" src="../assets/brand/coscool-icon.svg" alt="课思库" />
      <h1>课思库</h1>
      <p>本地题库管理桌面客户端</p>
      <a-form layout="vertical" :model="form" @finish="handleLogin">
        <a-form-item label="账号" name="account" :rules="[{ required: true, message: '请输入账号' }]">
          <a-input v-model:value="form.account" autocomplete="username" />
        </a-form-item>
        <a-form-item label="密码" name="password" :rules="[{ required: true, message: '请输入密码' }]">
          <a-input-password v-model:value="form.password" autocomplete="current-password" />
        </a-form-item>
        <a-button type="primary" html-type="submit" block :loading="loading">登录</a-button>
      </a-form>
      <div class="login-hint">默认账号 yaoyao / 123456</div>
    </section>
  </main>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { message } from 'ant-design-vue'
import { useRouter } from 'vue-router'
import { login } from '../api/native'

const router = useRouter()
const loading = ref(false)
const form = reactive({
  account: 'yaoyao',
  password: '123456'
})

async function handleLogin() {
  loading.value = true
  try {
    const teacher = await login(form.account, form.password)
    localStorage.setItem('coscool_teacher', JSON.stringify(teacher))
    await router.replace('/app/questions')
  } catch (error) {
    message.error(String(error))
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-page {
  display: grid;
  min-height: 100vh;
  place-items: center;
  padding: 24px;
  background:
    linear-gradient(120deg, rgba(15, 143, 131, 0.1), transparent 42%),
    #eef2f4;
}

.login-card {
  width: min(390px, 100%);
  padding: 34px;
  background: #ffffff;
  border: 1px solid #dfe6ee;
  border-radius: 8px;
  box-shadow: 0 18px 45px rgba(42, 56, 72, 0.13);
}

.brand-mark {
  width: 54px;
  height: 54px;
  margin-bottom: 18px;
  display: block;
  border-radius: 8px;
}

h1 {
  margin: 0;
  font-size: 30px;
  letter-spacing: 0;
}

p {
  margin: 8px 0 28px;
  color: #687789;
}

.login-hint {
  margin-top: 18px;
  color: #8794a4;
  text-align: center;
}
</style>
