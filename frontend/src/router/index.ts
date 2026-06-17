import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import HomeView from '@/views/HomeView.vue'
import LoginView from '@/views/LoginView.vue'
import RegisterView from '@/views/RegisterView.vue'
import ProfileView from '@/views/ProfileView.vue'
import AvatarBuilderView from '@/views/AvatarBuilderView.vue'
import ServersView from '@/views/ServersView.vue'
import FavoriteServersView from '@/views/FavoriteServersView.vue'
import RecentServersView from '@/views/RecentServersView.vue'
import ServerDashboardView from '@/views/ServerDashboardView.vue'
import FriendsView from '@/views/FriendsView.vue'
import FriendRequestAcceptView from '@/views/FriendRequestAcceptView.vue'
import ApiSwaggerView from '@/views/ApiSwaggerView.vue'
import DocumentationView from '@/views/DocumentationView.vue'
import ContactView from '@/views/ContactView.vue'
import SupportView from '@/views/SupportView.vue'

const POST_LOGIN_REDIRECT_KEY = 'post_login_redirect'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', name: 'home', component: HomeView },
    { path: '/login', name: 'login', component: LoginView, meta: { requiresGuest: true } },
    { path: '/register', name: 'register', component: RegisterView, meta: { requiresGuest: true } },
    { path: '/profile', name: 'profile', component: ProfileView, meta: { requiresAuth: true } },
    { path: '/profile/avatar-builder', name: 'avatar-builder', component: AvatarBuilderView, meta: { requiresAuth: true } },
    { path: '/servers', name: 'servers', component: ServersView, meta: { requiresAuth: true } },
    { path: '/servers/favorites', name: 'servers-favorites', component: FavoriteServersView, meta: { requiresAuth: true } },
    { path: '/servers/recent', name: 'servers-recent', component: RecentServersView, meta: { requiresAuth: true } },
    { path: '/servers/:id/dashboard', name: 'server-dashboard', component: ServerDashboardView, meta: { requiresAuth: true } },
    { path: '/friends', name: 'friends', component: FriendsView, meta: { requiresAuth: true } },
    { path: '/friends/accept', name: 'friend-request-accept', component: FriendRequestAcceptView, meta: { requiresAuth: true } },
    { path: '/api/swagger', name: 'api-swagger', component: ApiSwaggerView, meta: { requiresAuth: true } },
    { path: '/documentation', name: 'documentation', component: DocumentationView, meta: { requiresAuth: true } },
    { path: '/api/documentation', redirect: '/documentation', meta: { requiresAuth: true } },
    { path: '/contact', name: 'contact', component: ContactView },
    { path: '/support', name: 'support', component: SupportView },
  ],
})

router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore()
  const authToken = typeof to.query.auth_token === 'string' ? to.query.auth_token : null
  if (authToken) {
    authStore.setToken(authToken)
    await authStore.fetchProfile()
    next({ name: 'profile', replace: true })
    return
  }
  if (authStore.token && !authStore.user) await authStore.fetchProfile()

  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    localStorage.setItem(POST_LOGIN_REDIRECT_KEY, to.fullPath)
    next({ name: 'login' })
    return
  }

  if (authStore.isAuthenticated && to.name === 'profile') {
    const redirect = localStorage.getItem(POST_LOGIN_REDIRECT_KEY)
    if (redirect) {
      localStorage.removeItem(POST_LOGIN_REDIRECT_KEY)
      next(redirect)
      return
    }
  }

  if (to.meta.requiresGuest && authStore.isAuthenticated) next({ name: 'profile' })
  else next()
})

export default router
