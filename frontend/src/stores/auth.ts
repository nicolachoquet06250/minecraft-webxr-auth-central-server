import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi, userApi, type User, type RegisterData, type LoginData } from '@/api'

const ACCESS_STORAGE_KEY = 'auth_token'
const REFRESH_STORAGE_KEY = 'auth_refresh'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem(ACCESS_STORAGE_KEY))
  const loading = ref(false)
  const error = ref<string | null>(null)

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  const setAuth = async (authToken: string, authUser: User) => {
    token.value = authToken
    user.value = authUser
    localStorage.setItem(ACCESS_STORAGE_KEY, authToken)
    await issueRefreshToken()
  }

  const setToken = (authToken: string) => {
    token.value = authToken
    localStorage.setItem(ACCESS_STORAGE_KEY, authToken)
  }

  const clearAuth = () => {
    token.value = null
    user.value = null
    localStorage.removeItem(ACCESS_STORAGE_KEY)
    localStorage.removeItem(REFRESH_STORAGE_KEY)
  }

  const issueRefreshToken = async () => {
    try {
      const response = await authApi.issueRefresh()
      localStorage.setItem(REFRESH_STORAGE_KEY, response.data.refresh)
    } catch {
      // Access token still works; refresh will simply be unavailable until next login.
    }
  }

  const register = async (data: RegisterData) => {
    loading.value = true
    error.value = null
    try {
      const response = await authApi.register(data)
      await setAuth(response.data.token, response.data.user)
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Registration failed'
      return false
    } finally {
      loading.value = false
    }
  }

  const login = async (data: LoginData) => {
    loading.value = true
    error.value = null
    try {
      const response = await authApi.login(data)
      await setAuth(response.data.token, response.data.user)
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Login failed'
      return false
    } finally {
      loading.value = false
    }
  }

  const logout = async () => {
    const refresh = localStorage.getItem(REFRESH_STORAGE_KEY)
    if (refresh) {
      try { await authApi.revokeRefresh(refresh) } catch { /* ignore logout revoke errors */ }
    }
    clearAuth()
  }

  const fetchProfile = async () => {
    if (!token.value) return false
    loading.value = true
    error.value = null
    try {
      const response = await userApi.getProfile()
      user.value = response.data
      if (!localStorage.getItem(REFRESH_STORAGE_KEY)) await issueRefreshToken()
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to fetch profile'
      clearAuth()
      return false
    } finally {
      loading.value = false
    }
  }

  const updateProfile = async (data: Partial<User>) => {
    loading.value = true
    error.value = null
    try {
      const response = await userApi.updateProfile(data)
      user.value = response.data
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to update profile'
      return false
    } finally {
      loading.value = false
    }
  }

  const unlinkDiscord = async () => {
    loading.value = true
    error.value = null
    try {
      const response = await userApi.unlinkDiscord()
      user.value = response.data
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to unlink Discord'
      return false
    } finally {
      loading.value = false
    }
  }

  const deleteAccount = async () => {
    loading.value = true
    error.value = null
    try {
      await userApi.deleteAccount()
      clearAuth()
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to delete account'
      return false
    } finally {
      loading.value = false
    }
  }

  const getDiscordAuthUrl = async () => {
    try {
      const response = await authApi.getDiscordUrl()
      return response.data.url
    } catch (err) {
      error.value = 'Failed to get Discord auth URL'
      return null
    }
  }

  return {
    user,
    token,
    loading,
    error,
    isAuthenticated,
    register,
    login,
    logout,
    fetchProfile,
    updateProfile,
    unlinkDiscord,
    deleteAccount,
    getDiscordAuthUrl,
    setToken,
  }
})