#!/usr/bin/env bash
# Deck Demo Script — Showcase features for asciinema or screenshots

set -e

echo "🎴 Deck v0.1.0 Demo"
echo "===================="
echo ""

# 1. Init
echo "📦 Step 1: Initialize Deck"
deck init
echo ""

# 2. Fetch
echo "📥 Step 2: Fetch a problem"
deck fetch 1971D
echo ""

# 3. Stats
echo "📊 Step 3: Show stats"
deck stats alice_cp
echo ""

# 4. Config
echo "⚙️  Step 4: Show config"
deck config
echo ""

# 5. TUI (non-interactive preview)
echo "🖥️  Step 5: TUI Preview"
echo "   Run 'deck tui' for the interactive interface"
echo ""

# 6. Neovim commands
echo "📝 Step 6: Neovim Plugin Commands"
echo "   :DeckOpen 1971D  → Open problem in split layout"
echo "   :DeckRun          → Compile + run tests"
echo "   :DeckSubmit       → Submit solution"
echo "   :DeckStats        → Show analytics"
echo ""

echo "✨ Demo complete! Run 'deck tui' to start."
