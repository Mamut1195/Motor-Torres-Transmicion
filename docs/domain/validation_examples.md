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
| `example_05_member_weight_quantity` | Validate non-normative total member self-weight quantity trace. | `density * area * length * g / 1000` returns deterministic `kN` total weight and maps to `QTY-WEIGHT-001`. | Source-example approved for quantity evidence only; covered by WU4 Strict TDD tests and the source-example harness. |
| `example_06_tension_axial_stress_utilization` | Validate preliminary tension axial stress utilization trace. | `stress_mpa = abs(P_kN) / A_m2 / 1000` and `utilization = stress_mpa / Fy_MPa` map to `CHK-TENSION-001`. | Covered by validated-member-checks-v1 Strict TDD tests. |
| `example_07_compression_axial_stress_utilization` | Validate preliminary compression axial stress utilization trace without buckling claims. | `stress_mpa = abs(P_kN) / A_m2 / 1000` and `utilization = stress_mpa / Fy_MPa` map to `CHK-COMPRESSION-001`. | Covered by validated-member-checks-v1 Strict TDD tests. |
| `example_08_slenderness_effective_length_gate` | Define the required accepted-example shape before any `CHK-SLENDERNESS-001` implementation. | Must state length basis, `K` if applicable, axis-specific radii, units, member category, compression applicability, semantic choice (`L/r`, `K·L/r`, or blocked-only), expected output, trace ID, source clause, and reviewer/date. It must not contain pass/fail compliance unless the docs gate approves the source-backed limit. | Required but not approved; `CHK-SLENDERNESS-001` remains `TODO_DOMAIN_VALIDATION`. |
| `example_09_self_weight_nodal_distribution_gate` | Validate approved narrow runtime self-weight nodal generation. | Straight two-node axial member with uniform self-weight generates endpoint loads only: `fx = 0`, `fy = 0`, `fz = -W/2`. | Approved narrow runtime rule for `LOAD-SW-DIST-001`; all other load generation remains excluded. |

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

Proof note: `crates/tower-core/tests/analysis_solver.rs` references `example_01_simple_bar` and `NUM-AXIAL-001` through `NUM-AXIAL-004`. The source-example harness also validates `crates/tower-core/tests/fixtures/source_examples/example_01_simple_bar.toml` against `examples/simple_bar.toml` without embedding model text. The values follow `k = AE/L = 0.001 * 200,000,000 / 2 = 100,000 kN/m`, `UX = F/k = 10 / 100,000 = 0.0001 m`, support equilibrium `FX = -10 kN`, and axial tension `+10 kN`.

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

Status: source-example approved for quantity evidence only; independently proven in WU4 Strict TDD tests.

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

Proof note: `crates/tower-core/tests/design_checks.rs` references `example_05_member_weight_quantity` and `QTY-WEIGHT-001`. `crates/tower-core/tests/fixtures/source_examples/example_05_member_weight_quantity.toml` also approves this fixture for source-example harness execution of `tower_core_total_weight_check` only. This approval does not authorize nodal distribution, target nodes, signs, distribution factors, Matrix-derived formulas, or runtime self-weight load generation. Normative tension, compression, slenderness, and displacement checks remain blocked as `TODO_DOMAIN_VALIDATION` until approved formula-register entries and tests exist.

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

Mandatory non-runtime approval packet fields are: semantic choice; exact source title/edition/clause/page or source ID; inputs with units; applicability; limits if any; numeric example if available; tolerance rationale; reviewer identity; ISO approval date; and future tests-first runtime authorization status. This template is blocked-only until all fields are reviewer-approved.

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
| Runtime authorization | Future tests-first runtime authorization status; a separate tests-first runtime authorization is required before any executable behavior | not approved |

### Placeholder status

`example_08_slenderness_effective_length_gate` is a non-accepted placeholder. It must not be converted into a Rust test, runtime fixture, report output, optimizer constraint, or implementation formula until reviewer-approved clauses, inputs, limits if any, expected values, reviewer identity, and ISO date are recorded.

### `CHK-SLENDERNESS-001` civil-rag source evidence ledger

This ledger records restored `civil-rag` retrieval evidence for ASCE 10-15. It is source traceability only: retrieval resolved, reviewer approval remains missing, runtime authorization remains missing, and no runtime `civil-rag` dependency is introduced. Each row is candidate material for human review and does not approve a formula, limit, pass/fail compliance, accepted example, optimizer feasibility, or final-design claim.

| civil-rag source ID | standard/source identity | section | excerpt / meaning | candidate semantics | source type | approval boundary |
|---|---|---|---|---|---|---|
| `87c3e208-51b6-4838-930a-45c3331893f1` | ASCE 10-15, standard id `381eb1f0-0cf9-461f-b055-5b2346e38027` | §1.1 | Requirements apply to design, fabrication, and testing of members/connections for electrical transmission structures and hot-rolled/cold-formed shapes. | Source relevance for tower member checks. | `primary` | Candidate only; no reviewer approval or runtime authorization. |
| `6fb62a89-2654-4c75-bbda-57e4d88ab610` | ASCE 10-15, standard id `381eb1f0-0cf9-461f-b055-5b2346e38027` | §2.1 | Standard applies to latticed steel transmission structures with hot-rolled or cold-formed prismatic members connected by bolts. | Applicability to the tower domain and bolted prismatic member context. | `primary` | Candidate only; does not approve software semantics or examples. |
| `7d04473e-3b0b-47db-9492-46a43aeab525` | ASCE 10-15 | §3.4 | Leg members: `L/r <= 150`; other members: `KL/r <= 200`; redundant members: `KL/r` shall not exceed `250`. | Candidate limits and category split. | `primary` | Candidate only; no pass/fail threshold is approved without reviewer interpretation and accepted example. |
| `6eb7b9e4-100f-4a2c-89fb-c7d7975ca39c` | ASCE 10-15 | §3.6 | Defines compression formulas using `KL/r`; variables include `L = unbraced length`, `r = radius of gyration`, and `K = effective length coefficient`. | Candidate definitions for `K`, `L`, `r`, and `K·L/r`. | `primary` | Candidate only; capacity-related excerpts are excluded evidence boundaries only and must not authorize Euler, `Fa`, or column-capacity implementation. |
| `5817f4ec-d28b-4f86-8a33-fed63dfb7ea5` | ASCE 10-15 | §§3.5 / 3.7.4 excerpt | Captures equations including `KL/r = 30 + 0.75 L/r`, `60 + 0.5 L/r`, `KL/r = L/r` for a retrieved range, and partial-restraint equations. | Candidate effective-slenderness inventory for other compression members and restraint conditions. | `primary` | Candidate only; reviewer must choose bracing/end-condition semantics before runtime. |
| `dfa21e97-4af4-422d-9b7c-6c1850117dea` | ASCE 10-15 | §3.7.4.1 | For leg members bolted in both faces, `KL/r = L/r` for `0 <= L/r <= 150`. | Candidate geometric `L/r` path for a stated leg-member condition. | `primary` | Candidate only; no generic leg-member runtime rule is approved. |
| `051f5339-d2a7-4f72-b2d7-1e3b99fd1be0` | ASCE 10-15 | §3.7.4.2 | Other compression members with concentric load at both ends use `KL/r = L/r` for `0 <= L/r <= 120`. | Candidate other-compression-member path under concentric end loading. | `primary` | Candidate only; eccentric/restraint variants remain unresolved. |
| `d78b4be5-e1d6-4ed0-aea6-f8a279619248` | ASCE 10-15 | §3.7.4.3 | Redundant-member rules reference equations and include unrestrained and partial-restraint ranges. | Candidate redundant-member evidence. | `primary` | Candidate only; no generic engine or optimizer rule is approved. |
| `8f733827-febd-4672-98b7-9050d6f2b05a` | ASCE 10-15 | §3.7.4.6 | Where tests and/or analysis demonstrate different restraint, `KL/r` values may be modified. | Candidate exception/modification path. | `primary` | Candidate only; requires human engineering review and future approval. |
| `bb74de97-a8b0-4b5d-a81e-c19a7d5fb0a3` | ASCE 10-15 commentary | C3.7.4 / 8.2 | K factors depend on connection design; leg sections bolted in both legs use actual length (`K = 1`); `L` usually working point to working point; break point `L/r = 120`. | Candidate interpretation context for K factor, length basis, and break-point semantics. | `commentary` | Commentary only; it cannot replace reviewer approval. |
| `74402c38-ca86-4a58-9af5-447f16505003` | ASCE 10-15 examples | Appendix/example context | Examples are illustrative and should not be used without competent advice. | Boundary against blindly treating examples as fixtures. | `example` | Candidate only; not an accepted numeric example. |
| `dadf682b-a9fa-4ecb-8170-c991c205c165` | ASCE 10-15 | Appendix B / §3.18 chunk | Numeric fragments include `L/rz = 32/0.27 = 119` and `L/rz = 54/0.27 = 200`, with references to equations. | Candidate numeric/example material and capacity boundary. | `example` | Incomplete and capacity-related; not an executable fixture, pass/fail claim, or software acceptance example. |

Approval boundary: a future reviewer-owned packet must still provide semantic choice, exact source title/edition/clause/page or source ID, inputs with units, applicability, limits if any, numeric example if available, tolerance rationale, reviewer identity, ISO approval date, and future tests-first runtime authorization status. Until then, `CHK-SLENDERNESS-001` remains `TODO_DOMAIN_VALIDATION` and blocked-only.

## Approved runtime rule: `example_09_self_weight_nodal_distribution_gate`

Status: approved narrow runtime rule for `LOAD-SW-DIST-001` only.

This rule converts the already validated `QTY-WEIGHT-001` arithmetic into deterministic generated nodal loads for a straight two-node axial member with uniform self-weight only. Runtime generation uses `W = density * area * length * g / 1000`; for the approved fixture, total self-weight `0.153964405 kN` produces equal-end runtime value `-0.0769822025 kN` on `fz` at target nodes `fixed` and `free`, with `fx = 0` and `fy = 0`.

Mandatory ledger fields for `LOAD-SW-DIST-001` are complete for this narrow packet: source rule, clause/project-rule ID, reviewer interpretation, assumptions, target nodes, signs/directions, units, applicability limits, numeric trace, tolerance rationale, reviewer identity, ISO review date, and future tests-first runtime authorization status. The sign convention is z-up, so gravity/self-weight acts in negative `fz`. The approved runtime authorization is limited to the source-example fixture and the matching engine-owned generated load case.

Explicit exclusions: nonuniform members, beam fixed-end actions, eccentric loads, wind/conductor loads, load combinations, load factors, controlling-case execution, and final engineering design claims remain blocked. No runtime `civil-rag` lookup or source interpretation is permitted; captured civil-rag rows remain candidate evidence only.

## Load Model v2 accepted-example packet templates

Status: required but not approved. These templates are non-executable review packets. They must not be converted into tests, examples, runtime behavior, reports, optimizer constraints, or controlling-case execution until every approval field is complete.

| Example ID | Topic | Required inputs and units | Expected calculation shape | Tolerance / rationale | Current state |
|---|---|---|---|---|---|
| `example_09_self_weight_nodal_distribution_gate` | Self-weight nodal distribution | Member ID, end nodes, member length, area, density, gravity, source/project rule, coordinate/sign convention, distribution/lumping assumption, target nodes, output force units. | Compute `QTY-WEIGHT-001`, then apply the approved equal-end distribution rule to target nodes and negative `fz`. | Absolute `1e-10`, relative `1e-7`; approved for this fixture only. | Approved narrow runtime rule; straight two-node axial member with uniform self-weight only. |
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

Mandatory ledger fields for `LOAD-SW-DIST-001` are: source rule, clause/project-rule ID, reviewer interpretation, assumptions, target nodes, signs/directions, units, applicability limits, numeric trace, tolerance rationale, reviewer identity, ISO review date, and future tests-first runtime authorization status. Candidate inventory/arithmetic is not approved engineering evidence and does not authorize runtime execution.

Candidate arithmetic: the approved quantity-only trace provides total self-weight `0.153964405 kN`. Under an unapproved equal-end lumping assumption, the review-only arithmetic records an equal-end candidate value `0.0769822025 kN` at each end. These values are reviewer material only: axis/sign, target nodes, distribution factors, tolerance rationale, reviewer/date, and runtime authorization remain unapproved.

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
| Numeric example | Inputs, substitutions, intermediate values, expected nodal force result, and trace ID. | review-only arithmetic exists for total self-weight `0.153964405 kN` and equal-end candidate value `0.0769822025 kN`; not approved |
| Tolerance / rationale | Comparison tolerance and rationale if this later becomes a test fixture. | not approved |
| Reviewer / ISO date | Human reviewer identity and ISO date for the full packet. | not approved |
| Runtime authorization | Future SDD change that writes tests before any runtime load-generation implementation. | not approved |

Boundary rule: do not convert this checklist into Rust tests, runtime behavior, schemas, CLI behavior, reports, optimizer constraints, examples, or data until every field is complete and reviewer-approved in a future SDD change.

Harness note: `crates/tower-core/tests/fixtures/source_examples/example_09_self_weight_nodal_distribution_gate.toml` preserves this item as non-executable `TODO_DOMAIN_VALIDATION` metadata only. It does not approve runtime load generation, formulas, signs, or Matrix-derived self-weight distribution behavior.

### Candidate Matrix evidence for `example_09_self_weight_nodal_distribution_gate`

Matrix Structural Analysis is relevant source inventory for a future accepted example, but it is not an accepted example by itself. Exploration page pointers are recorded below so a reviewer can manually verify the PDF before approving any calculation.

| Candidate source ID | Candidate reference | Possible review use | Current blocker |
|---|---|---|---|
| `SRC-MATRIX-CH5-LOADS-BETWEEN-NODES` | Matrix Structural Analysis, Second Edition, Ch. 5 §5.2; exploration pointer: book p. 108 / PDF p. 129 | Review whether loads between joints and static-equivalent lumping can support a member self-weight distribution example. | Reviewer must approve interpretation, target nodes, signs, and applicability to this engine. |
| `SRC-MATRIX-CH5-FIXED-END-EQUIVALENT-LOADS` | Ch. 5 §5.2, Fig. 5.6, Table 5.1, Eq. 5.21; exploration pointer: book pp. 110-112 / PDF pp. 131-133 | Review equivalent nodal load treatment from fixed-end/reversed fixed-end force concepts. | Equations/signs require manual PDF review; frame/beam evidence cannot be blindly mapped to axial truss members. |
| `SRC-MATRIX-CH7-WORK-EQUIVALENT-LOADS` | Ch. 7 §7.5, Eq. 7.32; exploration pointer: book pp. 194-196 / PDF pp. 215-217 | Review effective/work-equivalent nodal load derivation and the explored uniformly loaded axial member apportionment note. | Reviewer must manually verify the equation/signs and decide whether it applies to gravity self-weight in the project coordinate convention. |

No numeric nodal-load values are approved for `example_09_self_weight_nodal_distribution_gate`. The candidate total and equal-end arithmetic above does not infer axis/sign, target nodes, distribution factors, tolerance rationale, reviewer/date, or runtime authorization. A future accepted example must still provide reviewer-approved inputs, substitutions, intermediate values, expected nodal forces, tolerance/rationale, trace ID, reviewer identity, ISO date, and runtime authorization through a future tests-first SDD.

### Captured civil-rag evidence ledger for `example_09_self_weight_nodal_distribution_gate`

This ledger records candidate evidence only. It is reviewer-facing source traceability, not approval, not an accepted numeric example, and it cannot authorize runtime generated loads.

| Candidate source ID | Excerpt / summary | Retrieval basis | Candidate relation | Approval blocker |
|---|---|---|---|---|
| `SRC-CIVIL-RAG-TOWER-SELF-WEIGHT-TRIBUTARY-JOINTS` | Tower self-weight references indicate member dead/self weight may be considered for structural joint or tributary-point review. | Captured `civil-rag` query for self-weight, tower, nodal distribution, tributary joints, and member dead load. | Provides candidate relation between `QTY-WEIGHT-001` quantity evidence and a future reviewer-owned nodal distribution packet. | Reviewer must still approve source rule, interpretation, assumptions, target nodes, signs/directions, units, applicability, numeric trace, tolerance, reviewer identity, ISO date, and runtime authorization. |
| `SRC-CIVIL-RAG-MATRIX-CH7-WORK-EQUIVALENT-LOADS` | Matrix Structural Analysis Ch. 7 work-equivalent/effective nodal load material may support a future manually reviewed distribution derivation. | Captured `civil-rag` query for work-equivalent loads, axial member distributed load, and Matrix Structural Analysis Ch. 7 Eq. 7.32 context. | Candidate relation to possible equivalent nodal load reasoning for member self-weight distribution. | Reviewer must manually verify equation text, signs, applicability to axial truss members, and all approval fields before any runtime SDD. |
| `SRC-CIVIL-RAG-MOP74-VERTICAL-AXIS-CONTEXT` | MOP 74 vertical-axis/sign material may provide context for documenting gravity direction and coordinate convention. | Captured `civil-rag` query for transmission tower vertical axis, gravity, sign convention, and load direction context. | Candidate relation to sign/direction review only. | It cannot infer force components, signs, target nodes, or generated loads. |

The retrieval basis and candidate relation fields are intentionally non-executable. They preserve traceability for human review while keeping `TODO_DOMAIN_VALIDATION`, missing approval blockers, and the no-runtime boundary intact.

## Failed-run examples

Failed-run reports generated from singular, unstable, validation-error, or unsupported-rule examples must include:

- the error category and affected model item;
- no final-design claim;
- the disclaimer `not for final engineering design`;
- trace links to the relevant formula/register or numerical-method entries once they exist;
- `TODO_DOMAIN_VALIDATION` for unsupported design-rule checks.
