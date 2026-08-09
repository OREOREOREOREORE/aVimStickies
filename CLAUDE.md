# CLAUDE.md

## Language

- Explain to the user in Traditional Chinese (繁體中文).
- Write documentation and repo files in English.

## Agent skills

### Issue tracker

Issues and specs live as GitHub issues, via the `gh` CLI. See `docs/agents/issue-tracker.md`.

### Triage labels

Five canonical labels matching the triage roles: `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: one `CONTEXT.md` + `docs/adr/` at the repo root. See `docs/agents/domain.md`.
