import { createResource, Show } from 'solid-js'
import { fetchStats } from '../api/client'
import dayjs from 'dayjs'

export function StatsPanel() {
  const [stats] = createResource(fetchStats)

  return (
    <Show when={stats()}>
      {(s) => (
        <div style={{ display: 'flex', gap: '1.5rem', padding: '0.75rem 1rem', background: '#1e1e2e', 'border-bottom': '1px solid #313244', 'flex-wrap': 'wrap' }}>
          <Stat label="Messages" value={s().totalMessages.toLocaleString()} />
          <Stat label="Projects" value={s().totalProjects.toLocaleString()} />
          <Stat label="Sessions" value={s().totalSessions.toLocaleString()} />
          <Show when={s().earliestTs}>
            <Stat label="Since" value={dayjs(s().earliestTs!).format('YYYY-MM-DD')} />
          </Show>
        </div>
      )}
    </Show>
  )
}

function Stat(props: { label: string; value: string }) {
  return (
    <div style={{ display: 'flex', 'flex-direction': 'column', 'align-items': 'center' }}>
      <span style={{ 'font-size': '0.7rem', color: '#6c7086', 'text-transform': 'uppercase', 'letter-spacing': '0.05em' }}>{props.label}</span>
      <span style={{ 'font-size': '1.1rem', 'font-weight': '600', color: '#cdd6f4' }}>{props.value}</span>
    </div>
  )
}
