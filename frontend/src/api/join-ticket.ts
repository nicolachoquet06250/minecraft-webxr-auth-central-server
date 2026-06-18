const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'

export interface JoinTicketResponse {
  ticket: string
  join_url: string
  expires_in_seconds: number
}

export async function createJoinTicket(serverId: string): Promise<JoinTicketResponse> {
  const token = localStorage.getItem('auth_token')
  const headers = new Headers()
  headers.set('Accept', 'application/json')
  if (token) headers.set('Authorization', `Bearer ${token}`)

  const response = await fetch(`${API_BASE_URL}/servers/${serverId}/join-ticket`, {
    method: 'POST',
    headers,
    credentials: 'include',
  })

  const data = await response.json().catch(() => null)
  if (!response.ok) {
    throw new Error(data?.message || data?.error || `HTTP error! status: ${response.status}`)
  }

  return data as JoinTicketResponse
}
