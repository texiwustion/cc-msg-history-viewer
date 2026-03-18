export interface Message {
  display: string
  timestamp: number
  project: string
  sessionId: string
}

export interface MessagesResponse {
  total: number
  messages: Message[]
}

export interface ProjectInfo {
  path: string
  count: number
}

export interface SessionInfo {
  sessionId: string
  project: string
  firstTs: number
  count: number
}

export interface Stats {
  totalMessages: number
  totalProjects: number
  totalSessions: number
  earliestTs: number | null
  latestTs: number | null
  dailyCounts: { date: string; count: number }[]
}

async function fetchJson<T>(input: RequestInfo): Promise<T> {
  const r = await fetch(input)
  if (!r.ok) throw new Error(`API ${r.status}: ${r.statusText}`)
  return r.json() as Promise<T>
}

function buildUrl(path: string, params: Record<string, string | number | undefined | null>): string {
  const sp = new URLSearchParams()
  for (const [k, v] of Object.entries(params)) {
    if (v !== undefined && v !== null && v !== '') {
      sp.set(k, String(v))
    }
  }
  const qs = sp.toString()
  return `/api${path}${qs ? '?' + qs : ''}`
}

export async function fetchMessages(params: {
  project?: string
  session?: string
  q?: string
  from?: number
  to?: number
  offset?: number
  limit?: number
}): Promise<MessagesResponse> {
  return fetchJson(buildUrl('/messages', params as Record<string, string | number | undefined | null>))
}

export async function fetchProjects(): Promise<ProjectInfo[]> {
  return fetchJson('/api/projects')
}

export async function fetchSessions(project?: string): Promise<SessionInfo[]> {
  return fetchJson(buildUrl('/sessions', { project }))
}

export async function fetchStats(): Promise<Stats> {
  return fetchJson('/api/stats')
}
