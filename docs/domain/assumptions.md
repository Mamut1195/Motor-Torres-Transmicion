# Assumptions

- The MVP is preliminary and internal; it is `not for final engineering design`.
- Tower behavior is simplified as a linear 3D truss with axial-only members.
- Nodes have three translational DOF; rotational DOF and frame behavior are out of scope.
- Material and section catalogs are data inputs, not normative authority.
- Unit conversion failures are validation errors, not warnings.
- Optimization prioritizes feasibility/safety before weight.
- Unsupported criteria remain visible as `TODO_DOMAIN_VALIDATION`.

## WU3 numerical assumptions

- WU3 solver tests use deterministic absolute tolerance and relative tolerance values from `docs/numerical-policy.md`.
- Those tolerances are initial engineering-software test tolerances only; they are not final validation of any design rule, code provision, or production engineering acceptance criterion.
- Every tolerance must be revisited after `example_01_simple_bar` and `example_02_simple_3d_truss` have approved reference values.
- WU3 must reject singular or unstable models clearly. Near-singular or ill-conditioned detection may be conservative and provisional while the solver backend does not expose condition numbers.
- The engine must not auto-repair missing supports, disconnected members, duplicate IDs, or invalid connectivity. Input/model problems must remain explicit validation or instability failures.
- Normative checks remain blocked until formula-register entries are source-backed and linked to tests.
