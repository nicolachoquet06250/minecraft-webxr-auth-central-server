import axios from 'axios'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'

export interface User {
  id: string
  username: string
  email: string
  avatar: string
  bio?: string
  birthdate: string
  age_verified: boolean
  discord_username?: string
  created_at: string
}

export interface RegisterData {
  username: string
  email: string
  password: string
  avatar: string
  birthdate: string
  bio?: string
}

export interface LoginData {
  email: string
  password: string
}

export interface AuthResponse {
  token: string
  user: User
}

export interface Server {
  id: string
  owner_id: string
  name: string
  game_domain: string
  description?: string
  is_active: boolean
  created_at: string
  updated_at: string
}

export interface CreateServerData {
  name: string
  game_domain: string
  description?: string
}

const api = axios.create({
  baseURL: API_BASE_URL,
  withCredentials: true,
  headers: {
    'Content-Type': 'application/json',
  },
})

api.interceptors.request.use((config) => {
  const token = localStorage.getItem('auth_token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

export const authApi = {
  register: (data: RegisterData) => api.post<AuthResponse>('/auth/register', data),
  login: (data: LoginData) => api.post<AuthResponse>('/auth/login', data),
  getDiscordUrl: () => api.get<{ url: string }>('/auth/discord/url'),
}

export const userApi = {
  getProfile: () => api.get<User>('/users/me'),
  getUserById: (id: string) => api.get<User>(`/users/${id}`),
  updateProfile: (data: Partial<User>) => api.put<User>('/users/me', data),
  deleteAccount: () => api.delete('/users/me'),
}

export const serverApi = {
  createServer: (data: CreateServerData) => api.post<Server>('/servers', data),
  getUserServers: () => api.get<Server[]>('/servers'),
  getServer: (id: string) => api.get<Server>(`/servers/${id}`),
  updateServer: (id: string, data: Partial<CreateServerData>) =>
    api.put<Server>(`/servers/${id}`, data),
  deleteServer: (id: string) => api.delete(`/servers/${id}`),
  getServerStats: async (gameDomain: string) => {
    const statsUrl = `${gameDomain.replace(/\/+$/, '')}/stats`
    const response = await fetch(statsUrl)
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }
    return response.json()
  },
}

export default api
