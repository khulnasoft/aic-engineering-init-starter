# AIC Governance

AI coding systems require explicit controls.

## Policy examples

```text
Never silently overwrite user changes.
Never bypass failing tests just to report success.
Never remove tests to make a build pass.
Never introduce dependencies without policy approval.
Never change production configuration automatically.
Never perform destructive commands without authorization.
Never change database schema without a migration plan.
Never mark work verified without evidence.
```

## Policy model

```yaml
policy:
  dependency_install:
    require_approval: true

  destructive_commands:
    require_approval: true

  production_files:
    read: allowed
    write: approval

  database_migrations:
    verification_required: true
```

## Execution boundaries

```yaml
execution:
  max_iterations: 20
  require_clean_tests: true
  require_verification: true
  stop_on_security_issue: true
  stop_on_architecture_violation: true
  stop_on_ambiguous_requirement: true
```

## Credentials

Provider credentials should be kept outside tracked project state. Recommended precedence:

```text
environment variable
→ OS credential store
→ configured secret manager
```

Avoid storing raw API keys in `.agent/config.yaml`.
