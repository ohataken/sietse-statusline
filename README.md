# sietse-statusline

my little statusline for Claude Code / Codex.

## installation

available on homebrew tap.

```sh
brew tap ohataken/sietse-statusline https://github.com/ohataken/sietse-statusline
brew install sietse-statusline
```

### .claude/settings.json example

```json
{
  "statusLine": {
    "type": "command",
    "command": "sietse-statusline"
  }
}
```

## available arguments

### data

| argument | description |
|---|---|
| `--current-dir-name` | name of the current working directory |
| `--project-dir-name` | name of the project root directory |
| `--branch-name` | current git branch name |
| `--branch-head-sha` | full commit SHA of HEAD |
| `--worktree` | prints "worktree" if inside a git worktree, nothing otherwise |

### colors

| argument | description |
|---|---|
| `--black` | black foreground |
| `--red` | red foreground |
| `--green` | green foreground |
| `--yellow` | yellow foreground |
| `--blue` | blue foreground |
| `--magenta` | magenta foreground |
| `--cyan` | cyan foreground |
| `--white` | white foreground |
| `--bright-black` | bright black foreground |
| `--bright-red` | bright red foreground |
| `--bright-green` | bright green foreground |
| `--bright-yellow` | bright yellow foreground |
| `--bright-blue` | bright blue foreground |
| `--bright-magenta` | bright magenta foreground |
| `--bright-cyan` | bright cyan foreground |
| `--bright-white` | bright white foreground |
| `--reset` | reset all styles |

### separators

| argument | description |
|---|---|
| `--space` | space character |
| `--comma` | comma character |
| `--slash` | slash character |
| `--hyphen` | hyphen character |
| `--underscore` | underscore character |
| `--break` | line break |
