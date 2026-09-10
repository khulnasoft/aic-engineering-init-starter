# AIC Verification and Evidence

## Principle

AIC should treat software claims as hypotheses until evidence supports them.

```text
CLAIM
 ↓
EVIDENCE
 ↓
EVALUATION
 ↓
VERIFIED STATE
```

## Evidence types

```text
Filesystem
AST
Symbol existence
Import graph
Unit tests
Integration tests
End-to-end tests
Typecheck
Lint
Formatter
Build
Runtime checks
Security scanners
Coverage
Git diff
AI review
Manual acceptance
```

## Verification pipeline

```text
Requirement
   ↓
Acceptance Criteria
   ↓
Implementation
   ↓
Deterministic Checks
   ↓
Evidence Aggregation
   ↓
Verification Decision
```

## Work-item verification example

```text
WI-023
────────────────────────
Requirement: authentication session repository

Evidence:
✓ file exists
✓ repository exported
✓ unit tests pass
✓ integration tests pass
✓ typecheck passes
✓ security checks pass

Result: VERIFIED
Confidence: HIGH
```

## Multi-model verification

For high-risk changes:

```text
             implementation result
                      ↓
          ┌───────────┼───────────┐
          ↓           ↓           ↓
       Model A      Model B    Tooling
          └───────────┼───────────┘
                      ↓
              Evidence Aggregator
                      ↓
                 Verification
```

Model disagreement should be visible rather than averaged into false certainty.

## Coverage

`aic coverage` should measure more than test coverage:

- requirements coverage
- roadmap coverage
- architecture coverage
- implementation coverage
- test coverage
- acceptance coverage
- security coverage
- documentation coverage
- verification coverage
