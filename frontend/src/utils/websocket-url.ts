const LOCAL_HOSTNAMES = new Set(['localhost', '127.0.0.1', '::1'])

const shouldKeepExplicitPort = (url: URL) => {
  return import.meta.env.DEV || LOCAL_HOSTNAMES.has(url.hostname)
}

const normalizeWebSocketUrl = (value: string | URL) => {
  const rawUrl = typeof value === 'string' ? value : value.toString()

  try {
    const url = new URL(rawUrl)

    if ((url.protocol === 'ws:' || url.protocol === 'wss:') && !shouldKeepExplicitPort(url)) {
      url.port = ''
    }

    return typeof value === 'string' ? url.toString() : url
  } catch {
    return value
  }
}

export const installWebSocketUrlNormalizer = () => {
  if (import.meta.env.DEV || typeof window === 'undefined' || typeof window.WebSocket === 'undefined') {
    return
  }

  const NativeWebSocket = window.WebSocket

  const NormalizedWebSocket = function WebSocket(
    this: WebSocket,
    url: string | URL,
    protocols?: string | string[]
  ) {
    const normalizedUrl = normalizeWebSocketUrl(url)

    if (protocols === undefined) {
      return new NativeWebSocket(normalizedUrl)
    }

    return new NativeWebSocket(normalizedUrl, protocols)
  } as unknown as typeof WebSocket

  Object.setPrototypeOf(NormalizedWebSocket, NativeWebSocket)
  NormalizedWebSocket.prototype = NativeWebSocket.prototype

  window.WebSocket = NormalizedWebSocket
}
