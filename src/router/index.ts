import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import countdownView from '../views/countdownView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/countdown',
      name: 'countdown',
      component: countdownView
    }

  ],
})

export default router
