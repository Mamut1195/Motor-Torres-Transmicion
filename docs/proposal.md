# Proposal: MAMUT Tower Engine

## Intent

Build a pure Rust MVP engine for preliminary, traceable analysis and greedy discrete section optimization of a simple self-supporting 69 kV transmission tower. The value is a reproducible internal engineering sandbox that proves numerical flow, explains failures, and avoids pretending to be final engineering design.

## Scope

In scope: domain knowledge ingestion, ADRs, TOML input design, explicit units, linear 3D truss analysis, traceable preliminary checks, greedy safety-first optimization, and reports that include `not for final engineering design`.

Out of scope: API, UI, database, cloud, Python runtime, runtime AI, BIM/IFC/DXF/DWG, foundations, detailed connections, nonlinear analysis, dynamics, advanced optimization, and copied standards text.

## Capabilities

- Domain ingestion register with bibliography, standards map, formulas, validation examples, assumptions, and open questions.
- Tower model input with explicit units.
- Stable 3D truss analysis with clear singular/unstable rejection.
- Preliminary traceable checks with unsupported criteria marked `TODO_DOMAIN_VALIDATION`.
- Greedy discrete section optimization returning feasible or explicit infeasible results.
- CLI/reporting boundary with mandatory disclaimer.

## Rollback

Stop before implementation if the domain gate cannot validate minimum formulas. WU1 creates docs only; rollback is removing `README.md` and `docs/` materialized planning files while retaining Engram history.
