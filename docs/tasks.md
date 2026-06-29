# Tasks: MAMUT Tower Engine

## Review Workload Forecast

| Field | Value |
|---|---|
| Estimated changed lines | Full MVP: 1,800-3,000; completed slices stayed under the active 800-line review budget |
| 400-line budget risk | High |
| 800-line budget risk | High for full MVP; Medium for remaining work units |
| Chained PRs recommended | Yes |
| Delivery strategy | auto-forecast with chained PRs for reviewable slices |
| Chain strategy | stacked-to-main |

Decision needed before apply: No for the next autonomous slice if it remains within the active review budget
Chained PRs recommended: Yes
Chain strategy: stacked-to-main
400-line budget risk: High

## Phase 0: Domain Gate and Planning Package

- [x] 0.1 Create `docs/scope.md`, `docs/risks.md`, `docs/acceptance-criteria.md`, and `docs/tickets.md`; explicitly exclude API/UI/DB/cloud/runtime AI.
- [x] 0.2 Create `docs/domain/{bibliography,standards_map,papers_map,formulas_register,validation_examples,assumptions,open_questions}.md` for `domain_knowledge_ingestion`.
- [x] 0.3 Create ADRs `docs/adr/0001-rust-puro-para-motor.md` through `0008-estrategia-de-optimizacion-inicial.md` with the required decisions.
- [x] 0.4 Add gate checklist in `docs/domain/acceptance_gate.md`: ingestion files, source-to-test traceability, tolerance policy, singularity policy, report disclaimer examples, review slicing.
- [x] 0.5 Define formula-register statuses and minimum validated formulas; mark unresolved rules `TODO_DOMAIN_VALIDATION`, never invented.

## Phase 1: Rust Workspace Foundation

- [x] 1.1 After Phase 0 gate only, create `Cargo.toml`, `crates/tower-core/src/lib.rs`, and `crates/tower-cli/src/main.rs`.
- [x] 1.2 Add module shells: `units.rs`, `errors.rs`, `geometry.rs`, `materials.rs`, `sections.rs`, `loads.rs`, `model.rs`.
- [x] 1.3 Add CI-quality tickets for `cargo fmt`, `cargo clippy`, `cargo test`, and focused-test guard.

## Phase 2: Core Domain Model and Input

- [x] 2.1 Implement explicit units and validation errors; test missing/ambiguous units and conversion failures.
- [x] 2.2 Implement TOML `TowerModel` parsing for nodes, members, supports, loads, materials, sections, metadata.
- [x] 2.3 Add data input placeholders as data inputs, not normative authority.
- [x] 2.4 Document the pre-WU3 numerical tolerance, singularity, validation-example, and source-to-test gate.

## Phase 3+: Implementation Work Units

- [x] WU3 first step: accepted/proven `example_01_simple_bar` and `example_02_simple_3d_truss_star`, then wrote failing Strict TDD tests referencing those fixture IDs and `NUM-AXIAL-*` trace IDs before solver implementation.
- [x] WU3 3D truss solver: implemented 3 DOF/node truss analysis with assembly, supports, reactions, axial forces, and deterministic singular rejection under Strict TDD.
- [x] WU4 Preliminary checks: enforce validated formula/register boundaries and traceable preliminary rule output.
- [x] WU5 optimizer core: deterministic greedy selection, safety-first behavior, infeasible result, and unresolved-constraint blocking.
- [x] WU5b reporting/examples: implement preliminary report output, deterministic examples, and focused reporting/example tests.
- [x] Phase 7 hardening/regression: added deterministic regression fixtures, covered invalid TOML, singular model, unsupported checks, infeasible optimization, failed/blocked report visibility, and re-verified the MVP contains no API/UI/database/cloud/runtime AI implementation.
