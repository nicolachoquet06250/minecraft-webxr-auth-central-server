import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi, userApi, type User, type RegisterData, type LoginData } from '@/api'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem('auth_token'))
  const loading = ref(false)
  const error = ref<string | null>(null)

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  const setAuth = (authToken: string, authUser: User) => {
    token.value = authToken
    user.value = authUser
    localStorage.setItem('auth_token', authToken)
  }

  const clearAuth = () => {
    token.value = null
    user.value = null
    localStorage.removeItem('auth_token')
  }

  const register = async (data: RegisterData) => {
    loading.value = true
    error.value = null
    try {
      const response = await authApi.register(data)
      setAuth(response.data.token, response.data.user)
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
      setAuth(response.data.token, response.data.user)
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Login failed'
      return false
    } finally {
      loading.value = false
    }
  }

  const logout = () => {
    clearAuth()
  }

  const fetchProfile = async () => {
    if (!token.value) return false
    
    loading.value = true
    error.value = null
    try {
      const response = await userApi.getProfile()
      user.value = response.data
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
    getDiscordAuthUrl,
  }
})
