# AIC Data Model

## Core entities

```text
Project
Stack
Architecture
Requirement
Milestone
Plan
Decision
WorkItem
Finding
Checkpoint
VerificationResult
Memory
Session
Policy
```

## Key relationships

```text
Project
 ├── Stack
 ├── Architecture
 ├── Requirement
 │     └── WorkItem
 ├── Milestone
 │     └── Plan
 │          └── WorkItem
 ├── Decision
 ├── Finding
 │     └── WorkItem
 ├── Checkpoint
 ├── VerificationResult
 └── Memory
```

## Work item example

```yaml
id: WI-023
type: feature
title: Add authentication session repository
status: pending
priority: high
requirements:
  - REQ-014
plan:
  - PLAN-009
files:
  - src/auth/session.ts
acceptance:
  - session can be created
  - session can be revoked
  - expiry enforced
verification:
  - typecheck
  - unit_test
  - integration_test
```

## Finding lifecycle

```text
Review / Security / Audit
        ↓
      Finding
        ↓
 WorkItem (optional)
        ↓
 Implementation
        ↓
 Verification
```

## Memory provenance

Every learned fact should ideally retain:

- source
- scope
- confidence
- evidence reference
- creation time
- last validation
