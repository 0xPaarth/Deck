# 🚀 Quick Start Guide

Get up and running with Deck in under 2 minutes.

---

## Install

### Option 1: NPM (Recommended)

```bash
npm install -g deck
deck init
```

The `postinstall` script automatically downloads the correct binary for your platform.

### Option 2: Cargo (From Source)

```bash
git clone https://github.com/deck/deck
cd deck
cargo build --release --workspace
# Binaries: target/release/deck and target/release/deck-tui
```

### Option 3: Homebrew (macOS)

```bash
brew tap deck/deck
brew install deck
```

### Option 4: Scoop (Windows)

```powershell
scoop bucket add deck https://github.com/deck/scoop-bucket
scoop install deck
```

---

## Initialize

```bash
deck init
```

This creates:
- `~/.deck/` — Deck home directory
- `~/.deck/workspace/` — Where solution files live
- `~/.deck/repo/` — Git repository for auto-commits
- `~/.deck/config.toml` — Your configuration
- `~/.cache/deck/deck.db` — SQLite database
- `~/.cache/deck/socket` — RPC socket

---

## Your First Problem

### 1. Fetch a Problem

```bash
deck fetch 1971D
```

Output:
```
Fetched problem: 1971D - Binary Cut
Platform: Codeforces
Time limit: 1000 ms
Memory limit: 256 MB
Tags: ["dp", "greedy", "strings"]
Samples: 1 test case(s)
```

### 2. Open in Neovim

```bash
deck open 1971D
```

This:
1. Fetches the problem from Codeforces
2. Creates `~/.deck/workspace/1971D_binary_cut.cpp` with a template
3. Opens Neovim with a split layout:
   - **Left (35%)**: Problem statement
   - **Right (65%)**: Solution file

### 3. Solve It

Edit the solution file. Then press `<leader>rt` (or run `:DeckRun`) to:
1. Compile your code
2. Run against all sample test cases
3. Show results in the quickfix list

### 4. Submit

Press `<leader>sb` (or run `:DeckSubmit`) to open the Codeforces submit page in your browser.

### 5. Check Stats

Run `:DeckStats` in Neovim or switch to the **Analytics** tab in the TUI.

---

## Using the TUI

```bash
deck tui
```

### Navigation

| Key | Action |
|---|---|
| `Tab` / `Shift+Tab` | Switch tabs |
| `j` / `k` | Move up/down in lists |
| `Enter` / `n` | Open selected problem |
| `r` | Refresh |
| `q` | Quit |
| `?` | Help |

### Tabs

1. **Dashboard** — Stats, streak, recommendations
2. **Problems** — Browse problems, filter by tags
3. **Analytics** — Weak tags, predictions, time distribution
4. **Team** — Member progress (if in a team)
5. **Contest** — Upcoming contests, join, track progress
6. **Target** — Rating goals, practice plans
7. **Config** — Settings, Git, language

---

## Next Steps

- [Install the Neovim plugin](/README.md#neovim-plugin)
- [Install the browser extension](/browser/README.md)
- [Join or create a team](/README.md#team-collaboration)
- [Configure Git auto-commit](config.toml)

---

## Troubleshooting

### "Deck backend not running" in Neovim

The TUI starts the RPC server automatically. If you only use Neovim:

```bash
deck tui &  # Start TUI in background
# or run the backend server directly from the backend binary
```

### "No pre-built binary for platform" on NPM install

Build from source with Cargo.

### Neovim plugin not loading

Ensure the plugin is in your runtimepath:

```vim
:set runtimepath?
```

For `lazy.nvim`, add `{ "deck/deck.nvim", config = true }` to your plugins.
