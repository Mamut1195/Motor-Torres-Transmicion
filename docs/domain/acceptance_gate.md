# Domain Ingestion Acceptance Gate

Milestone 1 implementation may begin only after this gate is accepted. This gate separates source inventory, formula validation, and implementation approval; these are not the same thing.

## Current Gate Status

| Field | Value |
|---|---|
| Inventory status | Completed for currently provided project PDFs and official URL text files. |
| Formula validation status | Preliminary axial stress utilization formulas `CHK-TENSION-001` and `CHK-COMPRESSION-001` are validated for software-level preliminary checks only. Normative design formulas remain pending or blocked. |
| Solver implementation status | Pre-WU3 documentation gate prepared; WU3 may start only after these docs are accepted. |
| Check implementation status | Preliminary tension and compression axial stress utilization checks may emit pass/fail only with validated trace IDs; all other checks remain blocked unless each check is source-backed or explicitly reports `TODO_DOMAIN_VALIDATION`. |
| Recommended next state | Accept or reject the pre-WU3 numerical policy and validation example gate. |
| Reviewer / date | User accepted with gaps on 2026-06-27. |

## Required Files

- [x] `docs/domain/bibliography.md`
- [x] `docs/domain/standards_map.md`
- [x] `docs/domain/papers_map.md`
- [x] `docs/domain/formulas_register.md`
- [x] `docs/domain/validation_examples.md`
- [x] `docs/domain/assumptions.md`
- [x] `docs/numerical-policy.md`
- [x] `docs/domain/open_questions.md`

## Completed Inventory Scope

- ASCE/SEI 10-15, ASCE MOP 74, IEC 60826, ASCE 7, AISC 360/341/358, ACI 318, Timoshenko & Gere, conceptual tower design PDF, and optimization paper PDFs were inventoried or mapped.
- AISC 360 and ASCE 7 are supporting/context sources unless a later formula-register entry validates exact MVP use.
- ACI 318, AISC 341, and AISC 358 are future/non-MVP references for foundations, seismic scope, or connections.
- Greedy discrete section sizing remains the only MVP optimization baseline; metaheuristics are future scope.

## Required Policies

- [x] Formula statuses use only `validated`, `pending`, `provisional`, or `TODO_DOMAIN_VALIDATION`.
- [x] Each implemented formula/check maps to a formula-register row and at least one test.
- [x] Numerical tolerance policy is documented before solver code.
- [x] Singularity and near-mechanism policy is documented before solver code.
- [x] Unit conversion failure behavior is field-specific and deterministic.
- [x] Catalogs are treated as data inputs, not normative authority.
- [x] Reports and failed-run examples include `not for final engineering design`.
- [x] Review slicing remains WU1 domain/ADRs, WU2 scaffold/model, WU3 solver, WU4 checks, WU5 optimizer/report/examples.

## Allowed Work After `Accepted with gaps`

If the reviewer accepts this gate with gaps, the following WU2 work may proceed:

- Create the Rust workspace and test runner.
- Create module shells and type boundaries.
- Implement explicit unit types and deterministic input validation scaffolding.
- Add placeholder tests for validation behavior.

The following remains blocked:

- Normative member strength formulas.
- Final tension/compression/slenderness checks.
- Normative load combinations.
- Solver acceptance claims without validation examples and numerical policy.
- Any report that implies final engineering design.

## Validated-member-checks-v1 gate update

The accepted implementation scope unblocks only preliminary member axial stress utilization for:

- `CHK-TENSION-001`
- `CHK-COMPRESSION-001`

The following remain blocked and must not emit pass/fail engineering compliance unless a future gate validates them with source-backed formulas and tests:

- buckling and column strength;
- slenderness limits;
- displacement limits;
- load combinations;
- provisional, pending, or `TODO_DOMAIN_VALIDATION` formulas.

For `CHK-SLENDERNESS-001`, blocked status includes any state where the approved quantity, source clauses, limits, required inputs, accepted examples, or reviewer approval are missing. Blocked/provisional slenderness must not be counted as report compliance, pass/fail evidence, optimizer feasibility evidence, buckling capacity, column strength, or final engineering design evidence.

## `CHK-SLENDERNESS-001` research gate

`CHK-SLENDERNESS-001` remains `TODO_DOMAIN_VALIDATION`. A future implementation request must be rejected or deferred unless all gate items below are complete:

- [ ] Approved semantic decision: geometric `L/r`, effective `K·L/r`, or blocked-only documentation.
- [ ] Exact source clause(s), interpretation notes, limitations, and reviewer/date are recorded.
- [ ] Required inputs are defined: member length basis, `K` if applicable, axis-specific radii, bracing/end conditions, member categories, compression applicability, units, and exclusions.
- [ ] Any pass/fail limit is source-backed and reviewer-approved.
- [ ] At least one accepted numeric example is recorded in `validation_examples.md` with trace ID `CHK-SLENDERNESS-001`.
- [ ] Reporting wording preserves `not for final engineering design` and does not claim slenderness compliance while status is blocked, pending, provisional, or `TODO_DOMAIN_VALIDATION`.
- [ ] Optimizer feasibility rules confirm blocked/provisional slenderness cannot count as validated pass evidence.

Until every item is accepted, documentation may describe only the gap and required evidence. Runtime, schema, CLI, test, or source-code changes for slenderness remain out of scope for this gate.

## Pre-WU3 gate

WU3 may start only after the reviewer accepts the following documentation set:

- `docs/numerical-policy.md`
- `docs/domain/validation_examples.md`
- `docs/domain/assumptions.md`
- `docs/domain/formulas_register.md`
- this acceptance gate

WU3 must use Strict TDD because the refreshed SDD init detected Cargo tests. The first WU3 tests must cover the accepted numerical examples, singular/unstable rejection, and source-to-test traceability before production solver behavior is implemented.

Normative checks remain blocked. A WU3 solver implementation may add numerical-method traces, but it must not implement final tension, compression, slenderness, displacement-limit, load-combination, optimizer, API, UI, DB, cloud, runtime AI, or final-design behavior.

## Acceptance Rule

If any minimum formula remains unresolved, implementation for that rule is blocked or must surface `TODO_DOMAIN_VALIDATION` without pretending support.

## Reviewer Acceptance

| Reviewer | Date | Decision | Notes |
|---|---|---|---|
| Jonnathan | 2026-06-27 | Accepted with gaps | WU2 scaffold/base may proceed. Solver acceptance, normative checks, and load combinations remain blocked until validation examples, numerical policy, and formula-register traceability are completed. |
