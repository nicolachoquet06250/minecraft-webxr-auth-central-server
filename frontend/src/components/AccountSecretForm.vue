<template>
  <form class="account-secret-form" @submit.prevent="confirmChange">
    <h3>Changer le mot de passe</h3>

    <template v-if="!codeSent">
      <label class="form-label">Nouveau mot de passe</label>
      <input v-model="nextSecret" type="password" class="form-input" minlength="8" maxlength="128" required />

      <label class="form-label">Confirmer le nouveau mot de passe</label>
      <input v-model="nextSecretConfirmation" type="password" class="form-input" minlength="8" maxlength="128" required />

      <button class="btn-submit" type="button" :disabled="sendingCode || !canRequestCode" @click="requestCode">
        {{ sendingCode ? 'Envoi du code...' : 'Envoyer le code par mail' }}
      </button>
    </template>

    <template v-else>
      <p class="hint">Un code de validation a été envoyé par mail. Il expire dans {{ expiresInMinutes }} minutes.</p>
      <label class="form-label">Code reçu par mail</label>
      <input v-model="code" class="form-input code-input" inputmode="numeric" maxlength="6" required />

      <div class="form-actions">
        <button class="btn-submit" type="submit" :disabled="saving || code.trim().length !== 6">
          {{ saving ? 'Enregistrement...' : 'Enregistrer le mot de passe' }}
        </button>
        <button class="btn-cancel" type="button" :disabled="sendingCode" @click="requestCode">Renvoyer un code</button>
      </div>
    </template>

    <p v-if="successMessage" class="success">{{ successMessage }}</p>
    <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
  </form>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { userApi } from '@/api'

const nextSecret = ref('')
const nextSecretConfirmation = ref('')
const code = ref('')
const codeSent = ref(false)
const expiresInMinutes = ref(10)
const sendingCode = ref(false)
const saving = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

const canRequestCode = computed(() => nextSecret.value.length >= 8 && nextSecret.value === nextSecretConfirmation.value)

async function requestCode() {
  successMessage.value = ''
  errorMessage.value = ''
  if (nextSecret.value.length < 8) {
    errorMessage.value = 'Le nouveau mot de passe doit contenir au moins 8 caractères.'
    return
  }
  if (nextSecret.value !== nextSecretConfirmation.value) {
    errorMessage.value = 'Les deux mots de passe ne correspondent pas.'
    return
  }
  sendingCode.value = true
  try {
    const response = await userApi.requestPasswordChangeCode({ next_secret: nextSecret.value, next_secret_confirmation: nextSecretConfirmation.value })
    expiresInMinutes.value = response.data.expires_in_minutes
    codeSent.value = true
    successMessage.value = 'Code envoyé par mail.'
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'Impossible d’envoyer le code.'
  } finally {
    sendingCode.value = false
  }
}

async function confirmChange() {
  successMessage.value = ''
  errorMessage.value = ''
  saving.value = true
  try {
    await userApi.confirmPasswordChange({ code: code.value.trim() })
    successMessage.value = 'Mot de passe modifié.'
    nextSecret.value = ''
    nextSecretConfirmation.value = ''
    code.value = ''
    codeSent.value = false
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'Code invalide ou expiré.'
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.account-secret-form { display: flex; flex-direction: column; gap: .65rem; margin-top: .85rem; padding-top: .85rem; border-top: 1px solid rgba(100,255,218,.22); }
.account-secret-form h3 { color: #ffd700; margin: 0; font-size: .78rem; }
.form-label { color: #ffd700; font-size: .62rem; }
.form-input { background: rgba(0, 0, 0, 0.6); border: 2px solid #424242; color: white; padding: .55rem; font-family: 'Courier New', monospace; font-size: .78rem; width: 100%; outline: none; border-radius: 5px; }
.code-input { text-align: center; letter-spacing: .35rem; font-weight: 800; }
.form-actions { display: flex; gap: .65rem; min-width: 0; }
.btn-submit, .btn-cancel { display: flex; align-items: center; justify-content: center; gap: .38rem; color: #fff; font-family: 'Press Start 2P', cursive; cursor: pointer; border-radius: 5px; text-decoration: none; transition: all 0.3s ease; padding: .62rem .75rem; min-width: 0; max-width: 100%; font-size: .58rem; line-height: 1.35; }
.btn-submit { background: linear-gradient(135deg, #4caf50, #66bb6a); border: 2px solid #2e7d32; flex: 1; }
.btn-cancel { background: rgba(255, 179, 0, 0.18); border: 2px solid #ffb300; color: #ffd700; flex: 1; }
.btn-submit:disabled, .btn-cancel:disabled { opacity: .55; cursor: not-allowed; }
.hint { color: rgba(255,255,255,.75); font-size: .68rem; line-height: 1.5; margin: 0; }
.success { color: #7cfc9a; font-weight: 700; font-size: .72rem; }
.error { color: #ff8a80; font-weight: 700; font-size: .72rem; }
@media (max-width: 520px) { .form-actions { flex-direction: column; } }
</style>
