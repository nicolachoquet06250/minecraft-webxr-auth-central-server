<template>
  <main class="support-page voxicraft-bg">
    <section class="voxicraft-panel support-panel">
      <header class="page-header">
        <h1 class="voxicraft-title">Support</h1>
        <p class="voxicraft-text">Décrivez votre problème pour l’envoyer au support Voxicraft.</p>
      </header>

      <form class="mail-form" @submit.prevent="submitSupport">
        <label class="form-label">Catégorie</label>
        <select v-model="form.category" class="form-input" required>
          <option value="account">Compte</option>
          <option value="server">Serveur</option>
          <option value="avatar">Avatar</option>
          <option value="bug">Bug</option>
          <option value="other">Autre</option>
        </select>

        <label class="form-label">URL du serveur concerné{{ isServerCategory ? '' : ', optionnel' }}</label>
        <input v-model="form.server_url" class="form-input" type="url" maxlength="240" placeholder="https://mon-serveur.example.com" :required="isServerCategory" />
        <p v-if="isBugCategory" class="field-hint">Si l’URL est renseignée, le propriétaire du serveur sera notifié.</p>

        <template v-if="authStore.isAuthenticated">
          <input type="hidden" :value="supportEmail" name="email" />
        </template>
        <template v-else>
          <label class="form-label">Email de réponse</label>
          <input v-model="form.email" type="email" class="form-input" maxlength="180" required />
        </template>

        <label class="form-label">Sujet</label>
        <input v-model="form.subject" class="form-input" maxlength="180" required />

        <label class="form-label">Message</label>
        <textarea v-model="form.message" class="form-input form-textarea" maxlength="5000" required></textarea>

        <button class="submit-button" type="submit" :disabled="sending">{{ sending ? 'Envoi...' : 'Envoyer au support' }}</button>
      </form>

      <p v-if="successMessage" class="success">{{ successMessage }}</p>
      <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
    </section>
  </main>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { mailApi } from '@/api'
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()
const form = ref({ category: 'bug', server_url: '', email: '', subject: '', message: '' })
const sending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')
const isServerCategory = computed(() => form.value.category === 'server')
const isBugCategory = computed(() => form.value.category === 'bug')
const supportEmail = computed(() => authStore.isAuthenticated ? authStore.user?.email || '' : form.value.email)

async function submitSupport() {
  sending.value = true
  successMessage.value = ''
  errorMessage.value = ''
  try {
    await mailApi.support({
      category: form.value.category,
      server_url: form.value.server_url || undefined,
      email: supportEmail.value,
      subject: form.value.subject,
      message: form.value.message,
    })
    successMessage.value = 'Demande envoyée au support.'
    form.value = { category: 'bug', server_url: '', email: '', subject: '', message: '' }
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'Erreur lors de l’envoi au support.'
  } finally {
    sending.value = false
  }
}
</script>

<style scoped>
.support-page { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.support-panel { max-width: 760px; margin: 0 auto; padding: 2rem; }
.page-header { text-align: center; margin-bottom: 2rem; }
.mail-form { display: flex; flex-direction: column; gap: .85rem; }
.form-label { color: #ffd700; font-weight: 700; }
.form-input { width: 100%; border: 2px solid #424242; border-radius: 8px; background: rgba(0,0,0,.55); color: #fff; padding: .85rem; }
.field-hint { margin: -.45rem 0 .25rem; color: #d7ccc8; font-size: .86rem; line-height: 1.5; }
.form-textarea { min-height: 180px; resize: vertical; }
.submit-button { margin-top: .5rem; border: 3px solid #2e7d32; border-radius: 8px; background: #4caf50; color: #fff; padding: .95rem 1rem; font-weight: 800; cursor: pointer; }
.submit-button:disabled { opacity: .6; cursor: not-allowed; }
.success { color: #7cfc9a; font-weight: 700; }
.error { color: #ff8a80; font-weight: 700; }
</style>