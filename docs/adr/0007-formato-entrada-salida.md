# ADR 0007: Input and Output Format

Status: accepted for MVP planning

## Context

The MVP needs reviewable examples and deterministic local execution.

## Decision

Use TOML as the primary input format and text/structured reports as initial outputs. Every report must include `not for final engineering design`.

## Consequences

- TOML schemas must include validation metadata and explicit units.
- Report tests must cover both success and failure paths.

## Next Review Trigger

Review this ADR before introducing another input format, report format, or report disclaimer policy.
