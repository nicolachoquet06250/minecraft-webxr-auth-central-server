<template>
  <div class="friend-accept voxicraft-bg">
    <div class="voxicraft-panel accept-panel">
      <h1 class="voxicraft-title">Invitation d'ami</h1>
      <p class="voxicraft-text">{{ message }}</p>
      <div v-if="loading" class="voxicraft-text loading-message">⏳ Acceptation en cours...</div>
      <div v-if="error" class="error-message">{{ error }}</div>
      <button v-if="error" class="voxicraft-button" @click="goToFriends">Retour aux amis</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { friendApi } from '@/api'

const route = useRoute()
const router = useRouter()
const loading = ref(true)
const error = ref<string | null>(null)
const message = ref('Nous validons votre invitation...')

onMounted(async () => {
  const requestId = typeof route.query.request_id === 'string' ? route.query.request_id : null
  if (!requestId) {
    loading.value = false
    error.value = 'Invitation invalide : identifiant de demande manquant.'
    message.value = 'Impossible d’accepter cette invitation.'
    return
  }

  try {
    await friendApi.acceptRequest(requestId)
    message.value = 'Invitation acceptée. Redirection vers vos amis...'
    await router.replace('/friends')
  } catch (err: any) {
    loading.value = false
    if (err.response?.status === 404) {
      error.value = 'Invitation introuvable, expirée, déjà traitée, ou destinée à un autre compte.'
    } else {
      error.value = err.response?.data?.message || 'Impossible d’accepter cette invitation.'
    }
    message.value = 'L’invitation n’a pas pu être acceptée.'
  }
})

const goToFriends = () => router.push('/friends')
</script>

<style scoped>
.friend-accept {
  min-height: calc(100vh - 80px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem 1rem;
}
.accept-panel {
  width: 100%;
  max-width: 560px;
  text-align: center;
}
.loading-message {
  margin-top: 1rem;
}
.error-message {
  color: #ff6b6b;
  margin: 1rem 0;
  background-color: rgba(255, 107, 107, 0.12);
  padding: 0.9rem;
  border-radius: 6px;
  border: 2px solid #ff6b6b;
}
</style>
