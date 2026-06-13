declare module 'swagger-ui-dist/swagger-ui-bundle' {
  type SwaggerUIOptions = {
    domNode?: HTMLElement
    spec?: Record<string, unknown>
    url?: string
    deepLinking?: boolean
    persistAuthorization?: boolean
    displayRequestDuration?: boolean
    tryItOutEnabled?: boolean
  }

  type SwaggerUIInstance = {
    authActions: {
      authorize(payload: Record<string, unknown>): void
    }
  }

  export default function SwaggerUI(options: SwaggerUIOptions): SwaggerUIInstance
}

declare module 'swagger-ui-dist/swagger-ui.css'
