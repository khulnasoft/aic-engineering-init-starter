# AIC Engineering Init Starter Template

First milestone for the AIC Engineering Control Plane.

## Decisions pinned for M1

- Canonical work-item identifier: `WI-###`
- CLI compatibility alias: `TODO-###` → canonical `WI-###`
- CLI runtime: Rust
- Milestone scope: starter repository/control-plane format only
- Autonomous execution: out of scope
- Existing YAML/template conventions: preserved

## Generated structure

`aic init` creates the `.agent` control plane, `skills`, `utils`, and
`configuration` directories plus the root repository guidance files. Existing
files are preserved; initialization is safe to run more than once.

## Repository conventions

- `configuration/` contains user-facing project and workflow configuration.
- `.agent/` contains structured state, decisions, plans, work items, and
	verification records.
- `skills/` contains reusable agent operating procedures.
- `utils/` is reserved for deterministic checks, generators, and scripts.
- Local caches, logs, and sessions are ignored and must never be committed.

The template is runtime-neutral for the target project. Rust is the runtime of
the AIC CLI itself, as recorded in `.agent/decisions/ADR-002-cli-runtime.yaml`.
