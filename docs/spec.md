# MAMUT Tower Engine Requirements Specification

## domain-knowledge-ingestion

The system MUST require a domain register before solver/check implementation. It MUST record sources, validation examples, assumptions, open questions, and formula statuses: `validated`, `pending`, `provisional`, or `TODO_DOMAIN_VALIDATION`. It MUST NOT copy copyrighted standards text.

Acceptance scenarios:
- Valid ingestion is accepted only when sources, examples, assumptions, and statuses are present.
- Missing validation is rejected for implementation or marked `TODO_DOMAIN_VALIDATION`.

## tower-model-input

The system MUST accept TOML as the primary input for a simplified 69 kV self-supporting tower with body, legs, crossarms, materials, sections, supports, loads, and validation metadata. All external values MUST declare units, and internal representations MUST use explicit unit types/newtypes.

## tower-linear-analysis

The system MUST analyze a stable 3D truss with 3 translational DOF per node, axial-only members, supports, loads, displacements, reactions, and axial forces. Singular or unstable models MUST be rejected with clear errors.

## tower-design-checks

The system MUST perform preliminary tension, compression, slenderness, and displacement checks with traceable inputs, formulas, status, and complete failure explanations. Unsupported checks MUST be marked `TODO_DOMAIN_VALIDATION`.

## tower-section-optimization

The system MUST provide greedy discrete section optimization that prioritizes feasibility and safety over weight and returns explicit `infeasible` status when constraints cannot be satisfied.

## tower-reporting

The system MUST report inputs, assumptions, analysis results, checks, optimization status, errors, and the disclaimer `not for final engineering design`. It MUST NOT expose API, UI, cloud, database, Python runtime, or runtime AI behavior.
