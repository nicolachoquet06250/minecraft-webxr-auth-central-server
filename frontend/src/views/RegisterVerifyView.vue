<template>
  <div class="register-verify voxicraft-bg">
    <section class="voxicraft-panel verify-panel">
      <h1 class="voxicraft-title">Vérification email</h1>
      <p class="voxicraft-text">Saisissez le code reçu par email pour finaliser la création du compte.</p>

      <form class="verify-form" @submit.prevent="submitCode">
        <label class="voxicraft-label">Email</label>
        <input v-model="email" type="email" class="voxicraft-input" required />

        <label class="voxicraft-label">Code de validation</label>
        <input v-model="code" class="voxicraft-input code-input" inputmode="numeric" maxlength="6" required />

        <button class="voxicraft-button" type="submit" :disabled="loading || code.trim().length !== 6">
          {{ loading ? 'Validation...' : 'Créer le compte' }}
        </button>
      </form>

      <p v-if="message" class="success-message">{{ message }}</p>
      <p v-if="error" class="error-message">{{ error }}</p>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const email = ref('')
const code = ref('')
const loading = ref(false)
const message = ref('')
const error = ref('')

async function submitCode() {
  loading.value = true
  message.value = ''
  error.value = ''
  try {
    message.value = 'Validation envoyée.'
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.register-verify { min-height: calc(100vh - 80px); display: flex; align-items: center; justify-content: center; padding: 2rem 1rem; }
.verify-panel { width: 100%; max-width: 520px; padding: 2rem; }
.verify-form { display: flex; flex-direction: column; gap: 1rem; }
.code-input { text-align: center; letter-spacing: .35rem; font-weight: 800; }
.success-message { color: #7cfc9a; font-weight: 700; }
.error-message { color: #ff8a80; font-weight: 700; }
</style>
