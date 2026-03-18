import { createResource, For, Show } from 'solid-js'
import { fetchProjects, fetchSessions } from '../api/client'
import { selectedProject, setSelectedProject, selectedSession, setSelectedSession } from '../stores/filters'
import dayjs from 'dayjs'

export function Sidebar() {
  const [projects] = createResource(fetchProjects)
  const [sessions] = createResource(selectedProject, (p) => fetchSessions(p))

  function selectProject(path: string) {
    if (selectedProject() === path) {
      setSelectedProject(undefined)
    } else {
      setSelectedProject(path)
    }
    setSelectedSession(undefined)
  }

  function selectSession(id: string) {
    setSelectedSession(selectedSession() === id ? undefined : id)
  }

  const labelStyle = {
    'font-size': '0.65rem',
    'text-transform': 'uppercase' as const,
    'letter-spacing': '0.08em',
    color: '#6c7086',
    padding: '0.5rem 0.75rem 0.25rem',
    display: 'block',
  }

  return (
    <aside style={{ width: '260px', 'min-width': '180px', 'border-right': '1px solid #313244', overflow: 'auto', background: '#181825', display: 'flex', 'flex-direction': 'column' }}>
      <span style={labelStyle}>Projects</span>
      <Show when={projects()} fallback={<div style={{ color: '#6c7086', padding: '0.5rem 0.75rem', 'font-size': '0.8rem' }}>Loading…</div>}>
        <For each={projects()}>
          {(p) => (
            <button
              onClick={() => selectProject(p.path)}
              style={{
                background: selectedProject() === p.path ? '#313244' : 'transparent',
                border: 'none',
                cursor: 'pointer',
                padding: '0.35rem 0.75rem',
                'text-align': 'left',
                color: selectedProject() === p.path ? '#cba6f7' : '#cdd6f4',
                'font-size': '0.82rem',
                'border-radius': '4px',
                display: 'flex',
                'justify-content': 'space-between',
                'align-items': 'center',
                width: '100%',
                'box-sizing': 'border-box',
              }}
            >
              <span style={{ overflow: 'hidden', 'text-overflow': 'ellipsis', 'white-space': 'nowrap', flex: 1 }} title={p.path}>
                {p.path.split('/').pop() || p.path}
              </span>
              <span style={{ 'font-size': '0.7rem', color: '#6c7086', 'margin-left': '0.4rem', 'flex-shrink': 0 }}>{p.count}</span>
            </button>
          )}
        </For>
      </Show>

      <Show when={selectedProject()}>
        <>
          <span style={{ ...labelStyle, 'margin-top': '0.5rem' }}>Sessions</span>
          <Show when={sessions()} fallback={<div style={{ color: '#6c7086', padding: '0.5rem 0.75rem', 'font-size': '0.8rem' }}>Loading…</div>}>
            <For each={sessions()}>
              {(s) => (
                <button
                  onClick={() => selectSession(s.sessionId)}
                  style={{
                    background: selectedSession() === s.sessionId ? '#313244' : 'transparent',
                    border: 'none',
                    cursor: 'pointer',
                    padding: '0.35rem 0.75rem',
                    'text-align': 'left',
                    color: selectedSession() === s.sessionId ? '#89b4fa' : '#a6adc8',
                    'font-size': '0.78rem',
                    'border-radius': '4px',
                    display: 'flex',
                    'justify-content': 'space-between',
                    width: '100%',
                    'box-sizing': 'border-box',
                  }}
                >
                  <span>{dayjs(s.firstTs).format('MM-DD HH:mm')}</span>
                  <span style={{ color: '#6c7086' }}>{s.count}</span>
                </button>
              )}
            </For>
          </Show>
        </>
      </Show>
    </aside>
  )
}
