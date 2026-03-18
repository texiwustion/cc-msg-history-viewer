import { StatsPanel } from './components/StatsPanel'
import { SearchBar } from './components/SearchBar'
import { Sidebar } from './components/Sidebar'
import { MessageList } from './components/MessageList'

export default function App() {
  return (
    <div style={{ display: 'flex', 'flex-direction': 'column', height: '100vh', background: '#1e1e2e', color: '#cdd6f4', 'font-family': 'system-ui, sans-serif' }}>
      <header style={{ 'border-bottom': '1px solid #313244' }}>
        <div style={{ padding: '0.5rem 1rem', 'font-size': '0.9rem', 'font-weight': '600', color: '#cba6f7' }}>
          Claude History Viewer
        </div>
        <StatsPanel />
        <SearchBar />
      </header>
      <div style={{ display: 'flex', flex: 1, overflow: 'hidden' }}>
        <Sidebar />
        <main style={{ flex: 1, display: 'flex', 'flex-direction': 'column', overflow: 'hidden' }}>
          <MessageList />
        </main>
      </div>
    </div>
  )
}
