const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'

type ApiResponse<T> = Promise<{ data: T }>

type ApiErrorResponse = {
  status: number
  data: any
}

class ApiError extends Error {
  response: ApiErrorResponse

  constructor(status: number, data: any) {
    const message = data?.message || data?.error || `HTTP error! status: ${status}`
    super(message)
    this.name = 'ApiError'
    this.response = { status, data }
  }
}

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

const request = async <T>(path: string, options: RequestInit = {}): ApiResponse<T> => {
  const token = localStorage.getItem('auth_token')
  const headers = new Headers(options.headers)

  if (!headers.has('Content-Type') && options.body !== undefined) {
    headers.set('Content-Type', 'application/json')
  }

  if (token) {
    headers.set('Authorization', `Bearer ${token}`)
  }

  const response = await fetch(`${API_BASE_URL}${path}`, {
    ...options,
    headers,
    credentials: 'include',
  })

  const data = await parseResponseBody(response)

  if (!response.ok) {
    throw new ApiError(response.status, data)
  }

  return { data: data as T }
}

const parseResponseBody = async (response: Response) => {
  if (response.status === 204) {
    return null
  }

  const contentType = response.headers.get('content-type') || ''

  if (contentType.includes('application/json')) {
    return response.json()
  }

  const text = await response.text()

  if (!text) {
    return null
  }

  try {
    return JSON.parse(text)
  } catch {
    return { message: text }
  }
}

const jsonBody = (data: unknown) => JSON.stringify(data)

export const authApi = {
  register: (data: RegisterData): ApiResponse<AuthResponse> => request<AuthResponse>('/auth/register', {
    method: 'POST',
    body: jsonBody(data),
  }),
  login: (data: LoginData): ApiResponse<AuthResponse> => request<AuthResponse>('/auth/login', {
    method: 'POST',
    body: jsonBody(data),
  }),
  getDiscordUrl: (): ApiResponse<{ url: string }> => request<{ url: string }>('/auth/discord/url'),
}

export const userApi = {
  getProfile: (): ApiResponse<User> => request<User>('/users/me'),
  getUserById: (id: string): ApiResponse<User> => request<User>(`/users/${id}`),
  updateProfile: (data: Partial<User>): ApiResponse<User> => request<User>('/users/me', {
    method: 'PUT',
    body: jsonBody(data),
  }),
  deleteAccount: (): ApiResponse<null> => request<null>('/users/me', {
    method: 'DELETE',
  }),
}

export const serverApi = {
  createServer: (data: CreateServerData): ApiResponse<Server> => request<Server>('/servers', {
    method: 'POST',
    body: jsonBody(data),
  }),
  getUserServers: (): ApiResponse<Server[]> => request<Server[]>('/servers'),
  getServer: (id: string): ApiResponse<Server> => request<Server>(`/servers/${id}`),
  updateServer: (id: string, data: Partial<CreateServerData>): ApiResponse<Server> => request<Server>(`/servers/${id}`, {
    method: 'PUT',
    body: jsonBody(data),
  }),
  deleteServer: (id: string): ApiResponse<null> => request<null>(`/servers/${id}`, {
    method: 'DELETE',
  }),
  getServerStats: async (gameDomain: string) => {
    const statsUrl = `${gameDomain.replace(/\/+$/, '')}/stats`
    const response = await fetch(statsUrl)
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }
    return response.json()
  },
}

export default request
