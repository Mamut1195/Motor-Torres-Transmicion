# MAMUT Tower Engine

MAMUT Tower Engine is an SDD-planned Rust MVP for preliminary, traceable analysis and discrete section optimization of a simplified self-supporting 69 kV transmission tower.

This repository is currently in the **Phase 7 hardening/regression** slice of the Rust MVP. The MVP includes a pure Rust workspace, deterministic TOML examples, preliminary reporting, and regression coverage, but it remains **not for final engineering design**.

## Current SDD Status

| Area | Status |
|---|---|
| Artifact store | Engram, with selected planning docs materialized in `docs/` |
| Current work unit | Phase 7 hardening/regression |
| Implementation code | Pure Rust core/CLI MVP present |
| Rust workspace | `crates/tower-core` and `crates/tower-cli` |
| Domain gate | Defined in `docs/domain/acceptance_gate.md` |

## What to Read First

1. `docs/scope.md` — what the MVP will and will not do.
2. `docs/acceptance-criteria.md` — acceptance gates for this planning slice and future implementation.
3. `docs/domain/acceptance_gate.md` — mandatory gate before solver/check implementation.
4. `docs/tickets.md` and `docs/tasks.md` — executable work tickets and current SDD task status.

## Safety Boundary

All future outputs must keep the exact disclaimer `not for final engineering design`. No solver, optimization, report, or example should imply final engineering approval.
