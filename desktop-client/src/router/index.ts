import { createRouter, createWebHashHistory } from 'vue-router'
import LoginView from '../views/LoginView.vue'
import MainLayout from '../views/MainLayout.vue'
import TeacherView from '../views/TeacherView.vue'
import StudentView from '../views/StudentView.vue'
import QuestionBankView from '../views/QuestionBankView.vue'
import QuestionEditorView from '../views/QuestionEditorView.vue'
import QuestionQueryView from '../views/QuestionQueryView.vue'
import SettingsView from '../views/SettingsView.vue'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/login' },
    { path: '/login', component: LoginView },
    { path: '/question-editor', component: QuestionEditorView, meta: { requiresAuth: true } },
    { path: '/question-editor/:id', component: QuestionEditorView, meta: { requiresAuth: true } },
    {
      path: '/app',
      component: MainLayout,
      redirect: '/app/questions/entry',
      meta: { requiresAuth: true },
      children: [
        { path: 'teachers', component: TeacherView },
        { path: 'students', component: StudentView },
        { path: 'questions', redirect: '/app/questions/entry' },
        { path: 'questions/entry', component: QuestionBankView },
        { path: 'questions/query', component: QuestionQueryView },
        { path: 'settings', component: SettingsView }
      ]
    }
  ]
})

router.beforeEach((to) => {
  if (to.matched.some((record) => record.meta.requiresAuth) && !localStorage.getItem('coscool_teacher')) {
    return '/login'
  }

  return true
})

export default router
