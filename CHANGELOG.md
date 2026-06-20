# Changelog

All notable changes to Deck will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] — 2026-06-20

### Added

#### Foundation
- **Codeforces problem fetcher** — Download and parse problems by ID (e.g., `1971D`)
  - Extracts title, statement, samples, time/memory limits, tags
  - Uses `reqwest` + `scraper` for HTML parsing
- **SQLite database** — Persistent storage in `~/.cache/deck/deck.db`
  - Tables: `users`, `problems`, `solutions`, `teams`
  - Auto-migration on first connect
- **Unix socket RPC server** — JSON-over-line protocol at `~/.cache/deck/socket`
  - Endpoints: `OpenProblem`, `GetStats`, `GetHeatmap`, `GetPredictions`, `GetWeakTags`, `CreateTeam`, `JoinTeam`, `GetTeamStatus`, `GetContests`, `GetContestStatus`

#### TUI (Ratatui Frontend)
- **7 tabs**: Dashboard, Problems, Analytics, Team, Contest, Target, Config
- **Dashboard** — Rating, solved count, streak, progress, recommended problems
- **Problems** — Table with ID/Title/Rating/Tags/Status + detail panel
- **Analytics** — Topic proficiency grid, time distribution, weak tags, predictions
- **Team** — Member progress bars, daily solve counts, team totals
- **Contest** — Upcoming contests, countdown timer, problem status grid
- **Target** — Rating goal progress, topic mastery, weekly practice plan
- **Config** — General settings, Git configuration

#### Neovim Plugin (`plugins/neovim/`)
- **Split layout** — 35% statement (left) + 65% solution (right)
- **Focus mode** — Toggle statement to full width
- **Commands**: `:DeckOpen`, `:DeckRun`, `:DeckSubmit`, `:DeckStats`, `:DeckTeam`, `:DeckShare`, `:DeckFocus`, `:DeckSync`, `:DeckHelp`
- **Keymaps** — `<leader>rt`, `<leader>sb`, `<leader>sa`, `<leader>sf`, `<leader>sn`, `<leader>sg`, `<leader>sh`
- **Templates** — C++, Rust, Python starter code
- **Test runner** — Compile, run samples, populate quickfix
- **Floating windows** — Stats, help, team status

#### Git Sync
- **Auto-commit on AC** — `GitManager::commit_solution` creates commit with metadata
- **metadata.json** — Problem ID, title, rating, tags, time taken, language, complexity
- **README.md generation** — Auto-updated table of solved problems
- **Auto-push** (optional) — Push to remote on commit

#### Analytics
- **User stats** — Rating, solved, streak, max rating
- **Topic accuracy** — Per-tag solved/attempted/accuracy percentages
- **Weak tags** — Priority-ranked tags with practice recommendations
- **Predictions** — Projected rating, time to next milestone
- **Time distribution** — 0-15m, 15-30m, 30-60m, 60m+ buckets

#### Browser Companion
- **Manifest V3 extension** for Codeforces
- **Auto-capture** problem statement, samples, limits on `/problem/` pages
- **Submit autofill** — Opens submit page from Neovim

#### Distribution
- **NPM package** — `npm install -g deck`
  - `postinstall.js` detects OS/arch, downloads binary from GitHub Releases
  - `preuninstall.js` cleans up `~/.deck/`
- **GitHub Actions** — Matrix builds for 6 targets (x64/arm64 × macOS/Linux/Windows)
  - Auto-attaches binaries + checksums to releases
  - Publishes to NPM on tag push

#### CLI Commands
- `deck init` — Initialize workspace, database, config
- `deck fetch <id>` — Download Codeforces problem
- `deck tui` — Start interactive TUI
- `deck stats [handle]` — Show user statistics
- `deck open <id>` — Open problem in Neovim
- `deck config` — Show current configuration
- `deck --version` / `deck -V`

[0.1.0]: https://github.com/deck/deck/releases/tag/v0.1.0
