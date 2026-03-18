#!/usr/bin/env bash
set -e

ROOT=$(cd "$(dirname "$0")" && pwd)
source "$HOME/.cargo/env" 2>/dev/null || true

# Start backend
cargo run --manifest-path "$ROOT/Cargo.toml" &
BACKEND_PID=$!

# Start frontend dev server
cd "$ROOT/frontend"
pnpm dev &
FRONTEND_PID=$!

trap "kill $BACKEND_PID $FRONTEND_PID 2>/dev/null" EXIT
wait
