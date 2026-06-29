# WU3 Numerical Policy

This policy defines the documentation gate for WU3 solver tests. It sets initial engineering-software test tolerances and failure behavior before any 3D truss solver implementation starts.

## Scope

Applies to WU3 linear 3D truss solver tests only:

- displacements;
- reactions;
- member axial forces;
- element and global stiffness matrix entries;
- singular or unstable model rejection.

It does not validate final tower design rules, normative checks, load combinations, optimization, or production engineering acceptance. Project validation fixtures use SI-derived project units: length in `m`, force in `kN`, stiffness in `kN/m`, elastic modulus in `kN/m^2`, and area in `m^2`.

## Initial tolerance defaults

These defaults are deterministic starting points for tests. Every tolerance must be revisited after the validation examples have approved reference values.

| Quantity | Absolute tolerance | Relative tolerance | Notes |
|---|---:|---:|---|
| Displacements | `1.0e-9 m` | `1.0e-7` | Use for nodal translations in solver fixtures. |
| Reactions | `1.0e-9 kN` | `1.0e-7` | Use for support reaction recovery and equilibrium checks. |
| Member axial forces | `1.0e-9 kN` | `1.0e-7` | Use for axial-only member force recovery. |
| Stiffness matrix entries | `1.0e-8 kN/m` | `1.0e-9` | Use for element/global assembly assertions. |

Comparison rule: a numeric comparison passes when either the absolute tolerance or relative tolerance criterion is satisfied. Tests must state the quantity, tolerance pair, expected value source, and formula/register or numerical-method ID.

## Validation examples required before implementation

WU3 may start only after these examples exist and are accepted:

1. `example_01_simple_bar` — axial bar with known displacement, reaction, and axial force.
2. `example_02_simple_3d_truss_star` — small stable axial-star 3D truss with proposed hand-derived reference values pending reviewer acceptance.
3. `example_03_singular_unstable_model` — insufficient supports or disconnected member, expected rejection.
4. `example_04_units_input_validation` — WU2 units and input-validation regression behavior remains in force.

Expected numeric values may be marked `proposed-reference` or `pending-reviewer-acceptance` until calculated and approved. Pending values are not implementation targets.

## Singularity and near-mechanism policy

WU3 must reject singular or unstable models clearly. Required behavior:

- insufficient supports fail with a deterministic instability error;
- disconnected members or disconnected structural components fail instead of being ignored;
- zero-length members fail as input/model validation errors;
- the solver must not auto-repair supports, connectivity, restraints, or geometry;
- failed-run reports must include the affected model item when known and the disclaimer `not for final engineering design`.

Near-singular or ill-conditioned detection may be conservative and provisional if the selected solver backend does not expose condition numbers yet. In that case, WU3 must document the limitation, prefer safe rejection over silent acceptance, and add a follow-up item before relying on the solver for broader examples.

## Source-to-test traceability

- Formula/register IDs must appear in tests and report traces once formulas exist.
- For WU3 solver work, numerical-method entries must map to tests even if no normative formula is involved.
- No check can be considered implemented if it lacks both a formula-register link and a test link.
- `TODO_DOMAIN_VALIDATION` remains the required marker for unsupported normative checks.

## Gate decision

WU3 may start only after this numerical policy, the validation examples, assumptions, formulas register, and domain acceptance gate are accepted. WU3 must use Strict TDD because refreshed SDD init detected Cargo tests.
