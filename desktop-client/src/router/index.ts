import { createRouter, createWebHashHistory } from 'vue-router'
import LoginView from '../views/LoginView.vue'
import MainLayout from '../views/MainLayout.vue'
import TeacherView from '../views/TeacherView.vue'
import StudentView from '../views/StudentView.vue'
import QuestionBankView from '../views/QuestionBankView.vue'
import QuestionQueryView from '../views/QuestionQueryView.vue'
import SettingsView from '../views/SettingsView.vue'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/login' },
    { path: '/login', component: LoginView },
    {
      path: '/app',
      component: MainLayout,
      redirect: '/app/questions/entry',
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
  if (to.path.startsWith('/app') && !localStorage.getItem('coscool_teacher')) {
    return '/login'
  }

  return true
})

export default router
