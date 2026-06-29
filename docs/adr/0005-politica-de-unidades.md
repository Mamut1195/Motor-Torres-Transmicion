# ADR 0005: Unit Policy

Status: accepted for MVP planning

## Context

Ambiguous units are a high-risk source of plausible wrong engineering results.

## Decision

All external TOML values must declare units. Internal implementation must use explicit unit types/newtypes instead of bare floats for domain quantities.

## Consequences

- Missing or ambiguous units are validation errors.
- Unit conversion behavior must be deterministic and covered by future tests.

## Next Review Trigger

Review this ADR before accepting unitless inputs, changing canonical units, or weakening unit validation.
