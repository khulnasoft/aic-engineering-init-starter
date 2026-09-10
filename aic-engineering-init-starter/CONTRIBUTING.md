# Contributing

## Before you start

1. Read `README.md` and `AGENT.md`.
2. Read the relevant decision records under `.agent/decisions/`.
3. Keep project state changes linked to a work item and evidence.

## Change requirements

- Preserve the canonical `WI-###` work-item format.
- Treat `TODO-###` as a CLI input alias only; never persist it.
- Do not add credentials, generated caches, logs, or local sessions.
- Do not mark work or verification complete without deterministic evidence.
- Keep changes small and update the relevant `.agent` state and documentation.

## Validation

Before submitting a change, inspect the template with:

```bash
find . -maxdepth 3 -type f | sort
```

Validate every YAML file with a YAML 1.2-compatible parser and confirm that
tracked files do not contain credentials or local runtime state.
