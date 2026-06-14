import { ArcRotateCamera, Color4, Engine, HemisphericLight, Mesh, Scene, Vector3 } from '@babylonjs/core'
import { buildCharacter } from '@/character-builder/character-builder'
import { createCharacterModelFromAvatar, createEditableAvatarFromApi } from '@/character-builder/avatar-editor'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'

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

export interface RegisterData { username: string; email: string; password: string; avatar: string; birthdate: string; bio?: string }
export interface LoginData { email: string; password: string }
export interface AuthResponse { token: string; user: User }

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

export interface CreateServerData { name: string; game_domain: string; description?: string }

export type AvatarTextureData = {
  version: 1
  palette: Record<string, readonly [number, number, number, number]>
  parts: Record<string, Record<string, { width: number; height: number; matrix: string[] }>>
}

export interface UserAvatar {
  id: string
  name: string
  base_kind: 'steve' | 'alex' | 'custom'
  is_active: boolean
  texture_data: AvatarTextureData
  created_at: string
  updated_at: string
}

export interface ActiveAvatarResponse { kind: 'default' | 'custom'; avatar: UserAvatar | null }
export interface SaveAvatarData { name: string; base_kind: string; texture_data: AvatarTextureData; preview_image_data_url?: string }
export interface UpdateAvatarData { name?: string; texture_data: AvatarTextureData; preview_image_data_url?: string }
export interface ContactMailData { name: string; email: string; subject: string; message: string }
export interface SupportMailData { name?: string; email?: string; category: string; subject: string; message: string; server_id?: string }
export interface MailStatus { enabled: boolean }
export interface MailSentResponse { sent: boolean }
export interface PasswordChangeCodeRequest { next_secret: string; next_secret_confirmation: string }
export interface PasswordChangeConfirmRequest { code: string }
export interface PasswordChangeCodeResponse { sent: boolean; expires_in_minutes: number }
export interface PasswordChangedResponse { changed: boolean }

type AvatarPreviewPayload = {
  name?: string
  base_kind?: string
  texture_data?: AvatarTextureData
}

const request = async <T>(path: string, options: RequestInit = {}): ApiResponse<T> => {
  const token = localStorage.getItem('auth_token')
  const headers = new Headers(options.headers)
  if (!headers.has('Content-Type') && options.body !== undefined) headers.set('Content-Type', 'application/json')
  if (token) headers.set('Authorization', `Bearer ${token}`)

  const response = await fetch(`${API_BASE_URL}${path}`, { ...options, headers, credentials: 'include' })
  const data = await parseResponseBody(response)
  if (!response.ok) throw new ApiError(response.status, data)
  return { data: data as T }
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

const captureAvatarPreviewImage = (payload?: AvatarPreviewPayload): string | undefined => {
  if (payload?.texture_data) {
    const renderedPreview = renderAvatarPreviewImage(payload)
    if (renderedPreview) return renderedPreview
  }

  const canvas = document.querySelector<HTMLCanvasElement>('.avatar-canvas')
  if (!canvas) return undefined
  try {
    return canvas.toDataURL('image/png')
  } catch {
    return undefined
  }
}

const renderAvatarPreviewImage = (payload: AvatarPreviewPayload): string | undefined => {
  if (!payload.texture_data) return undefined

  const canvas = document.createElement('canvas')
  canvas.width = 512
  canvas.height = 512

  let engine: Engine | null = null
  let scene: Scene | null = null
  let root: Mesh | null = null

  try {
    engine = new Engine(canvas, true, { preserveDrawingBuffer: true, stencil: true, antialias: true })
    scene = new Scene(engine)
    scene.clearColor = new Color4(0, 0, 0, 0)

    const camera = new ArcRotateCamera('avatar-mail-preview-camera', Math.PI * 0.76, Math.PI / 2.45, 4.3, new Vector3(0, 1, 0), scene)
    camera.fov = 0.48

    const light = new HemisphericLight('avatar-mail-preview-light', new Vector3(0.25, 1, -0.35), scene)
    light.intensity = 1.15

    const avatar: UserAvatar = {
      id: 'mail-preview',
      name: payload.name || 'Avatar',
      base_kind: normalizePreviewBaseKind(payload.base_kind),
      is_active: false,
      texture_data: payload.texture_data,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    }

    root = buildCharacter(scene, createCharacterModelFromAvatar(createEditableAvatarFromApi(avatar)), new Vector3(0, 0, 0), { physics: false })
    applyWalkingPreviewPose(root)
    scene.render()

    return canvas.toDataURL('image/png')
  } catch {
    return undefined
  } finally {
    root?.dispose()
    scene?.dispose()
    engine?.dispose()
  }
}

const normalizePreviewBaseKind = (value?: string): 'steve' | 'alex' | 'custom' => {
  if (value === 'steve' || value === 'alex') return value
  return 'custom'
}

const applyWalkingPreviewPose = (root: Mesh) => {
  root.rotation.y = -0.12
  rotateAvatarPart(root, 'head', 0.04, -0.06, 0)
  rotateAvatarPart(root, 'rightArm', 0.62, 0, -0.08)
  rotateAvatarPart(root, 'leftArm', -0.62, 0, 0.08)
  rotateAvatarPart(root, 'rightLeg', -0.48, 0, 0.04)
  rotateAvatarPart(root, 'leftLeg', 0.48, 0, -0.04)
}

const rotateAvatarPart = (root: Mesh, partName: string, x: number, y: number, z: number) => {
  const part = root.getChildMeshes().find((mesh) => mesh.name.endsWith(`_${partName}`))
  if (!part) return
  part.rotation.x = x
  part.rotation.y = y
  part.rotation.z = z
}

const withAvatarPreview = <T extends AvatarPreviewPayload & { preview_image_data_url?: string }>(data: T): T => {
  const previewImageDataUrl = captureAvatarPreviewImage(data)
  return previewImageDataUrl ? { ...data, preview_image_data_url: previewImageDataUrl } : data
}

export const authApi = {
  register: (data: RegisterData): ApiResponse<AuthResponse> => request<AuthResponse>('/auth/register', { method: 'POST', body: jsonBody(data) }),
  login: (data: LoginData): ApiResponse<AuthResponse> => request<AuthResponse>('/auth/login', { method: 'POST', body: jsonBody(data) }),
  getDiscordUrl: (): ApiResponse<{ url: string }> => request<{ url: string }>('/auth/discord/url'),
}

export const userApi = {
  getProfile: (): ApiResponse<User> => request<User>('/users/me'),
  getUserById: (id: string): ApiResponse<User> => request<User>(`/users/${id}`),
  updateProfile: (data: Partial<User>): ApiResponse<User> => request<User>('/users/me', { method: 'PUT', body: jsonBody(data) }),
  deleteAccount: (): ApiResponse<null> => request<null>('/users/me', { method: 'DELETE' }),
  requestPasswordChangeCode: (data: PasswordChangeCodeRequest): ApiResponse<PasswordChangeCodeResponse> => request<PasswordChangeCodeResponse>('/users/me/password/change-code', { method: 'POST', body: jsonBody(data) }),
  confirmPasswordChange: (data: PasswordChangeConfirmRequest): ApiResponse<PasswordChangedResponse> => request<PasswordChangedResponse>('/users/me/password', { method: 'PUT', body: jsonBody(data) }),
}

export const avatarApi = {
  getActive: (): ApiResponse<ActiveAvatarResponse> => request<ActiveAvatarResponse>('/users/me/avatar'),
  clearActive: (): ApiResponse<null> => request<null>('/users/me/avatar', { method: 'DELETE' }),
  list: (): ApiResponse<UserAvatar[]> => request<UserAvatar[]>('/users/me/avatars'),
  createCopy: (data: SaveAvatarData): ApiResponse<UserAvatar> => request<UserAvatar>('/users/me/avatars', { method: 'POST', body: jsonBody(withAvatarPreview(data)) }),
  update: (id: string, data: UpdateAvatarData): ApiResponse<UserAvatar> => request<UserAvatar>(`/users/me/avatars/${id}`, { method: 'PUT', body: jsonBody(withAvatarPreview(data)) }),
  delete: (id: string): ApiResponse<null> => request<null>(`/users/me/avatars/${id}`, { method: 'DELETE' }),
  select: (id: string): ApiResponse<null> => request<null>(`/users/me/avatars/${id}/select`, { method: 'PUT' }),
}

export const serverApi = {
  createServer: (data: CreateServerData): ApiResponse<Server> => request<Server>('/servers', { method: 'POST', body: jsonBody(data) }),
  getUserServers: (): ApiResponse<Server[]> => request<Server[]>('/servers'),
  getServer: (id: string): ApiResponse<Server> => request<Server>(`/servers/${id}`),
  updateServer: (id: string, data: Partial<CreateServerData>): ApiResponse<Server> => request<Server>(`/servers/${id}`, { method: 'PUT', body: jsonBody(data) }),
  deleteServer: (id: string): ApiResponse<null> => request<null>(`/servers/${id}`, { method: 'DELETE' }),
  getServerStats: async (gameDomain: string) => {
    const statsUrl = `${gameDomain.replace(/\/+$/, '')}/stats`
    const response = await fetch(statsUrl)
    if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`)
    return response.json()
  },
}

export const mailApi = {
  status: (): ApiResponse<MailStatus> => request<MailStatus>('/mail/status'),
  contact: (data: ContactMailData): ApiResponse<MailSentResponse> => request<MailSentResponse>('/contact', { method: 'POST', body: jsonBody(data) }),
  support: (data: SupportMailData): ApiResponse<MailSentResponse> => request<MailSentResponse>('/support', { method: 'POST', body: jsonBody(data) }),
}

export default request
