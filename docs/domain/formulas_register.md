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
| `QTY-WEIGHT-001` member self-weight quantity | `validated_quantity` | Formula register plus `CheckRule::TotalWeight` trace | Report as non-normative quantity evidence. | Converting the quantity into generated nodal loads without reviewer-approved lumping/modeling assumptions. |
| Wind, conductor, load combinations, load factors, displacement/design-level loading | `TODO_DOMAIN_VALIDATION` | No reviewer-approved clauses/examples recorded | Surface as blocked report evidence. | Emitting pass/fail compliance, code-compliant loading, or final engineering design claims. |

The `NUM-AXIAL-*` entries are numerical validation formulas/methods for software tests only. Their WU3 validation status does not make them normative design rules, and they must not be presented as final engineering design acceptance criteria.

The `QTY-WEIGHT-001` entry is a non-normative quantity trace for software-level preliminary reporting. It does not validate member strength, load combinations, or final engineering design acceptance.

The `CHK-TENSION-001` and `CHK-COMPRESSION-001` entries validate only preliminary axial stress utilization from solved member axial force, nominal section area, and material yield stress. They do not validate buckling, slenderness, displacement, load combinations, connection checks, or final engineering design acceptance.

### `CHK-SLENDERNESS-001` evidence gate

`CHK-SLENDERNESS-001` remains blocked as `TODO_DOMAIN_VALIDATION`. Documentation, reports, optimizers, and future implementation proposals must not treat it as pass/fail compliance, final design evidence, or feasibility evidence until the formula-register row is replaced by a reviewer-approved, source-backed status.

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

Current documentation state: `blocked-only`. This is a guardrail, not an approved runtime formula. It intentionally prevents source, schema, CLI, test, reporting, optimizer, or runtime behavior from depending on unapproved slenderness semantics.

## Source-to-test traceability convention

- Formula/register IDs must appear in tests and report traces once formulas exist.
- For WU3 solver work, numerical-method entries such as `NUM-TRUSS-001` through `NUM-TRUSS-005` must map to tests even when no normative formula is involved.
- No check can be considered implemented if it lacks both a formula-register link and a test link.
- Catalog data may be cited as input provenance, but catalog files are not formula/register authority.
- Normative checks remain blocked until their `TODO_DOMAIN_VALIDATION` status is replaced by an approved source-backed status and matching test evidence.
