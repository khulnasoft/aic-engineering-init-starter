# AIC Architecture

## Control-plane architecture

The initial implementation adopts Rust 2021 with Cargo. The CLI is the first
runtime boundary; provider integrations and autonomous execution are deferred
until project state and verification are trustworthy.

```text
                           ┌──────────────┐
                           │     User     │
                           └──────┬───────┘
                                  ↓
                           ┌──────────────┐
                           │     CLI      │
                           └──────┬───────┘
                                  ↓
                           ┌──────────────┐
                           │ Orchestrator │
                           └──────┬───────┘
                ┌─────────────────┼─────────────────┐
                ↓                 ↓                 ↓
          Project State        Context          Governance
                ↓                 ↓                 ↓
          Workflow Engine       Skills          Permissions
                └─────────────────┼─────────────────┘
                                  ↓
                           ┌──────────────┐
                           │ AI Runtime   │
                           └──────┬───────┘
                                  ↓
                         Structured Outputs
                                  ↓
                           ┌──────────────┐
                           │ Verification │
                           └──────┬───────┘
                                  ↓
                           State Update
                                  ↓
                           Memory / Learn
```

## Recommended modules

For a Rust implementation:

```text
crates/
├── cli
├── core
├── config
├── project
├── ai
├── stack
├── architecture
├── roadmap
├── planner
├── scaffold
├── context
├── index
├── work
├── verifier
├── review
├── security
├── memory
├── governance
├── templates
└── ui
```

## AI provider abstraction

```text
AIProvider
  ├── OpenAIProvider
  ├── AnthropicProvider
  ├── OpenAICompatibleProvider
  ├── LocalProvider
  └── CustomGatewayProvider
```

Providers should implement chat, streaming and structured-output operations without leaking provider-specific types into the rest of the CLI.

## Data flow

AI outputs should preferably be typed/structured objects:

```text
Requirement
Roadmap
Plan
Decision
WorkItem
Finding
Memory
VerificationResult
```

Markdown should be generated as a human-facing projection rather than being the primary source of truth.
