export type FriendAvatar = {
  kind: 'default' | 'custom'
  base_kind: string
  name: string
  url: string
}

export type FriendUser = {
  id: string
  username: string
  avatar: FriendAvatar
}

export type PaginatedUsersResponse = {
  items: FriendUser[]
  page: number
  page_size: number
  total: number
  total_pages: number
  next_url: string | null
  previous_url: string | null
}

export type FriendRequest = {
  id: string
  requester: FriendUser
  receiver: FriendUser
  status: 'pending' | 'accepted' | 'refused' | 'cancelled'
  created_at: string
  updated_at: string
}

export type FriendEntry = {
  user: FriendUser
  created_at: string
}
