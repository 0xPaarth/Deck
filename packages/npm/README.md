# Deck — Terminal-First CP Platform

Competitive programming, reimagined for the terminal. Solve problems from Codeforces, CSES, and AtCoder with a feature-rich TUI, Neovim integration, deep analytics, team collaboration, and Git sync.

## Install in 10 seconds

```bash
npm install -g deck
deck init
deck
```

## Features

- 🖥️ **Rich TUI** — Dashboard, Problems, Analytics, Team, Contest, Target, Config
- 📝 **Neovim Integration** — Split layout (35/65), LSP, test runner, quickfix
- 👥 **Team Collaboration** — Track member progress, share solutions
- 📦 **Git Sync** — Auto-commit on AC, metadata.json, profile README
- 📊 **Deep Analytics** — Weak tags, predictions, heatmap, topic proficiency
- 🏆 **Contest Mode** — Real-time timer, standings, problem grid
- 🌐 **Browser Companion** — Capture problem pages, autofill submit forms

## Quick Start

```bash
deck                          # Start TUI
deck fetch 1971D             # Fetch Codeforces problem
deck open 1971D              # Open in Neovim
```

## Neovim Plugin

Add to your plugin manager:

```lua
-- lazy.nvim
{ "deck/deck.nvim", dependencies = { "nvim-lua/plenary.nvim" }, config = true }
```

Commands:
- `:DeckOpen <id>` — Open problem in split layout
- `:DeckRun` — Run tests against samples
- `:DeckSubmit` — Submit solution
- `:DeckStats` — Show analytics floating window

## Configuration

Config paths:
- macOS: `~/Library/Application Support/deck/config.toml`
- Linux: `~/.config/deck/config.toml`
- Windows: `%APPDATA%\deck\config.toml`

## License

MIT
