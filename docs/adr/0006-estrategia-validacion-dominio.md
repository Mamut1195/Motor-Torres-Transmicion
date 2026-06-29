# ADR 0006: Domain Validation Strategy

Status: accepted for MVP planning

## Context

The engine must not invent formulas or copy protected standards text.

## Decision

Use `docs/domain/` as the domain ingestion gate. Formula statuses are `validated`, `pending`, `provisional`, and `TODO_DOMAIN_VALIDATION`.

## Consequences

- Solver/check implementation is blocked until minimum formulas and validation examples are accepted.
- Unsupported rules must remain visible in outputs instead of being guessed.

## Next Review Trigger

Review this ADR before implementing a formula, check, or standard-derived rule without accepted validation evidence.
