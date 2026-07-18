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

## Minimum Load Model v1 Source Handling

This section is not for final engineering design. It records how reporting may describe load evidence without approving load generation.

| Load evidence area | Candidate source status | Engine handling |
|---|---|---|
| Explicit nodal loads supplied in TOML | User input, not a standard-derived loading model | Report as `explicit_user_input` with source text; preserve force values only. |
| `QTY-WEIGHT-001` self-weight quantity | Validated software quantity trace | Report as `validated_quantity`; do not generate nodal loads from it. |
| Wind and conductor loads | Candidate sources only; no approved clauses/examples | Report as `TODO_DOMAIN_VALIDATION` if mentioned. |
| Load combinations and factors | Candidate sources only; no approved combinations/factors | Report as `TODO_DOMAIN_VALIDATION`; do not infer combinations or factors. |
| Displacement/design-level loading | Unapproved for the load-model slice | Report as `TODO_DOMAIN_VALIDATION`; do not emit final-design claims. |

## Load Model v2 Candidate Source Inventory

This inventory helps reviewers find sources. It is not approval. A source listed here remains `candidate` until the formula register records the exact clause/reference, interpretation, variables/units, applicability/limits, assumptions, accepted numeric example, tolerance/rationale if testable, reviewer, ISO date, and future SDD runtime authorization.

| Source ID | Load topic | Candidate source / clause reference | Intended review role | Evidence status | Runtime authorization |
|---|---|---|---|---|---|
| `SRC-ASCE-MOP74-LOADS-CANDIDATE` | Self-weight distribution, wind, conductor loads, combinations | ASCE MOP 74; exact edition/clause references not yet recorded | Candidate transmission-line loading guidance for reviewer extraction. | candidate | none; `TODO_DOMAIN_VALIDATION` remains. |
| `SRC-IEC-60826-LOADS-CANDIDATE` | Wind, conductor loads, reliability/loading cases | IEC 60826; exact edition/clause references not yet recorded | Candidate reliability/loading source for reviewer comparison. | candidate | none; `TODO_DOMAIN_VALIDATION` remains. |
| `SRC-ASCE7-WIND-CONTEXT` | Wind loading context | ASCE 7; exact clause references not approved for this engine | Supporting wind context only; not tower-specific approval by itself. | candidate/context | none; `TODO_DOMAIN_VALIDATION` remains. |
| `SRC-PROJECT-LOAD-CRITERIA-CANDIDATE` | Load combinations / factors, controlling-case prerequisites | Future reviewer-owned project criteria packet | Project-specific rule source required if standards do not define the MVP boundary directly. | blocked | none; waiting for reviewer-owned packet. |
| `SRC-QTY-WEIGHT-001-CONTEXT` | Self-weight nodal distribution | Existing validated quantity `QTY-WEIGHT-001` | Quantity source only; may support a future distribution rule but does not define lumping or nodal mapping. | validated quantity / distribution not approved | none for generated loads. |
| `SRC-MATRIX-CH3-DIRECT-STIFFNESS-CONTEXT` | Self-weight nodal distribution context | Matrix Structural Analysis, Second Edition, Ch. 3 §§3.1-3.2; exploration pointer: book pp. 31-46 / PDF pp. 52-67 | Supporting context for direct-stiffness global displacement and force-vector formulation. | candidate/context; page pointer pending manual PDF verification | none; not a distribution rule. |
| `SRC-MATRIX-CH5-LOADS-BETWEEN-NODES` | Self-weight nodal distribution | Matrix Structural Analysis, Second Edition, Ch. 5 §5.2 `Loads Between Nodal Points`; exploration pointer: book p. 108 / PDF p. 129 | Candidate evidence for loads between joints/natural nodes, artificial nodes, selected-node lumping, and static-equivalence requirements. | candidate/supporting; page pointer pending manual PDF verification | none; analyst-selected lumping still needs reviewer approval. |
| `SRC-MATRIX-CH5-FIXED-END-EQUIVALENT-LOADS` | Self-weight nodal distribution | Matrix Structural Analysis, Second Edition, Ch. 5 §5.2, Fig. 5.6, Table 5.1, Eq. 5.21; exploration pointer: book pp. 110-112 / PDF pp. 131-133 | Candidate evidence for fixed-end force and reversed fixed-end/equivalent nodal load treatment in frame/beam stiffness analysis. | candidate/supporting; equations/signs pending manual PDF review | none; do not apply frame/beam evidence blindly to axial truss members. |
| `SRC-MATRIX-CH7-WORK-EQUIVALENT-LOADS` | Self-weight nodal distribution | Matrix Structural Analysis, Second Edition, Ch. 7 §7.5, Eq. 7.32; exploration pointer: book pp. 194-196 / PDF pp. 215-217 | Closest candidate evidence for effective/work-equivalent nodal loads and the explored note that a uniformly loaded axial member may match simple apportionment to two joints. | candidate/supporting; equation/sign/page review and reviewer interpretation required | none; truss self-weight applicability, directions, target nodes, and signs are unapproved. |
| `SRC-MATRIX-CH13-DEAD-LOAD-CONTEXT` | Self-weight / dead-load context | Matrix Structural Analysis, Second Edition, Ch. 13 dead-load mention; exploration pointer: book p. 410 / PDF p. 431 | Non-supporting context only; exploration found it assumes a load vector and does not define self-weight generation or distribution. | non-supporting context; page pointer pending manual PDF verification | none. |

Inventory rule: do not infer formulas, factors, nodal distribution, wind pressure, conductor effects, or controlling-case behavior from this table. Approval must happen in `formulas_register.md` and `validation_examples.md` before any runtime phase can proceed.

Matrix review rule: Matrix Structural Analysis entries above are source inventory only. Before any approval, a human reviewer must manually verify the cited PDF pages, equations, signs, and extracted wording; record the reviewer interpretation; decide whether beam/frame evidence applies to this engine's axial truss members; approve target nodes, directions/signs, applicability limits, numeric example, tolerance, reviewer identity, ISO date, and future tests-first runtime authorization.

## `CHK-SLENDERNESS-001` Source Evidence Inventory

This table is an inventory and review checklist. It does not approve a formula, limit, example, or implementation path. The restored `civil-rag` retrieval resolved the previous source-retrieval blocker for ASCE 10-15, but reviewer approval and runtime authorization remain missing. A listed source remains `candidate` until a reviewer records the exact clause/reference, interpretation, reviewer name, ISO approval date, and future tests-first runtime authorization in the downstream evidence ledger.

| Source ID | Source / clause reference | Intended role | Evidence status | Reviewer / date | Notes |
|---|---|---|---|---|---|
| `87c3e208-51b6-4838-930a-45c3331893f1` | ASCE 10-15 standard id `381eb1f0-0cf9-461f-b055-5b2346e38027`, §1.1 | Source relevance for electrical transmission structure member/connection design scope | candidate primary evidence | not approved | Source relevance only; does not authorize runtime interpretation. |
| `6fb62a89-2654-4c75-bbda-57e4d88ab610` | ASCE 10-15 standard id `381eb1f0-0cf9-461f-b055-5b2346e38027`, §2.1 | Applicability to latticed steel transmission structures with bolted prismatic members | candidate primary evidence | not approved | Supports tower-domain applicability; not software formula approval. |
| `7d04473e-3b0b-47db-9492-46a43aeab525` | ASCE 10-15 §3.4 | Candidate member-category slenderness limits for leg, other, and redundant members | candidate primary evidence | not approved | May inform future limits only after reviewer interpretation; no pass/fail output is approved. |
| `6eb7b9e4-100f-4a2c-89fb-c7d7975ca39c` | ASCE 10-15 §3.6 | Candidate definitions for `K`, `L`, `r`, and `KL/r` | candidate primary / capacity-boundary evidence | not approved | Compression-stress/capacity context; Euler, `Fa`, and column capacity remain excluded evidence boundaries only. |
| `5817f4ec-d28b-4f86-8a33-fed63dfb7ea5` | ASCE 10-15 §§3.5 / 3.7.4 excerpt | Candidate effective-slenderness equation inventory for other compression member cases and partial-restraint context | candidate primary evidence | not approved | Reviewer must choose bracing/end-condition semantics before runtime. |
| `dfa21e97-4af4-422d-9b7c-6c1850117dea` | ASCE 10-15 §3.7.4.1 | Candidate leg-member path where `KL/r = L/r` under stated bolting/category conditions | candidate primary evidence | not approved | Limited to stated conditions; no generic `L/r` implementation is approved. |
| `051f5339-d2a7-4f72-b2d7-1e3b99fd1be0` | ASCE 10-15 §3.7.4.2 | Candidate other-compression-member path where concentric end loading uses `KL/r = L/r` over the retrieved range | candidate primary evidence | not approved | Does not cover eccentric or alternative restraint cases by itself. |
| `d78b4be5-e1d6-4ed0-aea6-f8a279619248` | ASCE 10-15 §3.7.4.3 | Candidate redundant-member effective-slenderness rules | candidate primary evidence | not approved | Does not approve a generic engine rule or optimizer feasibility threshold. |
| `8f733827-febd-4672-98b7-9050d6f2b05a` | ASCE 10-15 §3.7.4.6 | Candidate exception/modification path where tests or analysis demonstrate different restraint | candidate primary evidence | not approved | Requires human engineering review before any exception can be represented. |
| `bb74de97-a8b0-4b5d-a81e-c19a7d5fb0a3` | ASCE 10-15 commentary C3.7.4 / 8.2 | Interpretation context for K factor, working-point length, and break-point handling | candidate commentary evidence | not approved | Commentary can guide reviewer interpretation but cannot approve software semantics alone. |
| `74402c38-ca86-4a58-9af5-447f16505003` | ASCE 10-15 appendix/example context | Caution that illustrative examples require competent advice | candidate example evidence | not approved | Prevents treating retrieved examples as accepted fixtures. |
| `dadf682b-a9fa-4ecb-8170-c991c205c165` | ASCE 10-15 Appendix B / §3.18 chunk | Numeric fragments such as `L/rz = 32/0.27 = 119` and `L/rz = 54/0.27 = 200` | candidate example / capacity-boundary evidence | not approved | Incomplete and capacity-related; not an executable fixture or acceptance example. |
| `SRC-AISC-360-22-CONTEXT` | AISC 360-22; exact context clause(s) not recorded for this engine | Supporting steel-design context only | candidate | not approved | Context inventory must not override tower-specific criteria. It cannot approve transmission-tower slenderness limits by itself. |
| `SRC-TIMOSHENKO-GERE-CONTEXT` | Timoshenko & Gere; exact theory reference not recorded for this engine | Background theory for stability/slenderness concepts | candidate | not approved | Context-only source. It can explain concepts but cannot approve project limits, member-category rules, or compliance output. |
| `SRC-CHK-SLENDERNESS-APPROVAL` | Reviewer-owned source packet with exact clauses, examples, and interpretation | Approval record required before any future runtime/test/report/optimizer work | blocked | not approved | Blocked because reviewer-approved clauses, semantic choice, limits, accepted examples, reviewer identity, and ISO date are not present. |

Approval rule: candidate source inventory is not approval. `CHK-SLENDERNESS-001` remains `TODO_DOMAIN_VALIDATION` until the approval record provides reviewer-owned clauses, interpretation, variables/units, limits if any, accepted numeric example(s), reviewer, and ISO date.
