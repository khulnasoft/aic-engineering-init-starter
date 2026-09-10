# AIC State Model

## Four engineering states

### Desired state

What the product or project should become.

Sources:

- idea
- requirements
- acceptance criteria
- constraints

### Planned state

How the project intends to reach the desired state.

Sources:

- roadmap
- architecture
- plans
- decisions
- estimates

### Actual state

What currently exists in the repository and environment.

Sources:

- filesystem
- AST
- symbols
- imports
- dependency manifests
- git
- tests
- runtime metadata

### Verified state

What can be supported by evidence.

Sources:

- passing tests
- typecheck
- lint
- build
- static analysis
- security checks
- runtime checks
- acceptance verification
- review evidence

## State transition

```text
DESIRED
  ↓ plan
PLANNED
  ↓ implement
ACTUAL
  ↓ verify
VERIFIED
  ↓ learn
MEMORY
```

## Verification levels

```text
L0 UNKNOWN
L1 DETECTED
L2 IMPLEMENTED
L3 TESTED
L4 VERIFIED
L5 PRODUCTION-READY
```

The levels are not synonymous with confidence. A work item may be implemented but still lack evidence for verification.

## State invariant

Never mark a work item `verified` merely because a checkbox was selected or an AI response claimed completion.
