import { defineStore } from 'pinia'
import { ref } from 'vue'
import { serverApi, type FavoriteServerEntry, type Server, type ServerHistoryEntry, type CreateServerData } from '@/api'

export const useServerStore = defineStore('server', () => {
  const servers = ref<Server[]>([])
  const recentServers = ref<ServerHistoryEntry[]>([])
  const favoriteServers = ref<FavoriteServerEntry[]>([])
  const currentServer = ref<Server | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const fetchUserServers = async () => {
    loading.value = true
    error.value = null
    try {
      const response = await serverApi.getUserServers()
      servers.value = response.data
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to fetch servers'
      return false
    } finally {
      loading.value = false
    }
  }

  const fetchRecentServers = async () => {
    try {
      const response = await serverApi.getRecentServers()
      recentServers.value = response.data
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to fetch recent servers'
      return false
    }
  }

  const fetchFavoriteServers = async () => {
    try {
      const response = await serverApi.getFavoriteServers()
      favoriteServers.value = response.data
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to fetch favorite servers'
      return false
    }
  }

  const createServer = async (data: CreateServerData) => {
    loading.value = true
    error.value = null
    try {
      const response = await serverApi.createServer(data)
      servers.value.push(response.data)
      return true
    } catch (err: any) {
      if (err.response?.status === 503) {
        error.value = 'Le serveur de jeu est injoignable. Verifiez que le serveur est en ligne et que l URL est correcte.'
      } else if (err.response?.status === 409) {
        error.value = 'Ce domaine est deja enregistre par un autre serveur.'
      } else if (err.response?.status === 400) {
        error.value = 'Donnees invalides. Verifiez les champs du formulaire.'
      } else {
        error.value = err.response?.data?.message || 'Echec de la creation du serveur'
      }
      return false
    } finally {
      loading.value = false
    }
  }

  const updateServer = async (id: string, data: Partial<CreateServerData>) => {
    loading.value = true
    error.value = null
    try {
      const response = await serverApi.updateServer(id, data)
      const index = servers.value.findIndex((s: Server) => s.id === id)
      if (index !== -1) {
        servers.value[index] = response.data
      }
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to update server'
      return false
    } finally {
      loading.value = false
    }
  }

  const deleteServer = async (id: string) => {
    loading.value = true
    error.value = null
    try {
      await serverApi.deleteServer(id)
      servers.value = servers.value.filter((s: Server) => s.id !== id)
      recentServers.value = recentServers.value.filter((entry) => entry.server.id !== id)
      favoriteServers.value = favoriteServers.value.filter((entry) => entry.server.id !== id)
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to delete server'
      return false
    } finally {
      loading.value = false
    }
  }

  const recordServerVisit = async (serverUrl: string) => {
    try {
      const response = await serverApi.recordServerVisit(serverUrl)
      recentServers.value = [response.data, ...recentServers.value.filter((entry) => entry.server.id !== response.data.server.id)].slice(0, 10)
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to record server visit'
      return false
    }
  }

  const favoriteServer = async (id: string) => {
    try {
      const response = await serverApi.favoriteServer(id)
      favoriteServers.value = [response.data, ...favoriteServers.value.filter((entry) => entry.server.id !== id)]
      recentServers.value = recentServers.value.map((entry) => entry.server.id === id ? { ...entry, is_favorite: true, favorited_at: response.data.favorited_at } : entry)
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to favorite server'
      return false
    }
  }

  const unfavoriteServer = async (id: string) => {
    try {
      await serverApi.unfavoriteServer(id)
      favoriteServers.value = favoriteServers.value.filter((entry) => entry.server.id !== id)
      recentServers.value = recentServers.value.map((entry) => entry.server.id === id ? { ...entry, is_favorite: false, favorited_at: undefined } : entry)
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to unfavorite server'
      return false
    }
  }

  return {
    servers,
    recentServers,
    favoriteServers,
    currentServer,
    loading,
    error,
    fetchUserServers,
    fetchRecentServers,
    fetchFavoriteServers,
    createServer,
    updateServer,
    deleteServer,
    recordServerVisit,
    favoriteServer,
    unfavoriteServer,
  }
})
