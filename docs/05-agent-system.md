# AIC Agent System

## Agent contract

`AGENT.md` is a human-readable operating contract. Machine-readable state and policy belong under `.agent/`.

## Skills

A skill is more than a prompt:

```text
Knowledge
+ Rules
+ Tools
+ Verification
```

Suggested skill families:

```text
skills/
├── discovery/
├── architecture/
├── backend/
├── frontend/
├── database/
├── testing/
├── security/
├── performance/
├── git/
├── review/
└── deployment/
```

Skill package contents:

```text
SKILL.md
rules.yaml
checks/
prompts/
examples/
```

## Context system

Context should be layered:

```text
Project summary
Architecture
Conventions
Dependencies
Relevant modules
Relevant symbols
Active plan
Active work item
Relevant decisions
Relevant memories
```

Large repositories should use a structural index to select the minimal useful context.

## Work execution

```text
select eligible work item
→ load context
→ load skills
→ load linked requirements/plan/decisions
→ inspect relevant files
→ propose/execute changes
→ run deterministic checks
→ verify
→ checkpoint
→ update state
→ learn
```

## Autonomous loop

```text
while eligible_work_exists:
    task = select_next_task()
    context = load_context(task)
    result = execute(task, context)
    evidence = verify(result)
    checkpoint(result, evidence)
    if verified(result):
        learn(result)
        continue
    else:
        stop_or_replan()
```

## Stop conditions

- verification failure
- security concern
- architecture violation
- ambiguous requirement
- destructive action requiring approval
- policy violation
- configured iteration/token/time budget reached
- insufficient confidence for autonomous continuation
