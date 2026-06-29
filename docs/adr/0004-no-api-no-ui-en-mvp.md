# ADR 0004: No API or UI in the MVP

Status: accepted for MVP planning

## Context

The first risk is domain and numerical correctness, not product delivery surface.

## Decision

Do not build API, UI, database, cloud integration, or runtime AI in the MVP.

## Consequences

- The CLI remains an internal executable shell when added later.
- Product adapters require a later SDD change after the core is validated.

## Next Review Trigger

Review this ADR before adding API, UI, database, cloud integration, or runtime AI behavior.
