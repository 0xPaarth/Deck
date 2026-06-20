# ⌨️ Keyboard Shortcuts

Complete reference for all keyboard shortcuts in Deck.

---

## TUI Shortcuts

### Global

| Key | Action |
|---|---|
| `Tab` | Next tab |
| `Shift+Tab` | Previous tab |
| `q` | Quit Deck |
| `?` | Toggle help popup |
| `r` | Refresh current tab data |

### Navigation (Problems, Team, Contest tabs)

| Key | Action |
|---|---|
| `j` / `↓` | Next item |
| `k` / `↑` | Previous item |
| `Enter` / `n` | Open selected item |

### Dashboard Tab

| Key | Action |
|---|---|
| `Enter` on recommendation | Open problem |

### Problems Tab

| Key | Action |
|---|---|
| `j` / `k` | Navigate problem list |
| `Enter` / `n` | Open problem in Neovim |
| `/` | Search (placeholder) |

### Contest Tab

| Key | Action |
|---|---|
| `o` | Open selected problem |
| `r` | Refresh contest data |
| `t` | Toggle timer display |
| `q` | Quit contest view |

### Analytics Tab

| Key | Action |
|---|---|
| `r` | Refresh stats |
| `e` | Export analytics |
| `c` | Compare with friend |

### Target Tab

| Key | Action |
|---|---|
| `g` | Set new goal |
| `r` | Refresh progress |

---

## Neovim Shortcuts

### Global (in any buffer)

| Key | Command | Action |
|---|---|---|
| `<leader>rt` | `:DeckRun` | Compile and run tests |
| `<leader>rs` | `:DeckRunSingle` | Run single test (prompts for number) |
| `<leader>sb` | `:DeckSubmit` | Submit solution |
| `<leader>sa` | `:DeckStats` | Show stats floating window |
| `<leader>st` | `:DeckTeam` | Show team status |
| `<leader>sc` | `:DeckContest` | Show contest status |
| `<leader>sf` | `:DeckFocus` | Toggle focus mode |
| `<leader>sn` | `:DeckNext` | Open next recommended problem |
| `<leader>sg` | `:DeckSync` | Sync with backend |
| `<leader>sh` | `:DeckHelp` | Show help floating window |
| `<leader>ss` | `:DeckShare` | Share solution with team |

### Focus Mode

| Key | Action |
|---|---|
| `<leader>sf` | Toggle statement between 35% and 80% width |

### Quickfix (test results)

| Key | Action |
|---|---|
| `<CR>` | Jump to test detail / show diff |
| `q` | Close quickfix |

### Floating Windows (stats, help)

| Key | Action |
|---|---|
| `q` | Close window |
| `<Esc>` | Close window |

---

## CLI Commands

All commands support `--help` for detailed usage.

```bash
deck --version              # Show version
deck --help                 # Show help

deck init                   # Initialize workspace and config
deck fetch <problem_id>     # Download problem (e.g., 1971D)
deck open <problem_id>      # Open problem in Neovim
deck stats [handle]         # Show user statistics
deck tui                    # Start interactive TUI
deck config                 # Show current configuration
```

---

## Browser Extension

### Codeforces Problem Pages

The extension auto-captures problem data when you visit a `/problem/` page.

| Action | Result |
|---|---|
| Visit CF problem page | Auto-sent to Deck backend |
| Click extension icon | Manual capture + status popup |

---

## Customizing Keymaps

Edit your Neovim config to override defaults:

```lua
require("deck").setup({
    keymaps = {
        run_tests = "<C-r>",     -- Default: <leader>rt
        submit = "<C-s>",        -- Default: <leader>sb
        focus = "<C-f>",         -- Default: <leader>sf
        -- etc.
    },
})
```
