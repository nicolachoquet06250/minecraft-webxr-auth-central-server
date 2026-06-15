<template>
  <main class="api-docs voxicraft-bg">
    <section class="voxicraft-panel docs-panel">
      <header class="docs-header">
        <h1 class="voxicraft-title">Documentation Voxicraft</h1>
        <p class="voxicraft-text docs-intro">Quick start serveur, API centrale générée depuis OpenAPI et documentation expérimentale des mods.</p>
      </header>

      <div class="docs-layout">
        <aside class="docs-sidebar">
          <a href="#quick-start">Quick start serveur</a>
          <a href="#api-doc">API centrale</a>
          <a href="#mods-doc">Mods</a>
        </aside>

        <div class="docs-content">
          <section id="quick-start" class="docs-section">
            <span class="section-kicker">Quick start</span>
            <h2>Installer un serveur de jeu Voxicraft</h2>
            <p>Ces étapes permettent de récupérer, configurer puis lancer un serveur de jeu auto-hébergé.</p>
            <div class="doc-grid">
              <article class="doc-card primary-card">
                <h3>1. Télécharger</h3>
                <p>Toujours récupérer la dernière release publiée, sans dépendre du nom de version.</p>
                <a class="download-link" href="https://github.com/nicolachoquet06250/minecraft-webxr/releases/latest" target="_blank" rel="noopener noreferrer">Télécharger la dernière version</a>
              </article>
              <article class="doc-card"><h3>2. Extraire</h3><p>Décompresse l'archive sur la machine serveur.</p><pre>tar -xzf voxicraft-server-*.tar.gz
cd voxicraft-server</pre></article>
              <article class="doc-card"><h3>3. Configurer</h3><p>Prépare le fichier d'environnement du serveur.</p><pre>cp .env.example .env
nano .env</pre></article>
              <article class="doc-card"><h3>4. Lancer</h3><p>Démarre le binaire du serveur de jeu.</p><pre>chmod +x voxicraft_server
./voxicraft_server</pre></article>
            </div>
          </section>

          <section id="api-doc" class="docs-section">
            <span class="section-kicker">API</span>
            <h2>Documentation de l'API centrale</h2>
            <p>Cette section lit le JSON OpenAPI/Swagger et affiche les routes, les paramètres d'URL et le format JSON de réponse.</p>

            <article class="doc-card auth-card">
              <h3>Authentification</h3>
              <p>Les routes protégées utilisent un JWT dans l'en-tête HTTP.</p>
              <pre>Authorization: Bearer &lt;token&gt;</pre>
            </article>

            <article v-if="swaggerLoading" class="doc-card">Chargement du Swagger JSON...</article>
            <article v-else-if="swaggerError" class="doc-card warning-card">{{ swaggerError }}</article>

            <div v-else class="api-route-list">
              <article v-for="route in apiRoutes" :key="route.method + route.path" class="doc-card route-card">
                <div class="route-heading">
                  <span class="method-badge" :class="route.method.toLowerCase()">{{ route.method }}</span>
                  <code>{{ route.path }}</code>
                </div>
                <p v-if="route.operation.summary">{{ route.operation.summary }}</p>
                <p v-if="route.operation.description">{{ route.operation.description }}</p>

                <div class="route-block">
                  <h4>Paramètres d'URL</h4>
                  <table v-if="urlParameters(route.operation).length" class="param-table">
                    <thead><tr><th>Nom</th><th>Position</th><th>Type</th><th>Obligatoire</th><th>Regex / contrainte</th></tr></thead>
                    <tbody>
                      <tr v-for="parameter in urlParameters(route.operation)" :key="parameter.name + parameter.in">
                        <td><code>{{ parameter.name }}</code></td>
                        <td>{{ parameter.in }}</td>
                        <td><code>{{ schemaType(parameter.schema) }}</code></td>
                        <td>{{ parameter.required ? 'Oui' : 'Non' }}</td>
                        <td><code>{{ schemaConstraint(parameter.schema) || '—' }}</code></td>
                      </tr>
                    </tbody>
                  </table>
                  <p v-else class="empty-note">Aucun paramètre d'URL documenté.</p>
                </div>

                <div class="route-block">
                  <h4>Format de réponse</h4>
                  <pre>{{ responseFormat(route.operation) }}</pre>
                </div>
              </article>
            </div>
          </section>

          <section id="mods-doc" class="docs-section">
            <span class="section-kicker">Expérimental</span>
            <h2>Documentation des mods</h2>
            <p>Le système de mods est expérimental. Les fichiers client doivent rester accessibles publiquement pour fonctionner en solo et en multi.</p>
            <div class="doc-grid">
              <article class="doc-card"><h3>Manifest</h3><p>Liste des mods exposée par le serveur.</p><pre>GET /api/mods/manifest</pre></article>
              <article class="doc-card"><h3>Fichiers client</h3><p>Fichiers JavaScript et assets servis au navigateur.</p><pre>GET /mods/:id/client/...</pre></article>
              <article class="doc-card"><h3>Structure</h3><pre>mods/example-mod/
├── mod.json
├── client/mod.js
└── assets/...</pre></article>
              <article class="doc-card"><h3>Options</h3><p>Les clés optionnelles comme <code>assets</code> doivent être tolérées lorsqu'elles sont absentes.</p></article>
            </div>
          </section>
        </div>
      </div>
    </section>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'

type JsonMap = Record<string, any>

type ApiRoute = {
  path: string
  method: string
  operation: JsonMap
}

const swagger = ref<JsonMap | null>(null)
const swaggerLoading = ref(true)
const swaggerError = ref('')

const swaggerUrls = ['/api-docs/openapi.json', '/api-docs/swagger.json', '/openapi.json', '/swagger.json', '/api/openapi.json', '/api/swagger.json']

const apiRoutes = computed<ApiRoute[]>(() => {
  const methods = ['get', 'post', 'put', 'patch', 'delete']
  return Object.entries(swagger.value?.paths || {}).flatMap(([path, operations]) =>
    Object.entries(operations as JsonMap)
      .filter(([method]) => methods.includes(method.toLowerCase()))
      .map(([method, operation]) => ({ path, method: method.toUpperCase(), operation: operation as JsonMap })),
  )
})

onMounted(async () => {
  for (const url of swaggerUrls) {
    try {
      const response = await fetch(url)
      if (!response.ok) continue
      swagger.value = await response.json()
      swaggerLoading.value = false
      return
    } catch {
      // endpoint suivant
    }
  }
  swaggerLoading.value = false
  swaggerError.value = `Aucun JSON Swagger/OpenAPI trouvé : ${swaggerUrls.join(', ')}`
})

const urlParameters = (operation: JsonMap) => (operation.parameters || []).filter((parameter: JsonMap) => parameter.in === 'path' || parameter.in === 'query')

function responseFormat(operation: JsonMap) {
  const responses = operation.responses || {}
  const response = responses['200'] || responses['201'] || responses['204'] || responses.default || Object.values(responses)[0]
  const content = response?.content || {}
  const schema = content['application/json']?.schema || Object.values(content)[0]?.schema
  if (!schema) return 'Aucun body JSON documenté pour cette réponse.'
  return JSON.stringify(schemaExample(resolveSchema(schema)), null, 2)
}

function resolveSchema(schema: JsonMap): JsonMap {
  if (!schema?.$ref) return schema || {}
  const name = String(schema.$ref).split('/').pop() || ''
  return swagger.value?.components?.schemas?.[name] || swagger.value?.definitions?.[name] || schema
}

function schemaType(schema?: JsonMap): string {
  const resolved = resolveSchema(schema || {})
  if (resolved.$ref) return String(resolved.$ref).split('/').pop() || String(resolved.$ref)
  if (resolved.enum) return resolved.enum.map(String).join(' | ')
  if (resolved.format) return `${resolved.type || 'string'}:${resolved.format}`
  if (resolved.type === 'array') return `${schemaType(resolved.items)}[]`
  return resolved.type || 'object'
}

function schemaConstraint(schema?: JsonMap): string {
  const resolved = resolveSchema(schema || {})
  if (resolved.pattern) return resolved.pattern
  if (resolved.enum) return `enum: ${resolved.enum.map(String).join(', ')}`
  if (resolved.format) return `format: ${resolved.format}`
  return ''
}

function schemaExample(schema?: JsonMap): unknown {
  const resolved = resolveSchema(schema || {})
  if (resolved.allOf) return Object.assign({}, ...resolved.allOf.map(schemaExample))
  if (resolved.oneOf) return schemaExample(resolved.oneOf[0])
  if (resolved.anyOf) return schemaExample(resolved.anyOf[0])
  if (resolved.enum?.length) return resolved.enum[0]
  if (resolved.type === 'array') return [schemaExample(resolved.items)]
  if (resolved.type === 'boolean') return true
  if (resolved.type === 'integer' || resolved.type === 'number') return 0
  if (resolved.type === 'string') return resolved.format ? `<${resolved.format}>` : 'string'

  const output: JsonMap = {}
  for (const [key, value] of Object.entries(resolved.properties || {})) output[key] = schemaExample(value as JsonMap)
  return Object.keys(output).length ? output : {}
}
</script>

<style scoped>
.api-docs { min-height: calc(100vh - 80px); padding: 2.2rem 1rem; }
.docs-panel { max-width: 1480px; margin: 0 auto; padding: 2.25rem; }
.docs-header { text-align: center; margin-bottom: 2.5rem; }
.docs-intro { max-width: 920px; margin: 1rem auto 0; line-height: 1.9; }
.docs-layout { display: grid; grid-template-columns: 240px minmax(0, 1fr); gap: 2.5rem; align-items: start; }
.docs-sidebar { position: sticky; top: 100px; display: flex; flex-direction: column; gap: .9rem; padding: 1rem; background: rgba(0,0,0,.24); border: 1px solid rgba(100,255,218,.22); border-radius: 14px; }
.docs-sidebar a, .download-link { color: #1a1a1a; background: #64ffda; border: 2px solid #1a1a1a; border-radius: 8px; padding: .9rem 1rem; text-decoration: none; font-weight: 700; line-height: 1.45; box-shadow: 3px 3px 0 rgba(0,0,0,.35); }
.docs-content { min-width: 0; }
.docs-section { padding-top: 1.5rem; margin-top: 3.75rem; scroll-margin-top: 110px; }
.docs-section:first-child { margin-top: 0; }
.section-kicker { color: #ffd700; text-transform: uppercase; font-size: .75rem; letter-spacing: .08em; }
h2 { color: #64ffda; margin: .55rem 0 1.2rem; line-height: 1.35; }
h3, h4 { color: #ffd700; margin-bottom: 1rem; line-height: 1.45; }
p, li { color: rgba(255,255,255,.82); line-height: 1.95; }
.doc-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.35rem; }
.doc-card { background: rgba(0,0,0,.28); border: 1px solid rgba(100,255,218,.18); border-radius: 12px; padding: 1.45rem; min-width: 0; }
.primary-card { border-color: rgba(255,215,0,.55); }
.auth-card, .warning-card { margin-bottom: 1.5rem; }
.warning-card { border-color: rgba(255,193,7,.55); }
pre { overflow: auto; background: rgba(0,0,0,.58); color: #d8fff6; border-radius: 10px; padding: 1rem; font-size: .85rem; line-height: 1.7; }
code { color: #64ffda; overflow-wrap: anywhere; }
.api-route-list { display: grid; gap: 1.65rem; }
.route-heading { display: flex; flex-wrap: wrap; gap: .9rem; align-items: center; margin-bottom: 1rem; }
.method-badge { display: inline-flex; justify-content: center; min-width: 72px; padding: .45rem .65rem; border-radius: 8px; color: #111; font-weight: 800; background: #64ffda; }
.method-badge.post { background: #ffd166; }
.method-badge.put, .method-badge.patch { background: #f4a261; }
.method-badge.delete { background: #ef476f; color: #fff; }
.route-block { margin-top: 1.45rem; }
.param-table { width: 100%; border-collapse: collapse; color: rgba(255,255,255,.84); overflow-x: auto; display: block; }
.param-table th, .param-table td { padding: .8rem; border: 1px solid rgba(100,255,218,.18); text-align: left; line-height: 1.6; }
.param-table th { color: #ffd700; background: rgba(0,0,0,.28); }
.empty-note { color: rgba(255,255,255,.62); font-style: italic; }
@media (max-width: 900px) { .docs-layout { grid-template-columns: 1fr; gap: 1.25rem; } .docs-sidebar { position: static; } }
@media (max-width: 768px) { .api-docs { padding: 1rem .55rem; } .docs-panel { padding: 1rem; } .doc-grid { grid-template-columns: 1fr; } .doc-card { padding: 1rem; } .docs-sidebar a { text-align: center; } }
</style>
