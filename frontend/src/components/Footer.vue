<template>
  <footer class="footer">
    <div class="footer-container">
      <div class="footer-content">
        <div class="footer-section">
          <h3>⛏️ Voxicraft</h3>
          <p>Plateforme d'authentification sécurisée pour vos serveurs</p>
        </div>
        
        <div class="footer-section">
          <h4>Navigation</h4>
          <ul>
            <li><router-link to="/">Accueil</router-link></li>
            <li v-if="!authStore.isAuthenticated">
              <router-link to="/register">S'inscrire</router-link>
            </li>
            <li v-if="!authStore.isAuthenticated">
              <router-link to="/login">Se connecter</router-link>
            </li>
            <li v-if="authStore.isAuthenticated">
              <router-link to="/servers">Mes Serveurs</router-link>
            </li>
            <li v-if="authStore.isAuthenticated">
              <router-link to="/profile">Mon Profil</router-link>
            </li>
          </ul>
        </div>
        
        <div class="footer-section">
          <h4>À propos</h4>
          <ul>
            <li><a href="#" @click.prevent>Documentation</a></li>
            <li><a href="#" @click.prevent>API</a></li>
            <li><a href="#" @click.prevent>Support</a></li>
            <li><a href="#" @click.prevent>Contact</a></li>
          </ul>
        </div>
      </div>
      
      <div class="footer-bottom">
        <p>&copy; {{ date.getFullYear() }} Voxicraft Auth Platform. Tous droits réservés.</p>
        <p class="tech-stack">Propulsé par Rust 🦀 & Vue.js 💚</p>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()

const date = new Date();
</script>

<style scoped>
.footer {
  background-color: rgba(0, 0, 0, 0.9);
  border-top: 4px solid #4a4a4a;
  margin-top: auto;
  padding: 2rem 0 1rem;
}

.footer-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
}

.footer-content {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
  margin-bottom: 2rem;
}

.footer-section h3 {
  font-family: 'Voxicraft', monospace;
  color: #64ffda;
  margin-bottom: 0.5rem;
  text-shadow: 2px 2px 0 #000;
}

.footer-section h4 {
  font-family: 'Voxicraft', monospace;
  color: #64ffda;
  margin-bottom: 1rem;
  font-size: 1rem;
  text-shadow: 1px 1px 0 #000;
}

.footer-section p {
  color: #aaa;
  line-height: 1.6;
}

.footer-section ul {
  list-style: none;
  padding: 0;
}

.footer-section ul li {
  margin-bottom: 0.5rem;
}

.footer-section a {
  color: #aaa;
  text-decoration: none;
  transition: color 0.2s;
}

.footer-section a:hover {
  color: #64ffda;
}

.footer-bottom {
  text-align: center;
  padding-top: 1.5rem;
  border-top: 2px solid #4a4a4a;
  color: #888;
}

.footer-bottom p {
  margin: 0.25rem 0;
  font-size: 0.9rem;
}

.tech-stack {
  font-size: 0.85rem !important;
  opacity: 0.8;
}

@media (max-width: 768px) {
  .footer-content {
    grid-template-columns: 1fr;
    text-align: center;
  }
}
</style>
