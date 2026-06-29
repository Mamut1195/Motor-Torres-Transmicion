# ADR 0001: Use Pure Rust for the Engine

Status: accepted for MVP planning

## Context

The engine needs deterministic local execution, strong typing, and clear separation from UI/API concerns.

## Decision

Implement the future engine in pure Rust, with engineering behavior in `tower-core` and a thin executable boundary later.

## Consequences

- No Python runtime, API server, database, cloud dependency, or runtime AI in the MVP.
- Dependency choices must be ADR-backed before solver implementation.

## Next Review Trigger

Review this ADR before adding any runtime, external service, or non-Rust engine dependency.
