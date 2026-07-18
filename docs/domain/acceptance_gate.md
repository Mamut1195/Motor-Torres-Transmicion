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

## Minimum Load Model v1 gate update

This reporting and documentation boundary is not for final engineering design.

| Evidence category | Accepted status | Gate decision |
|---|---|---|
| Explicit user-provided nodal load cases | `explicit_user_input` | Accepted only as traceable user input with visible source/status. Force values are preserved; no normative loading claim is made. |
| `QTY-WEIGHT-001` member self-weight quantity | `validated_quantity` | Accepted only as non-normative quantity evidence. It must not generate nodal loads without approved lumping/modeling assumptions. |
| Self-weight nodal load generation | `validated_quantity` / approved narrow runtime rule | runtime generation is approved only for `LOAD-SW-DIST-001` straight two-node uniform axial self-weight. endpoint `fz = -0.0769822025 kN` at `fixed` and `free` is validated for the source example; no runtime `civil-rag` lookup or source interpretation is permitted. |
| Wind loads | `TODO_DOMAIN_VALIDATION` | Blocked. |
| Conductor loads | `TODO_DOMAIN_VALIDATION` | Blocked. |
| Load combinations and load factors | `TODO_DOMAIN_VALIDATION` | Blocked. |
| Displacement or design-level loading | `TODO_DOMAIN_VALIDATION` | Blocked. |

Acceptance rule: reports may expose status/source evidence and blocked categories only. They must not emit code-compliant loading behavior, load combinations/factors, generated self-weight nodal loads, wind/conductor loading, or final engineering design claims.

Approved narrow runtime rule: generated self-weight may be created only by the engine-owned `LOAD-SW-DIST-001` path for a straight two-node axial member with uniform self-weight using `W = density * area * length * g / 1000` and endpoint `fz = -W/2`. Nonuniform members, beam fixed-end actions, eccentric loads, wind/conductor loads, load combinations, load factors, controlling cases, and final engineering design claims remain blocked.

## Load Model v2 source evidence gate

This gate is a reviewer-facing evidence package only. It inventories candidate sources and approval requirements for a future load-model v2 phase, but it does not approve formulas, authorize runtime behavior, or change the current solver boundary.

### Runtime boundary

- The only analysis-ready load cases remain explicit user-input nodal loads that pass the current runtime validation boundary.
- Source inventory, candidate clauses, pending formulas, provisional notes, and `TODO_DOMAIN_VALIDATION` records must not be consumed by runtime code.
- This gate does not allow Rust runtime, schema, CLI, solver, reporting, optimizer, example, or test changes.
- This gate does not allow automatic self-weight nodal load generation, wind-load generation, conductor-load generation, load-combination execution, load-factor application, or controlling-case selection.

### Approval fields required before future runtime work

Every load-model v2 assumption must keep `TODO_DOMAIN_VALIDATION` until all fields below are complete and reviewer-approved:

| Approval field | Required content before approval |
|---|---|
| Source | Standard, paper, project rule, or reviewer-owned rule packet. |
| Edition / clause | Exact edition and clause/reference, or a precise project-rule identifier. |
| Interpretation | Reviewer-owned explanation of how the source applies to this engine. |
| Variables / units | Input variables, output variables, units, conversion policy, and sign convention. |
| Applicability / limits | Scope, exclusions, member/load categories, and any source-backed limits. |
| Assumptions | Modeling assumptions required to apply the rule safely. |
| Numeric example | Reviewed calculation with inputs, substitutions, intermediate values, expected result, and trace ID. |
| Tolerance / rationale | Software-comparison tolerance and rationale when the example becomes testable. |
| Reviewer | Human reviewer identity. |
| Approval date | ISO date covering the full evidence packet. |
| Runtime authorization | Explicit future SDD scope that converts the approved packet into tests before implementation. |

### Topic checklist

| Load-model v2 topic | Current gate state | Required before implementation |
|---|---|---|
| Self-weight nodal distribution | `TODO_DOMAIN_VALIDATION` | Approved source/project rule, exact edition/clause or project-rule ID, reviewer interpretation, lumping/modeling assumption, target nodes, directions/signs, variables/units, applicability/limits, numeric example, tolerance/rationale, reviewer, ISO date, and explicit future SDD runtime authorization. |
| Wind loading | `TODO_DOMAIN_VALIDATION` | Exact clauses, exposure/input assumptions, variables/units, applicability limits, numeric example, tolerance, reviewer, and ISO date. |
| Conductor loads | `TODO_DOMAIN_VALIDATION` | Approved conductor loading assumptions, variables/units, source-backed example, applicability limits, reviewer, and ISO date. |
| Load combinations / factors | `TODO_DOMAIN_VALIDATION` | Approved combinations, factors, load-case membership rules, numeric example, tolerance, reviewer, and ISO date. |
| Controlling-case prerequisites | `TODO_DOMAIN_VALIDATION` | Approved semantics for comparing cases, prerequisite combination/factor evidence, deterministic selection rule, reviewer, and ISO date. |

Acceptance rule: candidate source rows in `standards_map.md` are inventory only. A formula, rule, or assumption is approved only when the full approval packet is complete in `formulas_register.md` and its accepted example is complete in `validation_examples.md`.

### Self-weight nodal distribution source-review checklist

`LOAD-SW-DIST-001` and `example_09_self_weight_nodal_distribution_gate` remain `TODO_DOMAIN_VALIDATION`. This section is a blocked review checklist only; it does not approve generated nodal loads, provide a numeric example, name a reviewer/date, or authorize runtime work.

Mandatory ledger fields for a complete non-runtime packet are: source rule, clause/project-rule ID, reviewer interpretation, assumptions, target nodes, signs/directions, units, applicability limits, numeric trace, tolerance rationale, reviewer identity, ISO review date, and future tests-first runtime authorization status. Candidate inventory/arithmetic is not approved engineering evidence and does not authorize runtime execution.

| Checklist item | Required before approval | Current gate state |
|---|---|---|
| Source / project rule | Standard, paper, project rule, or reviewer-owned rule packet that governs distribution from member self-weight quantity to nodes. | missing; blocked |
| Edition / clause or rule ID | Exact edition and clause/reference, or precise project-rule identifier. | missing; blocked |
| Reviewer interpretation | Human-owned explanation of how the source applies to this engine and model abstraction. | missing; blocked |
| Distribution / lumping assumption | Approved assumption for converting `QTY-WEIGHT-001` member self-weight quantity into nodal loads. | missing; blocked |
| Target nodes | Which node or nodes receive the generated load and why. | missing; blocked |
| Directions / signs | Coordinate direction, sign convention, and output force components. | missing; blocked |
| Variables / units | Input variables, output variables, unit conversions, and gravity convention if used. | missing; blocked |
| Applicability / limits | Member categories, geometry assumptions, exclusions, and any source-backed limits. | missing; blocked |
| Numeric example | Reviewed inputs, substitutions, intermediate values, expected nodal force result, and trace ID. | missing; blocked |
| Tolerance / rationale | Software-comparison tolerance and rationale for a future tests-first SDD. | missing; blocked |
| Reviewer / ISO date | Human reviewer identity and ISO approval date for the complete packet. | missing; blocked |
| Runtime authorization | Explicit future SDD scope that writes tests before any load-generation implementation. | missing; blocked |

Boundary rule: `QTY-WEIGHT-001` is quantity-only evidence. It validates a member self-weight quantity trace; it is not a nodal distribution rule, not a load-generation rule, and not authorization for schema, CLI, solver, reporting, optimizer, example, test, or data changes.

Candidate arithmetic guard: total `0.153964405 kN` and equal-end candidate `0.0769822025 kN` per end may be recorded as review material only. This candidate arithmetic does not authorize schema, CLI, runtime, reports, optimizer, examples, or executable tests, and no target-node, axis/sign, tolerance, reviewer/date, or runtime authorization may be inferred from it.

### Matrix Structural Analysis candidate evidence guard

Matrix Structural Analysis, Second Edition has been recorded as candidate/supporting evidence for equivalent nodal and work-equivalent load concepts. This does not change the gate decision: `LOAD-SW-DIST-001` and `example_09_self_weight_nodal_distribution_gate` remain `TODO_DOMAIN_VALIDATION`.

| Candidate evidence | Current gate state |
|---|---|
| Ch. 5 §5.2, Fig. 5.6, Table 5.1, and Eq. 5.21 exploration pointers | Candidate/supporting only; manual PDF page, equation, and sign review required. |
| Ch. 7 §7.5 and Eq. 7.32 exploration pointers | Candidate/supporting only; reviewer must interpret applicability to axial truss self-weight members. |
| Ch. 3 §§3.1-3.2 direct-stiffness context | Context only; not a self-weight distribution rule. |
| Ch. 13 dead-load mention | Non-supporting context unless a future reviewer explicitly documents otherwise. |

Acceptance rule: Matrix evidence alone cannot approve generated nodal loads. Approval still requires reviewer identity, ISO date, interpretation, assumptions, signs, target nodes, applicability limits, reviewed numeric example, tolerance/rationale, and explicit future tests-first runtime authorization.

### Captured civil-rag candidate evidence guard

Captured `civil-rag` excerpts for `LOAD-SW-DIST-001` are reviewer evidence only. They are candidate evidence only and cannot authorize runtime generated loads, schema fields, CLI behavior, solver behavior, reports, optimizer constraints, examples, executable tests, target-node inference, force-component mapping, or approval metadata.

| Candidate source ID | Gate meaning | Runtime decision |
|---|---|---|
| `SRC-CIVIL-RAG-TOWER-SELF-WEIGHT-TRIBUTARY-JOINTS` | Candidate context for self-weight and tributary/joint review. | No runtime authorization; target nodes and allocation remain missing. |
| `SRC-CIVIL-RAG-MATRIX-CH7-WORK-EQUIVALENT-LOADS` | Candidate work-equivalent load context requiring manual equation/sign/applicability review. | No runtime authorization; distribution factors, signs, and applicability remain missing. |
| `SRC-CIVIL-RAG-MOP74-VERTICAL-AXIS-CONTEXT` | Candidate vertical-axis/sign context only. | No runtime authorization; force component mapping remains missing. |

Acceptance remains blocked until the complete reviewer-owned packet is recorded with source rule, reviewer interpretation, assumptions, target nodes, signs/directions, units, applicability, numeric trace, tolerance rationale, reviewer identity, ISO review date, and explicit future tests-first runtime authorization.

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

civil-rag/Postgres unavailable is an explicit blocker. The blocker may be cleared only by successful source retrieval or equivalent manual source review that records semantic choice; exact source title/edition/clause/page or source ID; inputs with units; applicability; limits if any; numeric example if available; tolerance rationale; reviewer identity; ISO approval date; and future tests-first runtime authorization status. Until then, the packet is blocked-only, a separate tests-first runtime authorization is required, and this gate must not authorize Rust runtime slenderness computation.

### `CHK-SLENDERNESS-001` non-runtime approval guard

This docs-only evidence package does not approve or implement slenderness behavior. Source inventory, context references, placeholder examples, and unchecked gate items are not approval evidence.

| Guard item | Required approval evidence | Current state |
|---|---|---|
| Formula semantics | Reviewer-approved choice of `L/r`, `K·L/r`, or explicit `blocked-only`, with exact clause trace | blocked-only; not approved for implementation |
| Clauses and limits | Exact source, edition, clause/reference, interpretation, limits if any, reviewer, and ISO date | blocked; unavailable |
| Inputs and units | Member length basis, `K` policy, axis radii, bracing/end conditions, member category, compression applicability, units, exclusions | blocked; unresolved |
| Accepted example | Inputs, units, expression, intermediate values, result, tolerance/rationale, trace ID, source clause, reviewer, and ISO date | blocked; placeholder only |
| Runtime authorization | Explicit future SDD scope after evidence approval | blocked; no source, schema, CLI, Rust, tests, reporting, optimizer, or runtime behavior is allowed here |

Acceptance rule for this check: no acceptance checkbox or status may close unless reviewer identity and ISO date evidence are present for the full source-backed packet. Until then, `CHK-SLENDERNESS-001` may be mentioned only as an unresolved validation gap and must not support compliance, pass/fail, feasibility, buckling, column-strength, or final-design claims.

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
