# Executable Tickets

## WU1: Planning Package and Domain Gate

### T-001 Define MVP scope and risks
- Output: `docs/scope.md`, `docs/risks.md`.
- Acceptance: exclusions are explicit; final-design claims are prohibited.

### T-002 Materialize SDD artifacts
- Output: `docs/proposal.md`, `docs/spec.md`, `docs/design.md`, `docs/design_review.md`, `docs/tasks.md`.
- Acceptance: documents are concise, complete enough for review, and align with Engram SDD artifacts.

### T-003 Create domain ingestion package
- Output: `docs/domain/*.md`.
- Acceptance: formulas, sources, assumptions, examples, open questions, and gate checklist are present.

### T-004 Record architecture decisions
- Output: ADRs 0001 through 0008.
- Acceptance: each ADR includes status, context, decision, consequences, and next review trigger.

## WU2: Workspace and Model Foundation

Blocked until WU1 gate acceptance. Create the Cargo workspace, explicit unit types, TOML model, catalogs as data inputs, and quality commands.

### T-005 Create Rust workspace boundary
- Output: root `Cargo.toml`, `crates/tower-core`, and `crates/tower-cli`.
- Acceptance: `tower-core` owns domain scaffolding; `tower-cli` stays a minimal shell and prints `not for final engineering design`.

### T-006 Add units and input validation scaffolding
- Output: explicit unit newtypes, typed validation errors, basic geometry, model/input structs, and WU2 tests.
- Acceptance: missing or ambiguous units fail deterministically; no solver, normative checks, or optimizer logic is implemented.

### T-007 Add quality command gate
- Output: workspace runs `cargo fmt --check`, `cargo test`, and `cargo clippy --workspace --all-targets -- -D warnings`.
- Acceptance: commands pass locally or any unavailable command is recorded honestly in apply progress.

## WU3: Solver

Blocked until WU2. Implement linear 3D truss solver with analytical tests and singular/unstable rejection.

## WU4: Checks

Blocked until source-to-test traceability exists. Implement preliminary checks and trace output.

## WU5: Optimization and Reporting

Implement greedy safety-first optimization, reports, examples, and failure-path regressions. All reports must include `not for final engineering design`.
