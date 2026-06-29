# Design: MAMUT Tower Engine

## Technical Approach

Future implementation will use a pure Rust Cargo workspace with exactly `crates/tower-core` and a minimal `crates/tower-cli`. WU1 does not create Rust crates; it defines the gate and reviewable execution plan.

## Architecture Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Workspace shape | `tower-core` plus thin `tower-cli` | Keeps engineering behavior isolated from executable concerns. |
| Domain gate | `docs/domain/*` before solver/check code | Prevents invented or copied normative rules. |
| Units | Explicit unit types/newtypes; TOML units required | Avoids ambiguous numerical input. |
| Solver | Linear 3D truss, 3 translational DOF/node, axial-only members | Matches preliminary MVP scope. |
| Checks and optimization | Traceable checks and safety-first greedy optimizer | Makes failures explainable and auditable. |

## Data Flow

`TOML -> validation/units -> TowerModel -> analysis -> design checks -> optimization -> report`

Reports must include `not for final engineering design`.

## Future Boundaries

- `tower-core`: units, model, analysis, checks, optimization, reporting, and errors.
- `tower-cli`: load TOML, invoke core, print reports.
- `data/`: catalogs and templates as data inputs, not normative authority.
- `docs/domain/`: source-backed authority and validation status.

## Testing Strategy

When a Rust test runner exists, future work units must add analytical unit tests, solver fixtures, check trace assertions, optimizer regressions, and report failure-path tests. Each implemented formula/check must map to `docs/domain/formulas_register.md` and at least one validation/regression test.
