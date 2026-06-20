# 🎴 Deck

[![NPM Version](https://img.shields.io/npm/v/deck)](https://npmjs.com/package/deck)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)](https://rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/deck/deck/release.yml)](https://github.com/deck/deck/actions)

**Terminal-first competitive programming platform** with Neovim integration, Git sync, team collaboration, and deep analytics.

<p align="center">
  <img src="docs/assets/tui-dashboard.png" alt="Dashboard" width="600">
</p>

---

## ✨ Features

| Feature | Description |
|---|---|
| 🖥️ **Rich TUI** | Dashboard, Problems, Analytics, Team, Contest, Target, Config tabs |
| 📝 **Neovim Integration** | Split layout (35% statement + 65% code), LSP, test runner, quickfix |
| 👥 **Team Collaboration** | Track member progress, share solutions, leaderboards |
| 📦 **Git Sync** | Auto-commit on AC, metadata.json, auto-generated profile README |
| 📊 **Deep Analytics** | Weak tags, predictions, heatmap, topic proficiency, time distribution |
| 🏆 **Contest Mode** | Upcoming contests, real-time timer, problem status grid |
| 🎯 **Target Tracking** | Rating goals, weekly practice plans, topic mastery |
| 🌐 **Browser Companion** | Capture problems from Codeforces, autofill submit forms |

---

## 🚀 Quick Start

```bash
# Install (10 seconds)
npm install -g deck

# Initialize
deck init

# Solve your first problem
deck fetch 1971D
deck open 1971D
```

Or view the full [Getting Started Guide](docs/getting-started.md).

---

## 📦 Installation

### NPM (Recommended)

```bash
npm install -g deck
```

### Cargo (From Source)

```bash
git clone https://github.com/deck/deck
cd deck
cargo build --release --workspace
```

### Platform Packages

| Platform | Command |
|---|---|
| macOS (Homebrew) | `brew install deck` |
| Windows (Scoop) | `scoop install deck` |
| Linux (AUR) | `yay -S deck` |

See [docs/installation.md](docs/installation.md) for full details.

---

## 🎮 Usage

### CLI

```bash
deck --version               # Show version
deck init                    # Initialize workspace
deck fetch 1971D             # Download a Codeforces problem
deck open 1971D              # Open in Neovim (35/65 split)
deck tui                     # Start the interactive TUI
deck stats                   # Show user statistics
deck config                  # Show configuration
```

### TUI

```bash
deck tui
```

| Key | Action |
|---|---|
| `Tab` / `Shift+Tab` | Switch tabs |
| `j` / `k` | Navigate lists |
| `Enter` / `n` | Open problem in Neovim |
| `q` | Quit |
| `?` | Help |

### Neovim Plugin

```lua
-- lazy.nvim
{
    "deck/deck.nvim",
    dependencies = { "nvim-lua/plenary.nvim" },
    config = function()
        require("deck").setup({ default_language = "cpp" })
    end,
}
```

**Commands:**
- `:DeckOpen 1971D` — Open problem in split layout
- `:DeckRun` — Compile and run tests
- `:DeckSubmit` — Submit solution
- `:DeckStats` — Show analytics
- `:DeckFocus` — Toggle focus mode

See [docs/neovim-guide.md](docs/neovim-guide.md) for full keymaps.

---

## 🖼️ Screenshots

<p align="center">
  <img src="docs/assets/tui-problems.png" alt="Problems" width="45%">
  <img src="docs/assets/tui-analytics.png" alt="Analytics" width="45%">
</p>
<p align="center">
  <img src="docs/assets/nvim-split.png" alt="Neovim Split" width="45%">
  <img src="docs/assets/nvim-quickfix.png" alt="Quickfix" width="45%">
</p>

---

## 📁 Project Structure

```
deck/
├── backend/           # Rust backend (RPC, fetcher, analytics, git, db)
│   └── src/
│       ├── main.rs    # CLI entry point
│       ├── rpc/       # Unix socket JSON protocol
│       ├── fetcher/   # Codeforces HTML parser
│       ├── db/        # SQLite with migrations
│       ├── git/       # Auto-commit, README generation
│       ├── analytics/ # Stats computation, predictions
│       ├── team/      # Collaboration
│       └── contest/   # Contest mode
├── tui/               # Ratatui TUI frontend
│   └── src/
│       ├── main.rs    # Event loop
│       └── tabs/      # Dashboard, Problems, Analytics, Team, Contest, Target, Config
├── plugins/neovim/    # Neovim plugin (Lua)
│   └── lua/deck/      # RPC client, layout, buffers, tests, submit, analytics
├── browser/           # Browser extension (MV3)
│   ├── content.js     # Capture problem pages
│   ├── background.js  # Forward to backend
│   ├── popup/         # Extension popup UI
│   └── icons/         # Extension icons
├── packages/npm/      # NPM distribution package
├── docs/              # Documentation
└── scripts/           # Build and demo scripts
```

---

## 📚 Documentation

- [Getting Started](docs/getting-started.md) — Install, init, first problem
- [Installation](docs/installation.md) — All install methods
- [Configuration](docs/configuration.md) — Full config reference
- [TUI Guide](docs/tui-guide.md) — All tabs and keybindings
- [Neovim Guide](docs/neovim-guide.md) — Plugin setup and keymaps
- [Team Guide](docs/team-guide.md) — Collaboration
- [Contest Mode](docs/contest-mode.md) — Contest dashboard
- [Git Sync](docs/git-sync.md) — Auto-commit and README
- [Troubleshooting](docs/troubleshooting.md) — Common issues

---

## 🛠️ Development

```bash
# Build everything
cargo build --release --workspace

# Run tests
cargo test --workspace

# Run CLI
cargo run --bin deck -- fetch 1971D

# Run TUI
cargo run --bin deck-tui
```

---

## 📦 Distribution

Binaries are built for 6 targets via [GitHub Actions](.github/workflows/release.yml):

| | macOS | Linux | Windows |
|---|---|---|---|
| **x64** | ✅ | ✅ | ✅ |
| **arm64** | ✅ | ✅ | ✅ |

Install via NPM for automatic platform detection:

```bash
npm install -g deck
```

---

## 📄 License

[MIT](LICENSE)
