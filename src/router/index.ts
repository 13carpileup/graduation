import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import countdownView from '../views/countdownView.vue'
import ConnView from '../views/connectionsView.vue'
const ConnectionsView = ConnView as any

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
    },
    {
      path: '/connections',
      name: 'connections',
      component: ConnectionsView
    },


  ],
})

export default router
