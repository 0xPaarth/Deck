# TUI Guide

Deck's terminal UI is built with [Ratatui](https://ratatui.rs/). It runs in your terminal with full keyboard control.

## Start

```bash
deck tui
# or
cargo run --bin deck-tui
```

## Layout

```
┌── Title bar: version + streak + rating ───────────────────┐
│  handle  |  Team: name  |  Git: Enabled                   │
├── Tab bar ────────────────────────────────────────────────┤
│  Dashboard  Problems  Analytics  Team  Contest  Target  Config │
├── Content area ───────────────────────────────────────────┤
│                                                            │
│  [Tab-specific content]                                   │
│                                                            │
├── Footer ─────────────────────────────────────────────────┤
│  [?] Help  [Tab] Switch  [q] Quit                        │
└───────────────────────────────────────────────────────────┘
```

## Global Shortcuts

| Key | Action |
|---|---|
| `Tab` | Next tab |
| `Shift+Tab` | Previous tab |
| `?` | Toggle help popup |
| `q` | Quit Deck |
| `r` | Refresh current tab |

---

## Dashboard Tab

Shows:
- **Stats**: Rating, solved count, streak, max rating
- **Progress**: Rating change this month
- **Recommended Problems**: 3 hand-picked problems based on weak tags

Navigation: `Enter` on a recommendation opens it in Neovim.

---

## Problems Tab

A table of problems with columns: ID, Title, Rating, Tags, Status.

| Key | Action |
|---|---|
| `j` / `k` | Move up/down |
| `Enter` / `n` | Open selected problem in Neovim |
| `r` | Refresh list |

Right panel shows details for the selected problem: time limit, memory limit, tags.

---

## Analytics Tab

| Key | Action |
|---|---|
| `r` | Refresh stats |
| `e` | Export (placeholder) |
| `c` | Compare with friend (placeholder) |

Sections:
- **Stats bar**: Rating, solved, streak, max rating
- **Time Analytics**: Average solve time, distribution histogram
- **Topic Proficiency**: Grid showing strength per rating × topic
- **Weak Tags**: Sorted by priority with practice recommendations
- **Predictions**: Projected rating, next milestone, focus area

---

## Team Tab

Shows team members and their progress.

| Key | Action |
|---|---|
| `j` / `k` | Navigate members |
| `Enter` | View member profile |
| `s` | Share solution |
| `l` | Leave team |

Sections:
- **Header**: Team name, member count, last active
- **Member cards**: Handle, rating, streak, daily solve progress bar
- **Team totals**: Weekly solved, total solved, weak tags

---

## Contest Tab

Shows upcoming contests and your current contest progress.

| Key | Action |
|---|---|
| `o` | Open selected problem |
| `r` | Refresh |
| `t` | Toggle timer display |
| `q` | Quit contest view |

Sections:
- **Header**: Contest name + countdown timer
- **Problem grid**: A–F with status (✅ solved, ⏳ attempting, ⬜ not started)
- **Standings**: Score, rank, rating change

---

## Target Tab

Track your rating goals.

| Key | Action |
|---|---|
| `g` | Set new goal |
| `r` | Refresh progress |

Sections:
- **Progress bar**: Current → target rating
- **Topics to Master**: Per-topic accuracy with target percentages
- **Weekly Plan**: Recommended problems and contests

---

## Config Tab

View current settings.

| Key | Action |
|---|---|
| `e` | Edit config.toml |
| `r` | Reload config |
| `i` | Reset to defaults |
