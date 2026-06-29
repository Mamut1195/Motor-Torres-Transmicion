# Scope

## Current Batch: WU1 / Phase 0

This batch creates the planning package, ADRs, and domain ingestion gate. It intentionally creates no Cargo workspace, Rust crates, solver code, tests, CLI, API, UI, database, cloud integration, Python runtime, or runtime AI behavior.

## MVP In Scope Later

- Pure Rust engine for preliminary 69 kV tower analysis.
- TOML input with explicit units.
- Linear 3D truss analysis for axial-only members.
- Preliminary checks with source-backed formulas or `TODO_DOMAIN_VALIDATION`.
- Greedy discrete section optimization prioritizing safety over weight.
- Reports containing `not for final engineering design`.

## Out of Scope

API, UI, database, cloud, runtime AI, RAG, BIM/IFC/DXF/DWG, foundations, connection design, final code compliance, nonlinear analysis, dynamics, fatigue, seismic design, and advanced optimization.
