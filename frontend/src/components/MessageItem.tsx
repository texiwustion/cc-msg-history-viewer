import type { Message } from '../api/client'
import dayjs from 'dayjs'

interface Props {
  message: Message
  highlight?: string
}

export function MessageItem(props: Props) {
  const ts = () => dayjs(props.message.timestamp).format('YYYY-MM-DD HH:mm')
  const project = () => props.message.project.split('/').pop() || props.message.project

  function highlight(text: string, query: string | undefined) {
    if (!query) return <>{text}</>
    const parts = text.split(new RegExp(`(${escapeRegex(query)})`, 'gi'))
    return (
      <>
        {parts.map((part) =>
          part.toLowerCase() === query.toLowerCase()
            ? <mark style={{ background: '#f9e2af', color: '#1e1e2e', 'border-radius': '2px', padding: '0 1px' }}>{part}</mark>
            : part
        )}
      </>
    )
  }

  return (
    <div style={{
      padding: '0.6rem 1rem',
      'border-bottom': '1px solid #313244',
      display: 'flex',
      'flex-direction': 'column',
      gap: '0.25rem',
    }}>
      <div style={{ display: 'flex', gap: '0.75rem', 'align-items': 'center', 'flex-wrap': 'wrap' }}>
        <span style={{ 'font-size': '0.72rem', color: '#6c7086', 'flex-shrink': 0 }}>{ts()}</span>
        <span style={{ 'font-size': '0.72rem', color: '#89b4fa', overflow: 'hidden', 'text-overflow': 'ellipsis', 'white-space': 'nowrap' }} title={props.message.project}>
          {project()}
        </span>
      </div>
      <div style={{ color: '#cdd6f4', 'font-size': '0.9rem', 'white-space': 'pre-wrap', 'word-break': 'break-word' }}>
        {highlight(props.message.display, props.highlight)}
      </div>
    </div>
  )
}

function escapeRegex(s: string) {
  return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}
