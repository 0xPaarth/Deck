# Team Guide

Deck supports team-based competitive programming. Track your teammates' progress, share solutions, and climb the leaderboard together.

## Create a Team

```bash
# Via TUI
deck tui
# → Team tab → press 'c' to create

# Or via CLI
curl -X POST http://localhost:4646 \
  -H "Content-Type: application/json" \
  -d '{"type": "CreateTeam", "payload": {"name": "CP-Squad"}}'
```

This creates a team entry in the SQLite database and initializes a Git repository at `~/.deck/repo/team/<team_id>/`.

## Join a Team

```bash
# Via TUI
# → Team tab → press 'j' to join, enter team ID

# Or via CLI
curl -X POST http://localhost:4646 \
  -H "Content-Type: application/json" \
  -d '{"type": "JoinTeam", "payload": {"team_id": "team-123", "handle": "alice_cp"}}'
```

## Team Data

Team data is stored in:
- SQLite: `teams` table
- Git: `team/<team_id>/members.json` — synced across members

### members.json

```json
{
  "team_id": "team-123",
  "name": "CP-Squad",
  "members": [
    {
      "handle": "alice_cp",
      "role": "Admin",
      "joined_at": "2026-06-20T10:00:00Z",
      "rating": 1600,
      "streak": 12
    }
  ],
  "solved_count": {
    "alice_cp": 145,
    "bob": 89
  }
}
```

## Share a Solution

### From Neovim

In a solution buffer, press `<leader>ss` or run `:DeckShare`.

This:
1. Saves the current file
2. Sends a `ShareSolution` RPC with problem ID and file path
3. The backend copies the file to the team Git repo under `team/<team_id>/shared/<problem_id>/`
4. Commits with message: `Share: alice_cp solved 1971D`
5. Pushes to remote (if configured)

### From TUI

In the Team tab, press `s` on a member to share your current problem with them.

## Team Tab

The TUI Team tab shows:
- Team name and member count
- Each member's progress bar (rating, streak, daily solves)
- Team totals (weekly solved, all-time solved)
- Team weak tags (aggregated across all members)

## Sync

Team data syncs automatically every 5 minutes via Git pull/push. To force sync:

```bash
# In Neovim
:DeckSync

# In TUI
Press 'g' (global sync)
```

## Permissions

| Role | Can |
|---|---|
| Admin | Create, invite, kick members, delete team |
| Member | Solve, share, view progress |

## Neovim Integration

Show team status:

```vim
:DeckTeam
```

Opens a floating window with team members, their ratings, and progress.
