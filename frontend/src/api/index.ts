import { generateProfileInfoAvatarMailPreview } from '../character-builder/avatar-mail-preview'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'
const ACCESS_STORAGE_KEY = 'auth_token'
const REFRESH_STORAGE_KEY = 'auth_refresh'

type ApiResponse<T> = Promise<{ data: T }>
type ApiErrorResponse = { status: number; data: any }

class ApiError extends Error {
  response: ApiErrorResponse
  constructor(status: number, data: any) {
    const message = data?.message || data?.error || `HTTP error! status: ${status}`
    super(message)
    this.name = 'ApiError'
    this.response = { status, data }
  }
}

export interface User { id: string; username: string; email: string; avatar: string; bio?: string; birthdate: string; age_verified: boolean; discord_username?: string; created_at: string }
export interface RegisterData { username: string; email: string; password: string; avatar: string; birthdate: string; bio?: string }
export interface LoginData { email: string; password: string }
export interface AuthResponse { token: string; user: User }
export interface RefreshIssueResponse { refresh: string }
export interface RefreshRotateResponse { token: string; refresh: string; user: User }
export interface RefreshRevokeResponse { revoked: boolean }
export interface Server { id: string; owner_id: string; name: string; game_domain: string; description?: string; is_active: boolean; created_at: string; updated_at: string }
export interface ServerHistoryEntry { server: Server; is_favorite: boolean; visited_at?: string; favorited_at?: string }
export interface FavoriteServerEntry { server: Server; is_favorite: boolean; favorited_at: string }
export interface CreateServerData { name: string; game_domain: string; description?: string }
export type AvatarTextureData = { version: 1; palette: Record<string, readonly [number, number, number, number]>; parts: Record<string, Record<string, { width: number; height: number; matrix: string[] }>> }
export interface UserAvatar { id: string; name: string; base_kind: 'steve' | 'alex' | 'custom'; is_active: boolean; texture_data: AvatarTextureData; created_at: string; updated_at: string }
export interface ActiveAvatarResponse { kind: 'default' | 'custom'; avatar: UserAvatar | null }
export interface SaveAvatarData { name: string; base_kind: string; texture_data: AvatarTextureData; preview_image_data_url?: string }
export interface UpdateAvatarData { name?: string; texture_data: AvatarTextureData; preview_image_data_url?: string }
export interface ContactMailData { name: string; email: string; subject: string; message: string }
export interface SupportMailData { name?: string; email?: string; category: string; subject: string; message: string; server_url?: string }
export interface MailStatus { enabled: boolean }
export interface MailSentResponse { sent: boolean }
export interface PasswordChangeCodeRequest { next_secret: string; next_secret_confirmation: string }
export interface PasswordChangeConfirmRequest { code: string }
export interface PasswordChangeCodeResponse { sent: boolean; expires_in_minutes: number }
export interface PasswordChangedResponse { changed: boolean }
export interface FriendAvatar { kind: 'default' | 'custom'; base_kind: string; name: string; url: string }
export interface FriendUser { id: string; username: string; avatar: FriendAvatar }
export interface PaginatedUsersResponse { items: FriendUser[]; page: number; page_size: number; total: number; total_pages: number; next_url: string | null; previous_url: string | null }
export interface FriendRequest { id: string; requester: FriendUser; receiver: FriendUser; status: 'pending' | 'accepted' | 'refused' | 'cancelled'; created_at: string; updated_at: string }
export interface FriendEntry { user: FriendUser; created_at: string }
export interface FriendPresenceServer { id: string; name: string; game_domain: string }
export interface FriendPresence { user_id: string; server: FriendPresenceServer | null }
export interface CreateFriendRequestData { receiver_user_id: string }

const request = async <T>(path: string, options: RequestInit = {}, retry = true): ApiResponse<T> => {
  const token = localStorage.getItem(ACCESS_STORAGE_KEY)
  const headers = new Headers(options.headers)
  if (!headers.has('Content-Type') && options.body !== undefined) headers.set('Content-Type', 'application/json')
  if (token) headers.set('Authorization', `Bearer ${token}`)
  const response = await fetch(`${API_BASE_URL}${path}`, { ...options, headers, credentials: 'include' })
  const data = await parseResponseBody(response)

  if (response.status === 401 && retry && !path.startsWith('/auth/refresh')) {
    const refreshed = await refreshAccessToken()
    if (refreshed) return request<T>(path, options, false)
  }

  if (!response.ok) throw new ApiError(response.status, data)
  return { data: data as T }
}

const refreshAccessToken = async () => {
  const refresh = localStorage.getItem(REFRESH_STORAGE_KEY)
  if (!refresh) return false

  const response = await fetch(`${API_BASE_URL}/auth/refresh`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
    body: JSON.stringify({ refresh }),
    credentials: 'include',
  })
  const data = await parseResponseBody(response)
  if (!response.ok) {
    localStorage.removeItem(ACCESS_STORAGE_KEY)
    localStorage.removeItem(REFRESH_STORAGE_KEY)
    return false
  }

  const rotated = data as RefreshRotateResponse
  localStorage.setItem(ACCESS_STORAGE_KEY, rotated.token)
  localStorage.setItem(REFRESH_STORAGE_KEY, rotated.refresh)
  return true
}

const parseResponseBody = async (response: Response) => {
  if (response.status === 204) return null
  const contentType = response.headers.get('content-type') || ''
  if (contentType.includes('application/json')) return response.json()
  const text = await response.text()
  if (!text) return null
  try { return JSON.parse(text) } catch { return { message: text } }
}

const jsonBody = (data: unknown) => JSON.stringify(data)
const joinStatsPath = (domain: string) => new URL('/stats', domain.endsWith('/') ? domain : `${domain}/`).toString()
const withProfileInformationPreview = <T extends SaveAvatarData | UpdateAvatarData>(data: T): T => {
  const baseKind = 'base_kind' in data ? data.base_kind : 'custom'
  const avatar: UserAvatar = { id: 'mail-preview', name: 'name' in data && data.name ? data.name : 'Avatar', base_kind: baseKind === 'steve' || baseKind === 'alex' ? baseKind : 'custom', is_active: false, texture_data: data.texture_data, created_at: new Date().toISOString(), updated_at: new Date().toISOString() }
  const preview = generateProfileInfoAvatarMailPreview(avatar)
  return preview ? { ...data, preview_image_data_url: preview } : data
}
const userSearchPath = (params: { q?: string; page?: number; page_size?: number } = {}) => {
  const search = new URLSearchParams()
  if (params.q?.trim()) search.set('q', params.q.trim())
  if (params.page) search.set('page', String(params.page))
  if (params.page_size) search.set('page_size', String(params.page_size))
  const query = search.toString()
  return query ? `/users/search?${query}` : '/users/search'
}

export const authApi = {
  register: (data: RegisterData): ApiResponse<AuthResponse> => request<AuthResponse>('/auth/register', { method: 'POST', body: jsonBody(data) }),
  login: (data: LoginData): ApiResponse<AuthResponse> => request<AuthResponse>('/auth/login', { method: 'POST', body: jsonBody(data) }),
  issueRefresh: (): ApiResponse<RefreshIssueResponse> => request<RefreshIssueResponse>('/auth/refresh/issue', { method: 'POST' }, false),
  refresh: (refresh: string): ApiResponse<RefreshRotateResponse> => request<RefreshRotateResponse>('/auth/refresh', { method: 'POST', body: jsonBody({ refresh }) }, false),
  revokeRefresh: (refresh: string): ApiResponse<RefreshRevokeResponse> => request<RefreshRevokeResponse>('/auth/refresh/revoke', { method: 'POST', body: jsonBody({ refresh }) }, false),
  getDiscordUrl: (): ApiResponse<{ url: string }> => request<{ url: string }>('/auth/discord/url')
}
export const userApi = { getProfile: (): ApiResponse<User> => request<User>('/users/me'), getUserById: (id: string): ApiResponse<User> => request<User>(`/users/${id}`), searchUsers: (params: { q?: string; page?: number; page_size?: number } = {}): ApiResponse<PaginatedUsersResponse> => request<PaginatedUsersResponse>(userSearchPath(params)), updateProfile: (data: Partial<User>): ApiResponse<User> => request<User>('/users/me', { method: 'PUT', body: jsonBody(data) }), unlinkDiscord: (): ApiResponse<User> => request<User>('/users/me/discord', { method: 'DELETE' }), deleteAccount: (): ApiResponse<null> => request<null>('/users/me', { method: 'DELETE' }), requestPasswordChangeCode: (data: PasswordChangeCodeRequest): ApiResponse<PasswordChangeCodeResponse> => request<PasswordChangeCodeResponse>('/users/me/password/change-code', { method: 'POST', body: jsonBody(data) }), confirmPasswordChange: (_data: PasswordChangeConfirmRequest): ApiResponse<PasswordChangedResponse> => request<PasswordChangedResponse>('/users/me/password', { method: 'PUT' }) }
export const avatarApi = { getActive: (): ApiResponse<ActiveAvatarResponse> => request<ActiveAvatarResponse>('/users/me/avatar'), clearActive: (): ApiResponse<null> => request<null>('/users/me/avatar', { method: 'DELETE' }), list: (): ApiResponse<UserAvatar[]> => request<UserAvatar[]>('/users/me/avatars'), createCopy: (data: SaveAvatarData): ApiResponse<UserAvatar> => request<UserAvatar>('/users/me/avatars', { method: 'POST', body: jsonBody(withProfileInformationPreview(data)) }), update: (id: string, data: UpdateAvatarData): ApiResponse<UserAvatar> => request<UserAvatar>(`/users/me/avatars/${id}`, { method: 'PUT', body: jsonBody(withProfileInformationPreview(data)) }), delete: (id: string): ApiResponse<null> => request<null>(`/users/me/avatars/${id}`, { method: 'DELETE' }), select: (id: string): ApiResponse<null> => request<null>(`/users/me/avatars/${id}/select`, { method: 'PUT' }) }
export const serverApi = { createServer: (data: CreateServerData): ApiResponse<Server> => request<Server>('/servers', { method: 'POST', body: jsonBody(data) }), getUserServers: (): ApiResponse<Server[]> => request<Server[]>('/servers'), getRecentServers: (): ApiResponse<ServerHistoryEntry[]> => request<ServerHistoryEntry[]>('/servers/recent'), getFavoriteServers: (): ApiResponse<FavoriteServerEntry[]> => request<FavoriteServerEntry[]>('/servers/favorites'), recordServerVisit: (serverUrl: string): ApiResponse<ServerHistoryEntry> => request<ServerHistoryEntry>('/servers/visit', { method: 'POST', body: jsonBody({ server_url: serverUrl }) }), favoriteServer: (id: string): ApiResponse<FavoriteServerEntry> => request<FavoriteServerEntry>(`/servers/${id}/favorite`, { method: 'POST' }), unfavoriteServer: (id: string): ApiResponse<null> => request<null>(`/servers/${id}/favorite`, { method: 'DELETE' }), getServer: (id: string): ApiResponse<Server> => request<Server>(`/servers/${id}`), updateServer: (id: string, data: Partial<CreateServerData>): ApiResponse<Server> => request<Server>(`/servers/${id}`, { method: 'PUT', body: jsonBody(data) }), deleteServer: (id: string): ApiResponse<null> => request<null>(`/servers/${id}`, { method: 'DELETE' }), getServerStats: async (gameDomain: string) => { const response = await fetch(joinStatsPath(gameDomain)); if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`); return response.json() } }
export const friendApi = { getFriends: (): ApiResponse<FriendEntry[]> => request<FriendEntry[]>('/friends'), getPresence: (): ApiResponse<FriendPresence[]> => request<FriendPresence[]>('/friends/presence'), getIncomingRequests: (): ApiResponse<FriendRequest[]> => request<FriendRequest[]>('/friends/requests/incoming'), getOutgoingRequests: (): ApiResponse<FriendRequest[]> => request<FriendRequest[]>('/friends/requests/outgoing'), createRequest: (data: CreateFriendRequestData): ApiResponse<FriendRequest> => request<FriendRequest>('/friends/requests', { method: 'POST', body: jsonBody(data) }), acceptRequest: (id: string): ApiResponse<FriendRequest> => request<FriendRequest>(`/friends/requests/${id}/accept`, { method: 'POST' }), refuseRequest: (id: string): ApiResponse<FriendRequest> => request<FriendRequest>(`/friends/requests/${id}/refuse`, { method: 'POST' }), deleteFriend: (userId: string): ApiResponse<null> => request<null>(`/friends/${userId}`, { method: 'DELETE' }) }
export const mailApi = { status: (): ApiResponse<MailStatus> => request<MailStatus>('/mail/status'), contact: (data: ContactMailData): ApiResponse<MailSentResponse> => request<MailSentResponse>('/contact', { method: 'POST', body: jsonBody(data) }), support: (data: SupportMailData): ApiResponse<MailSentResponse> => request<MailSentResponse>('/support', { method: 'POST', body: jsonBody(data) }) }

export default request
