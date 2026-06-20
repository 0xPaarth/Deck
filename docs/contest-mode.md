# Contest Mode

Track upcoming contests, join them, and monitor your progress in real time.

## Upcoming Contests

The **Contest** tab fetches upcoming contests from Codeforces API:

```
┌─────────────────────────────────────────────────────────────┐
│ 🏆 Codeforces Round #1234  [⏱️ 1:45:23 remaining]        │
├─────────────────────────────────────────────────────────────┤
│  A: ✅ +500  B: ⏳ 0/??  C: ⬜ 0/??  D: ⬜ 0/??  E: ⬜ 0/??  │
│  Score: 500  |  Rank: #234  |  Rating Δ: +45              │
├─────────────────────────────────────────────────────────────┤
│  [o] Open  [r] Refresh  [t] Timer  [q] Quit               │
└─────────────────────────────────────────────────────────────┘
```

## Join a Contest

```bash
# Via TUI
# → Contest tab → navigate to contest → press Enter

# Or via CLI
curl -X POST http://localhost:4646 \
  -H "Content-Type: application/json" \
  -d '{"type": "JoinContest", "payload": {"contest_id": "1234"}}'
```

## Problem Grid

Each problem shows:
- Letter (A, B, C...)
- Status: ✅ solved, ⏳ attempting, ⬜ not started
- Score: points earned

Colors:
- Green = solved
- Yellow = attempting
- Gray = not started

## Real-Time Timer

The header shows a countdown to contest end. Updates every second.

```
⏱️ 1:45:23 remaining
```

## Standings

- **Score**: Total points earned
- **Rank**: Current rank among participants
- **Rating Change**: Estimated Δ (from Codeforces API)

## During Contest

```bash
# Open a problem directly
deck open 1234A

# Inside Neovim
:DeckContest
# Shows timer, solved count, remaining problems
```

## After Contest

The backend auto-fetches your final standing and rating change. A summary is appended to `~/.deck/contest_history.md`.

## Contest History

```bash
cat ~/.deck/contest_history.md
```

Sample line:
```
| 1234 | Codeforces Round #1234 | 500 | #234 | +45 | 2026-06-20 |
```
