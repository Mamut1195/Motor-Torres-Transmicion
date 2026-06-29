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

## Failed-run examples

Failed-run reports generated from singular, unstable, validation-error, or unsupported-rule examples must include:

- the error category and affected model item;
- no final-design claim;
- the disclaimer `not for final engineering design`;
- trace links to the relevant formula/register or numerical-method entries once they exist;
- `TODO_DOMAIN_VALIDATION` for unsupported design-rule checks.
