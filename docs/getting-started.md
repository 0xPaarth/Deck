# Getting Started

Welcome to Deck. This guide takes you from zero to solving your first problem in under 5 minutes.

---

## Install

### NPM (recommended)

```bash
npm install -g deck
```

The postinstall script detects your OS and architecture, downloads the matching binary from GitHub Releases, verifies the SHA256 checksum, and installs it to `~/.deck/bin/`.

### Cargo (from source)

```bash
git clone https://github.com/deck/deck
cd deck
cargo build --release --workspace
```

Binaries: `target/release/deck` and `target/release/deck-tui`

---

## Initialize

```bash
deck init
```

This creates:

- `~/.deck/` — Deck home
- `~/.deck/workspace/` — Solution files
- `~/.deck/repo/` — Git repository for auto-commits
- `~/.deck/config.toml` — Your settings
- `~/.cache/deck/deck.db` — SQLite database

---

## Solve Your First Problem

### 1. Fetch

```bash
deck fetch 1971D
```

Downloads the Codeforces problem "Binary Cut" and displays:
- Title, tags, limits
- Sample test cases
- Problem statement

### 2. Start the TUI

```bash
deck tui
```

You will see the **Dashboard** tab with your stats and recommended problems.

### 3. Open a Problem

Navigate to the **Problems** tab with `Tab`, then:
- Use `j/k` to move through the list
- Press `n` or `Enter` to open the selected problem

This:
1. Sends an `OpenProblem` RPC request to the backend
2. Creates `~/.deck/workspace/1971D_binary_cut.cpp` with a C++ template
3. Launches Neovim with a **35/65 split**: statement on the left, solution on the right

### 4. Solve & Test

Inside Neovim:
- Edit the solution file
- Press `<leader>rt` to compile and run against all sample tests
- Results appear in the quickfix list (`:copen`)

### 5. Submit

Press `<leader>sb` to open the Codeforces submit page in your browser. The form is pre-filled from the RPC response.

### 6. Check Stats

Back in the TUI, press `Tab` to switch to the **Analytics** tab:
- Rating, solved, streak
- Topic proficiency grid
- Weak tags with practice recommendations
- Predicted rating

---

## Next Steps

- [Install the Neovim plugin](installation.md#neovim-plugin)
- [Configure Git auto-commit](git-sync.md)
- [Join or create a team](team-guide.md)
- [Customize your config](configuration.md)
