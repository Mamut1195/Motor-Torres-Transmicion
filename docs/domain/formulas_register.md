# Formulas Register

## Status Values

- `validated`: Source-backed and covered by at least one validation/regression test.
- `pending`: Source identified but not yet validated for implementation.
- `provisional`: Usable only for planning or examples; not accepted as production behavior.
- `TODO_DOMAIN_VALIDATION`: Known requirement with no approved formula yet; implementation must not silently guess.

## Minimum Formula Set Before Implementation

| ID | Formula / Rule | Purpose | Status | Required Evidence |
|---|---|---|---|---|
| `NUM-TRUSS-001` | 3D truss member length and direction cosines | Geometry and stiffness assembly | pending | Analytical example. |
| `NUM-TRUSS-002` | Axial bar local/global stiffness contribution | Solver core | pending | `example_01_simple_bar` and `example_02_simple_3d_truss_star`. |
| `NUM-TRUSS-003` | Support restraint application | Boundary conditions | pending | Stable truss and singular/unstable fixtures. |
| `NUM-TRUSS-004` | Reaction recovery | Solver reporting | pending | Equilibrium validation. |
| `NUM-TRUSS-005` | Singularity / instability rejection | Failed-run behavior | pending | Insufficient-support or disconnected-member fixture with deterministic error. |
| `NUM-AXIAL-001` | Axial stiffness `k = AE/L` using `A` in `m^2`, `E` in `kN/m^2`, and `L` in `m`, yielding `kN/m`. | Numerical validation method for axial bar/truss stiffness. | validated for WU3 software fixtures | `example_01_simple_bar`, `example_02_simple_3d_truss_star`, and `crates/tower-core/tests/analysis_solver.rs`. |
| `NUM-AXIAL-002` | Axial displacement `u = F/k` for a restrained linear axial member path, using `F` in `kN` and `k` in `kN/m`, yielding `m`. | Numerical validation method for displacement fixtures. | validated for WU3 software fixtures | Hand-derived displacement references in `example_01_simple_bar`, `example_02_simple_3d_truss_star`, and `crates/tower-core/tests/analysis_solver.rs`. |
| `NUM-AXIAL-003` | Static equilibrium reaction recovery, where support reactions balance applied nodal loads with opposite sign for the restrained direction. | Numerical validation method for reaction fixtures. | validated for WU3 software fixtures | Support reactions in `example_01_simple_bar`, `example_02_simple_3d_truss_star`, and `crates/tower-core/tests/analysis_solver.rs`. |
| `NUM-AXIAL-004` | Linear axial member force recovery from the applied axial load/displacement response; positive sign denotes tension in these fixtures. | Numerical validation method for axial force fixtures. | validated for WU3 software fixtures | Member axial forces in `example_01_simple_bar`, `example_02_simple_3d_truss_star`, and `crates/tower-core/tests/analysis_solver.rs`. |
| `QTY-WEIGHT-001` | Member self-weight quantity `density * area * length * g / 1000`, using density in `kg/m^3`, area in `m^2`, length in `m`, standard gravity `g = 9.80665 m/s^2`, yielding `kN`. | Non-normative preliminary quantity trace for member self-weight. | validated for WU4 software fixture | `example_05_member_weight_quantity` and `crates/tower-core/tests/design_checks.rs`. |
| `CHK-TENSION-001` | Preliminary tension axial stress utilization `utilization = (abs(P_kN) / A_m2 / 1000) / Fy_MPa`, where positive axial force denotes tension. | Preliminary member axial stress classification only. | validated | Hand calculation examples in `validation_examples.md` and regression coverage in `crates/tower-core/tests/design_checks.rs`. |
| `CHK-COMPRESSION-001` | Preliminary compression axial stress utilization `utilization = (abs(P_kN) / A_m2 / 1000) / Fy_MPa`, where negative axial force denotes compression stress. This is not buckling, column capacity, or final code strength. | Preliminary member axial stress classification only. | validated | Hand calculation examples in `validation_examples.md` and regression coverage in `crates/tower-core/tests/design_checks.rs`. |
| `CHK-SLENDERNESS-001` | Slenderness / effective-length check. The approved quantity is intentionally undecided: geometric `L/r`, effective `K·L/r`, or blocked-only documentation. | Preliminary member check | TODO_DOMAIN_VALIDATION | Reviewer-approved semantic choice; exact source clause(s); variables and units; member length basis; effective-length factor `K` if applicable; axis-specific radius policy (`rx`, `ry`, minor/governing radius); bracing/end-condition assumptions; member category and compression applicability; limitations; trace ID; accepted numeric example(s); reviewer/date. |
| `CHK-DISPLACEMENT-001` | Displacement limit check | Serviceability check | TODO_DOMAIN_VALIDATION | Approved project criterion and test. |

## Enforcement

Future code must reference formula IDs in tests and report traces. Unsupported rules must remain visible as `TODO_DOMAIN_VALIDATION`.

## Minimum Load Model v1 Evidence Boundary

This boundary is not for final engineering design. It separates accepted input evidence from blocked loading categories.

| Evidence | Runtime status | Source | Allowed use | Blocked use |
|---|---|---|---|---|
| Explicit nodal load cases | `explicit_user_input` | User-provided TOML `load_cases` records | Parse, validate node references, preserve `kN` force components, and show status/source in reports. | Treating placeholder loads as code-compliant wind, conductor, combination, factor, or final-design loading. |
| `QTY-WEIGHT-001` member self-weight quantity | `validated_quantity` | Formula register plus `CheckRule::TotalWeight` trace | Report as non-normative quantity evidence only. | Treating the quantity as a nodal distribution rule, load-generation rule, or approval to create generated nodal loads without reviewer-approved lumping/modeling assumptions. |
| Wind, conductor, load combinations, load factors, displacement/design-level loading | `TODO_DOMAIN_VALIDATION` | No reviewer-approved clauses/examples recorded | Surface as blocked report evidence. | Emitting pass/fail compliance, code-compliant loading, or final engineering design claims. |

The `NUM-AXIAL-*` entries are numerical validation formulas/methods for software tests only. Their WU3 validation status does not make them normative design rules, and they must not be presented as final engineering design acceptance criteria.

The `QTY-WEIGHT-001` entry is a non-normative quantity trace for software-level preliminary reporting. It does not validate member strength, load combinations, nodal distribution, load generation, or final engineering design acceptance.

The `CHK-TENSION-001` and `CHK-COMPRESSION-001` entries validate only preliminary axial stress utilization from solved member axial force, nominal section area, and material yield stress. They do not validate buckling, slenderness, displacement, load combinations, connection checks, or final engineering design acceptance.

## Load Model v2 Candidate Evidence Register

This register separates source inventory from approval. Rows below are not approved formulas or runtime authorization. Any row with missing approval evidence remains `TODO_DOMAIN_VALIDATION` and must not be converted into Rust runtime behavior, schema fields, CLI behavior, reports, optimizer constraints, examples, or tests.

| ID | Load topic | Candidate formula / rule | Status | Required approval evidence |
|---|---|---|---|---|
| `LOAD-SW-DIST-001` | Self-weight nodal distribution | approved runtime values: total `0.153964405 kN`; endpoint `fz = -0.0769822025 kN` at `fixed` and `free` from `W = density * area * length * g / 1000` and `fz = -W/2`. | validated for approved narrow runtime rule | Source-example fixture `example_09_self_weight_nodal_distribution_gate`, reviewer Jonnathan, ISO date 2026-07-10, target nodes `fixed/free`, z-up negative `fz`, tolerance, and explicit tests-first runtime authorization. nonuniform members, beam fixed-end actions, eccentric loads, wind/conductor loads, and load combinations remain excluded. |
| `LOAD-WIND-001` | Wind loading | Wind-derived tower load calculation and mapping to analysis nodes. | TODO_DOMAIN_VALIDATION | Exact source clauses, exposure/input assumptions, variables/units, applicability/limits, numeric example, tolerance/rationale, reviewer, ISO date, and explicit runtime authorization. |
| `LOAD-COND-001` | Conductor loads | Conductor-derived load assumptions and transfer to tower attachment nodes. | TODO_DOMAIN_VALIDATION | Source clauses/project rules, span/input assumptions, variables/units, load transfer interpretation, applicability/limits, numeric example, tolerance/rationale, reviewer, ISO date, and explicit runtime authorization. |
| `LOAD-COMB-001` | Load combinations / factors | Combination membership and factor application for load cases. | TODO_DOMAIN_VALIDATION | Source/project rule, exact combinations/factors, variables/units, applicability/limits, numeric example showing combined values, tolerance/rationale, reviewer, ISO date, and explicit runtime authorization. |
| `LOAD-CTRL-001` | Controlling-case prerequisites | Deterministic rule for selecting or reporting controlling cases after approved combinations exist. | TODO_DOMAIN_VALIDATION | Approved prerequisite combination/factor evidence, controlling metric, tie-breaking/reporting semantics, applicability/limits, numeric example, tolerance/rationale, reviewer, ISO date, and explicit runtime authorization. |

Approval rule: `candidate`, `pending`, `provisional`, and `TODO_DOMAIN_VALIDATION` are non-implementation states. Future code may reference a `LOAD-*` ID only after the row is replaced by a complete reviewer-approved evidence packet and a future SDD phase writes tests first.

`LOAD-SW-DIST-001` is now approved only for the narrow runtime rule above: straight two-node axial member with uniform self-weight, equal endpoint allocation, and negative `fz`. No runtime `civil-rag` lookup, source interpretation, wind/conductor loading, eccentric load, beam fixed-end action, nonuniform load, load combination, or final-design claim is approved.

### `LOAD-SW-DIST-001` blocked packet guard

`LOAD-SW-DIST-001` remains `TODO_DOMAIN_VALIDATION` because the approval packet is incomplete. The existing `QTY-WEIGHT-001` validation proves only the total member self-weight quantity; it does not decide how that quantity is distributed, which nodes receive it, or which force directions/signs apply.

Mandatory ledger fields for a complete non-runtime approval packet are: source rule, clause/project-rule ID, reviewer interpretation, assumptions, target nodes, signs/directions, units, applicability limits, numeric trace, tolerance rationale, reviewer identity, ISO review date, and future tests-first runtime authorization status. Candidate inventory/arithmetic is not approved engineering evidence and does not authorize runtime execution.

| Evidence field | Required value before approval | Current value | Gate state |
|---|---|---|---|
| Source / project rule | Governing standard, paper, project rule, or reviewer-owned packet for nodal distribution. | not recorded | blocked |
| Edition / clause or rule ID | Exact edition and clause/reference, or precise project-rule identifier. | not recorded | blocked |
| Reviewer interpretation | Human-owned interpretation of the source for this engine. | not recorded | blocked |
| Distribution / lumping assumption | Rule that maps member self-weight quantity to target node loads. | not recorded | blocked |
| Target nodes | Node selection and allocation semantics. | not recorded | blocked |
| Directions / signs | Coordinate direction, sign convention, and force component mapping. | not recorded | blocked |
| Variables / units | Inputs, outputs, unit conversions, and gravity convention if applicable. | quantity inputs exist for `QTY-WEIGHT-001`; distribution variables are not approved | blocked |
| Applicability / limits | Scope, member/load categories, exclusions, and source-backed limits. | not recorded | blocked |
| Numeric example | Inputs, substitutions, intermediate values, expected nodal force result, and trace ID. | candidate review values: total `0.153964405 kN`; equal-end lumping candidate `0.0769822025 kN` per end; not an approved formula, nodal distribution rule, load-generation rule, or runtime authorization | blocked |
| Tolerance / rationale | Future software-comparison tolerance and rationale. | not recorded | blocked |
| Reviewer / ISO date | Reviewer identity and ISO date covering the complete packet. | not recorded | blocked |
| Runtime authorization | Explicit future SDD that converts the approved packet into tests before implementation. | not recorded | blocked |

Runtime guard: this row must not be converted into Rust runtime behavior, schemas, CLI options, solver logic, reports, optimizer constraints, examples, data, or tests while any field remains blocked.

Missing approval packet: axis/sign, target nodes, distribution factors, tolerance rationale, reviewer/date, and runtime authorization remain required before any executable interpretation of `LOAD-SW-DIST-001`.

#### `LOAD-SW-DIST-001` Matrix Structural Analysis candidate evidence

The following entries are candidate/supporting evidence only. They do not replace the blocked approval fields above and do not authorize generated self-weight nodal loads.

| Candidate source ID | Source reference from exploration | Support level | Review status | Approval impact |
|---|---|---|---|---|
| `SRC-MATRIX-CH3-DIRECT-STIFFNESS-CONTEXT` | Matrix Structural Analysis, Second Edition, Ch. 3 §§3.1-3.2; exploration pointer: book pp. 31-46 / PDF pp. 52-67 | Context for direct-stiffness global displacement and force vectors. | Manual PDF page review and reviewer interpretation required. | Does not define self-weight distribution. |
| `SRC-MATRIX-CH5-LOADS-BETWEEN-NODES` | Ch. 5 §5.2 `Loads Between Nodal Points`; exploration pointer: book p. 108 / PDF p. 129 | Candidate support for loads between joints/natural nodes, selected-node lumping, and static equivalence. | Manual PDF page and wording review required. | Does not approve this engine's lumping rule or target nodes. |
| `SRC-MATRIX-CH5-FIXED-END-EQUIVALENT-LOADS` | Ch. 5 §5.2, Fig. 5.6, Table 5.1, Eq. 5.21; exploration pointer: book pp. 110-112 / PDF pp. 131-133 | Candidate support for fixed-end forces and reversed fixed-end/equivalent nodal loads in stiffness analysis. | Manual PDF page, equation, and sign review required. | Beam/frame fixed-end evidence must not be applied blindly to axial truss members. |
| `SRC-MATRIX-CH7-WORK-EQUIVALENT-LOADS` | Ch. 7 §7.5, Eq. 7.32; exploration pointer: book pp. 194-196 / PDF pp. 215-217 | Candidate support for effective/work-equivalent nodal loads; exploration notes possible simple apportionment for a uniformly loaded axial member. | Manual PDF page, equation, sign, and reviewer applicability review required. | Closest supporting evidence, but still does not approve gravity direction, sign convention, target nodes, or runtime behavior. |
| `SRC-MATRIX-CH13-DEAD-LOAD-CONTEXT` | Ch. 13 dead-load mention; exploration pointer: book p. 410 / PDF p. 431 | Non-supporting context only. | Manual PDF page review required if cited further. | Does not define a self-weight generation or distribution rule. |

Matrix approval blocker: extracted equations, signs, and page references remain untrusted until a human reviewer manually checks the PDF and records interpretation. `LOAD-SW-DIST-001` stays `TODO_DOMAIN_VALIDATION` until the complete approval packet is filled, including applicability to axial truss members, target nodes, directions/signs, reviewed numeric example, tolerance, reviewer identity, ISO date, and explicit future tests-first runtime authorization.

#### `LOAD-SW-DIST-001` captured civil-rag candidate evidence

The following `civil-rag` source identifiers are captured reviewer material only. They are not approved formulas, not source interpretation, and not runtime authorization. `LOAD-SW-DIST-001` remains `TODO_DOMAIN_VALIDATION` while the approval fields above are missing.

| Candidate source ID | Captured excerpt / summary | Retrieval basis | Candidate relation | Approval impact |
|---|---|---|---|---|
| `SRC-CIVIL-RAG-TOWER-SELF-WEIGHT-TRIBUTARY-JOINTS` | Tower self-weight context indicates member dead/self weight may be collected for review against structural joints or tributary points. | Captured `civil-rag` retrieval for self-weight, tower, nodal distribution, tributary joints, and member dead load. | Candidate context for why a reviewer may consider converting member self-weight quantity into nodal loads. | Does not approve target nodes, allocation factors, signs/directions, or generated load cases. |
| `SRC-CIVIL-RAG-MATRIX-CH7-WORK-EQUIVALENT-LOADS` | Matrix Structural Analysis Ch. 7 work-equivalent load discussion may support effective nodal load reasoning after manual equation and applicability review. | Captured `civil-rag` retrieval for work-equivalent loads, axial member distributed load, and Matrix Structural Analysis Ch. 7 Eq. 7.32 context. | Candidate evidence closest to a possible distribution rule for a future reviewer-owned packet. | Does not approve applicability to axial truss self-weight, gravity direction, sign convention, tolerance, or runtime behavior. |
| `SRC-CIVIL-RAG-MOP74-VERTICAL-AXIS-CONTEXT` | ASCE MOP 74 vertical-axis/sign context may help a reviewer document direction conventions. | Captured `civil-rag` retrieval for transmission tower vertical axis, gravity, sign convention, and load direction context. | Candidate sign/direction context only. | Does not approve force component mapping, signs, target nodes, or load generation. |

These rows are candidate evidence only and cannot authorize runtime generated loads. Future approval still requires source rule, reviewer interpretation, assumptions, target nodes, signs/directions, units, applicability, numeric trace, tolerance rationale, reviewer identity, ISO review date, and explicit tests-first runtime authorization.

### `CHK-SLENDERNESS-001` evidence gate

`CHK-SLENDERNESS-001` remains blocked as `TODO_DOMAIN_VALIDATION`. Documentation, reports, optimizers, and future implementation proposals must not treat it as pass/fail compliance, final design evidence, or feasibility evidence until the formula-register row is replaced by a reviewer-approved, source-backed status.

Mandatory non-runtime approval packet fields are: semantic choice; exact source title/edition/clause/page or source ID; inputs with units; applicability; limits if any; numeric example if available; tolerance rationale; reviewer identity; ISO approval date; and future tests-first runtime authorization status. Missing any field keeps the packet blocked and does not authorize Rust runtime slenderness computation.

Before any implementation can be approved, the domain gate must record:

- whether the checked quantity is geometric `L/r`, effective `K·L/r`, or intentionally blocked-only;
- exact clause references and interpretation notes from the governing source, with supporting sources labeled as context only;
- variables, units, sign/applicability rules, member length basis, axis-specific radius policy, `K` policy, bracing/end-condition assumptions, member category coverage, compression-only or other applicability constraints, and limitations;
- accepted numeric examples with trace IDs and reviewer/date; and
- explicit wording that no slenderness pass/fail, code compliance, buckling capacity, column strength, or final engineering design claim is allowed while the status is `TODO_DOMAIN_VALIDATION`, `pending`, or `provisional`.

#### `CHK-SLENDERNESS-001` compact evidence record

| Evidence field | Required value before approval | Current value | Gate state |
|---|---|---|---|
| Semantic choice | Exactly one of `L/r`, `K·L/r`, or explicit `blocked-only`, with clause trace and reviewer/date | `blocked-only` because no reviewer-owned semantic approval is recorded | blocked |
| Governing clause(s) | Exact source ID, edition, clause/reference, interpretation note, and trace ID | Candidate inventory exists in `standards_map.md`; exact approved clause(s) are unavailable | blocked |
| Variables and units | Member length basis, radius/radii units, `K` if applicable, and conversion policy | Required fields are identified, but no approved values or unit policy are recorded | blocked |
| Required inputs | Length basis, `K`, axis-specific radii (`rx`, `ry`, minor/governing policy), bracing/end conditions, member category, and compression applicability | Inputs remain unresolved and must stay visible in `open_questions.md` | blocked |
| Limits and applicability | Source-backed limit(s), exclusions, member categories, and whether any threshold supports later reporting | No approved limit or applicability rule exists; no compliance threshold may be inferred | blocked |
| Accepted numeric example | Inputs, expression, intermediate values, result, tolerance/rationale, trace ID, clause, reviewer, and ISO date | `example_08_slenderness_effective_length_gate` is a non-accepted template only | blocked |
| Reviewer approval | Reviewer identity plus ISO approval date for clauses, semantic choice, limits, and example | not approved | blocked |
| Runtime authorization | Explicit future tests-first runtime authorization status after a complete evidence packet is approved | not approved; a separate tests-first runtime authorization is required | blocked |

Current documentation state: `blocked-only`. This is a guardrail, not an approved runtime formula. It intentionally prevents source, schema, CLI, test, reporting, optimizer, or runtime behavior from depending on unapproved slenderness semantics.

#### `CHK-SLENDERNESS-001` civil-rag retrieval evidence status

The prior source-retrieval blocker is retrieval resolved for ASCE 10-15 evidence recorded in `standards_map.md` and `validation_examples.md`. Reviewer approval remains missing and runtime authorization remains missing. The formula status stays `TODO_DOMAIN_VALIDATION`; the ledger does not approve `L/r`, `K·L/r`, limits, pass/fail compliance, buckling, Euler/column capacity, optimizer feasibility, final-design claims, schema expansion, or runtime `civil-rag` dependency.

Capacity-related excerpts are excluded evidence boundaries only. Retrieved mentions of Euler behavior, `Fa`, compression stress formulas, column capacity, design strength, or appendix numeric fragments may be cited as boundary evidence, but they are not acceptance fixtures and must not become executable checks.

#### `CHK-SLENDERNESS-001` geometric `L/r` authorization formula boundary

This formula-register packet is documentation-only. Required packet fields are decision; semantic choice; applicability; scalar radius policy; units; numeric example; tolerance; reviewer identity; ISO date; and future tests-first runtime authorization status.

The runtime authorization decision is decision: `approved | rejected | deferred`; current decision: `deferred`. Because the packet is deferred due to missing reviewer identity, ISO date, numeric example, and tolerance, no runtime `L/r` computation is authorized.

If a later reviewer approves the packet, the only candidate quantity formula is `L/r = two-node member length / existing scalar section.radius_of_gyration`, with `L in m, scalar r in m, output dimensionless`, and semantic choice `geometric_scalar_L_over_r_quantity_only`. The scalar radius policy must explicitly accept or reject the existing scalar `section.radius_of_gyration` as `r` and record governing-axis implications before implementation.

Boundary: this packet does not authorize runtime `L/r`, does not authorize schema expansion, does not authorize pass/fail, does not authorize ASCE limits, does not authorize effective `K·L/r`, does not authorize Euler or column capacity, does not authorize optimizer feasibility, does not authorize runtime `civil-rag`, and does not authorize final-design claims.

## Source-to-test traceability convention

- Formula/register IDs must appear in tests and report traces once formulas exist.
- For WU3 solver work, numerical-method entries such as `NUM-TRUSS-001` through `NUM-TRUSS-005` must map to tests even when no normative formula is involved.
- No check can be considered implemented if it lacks both a formula-register link and a test link.
- Catalog data may be cited as input provenance, but catalog files are not formula/register authority.
- Normative checks remain blocked until their `TODO_DOMAIN_VALIDATION` status is replaced by an approved source-backed status and matching test evidence.
