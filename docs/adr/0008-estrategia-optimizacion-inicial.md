# ADR 0008: Initial Optimization Strategy

Status: accepted for MVP planning

## Context

The optimizer must be explainable and safe for a preliminary MVP.

## Decision

Use greedy discrete section selection that prioritizes feasibility and safety over weight. If no candidate set satisfies required checks, return explicit `infeasible` status.

## Consequences

- Advanced optimization is out of scope.
- Tie-breaking and candidate ordering must be deterministic and tested later.

## Next Review Trigger

Review this ADR before adding non-greedy optimization, stochastic search, or weight-first selection behavior.
