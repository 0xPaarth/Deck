# Neovim Guide

The Deck Neovim plugin provides a split-layout coding environment with LSP, test runner, and quickfix integration.

## Setup

### Requirements

- Neovim 0.9+
- LSP configured (`clangd`, `rust-analyzer`, `pyright`, etc.)
- Deck backend running (start `deck tui` first, or the backend auto-starts)

### lazy.nvim

```lua
{
    "deck/deck.nvim",
    dependencies = { "nvim-lua/plenary.nvim" },
    config = function()
        require("deck").setup({
            default_language = "cpp",
        })
    end,
}
```

### Commands

| Command | Description |
|---|---|
| `:DeckOpen <problem_id>` | Fetch and open a problem in split layout |
| `:DeckRun` | Compile and run all sample tests |
| `:DeckRunSingle` | Prompt for test number, run single test |
| `:DeckSubmit` | Send Submit RPC + open browser submit page |
| `:DeckStats` | Show analytics floating window |
| `:DeckTeam` | Show team status floating window |
| `:DeckContest` | Show contest status floating window |
| `:DeckFocus` | Toggle statement focus mode |
| `:DeckNext` | Open next recommended problem |
| `:DeckSync` | Reconnect to Deck backend |
| `:DeckShare` | Share current solution with team |
| `:DeckHelp` | Show keyboard shortcut help |

### Layout

When you open a problem:

```
┌───────── Statement (35%) ─────────┬───────── Solution (65%) ─────────┐
│                                    │                                   │
│  D. Binary Cut                     │   // 1971D - Binary Cut          │
│                                    │   #include <bits/stdc++.h>       │
│  time limit: 1 second             │   using namespace std;           │
│  memory limit: 256 MB             │                                    │
│                                    │   int main() {                   │
│  You are given a binary string... │       ios::sync_with_stdio(false);│
│                                    │       cin.tie(nullptr);          │
│  Input                             │       return 0;                  │
│  0110101                           │   }                              │
│                                    │                                   │
│  Output                            │                                   │
│  2                                 │                                   │
│                                    │                                   │
└────────────────────────────────────┴───────────────────────────────────┘
```

- **Statement buffer**: `buftype=nofile`, `modifiable=false`, `wrap=true`
- **Solution buffer**: Regular file in `~/.deck/workspace/`, LSP auto-attached

### Focus Mode

Press `<leader>sf` to toggle statement between 35% and 80% width.

### Test Runner

`:DeckRun` performs:
1. Saves the solution file
2. Compiles based on filetype (`g++` for C++, `rustc` for Rust, `python3` for Python)
3. Runs each sample test case
4. Compares output
5. Populates the quickfix list

```
quickfix list:
=== 2/2 passed ===
✅ Test 1: Passed (2ms)
❌ Test 2: Failed
   Input: 0101010
   Expected: 2
   Got: 3
```

Press `<CR>` on a failed test to see the diff.

### Buffer-Local Keymaps

When a solution buffer is active:

| Key | Command | Action |
|---|---|---|
| `<leader>rt` | `:DeckRun` | Compile & run tests |
| `<leader>rs` | `:DeckRunSingle` | Run single test |
| `<leader>sb` | `:DeckSubmit` | Submit solution |
| `<leader>sa` | `:DeckStats` | Show stats |
| `<leader>st` | `:DeckTeam` | Show team |
| `<leader>sc` | `:DeckContest` | Show contest |
| `<leader>sf` | `:DeckFocus` | Toggle focus |
| `<leader>sn` | `:DeckNext` | Next problem |
| `<leader>sg` | `:DeckSync` | Sync backend |
| `<leader>sh` | `:DeckHelp` | Show help |
| `<leader>ss` | `:DeckShare` | Share with team |

### Customizing Keymaps

```lua
require("deck").setup({
    keymaps = {
        run_tests = "<C-r>",
        submit = "<C-s>",
        focus = "<C-f>",
    },
})
```

### LSP Integration

The plugin automatically starts LSP clients when a solution buffer opens:

| Language | LSP Command |
|---|---|
| C++ | `clangd --background-index` |
| Rust | `rust-analyzer` |
| Python | `pyright` |

Configure in Neovim:

```lua
require("deck").setup({
    lsp = {
        cpp = { "clangd", "--background-index" },
        rust = { "rust-analyzer" },
        python = { "pyright" },
    },
})
```
