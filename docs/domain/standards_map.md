# Standards Map

This map connects candidate references to MAMUT Tower Engine domains. It is an ingestion aid, not implementation approval. A source may be listed here while its formulas/checks remain `pending` or `TODO_DOMAIN_VALIDATION` in `formulas_register.md`.

## MVP-Relevant Source Map

| Engine Area | Primary Candidate Sources | Supporting Sources | Repository Handling | Current Status |
|---|---|---|---|---|
| Tower vocabulary and member taxonomy | ASCE/SEI 10-15; conceptual tower design PDF | CIGRÉ/EPRI/Kalaga/Kiessling if later provided | Use for naming, grouping, and report terminology only after review. | pending-review |
| Transmission tower member checks | ASCE/SEI 10-15 | AISC 360-22 for general steel context | Store only cited interpretation, variables, units, limits, and validation status. | pending-review |
| Tension member checks | ASCE/SEI 10-15 | AISC 360-22 | Do not implement final normative strength until formula-register entry is validated. | pending-review |
| Compression and slenderness checks | ASCE/SEI 10-15 | Timoshenko & Gere; AISC 360-22 | Allow only preliminary/provisional checks until source clause, limits, and tests are recorded. | pending-review |
| Slenderness / effective-length semantics (`CHK-SLENDERNESS-001`) | ASCE/SEI 10-15 candidate clauses after domain review | AISC 360-22 and Timoshenko & Gere as context only | Candidate sources may inform `L/r` versus `K·L/r`, effective-length factor `K`, axis radius, bracing/end-condition, member category, and compression-applicability decisions, but they do not approve implementation or pass/fail limits by being listed here. | TODO_DOMAIN_VALIDATION |
| Stability and buckling background | Timoshenko & Gere | AISC 360-22 | Use as theory support, not as a tower-specific design rule by itself. | source-candidate |
| Transmission loading concepts | ASCE MOP 74; IEC 60826 | ASCE 7 for general load context | Keep loads/load cases as data inputs; normative combinations require source-backed rules. | pending-review |
| Wind loading context | ASCE MOP 74; IEC 60826 | ASCE 7 | MVP may use simple provisional loads with visible assumptions; no hidden advanced wind model. | pending-review |
| Load combinations | ASCE MOP 74; IEC 60826 | Project-approved criteria | Do not invent combinations; mark unresolved combinations `TODO_DOMAIN_VALIDATION`. | pending-review |
| Displacement/serviceability limits | ASCE/SEI 10-15 if applicable; project criteria | ASCE 7/AISC 360 context if relevant | Must be explicitly project-defined or source-backed before enforcing. | TODO_DOMAIN_VALIDATION |
| Materials and steel properties | AISC 360-22; project/catalog data | ASCE/SEI 10-15 if it references material requirements | Material catalogs are data; standards are not copied into catalog files. | source-candidate |
| Angle section properties | Manufacturer/catalog data; AISC Manual if later provided | AISC 360-22 | Section catalog must state source, units, and whether values are verified. | pending-review |
| Numerical validation examples | Matrix structural analysis reference; simple hand examples | Project-generated examples | Required for solver validation before structural checks depend on results. | inventory-only |
| Greedy section optimization | Optimization papers inventory | Arora, Christensen/Klarbring, Haftka/Gürdal, Kaveh if later provided | Papers guide algorithm design and benchmarks; do not import claims without review. | pending-review |

## Future / Non-MVP Source Map

| Future Area | Candidate Sources | Why It Is Not MVP |
|---|---|---|
| Concrete foundations | ACI 318 | Foundations and concrete design are explicitly out of MVP scope. |
| Seismic steel provisions | AISC 341-22 | MVP excludes seismic-specific design. |
| Prequalified steel moment connections | AISC 358-22 | MVP excludes detailed connection design and frame moment behavior. |
| Advanced wind/ice/reliability | IEC 60826; ASCE MOP 74; CIGRÉ/EPRI if later provided | MVP uses basic loads and visible assumptions only. |
| Nonlinear/global buckling/dynamics | Stability/FEM references | MVP is linear 3D truss only. |

## Implementation Rules

1. Standards guide the domain register; they are not copied into code or docs.
2. Source presence is not validation.
3. Every implemented formula/check MUST map to `formulas_register.md` with source, location, variables, units, limitations, status, and test evidence.
4. Catalog files under `data/` are data inputs, not normative authority.
5. Reports MUST expose assumptions, source/status, utilization, controlling case, and the disclaimer `not for final engineering design`.
6. Slenderness/effective-length source inventory for `CHK-SLENDERNESS-001` is a research gate only. Until exact clauses, limits, examples, and reviewer approval are recorded in `formulas_register.md` and `validation_examples.md`, the engine must not emit slenderness compliance, pass/fail results, buckling capacity, column strength, or optimizer feasibility evidence.
