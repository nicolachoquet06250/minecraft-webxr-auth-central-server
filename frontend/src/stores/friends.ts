import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { friendApi, userApi, type FriendEntry, type FriendPresence, type FriendPresenceServer, type FriendRequest, type FriendUser, type PaginatedUsersResponse } from '@/api'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'
const AUTH_TOKEN_STORAGE_KEY = 'auth_token'

type PresenceRealtimeMessage = {
  type?: 'multiplayer_join' | 'multiplayer_leave'
  payload?: {
    user_id?: string
    server?: FriendPresenceServer | null
  }
}

export const useFriendsStore = defineStore('friends', () => {
  const friends = ref<FriendEntry[]>([])
  const incomingRequests = ref<FriendRequest[]>([])
  const outgoingRequests = ref<FriendRequest[]>([])
  const friendPresence = ref<Record<string, FriendPresence>>({})
  const searchResults = ref<PaginatedUsersResponse | null>(null)
  const loading = ref(false)
  const presenceLoading = ref(false)
  const searchLoading = ref(false)
  const error = ref<string | null>(null)
  let presenceSocket: WebSocket | null = null
  let presenceReconnectTimer: number | null = null

  const friendIds = computed(() => new Set(friends.value.map((entry) => entry.user.id)))
  const outgoingRequestReceiverIds = computed(() => new Set(outgoingRequests.value.map((request) => request.receiver.id)))
  const incomingRequestRequesterIds = computed(() => new Set(incomingRequests.value.map((request) => request.requester.id)))
  const incomingRequestCount = computed(() => incomingRequests.value.length)

  const fetchAll = async () => {
    loading.value = true
    error.value = null
    try {
      const [friendsResponse, incomingResponse, outgoingResponse] = await Promise.all([
        friendApi.getFriends(),
        friendApi.getIncomingRequests(),
        friendApi.getOutgoingRequests(),
      ])
      friends.value = friendsResponse.data
      incomingRequests.value = incomingResponse.data
      outgoingRequests.value = outgoingResponse.data
      await fetchPresence()
      connectPresenceSocket()
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Impossible de charger les amis.'
      return false
    } finally {
      loading.value = false
    }
  }

  const fetchPresence = async () => {
    presenceLoading.value = true
    try {
      const response = await friendApi.getPresence()
      friendPresence.value = Object.fromEntries(response.data.map((presence) => [presence.user_id, presence]))
      return true
    } catch {
      return false
    } finally {
      presenceLoading.value = false
    }
  }

  const connectPresenceSocket = () => {
    const token = localStorage.getItem(AUTH_TOKEN_STORAGE_KEY)
    if (!token || presenceSocket?.readyState === WebSocket.OPEN || presenceSocket?.readyState === WebSocket.CONNECTING) return
    clearPresenceReconnectTimer()

    const socket = new WebSocket(resolvePresenceSocketUrl(token))
    presenceSocket = socket
    socket.addEventListener('message', (event) => applyPresenceRealtimeMessage(event.data))
    socket.addEventListener('close', () => {
      if (presenceSocket === socket) presenceSocket = null
      const latestToken = localStorage.getItem(AUTH_TOKEN_STORAGE_KEY)
      if (latestToken) presenceReconnectTimer = window.setTimeout(connectPresenceSocket, 1500)
    })
    socket.addEventListener('error', () => socket.close())
  }

  const disconnectPresenceSocket = () => {
    clearPresenceReconnectTimer()
    presenceSocket?.close(1000, 'client_disconnect')
    presenceSocket = null
  }

  const refreshIncomingRequests = async () => {
    try {
      const previousIds = new Set(incomingRequests.value.map((request) => request.id))
      const response = await friendApi.getIncomingRequests()
      incomingRequests.value = response.data
      return response.data.filter((request) => !previousIds.has(request.id))
    } catch {
      return []
    }
  }

  const searchUsers = async (params: { q?: string; page?: number; page_size?: number } = {}) => {
    searchLoading.value = true
    error.value = null
    try {
      const response = await userApi.searchUsers(params)
      searchResults.value = response.data
      return true
    } catch (err: any) {
      error.value = err.response?.status === 400
        ? 'La recherche doit contenir au moins 2 caractères, ou être vide pour tout lister.'
        : err.response?.data?.message || 'Impossible de rechercher les utilisateurs.'
      return false
    } finally {
      searchLoading.value = false
    }
  }

  const sendRequest = async (receiverUserId: string) => {
    error.value = null
    try {
      const response = await friendApi.createRequest({ receiver_user_id: receiverUserId })
      outgoingRequests.value = [response.data, ...outgoingRequests.value.filter((request) => request.id !== response.data.id)]
      return true
    } catch (err: any) {
      if (err.response?.status === 409) error.value = 'Une demande existe déjà ou cet utilisateur est déjà ton ami.'
      else if (err.response?.status === 404) error.value = 'Utilisateur introuvable.'
      else if (err.response?.status === 400) error.value = 'Impossible de t’ajouter toi-même.'
      else error.value = err.response?.data?.message || 'Impossible d’envoyer la demande.'
      return false
    }
  }

  const acceptRequest = async (requestId: string) => {
    error.value = null
    try {
      await friendApi.acceptRequest(requestId)
      await fetchAll()
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Impossible d’accepter la demande.'
      return false
    }
  }

  const refuseRequest = async (requestId: string) => {
    error.value = null
    try {
      await friendApi.refuseRequest(requestId)
      incomingRequests.value = incomingRequests.value.filter((request) => request.id !== requestId)
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Impossible de refuser la demande.'
      return false
    }
  }

  const removeFriend = async (userId: string) => {
    error.value = null
    try {
      await friendApi.deleteFriend(userId)
      friends.value = friends.value.filter((entry) => entry.user.id !== userId)
      outgoingRequests.value = outgoingRequests.value.filter((request) => request.receiver.id !== userId)
      incomingRequests.value = incomingRequests.value.filter((request) => request.requester.id !== userId)
      const nextPresence = { ...friendPresence.value }
      delete nextPresence[userId]
      friendPresence.value = nextPresence
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Impossible de supprimer cet ami.'
      return false
    }
  }

  const relationStatus = (user: FriendUser) => {
    if (friendIds.value.has(user.id)) return 'friend'
    if (outgoingRequestReceiverIds.value.has(user.id)) return 'outgoing'
    if (incomingRequestRequesterIds.value.has(user.id)) return 'incoming'
    return 'none'
  }

  const presenceFor = (userId: string) => friendPresence.value[userId]?.server ?? null

  const applyPresenceRealtimeMessage = (raw: unknown) => {
    if (typeof raw !== 'string') return
    try {
      const message = JSON.parse(raw) as PresenceRealtimeMessage
      const userId = message.payload?.user_id
      if (!userId || !friendIds.value.has(userId)) return
      friendPresence.value = {
        ...friendPresence.value,
        [userId]: {
          user_id: userId,
          server: message.type === 'multiplayer_join' ? message.payload?.server ?? null : null,
        },
      }
    } catch {
      // Ignore invalid realtime messages.
    }
  }

  const resolvePresenceSocketUrl = (token: string) => {
    const baseUrl = new URL(API_BASE_URL)
    baseUrl.protocol = baseUrl.protocol === 'https:' ? 'wss:' : 'ws:'
    baseUrl.pathname = `${baseUrl.pathname.replace(/\/+$/, '')}/friends/presence/realtime`
    baseUrl.search = `auth=${encodeURIComponent(token)}`
    return baseUrl.toString()
  }

  const clearPresenceReconnectTimer = () => {
    if (presenceReconnectTimer === null) return
    window.clearTimeout(presenceReconnectTimer)
    presenceReconnectTimer = null
  }

  return {
    friends,
    incomingRequests,
    outgoingRequests,
    friendPresence,
    searchResults,
    loading,
    presenceLoading,
    searchLoading,
    error,
    friendIds,
    outgoingRequestReceiverIds,
    incomingRequestRequesterIds,
    incomingRequestCount,
    fetchAll,
    fetchPresence,
    connectPresenceSocket,
    disconnectPresenceSocket,
    refreshIncomingRequests,
    searchUsers,
    sendRequest,
    acceptRequest,
    refuseRequest,
    removeFriend,
    relationStatus,
    presenceFor,
  }
})