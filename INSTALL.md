# Installation Guide

## Prerequisites

- **Rust** (stable ≥1.78) — install via [rustup](https://rustup.rs)
- **Node.js** (≥18) — install via [nvm](https://github.com/nvm-sh/nvm) or your package manager

## Step-by-step

### 1. Run the installer

```bash
bash install.sh
```

This script:
- Installs Node.js 20 if not present
- Installs npm packages (`tailwindcss`, `@fontsource-variable/onest`)
- Builds the Tailwind CSS output file
- Copies font files to `static/fonts/`
- Compiles the Rust project (`cargo build`)

The script is **idempotent** — safe to run multiple times.

### 2. Start the server

```bash
cargo run
```

The server:
- Creates `ridge.db` (SQLite) if missing
- Creates the schema and seeds 20 sample flashcards on first run
- Binds to `0.0.0.0:8080` (or `$PORT` if set)

Open **http://localhost:8080** in your browser.

### 3. Verify

- The dashboard shows cards due for review
- Click **Study** to flip cards and rate your recall
- Click **Cards** to browse, add, edit, or delete flashcards
- Toggle the theme button in the bottom navigation bar

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `cargo: not found` | Run `. "$HOME/.cargo/env"` or restart your shell |
| `npm: not found` | `install.sh` will install Node.js automatically |
| Port already in use | Set a different port: `PORT=9090 cargo run` |
| Missing styles | Re-run `npx tailwindcss -i ./static/css/input.css -o ./static/css/output.css --minify` |
