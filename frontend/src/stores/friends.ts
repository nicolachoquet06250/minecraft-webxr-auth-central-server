import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { friendApi, userApi, type FriendEntry, type FriendRequest, type FriendUser, type PaginatedUsersResponse } from '@/api'

export const useFriendsStore = defineStore('friends', () => {
  const friends = ref<FriendEntry[]>([])
  const incomingRequests = ref<FriendRequest[]>([])
  const outgoingRequests = ref<FriendRequest[]>([])
  const searchResults = ref<PaginatedUsersResponse | null>(null)
  const loading = ref(false)
  const searchLoading = ref(false)
  const error = ref<string | null>(null)

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
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Impossible de charger les amis.'
      return false
    } finally {
      loading.value = false
    }
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

  return {
    friends,
    incomingRequests,
    outgoingRequests,
    searchResults,
    loading,
    searchLoading,
    error,
    friendIds,
    outgoingRequestReceiverIds,
    incomingRequestRequesterIds,
    incomingRequestCount,
    fetchAll,
    refreshIncomingRequests,
    searchUsers,
    sendRequest,
    acceptRequest,
    refuseRequest,
    removeFriend,
    relationStatus,
  }
})
