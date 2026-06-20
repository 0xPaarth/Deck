# Installation

## Requirements

- **Rust 1.75+** (for building from source)
- **Node.js 14+** (for NPM install)
- **Neovim 0.9+** (for plugin)
- **Git** (for sync features)

## Install via NPM

```bash
npm install -g deck
```

The postinstall script:
1. Detects your platform (`darwin` / `linux` / `win32`) and architecture (`x64` / `arm64`)
2. Downloads the correct binary from GitHub Releases
3. Verifies the SHA256 checksum
4. Extracts to `~/.deck/bin/`
5. Makes it executable

### Troubleshoot NPM Install

| Error | Fix |
|---|---|
| "No pre-built binary" | Build from source with Cargo |
| "Checksum mismatch" | Re-run `npm install -g deck` |
| "No network" | Check internet connection, then retry |

## Install via Cargo

```bash
cargo install --git https://github.com/deck/deck
```

This compiles both `deck` (CLI) and `deck-tui` (TUI) binaries.

## Platform-Specific Notes

### macOS

Works on Intel and Apple Silicon. If you see a security warning about an untrusted binary:

```bash
xattr -d com.apple.quarantine ~/.deck/bin/deck
xattr -d com.apple.quarantine ~/.deck/bin/deck-tui
```

### Linux

No additional setup required. If your distribution doesn't have `deck` in its package repositories, use NPM or Cargo.

### Windows

Requires Windows 10/11. The binary is `deck.exe`. Add `%USERPROFILE%\.deck\bin` to your PATH if it wasn't added automatically.

---

## Neovim Plugin

### lazy.nvim

```lua
{
    "deck/deck.nvim",
    dependencies = { "nvim-lua/plenary.nvim" },
    config = function()
        require("deck").setup({
            default_language = "cpp",
            socket_path = "127.0.0.1:4647",
        })
    end,
}
```

### packer.nvim

```lua
use {
    "deck/deck.nvim",
    requires = { "nvim-lua/plenary.nvim" },
    config = function()
        require("deck").setup()
    end,
}
```

### Manual

Clone into your plugin directory:

```bash
git clone https://github.com/deck/deck.nvim ~/.local/share/nvim/site/pack/deck/start/deck.nvim
```

---

## Browser Extension

1. Open Chrome/Edge/Brave → `chrome://extensions/`
2. Enable **Developer mode**
3. Click **Load unpacked**
4. Select the `browser/` directory from this repo
5. The extension icon appears in your toolbar
