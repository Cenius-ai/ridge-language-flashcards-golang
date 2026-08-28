#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

echo "=== Ridge: installing dependencies ==="

# ── Node.js ──────────────────────────────────────────────────────
if ! command -v node &>/dev/null; then
  echo "→ Installing Node.js 20..."
  export DEBIAN_FRONTEND=noninteractive
  curl -fsSL https://deb.nodesource.com/setup_20.x | bash -s -- -y &>/dev/null
  apt-get install -y -qq nodejs &>/dev/null
  echo "   Node $(node --version) / npm $(npm --version)"
fi

# ── npm packages ─────────────────────────────────────────────────
echo "→ Installing npm packages..."
npm install --no-audit --no-fund --silent

# ── Tailwind CSS ──────────────────────────────────────────────────
echo "→ Building Tailwind CSS..."
npx tailwindcss -i ./static/css/input.css -o ./static/css/output.css --minify

# ── Fonts ─────────────────────────────────────────────────────────
echo "→ Copying font files..."
mkdir -p static/fonts
FONT_DIR="node_modules/@fontsource-variable/onest/files"
if [ -d "$FONT_DIR" ]; then
  cp "$FONT_DIR"/*.woff2 static/fonts/ 2>/dev/null || true
  echo "   Font files copied."
else
  echo "   Warning: font directory not found at $FONT_DIR"
fi

# ── Rust ──────────────────────────────────────────────────────────
echo "→ Ensuring Cargo is available..."
. "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.cargo/bin:$PATH"

echo "→ Building Rust project (debug profile)..."
cargo build

echo ""
echo "=== Install complete ==="
echo "Start the server: cargo run"
echo "Then open http://localhost:\${PORT:-8080}"
