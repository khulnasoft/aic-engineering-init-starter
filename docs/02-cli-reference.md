# AIC CLI Reference

## Project

```bash
aic init
aic inspect
aic status
aic doctor
aic upgrade
```

- `init`: bootstrap `.agent`, skills, configuration and operating instructions.
- `inspect`: analyze repository structure and engineering health.
- `status`: render current engineering state.
- `doctor`: detect broken configuration, stale state, missing tooling and inconsistencies.
- `upgrade`: update control-plane templates and compatible schema versions.

The initial Rust implementation supports `init`, `config`, `stack`, `inspect`,
`status`, and `doctor`. Other commands are reserved for later roadmap phases.

## Discovery

```bash
aic stack
aic architecture
aic dependencies
aic conventions
aic context
aic index
```

- `stack`: detect language, runtime, framework, database, package manager, build/test/lint/CI systems.
- `architecture`: infer and maintain component boundaries and architectural maps.
- `dependencies`: build dependency graph and find cycles, duplicates, unused packages and conflicts.
- `conventions`: infer repository conventions for code generation and review.
- `context`: build reusable project context for agents.
- `index`: build searchable structural index; SQLite is recommended for larger projects.

## Product and planning

```bash
aic idea
aic requirements
aic roadmap
aic planning
aic decision
aic estimate
```

- `idea`: turn an informal idea into a structured product definition.
- `requirements`: create normalized requirements with acceptance and verification criteria.
- `roadmap`: generate milestone/capability roadmap.
- `planning`: convert roadmap or requirements into implementation plans.
- `decision`: create architecture/design decision records.
- `estimate`: estimate scope, complexity, dependencies and risk with explicit uncertainty.

## Build

```bash
aic scaffold
aic generate
aic migrate
aic refactor
```

Safety rules:

- never silently overwrite user changes;
- show create/update/skip/conflict decisions;
- require verification after structural changes;
- preserve links to requirements, plans and work items.

## Work and execution

```bash
aic todo
aic todo list
aic todo create
aic todo next
aic todo show TODO-001
aic todo verify
aic todo sync

aic work
aic loop
aic resume
aic checkpoint
```

Internally, TODOs are `WorkItem` entities. Persisted work-item identifiers use
the canonical `WI-###` format and may represent features, bugs, refactors,
migrations, research, tests, docs or security work.

## Quality and verification

```bash
aic test
aic verify
aic review
aic security
aic audit
aic coverage
```

`todo verify` verifies individual work-item claims.

`verify` verifies implementation against requirements, plans and acceptance criteria.

## Knowledge

```bash
aic memory
aic learn
aic explain
```

## Governance

```bash
aic policy
aic rules
aic permissions
```

## Configuration

```bash
aic config
aic provider
aic model
aic skill
aic template
```

## Suggested aliases

Long names remain canonical. Optional aliases:

```text
r = roadmap
p = planning
s = stack
c = context
t = todo
v = verify
w = work
```

## Interactive configuration example

```text
◆ Configure AI
│
◇ Provider
│  ● OpenAI
│  ○ Anthropic
│  ○ OpenAI-compatible
│  ○ Local
│
◇ Model
│  ● GPT-5.6
│  ○ GPT-5
│  ○ Custom
│
◇ Endpoint
│  https://api.openai.com/v1
│
◇ Credentials
│  ● Environment / OS keychain
│  ○ Do not persist
│
◇ Reasoning
│  ● Automatic
│  ○ Low
│  ○ Medium
│  ○ High
│
◇ Done
└
```

API keys must not be written into tracked project configuration. Prefer environment variables or OS credential storage.
