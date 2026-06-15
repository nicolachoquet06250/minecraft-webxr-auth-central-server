import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useAuthStore } from './auth'
import { userApi } from '@/api'

vi.mock('@/api', () => ({
  authApi: {
    register: vi.fn(),
    login: vi.fn(),
    getDiscordUrl: vi.fn(),
  },
  userApi: {
    getProfile: vi.fn(),
    updateProfile: vi.fn(),
    unlinkDiscord: vi.fn(),
    deleteAccount: vi.fn(),
  },
}))

const localStorageMock = () => {
  const values = new Map<string, string>()
  return {
    getItem: vi.fn((key: string) => values.get(key) ?? null),
    setItem: vi.fn((key: string, value: string) => values.set(key, value)),
    removeItem: vi.fn((key: string) => values.delete(key)),
    clear: vi.fn(() => values.clear()),
  }
}

describe('auth functional flow', () => {
  beforeEach(() => {
    vi.stubGlobal('localStorage', localStorageMock())
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('supprime le compte puis vide la session locale', async () => {
    const store = useAuthStore()
    store.setToken('jwt-token')
    vi.mocked(userApi.deleteAccount).mockResolvedValue({ data: null })

    const result = await store.deleteAccount()

    expect(result).toBe(true)
    expect(store.token).toBeNull()
    expect(store.user).toBeNull()
    expect(localStorage.removeItem).toHaveBeenCalledWith('auth_token')
  })
})
