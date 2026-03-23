---
title: "Git-Core Protocol - Agent Configuration"
type: CONFIGURATION
id: "config-agents"
created: 2025-12-01
updated: 2026-03-03
agent: copilot
model: claude-sonnet-4
requested_by: system
summary: |
  Minimal operational profile for Git-Core 3.5.1.
  Uses .gitcore/planning for planning and task state, and gc as primary interface.
keywords: [agents, protocol, workflow, planning, task-management, gc-cli]
tags: ["#gitcore", "#protocol", "#v3.5.1", "#minimal"]
project: Git-Core-Protocol
protocol_version: 3.5.1
---

# Git-Core Protocol (Minimal Core)

## Prime Directive

Use GitHub Issues as long-term state and `.gitcore/planning/TASK.md` as session execution state.

## Canonical Files

| File | Purpose |
|------|---------|
| `.gitcore/ARCHITECTURE.md` | Non-negotiable architecture decisions |
| `.gitcore/AGENT_INDEX.md` | Role routing and context loading order |
| `.gitcore/features.json` | Feature status and verification commands |
| `.gitcore/CLI_CONFIG.md` | Service/tool usage matrix |
| `.gitcore/planning/PLANNING.md` | Scope, phases, constraints |
| `.gitcore/planning/TASK.md` | Active tasks, milestones, discovered work |
| `.github/copilot-instructions.md` | IDE-facing operational rules |

## Jules Agent Integration

When operating as the `Jules` agent, you MUST follow these initialization steps before proceeding with any assigned issue:

1. **Read context mandatory:** Execute `cat .gitcore/ARCHITECTURE.md` and `cat AGENTS.md`.
2. **Follow protocol:** You must strictly follow the Git-Core Protocol. Do NOT track state in `TODO.md` or any unauthorized file.
3. **Finish task correctly:** Execute the changes and ensure you run `./scripts/ai-report.ps1` before finishing the task.

## Mandatory Execution Loop

### Phase 0: Health Check

```bash
gc check
gc issue list --limit 5
cat .gitcore/features.json
cat .gitcore/planning/PLANNING.md
cat .gitcore/planning/TASK.md
```

### Phase 1: Read Context

```bash
cat .gitcore/ARCHITECTURE.md
cat .gitcore/AGENT_INDEX.md
cat docs/agent-docs/RESEARCH_STACK_CONTEXT.md
```

### Phase 2: Execute

```bash
gh issue edit <id> --add-assignee "@me"
git checkout -b feat/issue-<id>
```

### Phase 3: Update State

```bash
git commit -m "feat(scope): short description (closes #<id>)"
gh pr create --fill
gc report
```

Additionally update `.gitcore/planning/TASK.md` with:

1. Task status transition.
2. Issue reference.
3. Commit hash for completed items.

## Planning & Task Management Contract

### Location Rule

Planning artifacts are only valid at:

- `.gitcore/planning/PLANNING.md`
- `.gitcore/planning/TASK.md`

### Content Rule

`PLANNING.md` must contain:

1. Scope and objectives.
2. Constraints and decisions.
3. Phases and success criteria.

`TASK.md` must contain:

1. Current phase and objective.
2. Active tasks table.
3. Milestones completed.
4. Technical debt.
5. Tasks discovered during execution.

### Update Rule

Before coding, read both planning files.
After meaningful progress, update `TASK.md`.

## Forbidden Files (Outside Canonical Planning Path)

Do not create task/state files outside `.gitcore/planning/`:

- `TODO.md`, `TASKS.md`, `BACKLOG.md`
- `PLANNING.md`, `TASK.md`, `ROADMAP.md`, `PROGRESS.md` (outside canonical path)
- `NOTES.md`, `SCRATCH.md`, `STATUS.md`, `CHECKLIST.md`
- `IMPLEMENTATION*.md`, `SUMMARY.md`, `REPORT.md`

## Allowed Documentation Policy

Allowed paths:

- `README*.md`, `AGENTS.md`, `CHANGELOG.md`, `LICENSE*`, `CONTRIBUTING*`
- `.gitcore/**/*.md`
- `.github/**/*.md`
- `docs/**/*.md`

Constraint: allowed files cannot replace issue or task tracking responsibilities.

## Service & Tool Priority

1. Primary: `gc` CLI.
2. Secondary: `gh` for GitHub operations.
3. Legacy scripts: compatibility wrappers only.

## Architecture First Rule

If issue text conflicts with `.gitcore/ARCHITECTURE.md`, architecture wins.

## Commit Standard

```text
<type>(<scope>): <description> #<issue>

[optional body]

AI-Context: reference | reasoning
```

