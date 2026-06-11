<template>
  <div class="home minecraft-bg">
    <!-- Hero Section -->
    <section class="hero-section">
      <div class="hero-content minecraft-container">
        <div class="hero-badge">
          <span class="badge-icon">⭐</span>
          <span class="badge-text">Plateforme d'authentification #1 pour Minecraft WebXR</span>
        </div>
        
        <h1 class="hero-title">
          Votre passerelle vers<br/>
          <span class="gradient-text">Minecraft en VR</span>
        </h1>
        
        <p class="hero-description">
          Gérez, sécurisez et analysez vos serveurs Minecraft WebXR depuis une plateforme centralisée. 
          Une authentification unique, des statistiques en temps réel, et une expérience immersive.
        </p>
        
        <div class="hero-buttons">
          <template v-if="!authStore.isAuthenticated">
            <router-link to="/register" class="btn btn-primary">
              <span class="btn-icon">✨</span>
              Commencer gratuitement
            </router-link>
            <router-link to="/login" class="btn btn-secondary">
              <span class="btn-icon">🔐</span>
              Se connecter
            </router-link>
          </template>
          <template v-else>
            <router-link to="/servers" class="btn btn-primary">
              <span class="btn-icon">🖥️</span>
              Mes serveurs
            </router-link>
            <router-link to="/profile" class="btn btn-secondary">
              <span class="btn-icon">👤</span>
              Mon profil
            </router-link>
          </template>
        </div>
        
        <div class="hero-stats">
          <div class="stat-item">
            <div class="stat-value">1000+</div>
            <div class="stat-label">Joueurs actifs</div>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <div class="stat-value">50+</div>
            <div class="stat-label">Serveurs hébergés</div>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <div class="stat-value">99.9%</div>
            <div class="stat-label">Uptime garanti</div>
          </div>
        </div>
      </div>
      
      <div class="hero-decoration">
        <div class="cube cube-1"></div>
        <div class="cube cube-2"></div>
        <div class="cube cube-3"></div>
      </div>
    </section>

    <!-- Features Section -->
    <section class="features-section">
      <div class="minecraft-container">
        <div class="section-header">
          <span class="section-badge">✨ Fonctionnalités</span>
          <h2 class="section-title">Tout ce dont vous avez besoin</h2>
          <p class="section-description">
            Une plateforme complète pour gérer vos serveurs Minecraft WebXR
          </p>
        </div>
        
        <div class="features-grid">
          <div class="feature-card">
            <div class="feature-icon">🎮</div>
            <h3 class="feature-title">Authentification unifiée</h3>
            <p class="feature-description">
              Connectez-vous une seule fois et accédez à tous vos serveurs. 
              Plus besoin de gérer plusieurs comptes.
            </p>
            <div class="feature-badge">SSO</div>
          </div>
          
          <div class="feature-card">
            <div class="feature-icon">📊</div>
            <h3 class="feature-title">Dashboard Analytics</h3>
            <p class="feature-description">
              Suivez vos statistiques en temps réel avec des graphiques interactifs. 
              Comprenez vos joueurs.
            </p>
            <div class="feature-badge">Real-time</div>
          </div>
          
          <div class="feature-card">
            <div class="feature-icon">🌐</div>
            <h3 class="feature-title">WebXR natif</h3>
            <p class="feature-description">
              Expérience VR/AR immersive directement dans votre navigateur. 
              Aucune installation requise.
            </p>
            <div class="feature-badge">VR Ready</div>
          </div>
          
          <div class="feature-card">
            <div class="feature-icon">🔒</div>
            <h3 class="feature-title">Sécurité renforcée</h3>
            <p class="feature-description">
              Chiffrement end-to-end, authentification JWT, et protection DDoS 
              pour vos serveurs.
            </p>
            <div class="feature-badge">Secure</div>
          </div>
          
          <div class="feature-card">
            <div class="feature-icon">⚡</div>
            <h3 class="feature-title">Performance optimale</h3>
            <p class="feature-description">
              Infrastructure optimisée pour des temps de latence minimaux. 
              Serveurs CDN mondiaux.
            </p>
            <div class="feature-badge">Fast</div>
          </div>
          
          <div class="feature-card">
            <div class="feature-icon">🛠️</div>
            <h3 class="feature-title">Gestion simplifiée</h3>
            <p class="feature-description">
              Interface intuitive pour gérer vos serveurs, utilisateurs et 
              configurations en quelques clics.
            </p>
            <div class="feature-badge">Easy</div>
          </div>
        </div>
      </div>
    </section>

    <!-- How It Works Section -->
    <section class="how-it-works-section">
      <div class="minecraft-container">
        <div class="section-header">
          <span class="section-badge">🚀 Comment ça marche</span>
          <h2 class="section-title">Lancez-vous en 3 étapes simples</h2>
        </div>
        
        <div class="steps-container">
          <div class="step-card">
            <div class="step-number">1</div>
            <div class="step-icon">✍️</div>
            <h3 class="step-title">Créez votre compte</h3>
            <p class="step-description">
              Inscription gratuite en moins de 2 minutes. Choisissez votre avatar 
              Minecraft préféré (Steve, Alex, ou Creeper).
            </p>
            <router-link v-if="!authStore.isAuthenticated" to="/register" class="step-link">
              S'inscrire maintenant →
            </router-link>
          </div>
          
          <div class="step-arrow">→</div>
          
          <div class="step-card">
            <div class="step-number">2</div>
            <div class="step-icon">🖥️</div>
            <h3 class="step-title">Enregistrez vos serveurs</h3>
            <p class="step-description">
              Ajoutez vos serveurs de jeu et de relais en quelques clics. 
              Configuration automatique et vérification de connectivité.
            </p>
            <router-link v-if="authStore.isAuthenticated" to="/servers" class="step-link">
              Gérer mes serveurs →
            </router-link>
          </div>
          
          <div class="step-arrow">→</div>
          
          <div class="step-card">
            <div class="step-number">3</div>
            <div class="step-icon">🎯</div>
            <h3 class="step-title">Jouez et analysez</h3>
            <p class="step-description">
              Profitez de votre expérience VR et suivez vos statistiques en 
              temps réel depuis le dashboard.
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Benefits Section -->
    <section class="benefits-section">
      <div class="minecraft-container">
        <div class="benefits-content">
          <div class="benefits-text">
            <span class="section-badge dark">💎 Avantages</span>
            <h2 class="benefits-title">Pourquoi choisir notre plateforme ?</h2>
            <p class="benefits-description">
              Nous avons conçu cette plateforme pour offrir la meilleure expérience 
              possible aux administrateurs de serveurs Minecraft WebXR.
            </p>
            
            <div class="benefits-list">
              <div class="benefit-item">
                <div class="benefit-icon">✅</div>
                <div class="benefit-content">
                  <h4>Gratuit pour toujours</h4>
                  <p>Pas de frais cachés, pas de carte bancaire requise</p>
                </div>
              </div>
              
              <div class="benefit-item">
                <div class="benefit-icon">✅</div>
                <div class="benefit-content">
                  <h4>Support technique 24/7</h4>
                  <p>Notre équipe est là pour vous aider à tout moment</p>
                </div>
              </div>
              
              <div class="benefit-item">
                <div class="benefit-icon">✅</div>
                <div class="benefit-content">
                  <h4>Mises à jour régulières</h4>
                  <p>Nouvelles fonctionnalités ajoutées chaque mois</p>
                </div>
              </div>
              
              <div class="benefit-item">
                <div class="benefit-icon">✅</div>
                <div class="benefit-content">
                  <h4>Open Source</h4>
                  <p>Code transparent et contributions communautaires</p>
                </div>
              </div>
            </div>
          </div>
          
          <div class="benefits-visual">
            <div class="visual-card">
              <div class="visual-icon">📈</div>
              <div class="visual-stat">
                <span class="visual-number">+250%</span>
                <span class="visual-label">Croissance moyenne</span>
              </div>
            </div>
            
            <div class="visual-card">
              <div class="visual-icon">⚡</div>
              <div class="visual-stat">
                <span class="visual-number">&lt;50ms</span>
                <span class="visual-label">Latence moyenne</span>
              </div>
            </div>
            
            <div class="visual-card">
              <div class="visual-icon">🛡️</div>
              <div class="visual-stat">
                <span class="visual-number">100%</span>
                <span class="visual-label">Sécurisé</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA Section -->
    <section class="cta-section">
      <div class="minecraft-container">
        <div class="cta-content">
          <h2 class="cta-title">Prêt à commencer votre aventure ?</h2>
          <p class="cta-description">
            Rejoignez des centaines d'administrateurs qui font confiance à notre plateforme 
            pour gérer leurs serveurs Minecraft WebXR.
          </p>
          
          <div class="cta-buttons">
            <template v-if="!authStore.isAuthenticated">
              <router-link to="/register" class="btn btn-cta">
                <span class="btn-icon">🚀</span>
                Créer mon compte gratuitement
              </router-link>
              <a href="https://github.com/nicolachoquet06250/minecraft-webxr" 
                 target="_blank" 
                 class="btn btn-outline">
                <span class="btn-icon">⭐</span>
                Voir sur GitHub
              </a>
            </template>
            <template v-else>
              <router-link to="/servers" class="btn btn-cta">
                <span class="btn-icon">🖥️</span>
                Accéder à mes serveurs
              </router-link>
            </template>
          </div>
          
          <div class="cta-note">
            <span class="note-icon">🎁</span>
            <span class="note-text">Gratuit pour toujours • Aucune carte bancaire requise</span>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()
</script>

<style scoped>
/* ========== Global Styles ========== */
.home {
  min-height: 100vh;
}

/* ========== Hero Section ========== */
.hero-section {
  position: relative;
  padding: 8rem 2rem 6rem;
  overflow: hidden;
}

.hero-content {
  max-width: 900px;
  margin: 0 auto;
  text-align: center;
  position: relative;
  z-index: 2;
}

.hero-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1.25rem;
  background: rgba(100, 255, 218, 0.1);
  border: 2px solid rgba(100, 255, 218, 0.3);
  border-radius: 50px;
  margin-bottom: 2rem;
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.05); }
}

.badge-icon {
  font-size: 1rem;
}

.badge-text {
  font-size: 0.65rem;
  color: #64ffda;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.hero-title {
  font-size: 3.5rem;
  line-height: 1.2;
  margin-bottom: 1.5rem;
  text-shadow: 4px 4px 0 rgba(0, 0, 0, 0.5);
  color: #fff;
}

.gradient-text {
  background: linear-gradient(135deg, #64ffda 0%, #4caf50 50%, #ffd700 100%);
  background-size: 200% 200%;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  animation: gradientMove 3s ease infinite;
}

@keyframes gradientMove {
  0%, 100% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
}

.hero-description {
  font-size: 0.9rem;
  line-height: 1.8;
  color: rgba(255, 255, 255, 0.8);
  max-width: 700px;
  margin: 0 auto 2.5rem;
}

.hero-buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
  margin-bottom: 4rem;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.9rem 2rem;
  font-size: 0.85rem;
  font-family: 'Press Start 2P', cursive;
  text-decoration: none;
  border: 3px solid;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.btn::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  transform: translate(-50%, -50%);
  transition: width 0.6s, height 0.6s;
}

.btn:hover::before {
  width: 300px;
  height: 300px;
}

.btn-icon {
  font-size: 1rem;
  position: relative;
  z-index: 1;
}

.btn-primary {
  background: linear-gradient(135deg, #4caf50, #66bb6a);
  border-color: #2e7d32;
  color: #fff;
}

.btn-primary:hover {
  transform: translateY(-3px);
  box-shadow: 0 8px 20px rgba(76, 175, 80, 0.4);
}

.btn-secondary {
  background: linear-gradient(135deg, #2196f3, #42a5f5);
  border-color: #1565c0;
  color: #fff;
}

.btn-secondary:hover {
  transform: translateY(-3px);
  box-shadow: 0 8px 20px rgba(33, 150, 243, 0.4);
}

.hero-stats {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 2rem;
  flex-wrap: wrap;
  padding: 2rem;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 15px;
  border: 2px solid rgba(100, 255, 218, 0.2);
}

.stat-item {
  text-align: center;
}

.stat-value {
  font-size: 2rem;
  color: #64ffda;
  margin-bottom: 0.5rem;
  text-shadow: 2px 2px 0 rgba(0, 0, 0, 0.5);
}

.stat-label {
  font-size: 0.65rem;
  color: rgba(255, 255, 255, 0.7);
  text-transform: uppercase;
  letter-spacing: 1px;
}

.stat-divider {
  width: 2px;
  height: 40px;
  background: rgba(100, 255, 218, 0.3);
}

/* Hero Decoration */
.hero-decoration {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  opacity: 0.1;
}

.cube {
  position: absolute;
  width: 100px;
  height: 100px;
  background: #64ffda;
  animation: float 6s ease-in-out infinite;
}

.cube-1 {
  top: 10%;
  left: 10%;
  animation-delay: 0s;
}

.cube-2 {
  top: 60%;
  right: 10%;
  animation-delay: 2s;
}

.cube-3 {
  bottom: 10%;
  left: 50%;
  animation-delay: 4s;
}

@keyframes float {
  0%, 100% { transform: translateY(0px) rotate(0deg); }
  50% { transform: translateY(-20px) rotate(180deg); }
}

/* ========== Features Section ========== */
.features-section {
  padding: 6rem 2rem;
  background: rgba(0, 0, 0, 0.2);
}

.section-header {
  text-align: center;
  margin-bottom: 4rem;
}

.section-badge {
  display: inline-block;
  padding: 0.5rem 1.5rem;
  background: rgba(100, 255, 218, 0.1);
  border: 2px solid rgba(100, 255, 218, 0.3);
  border-radius: 50px;
  font-size: 0.65rem;
  color: #64ffda;
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 1.5rem;
}

.section-badge.dark {
  background: rgba(0, 0, 0, 0.3);
}

.section-title {
  font-size: 2.5rem;
  margin-bottom: 1rem;
  text-shadow: 3px 3px 0 rgba(0, 0, 0, 0.5);
  color: #fff;
}

.section-description {
  font-size: 0.85rem;
  color: rgba(255, 255, 255, 0.7);
  max-width: 600px;
  margin: 0 auto;
  line-height: 1.6;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.feature-card {
  background: rgba(0, 0, 0, 0.4);
  border: 3px solid rgba(100, 255, 218, 0.2);
  padding: 2rem;
  border-radius: 10px;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.feature-card::before {
  content: '';
  position: absolute;
  top: -2px;
  left: -2px;
  right: -2px;
  bottom: -2px;
  background: linear-gradient(135deg, #64ffda, #4caf50);
  border-radius: 10px;
  opacity: 0;
  transition: opacity 0.3s ease;
  z-index: -1;
}

.feature-icon {
  font-size: 3rem;
  margin-bottom: 1.5rem;
  display: block;
}

.feature-title {
  font-size: 1.1rem;
  margin-bottom: 1rem;
  color: #64ffda;
}

.feature-description {
  font-size: 0.7rem;
  line-height: 1.6;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 1.5rem;
}

.feature-badge {
  display: inline-block;
  padding: 0.4rem 1rem;
  background: rgba(100, 255, 218, 0.2);
  border: 2px solid #64ffda;
  border-radius: 50px;
  font-size: 0.6rem;
  color: #64ffda;
  text-transform: uppercase;
}

/* ========== How It Works Section ========== */
.how-it-works-section {
  padding: 6rem 2rem;
  overflow-x: hidden;
}

.steps-container {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 2rem;
  max-width: 1200px;
  margin: 0 auto;
  flex-wrap: wrap;
}

.step-card {
  flex: 1;
  min-width: 280px;
  max-width: 350px;
  background: rgba(0, 0, 0, 0.4);
  border: 3px solid rgba(100, 255, 218, 0.2);
  padding: 2.5rem 2rem;
  border-radius: 15px;
  text-align: center;
  position: relative;
}

.step-number {
  position: absolute;
  top: -20px;
  left: 50%;
  transform: translateX(-50%);
  width: 50px;
  height: 50px;
  background: linear-gradient(135deg, #64ffda, #4caf50);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  font-weight: bold;
  color: #000;
  border: 3px solid #1a1a1a;
}

.step-icon {
  font-size: 3rem;
  margin: 1.5rem 0;
}

.step-title {
  font-size: 1.1rem;
  margin-bottom: 1rem;
  color: #64ffda;
}

.step-description {
  font-size: 0.7rem;
  line-height: 1.6;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 1.5rem;
}

.step-link {
  display: inline-block;
  color: #64ffda;
  text-decoration: none;
  font-size: 0.7rem;
  transition: all 0.3s ease;
}

.step-link:hover {
  transform: translateX(5px);
  text-decoration: underline;
}

.step-arrow {
  font-size: 2rem;
  color: rgba(100, 255, 218, 0.5);
  animation: bounce 2s ease-in-out infinite;
}

@keyframes bounce {
  0%, 100% { transform: translateX(0); }
  50% { transform: translateX(10px); }
}

/* ========== Benefits Section ========== */
.benefits-section {
  padding: 6rem 2rem;
  background: rgba(0, 0, 0, 0.2);
  overflow-x: hidden;
}

.benefits-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4rem;
  align-items: center;
  max-width: 1200px;
  margin: 0 auto;
}

.benefits-text {
  overflow-wrap: break-word;
  word-wrap: break-word;
}

.benefits-title {
  font-size: 2.5rem;
  margin: 1rem 0 1.5rem;
  text-shadow: 3px 3px 0 rgba(0, 0, 0, 0.5);
  color: #fff;
  word-wrap: break-word;
}

.benefits-description {
  font-size: 0.85rem;
  line-height: 1.6;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 2rem;
  word-wrap: break-word;
}

.benefits-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.benefit-item {
  display: flex;
  gap: 1rem;
  align-items: flex-start;
}

.benefit-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.benefit-content h4 {
  font-size: 0.9rem;
  color: #64ffda;
  margin-bottom: 0.5rem;
}

.benefit-content p {
  font-size: 0.7rem;
  color: rgba(255, 255, 255, 0.7);
  line-height: 1.5;
}

.benefits-visual {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.visual-card {
  background: rgba(0, 0, 0, 0.4);
  border: 3px solid rgba(100, 255, 218, 0.2);
  padding: 2rem;
  border-radius: 15px;
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.visual-icon {
  font-size: 3rem;
}

.visual-stat {
  display: flex;
  flex-direction: column;
}

.visual-number {
  font-size: 2rem;
  color: #64ffda;
  text-shadow: 2px 2px 0 rgba(0, 0, 0, 0.5);
}

.visual-label {
  font-size: 0.65rem;
  color: rgba(255, 255, 255, 0.7);
  text-transform: uppercase;
  margin-top: 0.5rem;
}

/* ========== CTA Section ========== */
.cta-section {
  padding: 6rem 2rem;
  text-align: center;
}

.cta-content {
  max-width: 800px;
  margin: 0 auto;
  padding: 4rem 3rem;
  background: rgba(0, 0, 0, 0.4);
  border: 3px solid rgba(100, 255, 218, 0.3);
  border-radius: 20px;
  position: relative;
  overflow: hidden;
}

.cta-content::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: radial-gradient(circle, rgba(100, 255, 218, 0.1) 0%, transparent 70%);
  animation: rotate 10s linear infinite;
}

@keyframes rotate {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.cta-title {
  font-size: 2.5rem;
  margin-bottom: 1.5rem;
  text-shadow: 3px 3px 0 rgba(0, 0, 0, 0.5);
  position: relative;
  z-index: 1;
}

.cta-description {
  font-size: 0.85rem;
  line-height: 1.6;
  color: rgba(255, 255, 255, 0.8);
  margin-bottom: 2.5rem;
  position: relative;
  z-index: 1;
}

.cta-buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
  margin-bottom: 2rem;
  position: relative;
  z-index: 1;
}

.btn-cta {
  background: linear-gradient(135deg, #ffd700, #ffed4e);
  border-color: #ffa000;
  color: #000;
  font-weight: bold;
  padding: 1rem 2.5rem;
}

.btn-cta:hover {
  transform: translateY(-5px) scale(1.05);
  box-shadow: 0 10px 30px rgba(255, 215, 0, 0.5);
}

.btn-outline {
  background: transparent;
  border-color: #64ffda;
  color: #64ffda;
}

.btn-outline:hover {
  background: rgba(100, 255, 218, 0.1);
  transform: translateY(-3px);
}

.cta-note {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  font-size: 0.7rem;
  color: rgba(255, 255, 255, 0.7);
  position: relative;
  z-index: 1;
}

.note-icon {
  font-size: 1rem;
}

/* ========== Responsive Design ========== */
@media (max-width: 1024px) {
  .benefits-content {
    grid-template-columns: 1fr;
    gap: 2rem;
  }
  
  .steps-container {
    flex-direction: column;
    align-items: center;
  }
  
  .step-card {
    max-width: 100%;
    width: 100%;
  }
  
  .step-arrow {
    transform: rotate(90deg);
  }
}

@media (max-width: 768px) {
  .hero-section {
    padding: 4rem 1rem 3rem;
  }
  
  .hero-title {
    font-size: 2rem;
    word-wrap: break-word;
  }
  
  .hero-description {
    font-size: 0.75rem;
  }
  
  .section-title {
    font-size: 1.8rem;
    word-wrap: break-word;
  }
  
  .how-it-works-section {
    padding: 3rem 1rem;
    overflow-x: hidden;
  }
  
  .steps-container {
    gap: 1.5rem;
    align-items: center;
    width: 100%;
  }
  
  .step-card {
    min-width: auto;
    width: 100%;
    max-width: 100%;
    padding: 2rem 1.5rem;
    margin: 0 auto;
  }
  
  .step-arrow {
    display: none;
  }
  
  .step-title {
    font-size: 1rem;
  }
  
  .step-description {
    font-size: 0.65rem;
  }
  
  .benefits-section {
    padding: 3rem 1rem;
  }
  
  .benefits-content {
    gap: 1.5rem;
  }
  
  .benefits-title {
    font-size: 1.5rem;
    word-wrap: break-word;
  }
  
  .benefits-description {
    font-size: 0.75rem;
  }
  
  .benefit-content h4 {
    font-size: 0.8rem;
  }
  
  .benefit-content p {
    font-size: 0.65rem;
  }
  
  .visual-card {
    padding: 1.5rem;
    gap: 1rem;
  }
  
  .visual-number {
    font-size: 1.5rem;
  }
  
  .cta-title {
    font-size: 1.8rem;
    word-wrap: break-word;
  }
  
  .features-grid {
    grid-template-columns: 1fr;
  }
  
  .hero-buttons {
    flex-direction: column;
    width: 100%;
  }
  
  .btn {
    width: 100%;
    justify-content: center;
  }
  
  .hero-stats {
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .stat-divider {
    width: 80%;
    height: 2px;
  }
  
  .cta-content {
    padding: 2rem 1.5rem;
  }
}
</style>
