# ADR 0002: Initial Analysis as a 3D Space Truss

Status: accepted for MVP planning

## Context

The MVP must prove traceable numerical flow without expanding into frame, nonlinear, dynamic, or connection design.

## Decision

Model the initial tower as a linear 3D truss with axial-only members and three translational DOF per node.

## Consequences

- Rotational DOF, bending, nonlinear behavior, and dynamics are out of scope.
- Solver work must include stable, singular, and unstable validation examples.

## Next Review Trigger

Review this ADR before adding rotational DOF, frame behavior, nonlinear analysis, or dynamic analysis.
