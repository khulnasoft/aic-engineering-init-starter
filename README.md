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

The first executable milestone bootstraps and inspects project state. Planning,
work execution, and verification commands remain documented design targets.

`aic init` creates a non-destructive `.agent` directory containing the tracked
project, execution, policy, and agent-contract files. Existing files are
preserved. `aic stack` writes the detected repository stack to
`.agent/stack.yaml`.

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
- `templates/AGENT.md` — starter agent contract
- `templates/*.yaml` — starter project configuration and policy
- `examples/*` — example roadmap, plan, work item and verification
- `src/main.rs` — initial Rust CLI implementation

## Runtime

The starter implementation uses Rust 2021 and Cargo. Build and test it with:

```bash
cargo test
cargo run -- init
```

Persisted work items use `WI-###` identifiers. The user-facing `todo` command
is reserved for the work-management milestone.

