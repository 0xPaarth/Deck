# Git Sync

Deck auto-commits your solutions to a Git repository on every Accepted (AC) verdict. This gives you a complete history of your competitive programming journey.

## How It Works

When you run `:DeckSubmit` or `:DeckRun` and get AC:

1. Backend detects AC verdict
2. Creates file structure in `~/.deck/repo/`:
   ```
   solutions/
   └── codeforces/
       └── 1971D/
           ├── solution.cpp
           └── metadata.json
   ```
3. Generates commit message: `Solved CF 1971D (Binary Cut) [1500] dp, greedy`
4. Commits to local Git repository
5. Optionally pushes to remote

## metadata.json

```json
{
  "problem_id": "1971D",
  "platform": "codeforces",
  "title": "Binary Cut",
  "rating": 1500,
  "tags": ["dp", "greedy"],
  "solved_at": "2026-06-20T14:32:00Z",
  "time_taken_seconds": 245,
  "language": "cpp",
  "execution_time_ms": 2,
  "memory_used_kb": 256,
  "approach": "Count alternating segments",
  "complexity": "O(n)",
  "attempts": 2
}
```

## README.md Generation

After each commit, Deck auto-generates `README.md`:

```markdown
# 🏆 My CP Solutions

## Recent Solutions
| Problem | Platform | Rating | Tags | Date |
|---------|----------|--------|------|------|
| 1971D | Codeforces | 1500 | dp, greedy | 2026-06-20 |
| 1971C | Codeforces | 900 | strings | 2026-06-19 |
```

## Configuration

```toml
[git]
enabled = true
repo_path = "~/.deck/repo"
auto_commit = true
auto_push = false
```

### Auto-Push

Set `auto_push = true` to push commits automatically. You'll need to configure a remote:

```bash
cd ~/.deck/repo
git remote add origin git@github.com:yourname/cp-solutions.git
```

Then update `config.toml`:

```toml
[git.remote]
url = "git@github.com:yourname/cp-solutions.git"
name = "origin"
```

### Private Repos

Set `private = true` in config to omit your handle from the README.

## Commit Message Format

Default template:

```
Solved {platform} {problem_id} ({title}) [{rating}] {tags}
```

Example:
```
Solved Codeforces 1971D (Binary Cut) [1500] dp, greedy
```

Customize in `config.toml`:

```toml
commit_message_template = "AC: {platform} {problem_id} ({title})"
```

## Manual Commit

If auto-commit is disabled, commit manually:

```bash
cd ~/.deck/repo
git add solutions/
git commit -m "Manual: solved 1971D"
```
