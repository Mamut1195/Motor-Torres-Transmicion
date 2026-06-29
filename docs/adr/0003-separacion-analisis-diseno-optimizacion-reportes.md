# ADR 0003: Separate Analysis, Checks, Optimization, and Reporting

Status: accepted for MVP planning

## Context

Solver results, design checks, optimization decisions, and report text have different responsibilities and failure modes.

## Decision

Keep analysis, preliminary checks, optimization, and reporting as separate boundaries in `tower-core`.

## Consequences

- Optimizer consumes public analysis/check results instead of hiding failures.
- Reports must expose assumptions, validation status, and `not for final engineering design`.

## Next Review Trigger

Review this ADR before merging analysis, checks, optimization, or reporting into shared control flow.
