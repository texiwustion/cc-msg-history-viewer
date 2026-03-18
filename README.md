# cc-msg-history-viewer

A single-binary web app for browsing your Claude Code message history. Built with Rust (Axum) + SolidJS.

## Features

- Full-text search with keyword highlighting
- Virtual scrolling for large histories
- Date range filtering and date-grouped display
- Project / session sidebar navigation
- Stats panel (messages, projects, sessions)
- Auto-reload on `history.jsonl` file changes
- Dark theme (Catppuccin Mocha)

## Quick Start

Download the latest binary from [Releases](https://github.com/texiwustion/cc-msg-history-viewer/releases), then:

```bash
tar -xzf cc-msg-viewer-*.tar.gz
cd cc-msg-viewer-*/
./cc-msg-viewer
# Open http://localhost:3001
```

### CLI Options

```
cc-msg-viewer [OPTIONS]

Options:
  --history-file <PATH>  Path to history.jsonl (default: ~/.claude/history.jsonl)
                         Env: HISTORY_PATH
  --port <PORT>          Port to listen on (default: 3001)
                         Env: PORT
```

## Development

### Prerequisites

- Rust 1.75+
- Node.js 22+
- pnpm

### Build & Run

```bash
# Build frontend → static/
cd frontend && pnpm install && pnpm build && cd ..

# Run backend (serves embedded frontend)
cargo run

# Or run backend in dev mode while frontend dev server is on :5173
cargo run &
cd frontend && pnpm dev
```

### Project Structure

```
src/            Rust backend (Axum HTTP server + file watcher)
frontend/       SolidJS frontend (Vite + TypeScript)
static/         Frontend build output (embedded by rust-embed)
```

### API Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /api/messages` | Query messages (pagination, search, date filter) |
| `GET /api/projects` | List projects |
| `GET /api/sessions` | List sessions |
| `GET /api/stats` | Summary statistics |

## License

[MIT](LICENSE)
