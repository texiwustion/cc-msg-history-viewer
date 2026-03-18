import { createSignal, onCleanup } from 'solid-js'
import { setSearchQuery, setFromTs, setToTs, setSelectedProject, setSelectedSession } from '../stores/filters'

export function SearchBar() {
  const [inputVal, setInputVal] = createSignal('')
  let debounceTimer: ReturnType<typeof setTimeout>

  function onInput(e: InputEvent) {
    const v = (e.target as HTMLInputElement).value
    setInputVal(v)
    clearTimeout(debounceTimer)
    debounceTimer = setTimeout(() => setSearchQuery(v), 300)
  }

  onCleanup(() => clearTimeout(debounceTimer))

  function onFromChange(e: Event) {
    const v = (e.target as HTMLInputElement).value
    setFromTs(v ? new Date(v).getTime() : undefined)
  }

  function onToChange(e: Event) {
    const v = (e.target as HTMLInputElement).value
    setToTs(v ? new Date(v).getTime() : undefined)
  }

  function onClear() {
    setInputVal('')
    setSearchQuery('')
    setFromTs(undefined)
    setToTs(undefined)
    setSelectedProject(undefined)
    setSelectedSession(undefined)
  }

  return (
    <div style={{ display: 'flex', gap: '0.5rem', padding: '0.5rem 1rem', background: '#181825', 'border-bottom': '1px solid #313244', 'flex-wrap': 'wrap', 'align-items': 'center' }}>
      <input
        type="text"
        placeholder="Search messages…"
        value={inputVal()}
        onInput={onInput}
        style={{ flex: '1 1 200px', padding: '0.4rem 0.6rem', background: '#1e1e2e', border: '1px solid #45475a', 'border-radius': '6px', color: '#cdd6f4', 'font-size': '0.9rem', outline: 'none' }}
      />
      <label style={{ color: '#6c7086', 'font-size': '0.8rem' }}>
        From&nbsp;
        <input type="date" onChange={onFromChange} style={dateInputStyle} />
      </label>
      <label style={{ color: '#6c7086', 'font-size': '0.8rem' }}>
        To&nbsp;
        <input type="date" onChange={onToChange} style={dateInputStyle} />
      </label>
      <button onClick={onClear} style={{ padding: '0.35rem 0.7rem', background: '#313244', border: 'none', 'border-radius': '6px', color: '#cdd6f4', cursor: 'pointer', 'font-size': '0.8rem' }}>
        Clear
      </button>
    </div>
  )
}

const dateInputStyle = {
  padding: '0.3rem 0.4rem',
  background: '#1e1e2e',
  border: '1px solid #45475a',
  'border-radius': '4px',
  color: '#cdd6f4',
  'font-size': '0.8rem',
}
