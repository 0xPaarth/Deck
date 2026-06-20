# Configuration

Deck uses a TOML config file. On first run of `deck init`, it is created automatically.

## Config Location

| OS | Path |
|---|---|
| macOS | `~/Library/Application Support/deck/config.toml` |
| Linux | `~/.config/deck/config.toml` |
| Windows | `%APPDATA%\deck\config.toml` |

## Full Config Reference

```toml
[general]
handle = "alice_cp"           # Competitive programming handle
default_language = "cpp"      # cpp | rust | python | go | java
rating_goal = 1700            # Target rating for progress tracking
editor = "nvim"               # Command to launch editor
statement_width = 0.35        # Fraction of screen for statement (0.0–1.0)

[git]
enabled = true                # Enable Git sync
repo_path = "~/.deck/repo"    # Path to Git repository
auto_commit = true            # Auto-commit on AC verdict
auto_push = false             # Auto-push to remote
branch = "main"               # Default branch
commit_message_template = "Solved {platform} {problem_id} ({result})"
private = false               # Private repo? (affects README)

[git.remote]
url = ""                      # e.g., "git@github.com:user/cp-solutions.git"
name = "origin"

[rpc]
socket_path = "127.0.0.1:4647"
bind_address = "localhost"
port = 4646                   # HTTP port for browser extension

[tui]
theme = "default"             # "default" | "dark" | "light"
refresh_interval = 30         # Seconds between auto-refresh

[tui.keymaps]
next_tab = "Tab"
prev_tab = "Shift+Tab"
quit = "q"
help = "?"
refresh = "r"
open_problem = "Enter"
navigate_down = "j"
navigate_up = "k"

[lsp]
cpp = ["clangd", "--background-index"]
rust = ["rust-analyzer"]
python = ["pyright"]
go = ["gopls"]
java = ["jdtls"]

[browser]
auto_capture = true           # Auto-capture on problem page load
forward_url = "http://localhost:4646/capture"

[team]
auto_sync = true              # Sync team data via Git
sync_interval = 300           # Seconds between syncs
```

## Neovim Plugin Config

Override defaults in your Neovim config:

```lua
require("deck").setup({
    default_language = "cpp",
    statement_width = 0.35,
    render_markdown = true,
    keymaps = {
        run_tests = "<leader>rt",
        submit = "<leader>sb",
        stats = "<leader>sa",
        team = "<leader>st",
        contest = "<leader>sc",
        focus = "<leader>sf",
        next = "<leader>sn",
        sync = "<leader>sg",
        help = "<leader>sh",
    },
    lsp = {
        cpp = { "clangd", "--background-index" },
        rust = { "rust-analyzer" },
        python = { "pyright" },
    },
})
```
