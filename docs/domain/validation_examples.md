# Validation Examples

These examples define the minimum evidence gate before WU3 solver implementation can start. They are engineering-software test fixtures for software validation, not normative design examples and not final validation of tower design rules.

All proposed numeric fixtures use SI-derived project units: length in `m`, force in `kN`, stiffness in `kN/m`, elastic modulus in `kN/m^2`, and area in `m^2`.

## WU3 minimum example set

| Example ID | Purpose | Required result | Current state |
|---|---|---|---|
| `example_01_simple_bar` | Validate axial bar stiffness, displacement, reaction, and axial force. | Closed-form displacement, reaction, and axial force match the accepted fixture within the WU3 absolute tolerance and relative tolerance policy. | Independently proven in WU3 Strict TDD tests. |
| `example_02_simple_3d_truss_star` | Validate 3D DOF mapping, support application, reactions, and member axial forces on a small stable axial-star truss. | Reference displacements, reactions, and axial forces match the accepted fixture within the WU3 tolerance policy. | Independently proven in WU3 Strict TDD tests. |
| `example_03_singular_unstable_model` | Validate singular or unstable model rejection. | Insufficient supports or disconnected member input is rejected with a deterministic instability error and failed-run report trace. | Fixture pending; expected outcome is rejection, not auto-repair. |
| `example_04_units_input_validation` | Preserve WU2 validation behavior. | Missing/ambiguous units, duplicate IDs, and unknown references continue to fail with field-specific errors. | Covered by WU2 tests; keep as regression evidence. |
| `example_05_member_weight_quantity` | Validate non-normative total member self-weight quantity trace. | `density * area * length * g / 1000` returns deterministic `kN` total weight and maps to `QTY-WEIGHT-001`. | Covered by WU4 Strict TDD tests. |
| `example_06_tension_axial_stress_utilization` | Validate preliminary tension axial stress utilization trace. | `stress_mpa = abs(P_kN) / A_m2 / 1000` and `utilization = stress_mpa / Fy_MPa` map to `CHK-TENSION-001`. | Covered by validated-member-checks-v1 Strict TDD tests. |
| `example_07_compression_axial_stress_utilization` | Validate preliminary compression axial stress utilization trace without buckling claims. | `stress_mpa = abs(P_kN) / A_m2 / 1000` and `utilization = stress_mpa / Fy_MPa` map to `CHK-COMPRESSION-001`. | Covered by validated-member-checks-v1 Strict TDD tests. |
| `example_08_slenderness_effective_length_gate` | Define the required accepted-example shape before any `CHK-SLENDERNESS-001` implementation. | Must state length basis, `K` if applicable, axis-specific radii, units, member category, compression applicability, semantic choice (`L/r`, `K·L/r`, or blocked-only), expected output, trace ID, source clause, and reviewer/date. It must not contain pass/fail compliance unless the docs gate approves the source-backed limit. | Required but not approved; `CHK-SLENDERNESS-001` remains `TODO_DOMAIN_VALIDATION`. |

## Proposed reference fixture: `example_01_simple_bar`

Status: independently proven in WU3 Strict TDD tests.

This is a hand-derived axial bar fixture for WU3 Strict TDD solver tests. It is not a normative design example.

### Model

| Item | Value |
|---|---:|
| Geometry | 2-node axial bar along global X |
| Node A | `x = 0 m`, fixed |
| Node B | `x = 2 m`, free |
| Length `L` | `2 m` |
| Elastic modulus `E` | `200,000,000 kN/m^2` |
| Area `A` | `0.001 m^2` |
| Load at Node B | `FX = +10 kN` |

### Expected values

| Quantity | Proposed reference value | Units | Trace |
|---|---:|---|---|
| Axial stiffness `k = AE/L` | `100,000` | `kN/m` | `NUM-AXIAL-001` |
| Node B displacement `UX = F/k` | `0.0001` | `m` | `NUM-AXIAL-002` |
| Support reaction at Node A `FX` | `-10` | `kN` | `NUM-AXIAL-003` |
| Member axial force | `+10` | `kN` tension | `NUM-AXIAL-004` |

Proof note: `crates/tower-core/tests/analysis_solver.rs` references `example_01_simple_bar` and `NUM-AXIAL-001` through `NUM-AXIAL-004`. The values follow `k = AE/L = 0.001 * 200,000,000 / 2 = 100,000 kN/m`, `UX = F/k = 10 / 100,000 = 0.0001 m`, support equilibrium `FX = -10 kN`, and axial tension `+10 kN`.

## Proposed reference fixture: `example_02_simple_3d_truss_star`

Status: independently proven in WU3 Strict TDD tests.

This is a hand-derived decoupled axial-star fixture for WU3 Strict TDD solver tests. It is not a normative design example.

### Model

One free node at the origin is connected by three axial bars to fixed supports on the negative X, Y, and Z axes.

| Member | Fixed support node | Free node | Length `L` | `E` | `A` |
|---|---|---|---:|---:|---:|
| X bar | `(-1, 0, 0)` | `(0, 0, 0)` | `1 m` | `200,000,000 kN/m^2` | `0.001 m^2` |
| Y bar | `(0, -1, 0)` | `(0, 0, 0)` | `1 m` | `200,000,000 kN/m^2` | `0.001 m^2` |
| Z bar | `(0, 0, -1)` | `(0, 0, 0)` | `1 m` | `200,000,000 kN/m^2` | `0.001 m^2` |

Each member axial stiffness is `k = AE/L = 200,000 kN/m`.

Load at the free node: `FX = +10 kN`, `FY = +20 kN`, `FZ = +30 kN`.

### Expected values

| Quantity | Proposed reference value | Units | Trace |
|---|---:|---|---|
| X bar axial stiffness | `200,000` | `kN/m` | `NUM-AXIAL-001` |
| Y bar axial stiffness | `200,000` | `kN/m` | `NUM-AXIAL-001` |
| Z bar axial stiffness | `200,000` | `kN/m` | `NUM-AXIAL-001` |
| Free-node displacement `UX` | `0.00005` | `m` | `NUM-AXIAL-002` |
| Free-node displacement `UY` | `0.00010` | `m` | `NUM-AXIAL-002` |
| Free-node displacement `UZ` | `0.00015` | `m` | `NUM-AXIAL-002` |
| Support X reaction `FX` | `-10` | `kN` | `NUM-AXIAL-003` |
| Support Y reaction `FY` | `-20` | `kN` | `NUM-AXIAL-003` |
| Support Z reaction `FZ` | `-30` | `kN` | `NUM-AXIAL-003` |
| X bar axial force | `+10` | `kN` tension | `NUM-AXIAL-004` |
| Y bar axial force | `+20` | `kN` tension | `NUM-AXIAL-004` |
| Z bar axial force | `+30` | `kN` tension | `NUM-AXIAL-004` |

Proof note: `crates/tower-core/tests/analysis_solver.rs` references `example_02_simple_3d_truss_star` and `NUM-AXIAL-001` through `NUM-AXIAL-004`. Each orthogonal bar has `k = AE/L = 0.001 * 200,000,000 / 1 = 200,000 kN/m`; therefore `UX/UY/UZ = 10/20/30 divided by 200,000 = 0.00005/0.00010/0.00015 m`, reactions balance the applied loads, and positive member forces are `+10/+20/+30 kN` tension.

## Pending numeric approvals

Expected numeric values for `example_01_simple_bar` and `example_02_simple_3d_truss_star` were independently re-derived before WU3 implementation and are now covered by Strict TDD solver tests. Future numeric changes require updating the derivation and tests together.

## Preliminary quantity fixture: `example_05_member_weight_quantity`

Status: independently proven in WU4 Strict TDD tests.

This fixture validates only member self-weight quantity tracing. It is not a normative design check and must not be used as a final engineering design acceptance criterion.

### Model

| Item | Value |
|---|---:|
| Geometry | 2-node axial bar along global X |
| Length `L` | `2 m` |
| Density `ρ` | `7,850 kg/m^3` |
| Area `A` | `0.001 m^2` |
| Standard gravity `g` | `9.80665 m/s^2` |

### Expected values

| Quantity | Reference value | Units | Trace |
|---|---:|---|---|
| Member mass `ρAL` | `15.7` | `kg` | `QTY-WEIGHT-001` |
| Member self-weight `ρALg / 1000` | `0.153964405` | `kN` | `QTY-WEIGHT-001` |

Proof note: `crates/tower-core/tests/design_checks.rs` references `example_05_member_weight_quantity` and `QTY-WEIGHT-001`. Normative tension, compression, slenderness, and displacement checks remain blocked as `TODO_DOMAIN_VALIDATION` until approved formula-register entries and tests exist.

### Minimum Load Model v1 reporting use

Status: `validated_quantity` evidence only; not for final engineering design.

`QTY-WEIGHT-001` may appear in load-model evidence reports as source-backed member self-weight quantity provenance. It must not create self-weight nodal loads, wind loads, conductor loads, load combinations, load factors, or code-compliant loading results. Missing lumping/modeling assumptions remain `TODO_DOMAIN_VALIDATION`.

## Preliminary axial stress utilization fixtures: `example_06` and `example_07`

Status: independently proven in validated-member-checks-v1 Strict TDD tests.

These fixtures validate only preliminary axial stress utilization classification. They are not normative design examples and must not be used as final engineering design acceptance criteria.

### Model

| Item | Tension example | Compression example |
|---|---:|---:|
| Axial force `P` | `+10 kN` | `-10 kN` |
| Nominal area `A` | `0.001 m²` | `0.001 m²` |
| Yield stress `Fy` | `250 MPa` | `250 MPa` |

### Expected values

| Quantity | Reference value | Units | Trace |
|---|---:|---|---|
| Tension axial stress | `10` | `MPa` | `CHK-TENSION-001` |
| Tension utilization | `0.04` | ratio | `CHK-TENSION-001` |
| Compression axial stress | `10` | `MPa` | `CHK-COMPRESSION-001` |
| Compression utilization | `0.04` | ratio | `CHK-COMPRESSION-001` |

Unit proof: `stress_mpa = abs(P_kN) / A_m2 / 1000 = abs(10 kN) / 0.001 m² / 1000 = 10 MPa`. The conversion works because `1 kN/m² = 0.001 MPa`, so dividing `kN / m²` by `1000` yields `MPa`. Utilization is therefore `10 MPa / 250 MPa = 0.04` for both the tension and compression axial stress examples.

Proof note: `crates/tower-core/tests/design_checks.rs` references `example_06_tension_axial_stress_utilization`, `example_07_compression_axial_stress_utilization`, `CHK-TENSION-001`, and `CHK-COMPRESSION-001`. Buckling, slenderness, displacement, load combinations, and provisional or non-validated formulas remain blocked.

## Required slenderness/effective-length fixture shape: `example_08_slenderness_effective_length_gate`

Status: required but not approved. This section is a gate template only; it is not a numeric validation fixture and must not be used as implementation approval.

Before `CHK-SLENDERNESS-001` can move beyond `TODO_DOMAIN_VALIDATION`, an accepted fixture must provide:

| Required field | Purpose |
|---|---|
| Source clause and reviewer/date | Proves the example is source-backed and approved. |
| Semantic choice | States whether the expected quantity is geometric `L/r`, effective `K·L/r`, or blocked-only documentation. |
| Member length basis | Defines the physical or analytical length used by the check. |
| Effective-length factor `K` | Required if the accepted semantic choice is effective slenderness; otherwise explicitly not applicable. |
| Axis-specific radii | Defines `rx`, `ry`, and whether the governing/minor radius or another source-defined radius is used. |
| Units | States length and radius units and any conversion policy. |
| Applicability | Defines member category, compression applicability, bracing/end-condition assumptions, and exclusions. |
| Expected output | Gives the source-backed numeric output only after approval; no pass/fail unless an approved limit is also recorded. |
| Trace ID | Links the fixture to `CHK-SLENDERNESS-001` and the future test/report trace. |

Until this fixture is approved, reports and optimizer output may mention the validation gap only. They must not emit slenderness compliance, pass/fail status, buckling capacity, column strength, or feasibility evidence.

### Accepted-example evidence template

The following shape defines the evidence that a future reviewer must supply. It is intentionally non-executable until every approval field is complete.

| Field | Required content for an accepted example | Current value |
|---|---|---|
| Inputs | Member length basis; `K` if applicable; axis-specific `rx`/`ry` values; governing radius policy; member category; bracing/end conditions; compression applicability | not approved |
| Units | Units for length, radius, and any conversion policy used by the expression | not approved |
| Expression | Required semantic expression: `L/r`, `K·L/r`, or explicit `blocked-only`, with reviewer-owned source trace before implementation | `blocked-only` |
| Intermediate values | Source-backed intermediate substitutions and calculations | not approved |
| Expected result | Numeric result only after reviewer approval; no compliance outcome unless a source-backed limit is approved separately | not approved |
| Tolerance / rationale | Numerical tolerance and rationale for software comparison, if the example later becomes a test fixture | not approved |
| Trace ID | `CHK-SLENDERNESS-001` plus source clause trace ID(s) | not approved |
| Source clause | Exact source, edition, clause/reference, and interpretation note | not approved |
| Reviewer / date | Reviewer identity and ISO approval date | not approved |

### Placeholder status

`example_08_slenderness_effective_length_gate` is a non-accepted placeholder. It must not be converted into a Rust test, runtime fixture, report output, optimizer constraint, or implementation formula until reviewer-approved clauses, inputs, limits if any, expected values, reviewer identity, and ISO date are recorded.

## Load Model v2 accepted-example packet templates

Status: required but not approved. These templates are non-executable review packets. They must not be converted into tests, examples, runtime behavior, reports, optimizer constraints, or controlling-case execution until every approval field is complete.

| Example ID | Topic | Required inputs and units | Expected calculation shape | Tolerance / rationale | Current state |
|---|---|---|---|---|---|
| `example_09_self_weight_nodal_distribution_gate` | Self-weight nodal distribution | Member ID, end nodes, member length, area, density, gravity, source/project rule, coordinate/sign convention, distribution/lumping assumption, target nodes, output force units. | Compute `QTY-WEIGHT-001`, then apply the approved distribution rule to target nodes and force directions only after the rule is approved. | Required if converted to a future software comparison; not approved. | `TODO_DOMAIN_VALIDATION`; source/project rule, exact clause or rule ID, interpretation, distribution assumption, target nodes, directions/signs, numeric example, tolerance, reviewer, ISO date, and runtime authorization missing. |
| `example_10_wind_load_gate` | Wind loading | Approved source inputs such as geometry, exposure/pressure assumptions, direction, tributary area or equivalent mapping, units, and applicability limits. | Apply the approved wind source interpretation and map resulting loads to the model representation. | Required if converted to a future software comparison; not approved. | `TODO_DOMAIN_VALIDATION`; exact clauses, interpretation, reviewer, and ISO date missing. |
| `example_11_conductor_load_gate` | Conductor loads | Approved conductor/span inputs, attachment nodes, load direction/sign convention, units, and transfer assumptions. | Apply the approved conductor load rule and map forces to tower attachment nodes. | Required if converted to a future software comparison; not approved. | `TODO_DOMAIN_VALIDATION`; exact assumptions, source, reviewer, and ISO date missing. |
| `example_12_load_combination_factor_gate` | Load combinations / factors | Approved load case IDs, participating load categories, factors, units, sign convention, and source/project rule. | Combine approved component loads using approved factors and show intermediate and final combined vectors. | Required if converted to a future software comparison; not approved. | `TODO_DOMAIN_VALIDATION`; combinations, factors, reviewer, and ISO date missing. |
| `example_13_controlling_case_gate` | Controlling-case prerequisites | Approved combined case results, governing metric, tie-breaking/reporting rule, blocked-case handling, and units where applicable. | Select or report the controlling case only from approved combinations using the approved deterministic rule. | Required if converted to a future software comparison; not approved. | `TODO_DOMAIN_VALIDATION`; prerequisite combinations and controlling semantics missing. |

### Required approval fields for each load-model v2 example

| Field | Required content before example acceptance |
|---|---|
| Source clause | Exact source, edition, clause/reference, or reviewer-owned project-rule ID. |
| Interpretation | How the reviewer applies the source to this engine. |
| Variables / units | Input variables, output variables, unit conversions, and sign convention. |
| Applicability / limits | Scope, exclusions, and assumptions required for safe use. |
| Numeric example | Inputs, substitutions, intermediate values, final expected result, and trace ID. |
| Tolerance / rationale | Comparison tolerance and rationale if the example becomes a future test. |
| Reviewer / date | Human reviewer identity and ISO approval date. |
| Runtime authorization | Future SDD change that writes tests before implementing runtime behavior. |

Placeholder rule: every row in this section remains `TODO_DOMAIN_VALIDATION`. Candidate source inventory does not make these accepted examples, and accepted examples are required before load-model v2 implementation can begin.

### Blocked review checklist: `example_09_self_weight_nodal_distribution_gate`

Status: `TODO_DOMAIN_VALIDATION`. This is a checklist/template only. It is not a numeric validation example, not an executable fixture, and not approval for generated self-weight nodal loads.

| Field | Required content for future acceptance | Current value |
|---|---|---|
| Source / project rule | Governing standard, paper, project rule, or reviewer-owned packet for distributing member self-weight to nodes. | not approved |
| Edition / clause or rule ID | Exact edition and clause/reference, or precise project-rule identifier. | not approved |
| Reviewer interpretation | Explanation of how the source applies to this engine and model abstraction. | not approved |
| Inputs | Member ID, end nodes, member length, area, density, gravity convention if used, and any source-required geometry variables. | not approved |
| Quantity boundary | `QTY-WEIGHT-001` may provide only the member self-weight quantity; it does not define nodal distribution. | quantity-only; distribution not approved |
| Distribution / lumping assumption | Rule for splitting or assigning the quantity to target nodes. | not approved |
| Target nodes | Nodes that receive generated loads and the allocation assigned to each. | not approved |
| Directions / signs | Coordinate direction, sign convention, and force component mapping for each target node. | not approved |
| Variables / units | Input variables, output variables, unit conversions, and output nodal force units. | not approved |
| Applicability / limits | Member categories, geometry assumptions, exclusions, and any source-backed limits. | not approved |
| Numeric example | Inputs, substitutions, intermediate values, expected nodal force result, and trace ID. | not approved |
| Tolerance / rationale | Comparison tolerance and rationale if this later becomes a test fixture. | not approved |
| Reviewer / ISO date | Human reviewer identity and ISO date for the full packet. | not approved |
| Runtime authorization | Future SDD change that writes tests before any runtime load-generation implementation. | not approved |

Boundary rule: do not convert this checklist into Rust tests, runtime behavior, schemas, CLI behavior, reports, optimizer constraints, examples, or data until every field is complete and reviewer-approved in a future SDD change.

## Failed-run examples

Failed-run reports generated from singular, unstable, validation-error, or unsupported-rule examples must include:

- the error category and affected model item;
- no final-design claim;
- the disclaimer `not for final engineering design`;
- trace links to the relevant formula/register or numerical-method entries once they exist;
- `TODO_DOMAIN_VALIDATION` for unsupported design-rule checks.
