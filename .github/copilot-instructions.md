# GitHub Copilot Instructions (GitCore Minimal)

## Prime Directive

Operate under Git-Core `3.5.1` minimal profile.

- Long-term state: GitHub Issues.
- Session state: `.gitcore/planning/TASK.md`.

## Required Context Load Order

1. `.gitcore/ARCHITECTURE.md`
2. `.gitcore/AGENT_INDEX.md`
3. `.gitcore/features.json`
4. `.gitcore/planning/PLANNING.md`
5. `.gitcore/planning/TASK.md`
6. `docs/agent-docs/RESEARCH_STACK_CONTEXT.md`

## Mandatory Health Check

```bash
gc check
gc issue list --limit 5
cat .gitcore/features.json
```

## Planning Contract (Canonical Path Only)

Valid planning files:

- `.gitcore/planning/PLANNING.md`
- `.gitcore/planning/TASK.md`

Do not use planning/task files elsewhere.

## TASK.md Update Protocol

When progress happens:

1. Update task status.
2. Add issue reference.
3. Add commit hash when completed.
4. Register newly discovered work.

## Forbidden Tracking Files (Outside Canonical Planning Path)

- `TODO.md`, `TASKS.md`, `BACKLOG.md`
- `PLANNING.md`, `TASK.md`, `ROADMAP.md`, `PROGRESS.md`
- `NOTES.md`, `SCRATCH.md`, `STATUS.md`, `CHECKLIST.md`
- `IMPLEMENTATION*.md`, `SUMMARY.md`, `REPORT.md`

## Allowed Documentation

- `README*.md`, `AGENTS.md`, `CHANGELOG.md`, `LICENSE*`, `CONTRIBUTING*`
- `.gitcore/**/*.md`
- `.github/**/*.md`
- `docs/**/*.md`

## Tool Priority

1. `gc` as primary interface.
2. `gh` for GitHub-specific operations.
3. Legacy scripts only as compatibility shims.

## Standard Flow

```bash
# Assign and start
gh issue edit <id> --add-assignee "@me"
git checkout -b feat/issue-<id>

# Implement, verify, and update task state
gc check

# Commit and PR
git commit -m "feat(scope): description (closes #<id>)"
gh pr create --fill
gc report
```

## Architecture Override Rule

If issue text conflicts with `.gitcore/ARCHITECTURE.md`, architecture wins.

