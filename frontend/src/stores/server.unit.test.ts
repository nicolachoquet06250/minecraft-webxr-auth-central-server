import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useServerStore } from './server'
import { serverApi, type ServerHistoryEntry } from '@/api'

vi.mock('@/api', () => ({
  serverApi: {
    getUserServers: vi.fn(),
    getRecentServers: vi.fn(),
    getFavoriteServers: vi.fn(),
    createServer: vi.fn(),
    updateServer: vi.fn(),
    deleteServer: vi.fn(),
    recordServerVisit: vi.fn(),
    favoriteServer: vi.fn(),
    unfavoriteServer: vi.fn(),
  },
}))

const makeHistoryEntry = (id: string): ServerHistoryEntry => ({
  server: {
    id,
    owner_id: 'owner-1',
    name: `Server ${id}`,
    game_domain: `https://server-${id}.example.com`,
    is_active: true,
    created_at: '2026-01-01T00:00:00Z',
    updated_at: '2026-01-01T00:00:00Z',
  },
  is_favorite: false,
  visited_at: '2026-01-01T00:00:00Z',
})

describe('server store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('place une visite existante en tête sans doublon', async () => {
    const store = useServerStore()
    store.recentServers = [makeHistoryEntry('a'), makeHistoryEntry('b')]
    vi.mocked(serverApi.recordServerVisit).mockResolvedValue({ data: makeHistoryEntry('b') })

    const result = await store.recordServerVisit('https://server-b.example.com')

    expect(result).toBe(true)
    expect(store.recentServers.map((entry) => entry.server.id)).toEqual(['b', 'a'])
  })

  it('limite les visites récentes à 10 entrées', async () => {
    const store = useServerStore()
    store.recentServers = Array.from({ length: 10 }, (_, index) => makeHistoryEntry(String(index)))
    vi.mocked(serverApi.recordServerVisit).mockResolvedValue({ data: makeHistoryEntry('new') })

    await store.recordServerVisit('https://server-new.example.com')

    expect(store.recentServers).toHaveLength(10)
    expect(store.recentServers[0].server.id).toBe('new')
    expect(store.recentServers.some((entry) => entry.server.id === '9')).toBe(false)
  })

  it('synchronise le favori dans la liste des récents', async () => {
    const store = useServerStore()
    store.recentServers = [makeHistoryEntry('a')]
    vi.mocked(serverApi.favoriteServer).mockResolvedValue({
      data: {
        server: makeHistoryEntry('a').server,
        is_favorite: true,
        favorited_at: '2026-01-02T00:00:00Z',
      },
    })

    const result = await store.favoriteServer('a')

    expect(result).toBe(true)
    expect(store.favoriteServers).toHaveLength(1)
    expect(store.recentServers[0].is_favorite).toBe(true)
  })
})
