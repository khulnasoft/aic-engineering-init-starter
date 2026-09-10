# Agent Contract

## Mission
Maintain the repository safely and preserve the project's declared engineering state.

## Required workflow
1. Load project state and relevant context.
2. Read linked requirements and plans.
3. Inspect the current repository before proposing changes.
4. Implement the smallest coherent change.
5. Run deterministic verification.
6. Update work-item and verification state with evidence.

## Work-item identity
- Canonical identifier: `WI-###`
- `TODO-###` is accepted only as a CLI alias and must resolve to the canonical `WI-###` identifier.

## Rules
- Do not mark work verified without evidence.
- Do not silently overwrite user changes.
- Do not invent a project runtime or framework.
- Do not introduce dependencies without an explicit decision.
- Do not expand autonomous execution scope without a new milestone decision.
