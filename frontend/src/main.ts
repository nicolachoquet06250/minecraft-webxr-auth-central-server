import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import './style.css'
import './profile-restore.css'
import './profile-mobile-fix.css'
import './friends-mobile-fix.css'
import './friends-request-card-exact.css'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)

app.mount('#app')
