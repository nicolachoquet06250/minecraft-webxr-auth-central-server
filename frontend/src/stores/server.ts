import { defineStore } from 'pinia'
import { ref } from 'vue'
import { serverApi, type Server, type CreateServerData } from '@/api'

export const useServerStore = defineStore('server', () => {
  const servers = ref<Server[]>([])
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
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to delete server'
      return false
    } finally {
      loading.value = false
    }
  }

  return {
    servers,
    currentServer,
    loading,
    error,
    fetchUserServers,
    createServer,
    updateServer,
    deleteServer,
  }
})
