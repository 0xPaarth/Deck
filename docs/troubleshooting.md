# Troubleshooting

## Common Issues

### "Deck backend not running" in Neovim

The TUI starts the RPC server automatically. If you only use Neovim, start it manually:

```bash
deck tui &   # Or use the backend binary directly
deck &
```

Alternatively, set up a systemd service or launch agent.

---

### "No pre-built binary for platform" (NPM install)

Your platform isn't in the release matrix yet. Build from source:

```bash
cargo build --release --workspace
# Binaries: target/release/deck and target/release/deck-tui
cp target/release/deck ~/.deck/bin/
cp target/release/deck-tui ~/.deck/bin/
```

---

### "Problem ID not found"

Verify the problem ID format. For Codeforces:
- Use the format `1971D` (contest + problem letter)
- Do not include spaces or dashes

Examples:
- ✅ `1971D` (Round 1971, problem D)
- ✅ `1900A` (ER 1900, problem A)
- ❌ `1971 D` (space)
- ❌ `D` (missing contest number)

For CSES, use the numeric task ID (e.g., `1621`).

---

### "Connection refused" (RPC)

The Unix socket at `~/.cache/deck/socket` isn't available. Check:

```bash
ls -la ~/.cache/deck/socket
```

If missing, restart the backend:

```bash
rm -f ~/.cache/deck/socket
deck tui
```

---

### "Compilation error" in Neovim

Make sure the required compilers are installed:

| Language | Required |
|---|---|
| C++ | `g++` (or `clang++`) |
| Rust | `rustc` |
| Python | `python3` |
| Go | `go` |
| Java | `javac` |

Test:

```bash
g++ --version
rustc --version
python3 --version
```

---

### "Checksum mismatch" (NPM install)

The downloaded binary might be corrupted. Retry:

```bash
npm uninstall -g deck
npm install -g deck
```

If it persists, the release artifact might have changed. Open an issue at [github.com/deck/deck/issues](https://github.com/deck/deck/issues).

---

### "Git push failed"

Auto-push only works if:
1. `auto_push = true` in `config.toml`
2. A remote is configured (`git remote -v`)
3. SSH key authentication is set up

```bash
cd ~/.deck/repo
git remote -v
# Should show: origin  git@github.com:user/repo.git (push)

ssh -T git@github.com
# Should say: Hi user! You've successfully authenticated
```

---

### "Extension icon missing" in Chrome

The `browser/icons/` directory must contain `icon16.png`, `icon48.png`, `icon128.png`.

Regenerate them:

```bash
cd browser/icons
convert -size 16x16 xc:navy icon16.png
convert -size 48x48 xc:navy icon48.png
convert -size 128x128 xc:navy icon128.png
```

Then reload the extension at `chrome://extensions/`.

---

### "Socket already in use"

A stale socket file exists. Remove it:

```bash
rm -f ~/.cache/deck/socket
lsof ~/.cache/deck/socket 2>/dev/null || true
```

---

### "Database locked"

SQLite might be locked if two processes access it simultaneously. Restart both the TUI and backend:

```bash
pkill -f deck-tui
pkill -f deck
rm -f ~/.cache/deck/socket
```

---

### "LSP not attaching" in Neovim

Check that the LSP server is installed:

```bash
which clangd
which rust-analyzer
which pyright
```

Configure the correct path in `config.toml`:

```toml
[lsp]
cpp = ["/usr/local/bin/clangd", "--background-index"]
```

---

### Slow problem fetch

Codeforces rate-limits requests. If fetching is slow:
1. Wait a few seconds and retry
2. The problem is cached in SQLite after the first fetch

---

## Reporting Bugs

Open an issue at [github.com/deck/deck/issues](https://github.com/deck/deck/issues) and include:

- OS and architecture
- `deck --version` output
- Steps to reproduce
- Full error message or screenshot
