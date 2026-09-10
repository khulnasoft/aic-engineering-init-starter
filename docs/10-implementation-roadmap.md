# AIC Implementation Roadmap

## Phase 0 — Foundation

- CLI shell (implemented)
- configuration loader (implemented)
- project root detection (implemented)
- `.agent` bootstrap (implemented)
- schema versioning (initial version implemented)
- logging (deferred)

## Phase 1 — Discovery

- `init` (implemented)
- `inspect` (implemented)
- `stack` (implemented)
- `doctor` (implemented)
- project state (initial projection implemented)
- repository index (deferred)

## Phase 2 — Planning

- `idea`
- `requirements`
- `roadmap`
- `planning`
- `decision`
- `estimate`

## Phase 3 — Context and Build

- `context`
- `architecture`
- `conventions`
- `dependencies`
- `scaffold`
- `generate`

## Phase 4 — Work Management

- structured `WorkItem`
- TODO CLI
- task selection
- checkpointing
- resume

## Phase 5 — Verification

- evidence engine
- test runner integration
- typecheck/lint/build adapters
- security adapters
- `todo verify`
- `verify`
- `coverage`
- `review`

## Phase 6 — Agent Execution

- `work`
- execution sandbox
- permission boundaries
- budget controls
- `loop`
- multi-step recovery

## Phase 7 — Learning and Governance

- memory
- learn
- policies
- permissions
- failure catalog
- architectural rule enforcement

## Phase 8 — Advanced Platform

- multi-model verification
- remote/shared control plane
- plugin/skill registry
- provider gateway
- team collaboration
- CI integration
- PR automation

## Suggested first production milestone

The smallest high-value release should include:

```text
init
config
stack
inspect
context
roadmap
planning
todo
todo verify
status
doctor
```

Do not start with full autonomous execution. Build trustworthy state and verification first.
