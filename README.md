# AIC — Engineering Control Plane

AI-native project operating system for software engineering.

AIC is designed as more than a project generator: it is a persistent engineering control plane that coordinates project discovery, requirements, roadmap, planning, scaffolding, context, work items, execution, verification, governance, memory, and learning.

## Core model

```text
DESIRED STATE
  requirements / product goals
        ↓
PLANNED STATE
  roadmap / architecture / plans / decisions
        ↓
ACTUAL STATE
  repository / dependencies / implementation
        ↓
VERIFIED STATE
  tests / evidence / acceptance / security
        ↓
LEARNING
  memory / lessons / decisions
        ↓
NEXT ITERATION
```

## Primary workflow

```bash
aic init
aic config
aic doctor
aic inspect
aic stack
aic status
```

Running `aic init` opens an interactive project setup wizard for the project
name, stack, AI workflow features, provider, and model. API keys are read from
environment variables and are never persisted. Use `aic init --non-interactive`
to bootstrap the starter defaults in scripts and CI.

The first executable milestone bootstraps and inspects project state. Planning,
work execution, and verification commands remain documented design targets.

`aic init` creates a non-destructive `.agent` directory containing the tracked
project, state, decisions, work, verification, skills, utilities, and root
repository guidance files. Existing files are preserved. `aic stack` writes
the detected repository stack to `.agent/stack.yaml`.

Autonomous mode:

```bash
aic loop
```

## Design principle

AI proposes. Deterministic tools inspect and execute. Evidence establishes truth. Structured project state preserves continuity.

## Bundle contents

- `docs/01-product.md` — product vision and positioning
- `docs/02-cli-reference.md` — complete CLI taxonomy and UX
- `docs/03-architecture.md` — control-plane architecture
- `docs/04-state-model.md` — desired/planned/actual/verified state model
- `docs/05-agent-system.md` — agent runtime, skills, context, memory
- `docs/06-verification.md` — evidence and verification architecture
- `docs/07-governance.md` — policy, permissions and safety boundaries
- `docs/08-workflow.md` — end-to-end lifecycle and autonomous loop
- `docs/09-data-model.md` — persistent entities and relationships
- `docs/10-implementation-roadmap.md` — phased engineering plan
- `schemas/*.yaml` — example machine-readable state schemas
- `AGENT.md` — starter agent contract
- `.agent/` — structured control-plane state and decisions
- `configuration/` — project and workflow configuration
- `skills/` — reusable agent operating procedures
- `utils/` — reserved deterministic checks, generators, and scripts
- `TEMPLATE-MANIFEST.yaml` — template contents and compatibility contract
- `examples/*` — example roadmap, plan, work item and verification
- `src/main.rs` — initial Rust CLI implementation

The repository itself is the canonical AIC Engineering Init starter template.
The Rust CLI implementation lives alongside the template and materializes only
the template-owned files when `aic init` runs; it does not copy source code,
documentation, examples, schemas, or build output into a target project.

## Runtime

The starter implementation uses Rust 2021 and Cargo. Build and test it with:

```bash
cargo test
cargo run -- init
```

Persisted work items use `WI-###` identifiers. The user-facing `todo` command
is reserved for the work-management milestone.

## Repository conventions

- `configuration/` contains user-facing project and workflow configuration.
- `.agent/` contains structured state, decisions, plans, work items, and
  verification records.
- `skills/` contains reusable agent operating procedures.
- `utils/` is reserved for deterministic checks, generators, and scripts.
- Local caches, logs, sessions, build output, and secrets are ignored.

