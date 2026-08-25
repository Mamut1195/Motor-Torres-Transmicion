# Open Questions

These questions protect the project from pretending that source inventory equals validated engineering behavior. Each blocking question must be resolved, explicitly deferred, or surfaced as `TODO_DOMAIN_VALIDATION` before related implementation is accepted.

## Blocking Questions Before Solver / Checks

| ID | Question | Blocks Implementation? | Owner / Resolution Path | Current Status |
|---|---|---|---|---|
| Q-DOM-001 | Which exact standard edition governs preliminary transmission-tower member checks for the MVP? | Yes, for checks | Domain review of ASCE/SEI 10-15 and project criteria. | open |
| Q-DOM-002 | Which formulas/limits are acceptable for preliminary tension utilization? | Yes, for tension checks | Add formula-register entry with source, variables, units, limitations, and validation test. | open |
| Q-DOM-003 | Which formulas/limits are acceptable for preliminary compression and slenderness utilization? | Yes, for compression/slenderness checks | Review ASCE/SEI 10-15 with AISC 360/Timoshenko as support only. | open |
| Q-DOM-003A | For `CHK-SLENDERNESS-001`, is the accepted output geometric `L/r`, effective `K·L/r`, or blocked-only documentation, and what are the required `K`, axis radius, bracing/end-condition, member category, and compression-applicability rules? | Yes, for slenderness checks, reporting, and optimizer feasibility evidence | Domain reviewer must approve source clauses, semantic choice, required inputs, numeric examples, trace IDs, and reviewer/date before any implementation. | open |
| Q-DOM-004 | What displacement limits apply to the simplified 69 kV example? | Yes, for displacement pass/fail | Approve project criterion or source-backed limit; otherwise report `TODO_DOMAIN_VALIDATION`. | open |
| Q-DOM-005 | Which load cases and load combinations are allowed in the MVP example? | Yes, for normative load combinations | Review ASCE MOP 74 / IEC 60826; allow provisional nodal loads as explicit assumptions. | open |
| Q-DOM-006 | Which wind-load simplification, if any, is acceptable for MVP examples? | Yes, for wind-derived loads | Keep as explicit provisional input until source-backed. | open |
| Q-DOM-007 | Which material properties and angle-section catalog source will be used? | Yes, for weight/check reliability | Select catalog/source with units and verification status. | open |
| Q-DOM-008 | Which validation examples are sufficient for first solver acceptance? | Yes, for solver acceptance | Provide simple bar, small 3D truss, and tower-like benchmark with expected results. | open |
| Q-DOM-009 | What singularity threshold and conditioning policy will be accepted? | Yes, for solver | Numerical policy ADR/update before WU3. | open |
| Q-DOM-010 | Which linear algebra crate will be used, and what are its failure modes? | Yes, before solver | Dependency decision in WU2/WU3 with license, determinism, and replacement cost. | open |

## Minimum Load Model v1 Blocking Questions

These questions remain open and must be reported as `TODO_DOMAIN_VALIDATION` until reviewer-approved evidence exists. This boundary is not for final engineering design.

| ID | Question | Current reporting status |
|---|---|---|
| Q-LOAD-001 | What approved lumping/modeling assumption converts `QTY-WEIGHT-001` member self-weight quantity into nodal loads? | Resolved only for the approved narrow runtime rule: straight two-node axial member with uniform self-weight, target nodes `fixed/free`, z-up negative `fz`, endpoint `fz = -W/2`. All other self-weight generation remains `TODO_DOMAIN_VALIDATION`. |
| Q-LOAD-002 | Which exact wind-loading clauses, inputs, exposure assumptions, and examples are approved? | `TODO_DOMAIN_VALIDATION`; no wind loading behavior. |
| Q-LOAD-003 | Which conductor loading assumptions and source-backed examples are approved? | `TODO_DOMAIN_VALIDATION`; no conductor loading behavior. |
| Q-LOAD-004 | Which load combinations and load factors are approved for MVP reports? | `TODO_DOMAIN_VALIDATION`; no combinations or factors. |
| Q-LOAD-005 | Which displacement or design-level loading criteria are approved? | `TODO_DOMAIN_VALIDATION`; no final-design loading claim. |

## Load Model v2 Evidence Questions

These questions are the required answer set for future load-model v2 work. Until an answer includes the full approval evidence packet, the related formula or rule remains `TODO_DOMAIN_VALIDATION` and blocked from runtime consumption.

| ID | Topic | Answer required before implementation | Blocks implementation? | Current status |
|---|---|---|---|---|
| Q-LOAD-V2-001 | Self-weight nodal distribution | Which source/project rule, exact edition/clause or rule ID, reviewer interpretation, lumping/modeling assumption, target nodes, directions/signs, variables/units, applicability/limits, numeric example, tolerance/rationale, reviewer, ISO date, and future SDD runtime authorization convert `QTY-WEIGHT-001` quantity-only evidence into approved nodal distribution? | Yes, for generated self-weight nodal loads | `TODO_DOMAIN_VALIDATION`; `QTY-WEIGHT-001` validates quantity only, distribution rule and approval packet are not approved. |
| Q-LOAD-V2-002 | Wind loading | Which exact source clauses, edition, exposure assumptions, tower geometry inputs, pressure/force variables, units, limitations, and numeric example are approved? | Yes, for wind-derived loads | `TODO_DOMAIN_VALIDATION`; source candidates only. |
| Q-LOAD-V2-003 | Conductor loads | Which conductor loading assumptions, span/input variables, units, load transfer semantics, applicability limits, and numeric example are approved? | Yes, for conductor-derived loads | `TODO_DOMAIN_VALIDATION`; source candidates only. |
| Q-LOAD-V2-004 | Load combinations / factors | Which load cases participate, which factors apply, which source/project rule governs, and what example proves the combined load vector? | Yes, for combinations and factors | `TODO_DOMAIN_VALIDATION`; no combinations or factors approved. |
| Q-LOAD-V2-005 | Controlling-case prerequisites | After combinations/factors are approved, what deterministic rule selects or reports the controlling case, and what evidence prevents unapproved cases from becoming feasibility evidence? | Yes, for controlling-case execution | `TODO_DOMAIN_VALIDATION`; controlling-case semantics blocked. |

Each answer must identify source, edition/clause, interpretation, variables/units, applicability/limits, assumptions, numeric example, tolerance/rationale if testable, reviewer, ISO date, and future SDD runtime authorization. Candidate-source inventory alone is not an answer.

`Q-LOAD-V2-001` is closed only for the approved narrow runtime rule: straight two-node axial member with uniform self-weight, target nodes `fixed/free`, signs/directions `fz = -W/2`, units `kN`, reviewer Jonnathan, ISO date 2026-07-10, and tests-first runtime authorization. Candidate inventory/arithmetic is not approved engineering evidence for any broader scope and does not authorize runtime execution beyond this packet.

Approved narrow runtime rule exclusions: nonuniform members, beam fixed-end actions, eccentric loads, wind/conductor loads, load combinations, load factors, controlling-case execution, and final engineering design claims remain open and blocked.

For `LOAD-SW-DIST-001`, the mandatory ledger fields remain source rule, clause/project-rule ID, reviewer interpretation, assumptions, target nodes, signs/directions, units, applicability limits, numeric trace, tolerance rationale, reviewer identity, ISO review date, and future tests-first runtime authorization status. Candidate inventory/arithmetic is not approved engineering evidence and does not authorize runtime execution.

### Matrix Structural Analysis review blockers for `Q-LOAD-V2-001`

Matrix Structural Analysis is now candidate/supporting source evidence, but it does not close `Q-LOAD-V2-001`. The reviewer must still resolve the following before generated self-weight nodal loads can be considered for a future tests-first SDD:

| Blocker | Required resolution | Current status |
|---|---|---|
| PDF page and equation verification | Manually verify Ch. 5 §5.2/Fig. 5.6/Table 5.1/Eq. 5.21 and Ch. 7 §7.5/Eq. 7.32 against the PDF pages recorded in `standards_map.md` and `formulas_register.md`. | open |
| Applicability to axial truss members | Decide whether beam/frame equivalent-load evidence and the explored uniformly loaded axial member note apply to this engine's axial truss abstraction. | open |
| Target nodes and allocation | Approve which member end nodes receive generated self-weight loads and how the quantity is allocated. | open |
| Directions and signs | Approve gravity direction, coordinate convention, sign convention, and output force components. | open |
| Numeric example and tolerance | Provide reviewed inputs, substitutions, intermediate values, expected nodal force results, trace ID, and comparison tolerance/rationale. | open |
| Reviewer authorization | Record reviewer identity, ISO date, interpretation, and explicit future runtime authorization. | open |

### `CHK-SLENDERNESS-001` blocker checklist

The previous `civil-rag` source-retrieval blocker is retrieval resolved by the ASCE 10-15 source evidence ledger. The following blockers remain open for `Q-DOM-003A`. Candidate source inventory does not resolve reviewer approval or runtime authorization.

The required answer packet must include semantic choice; exact source title/edition/clause/page or source ID; inputs with units; applicability; limits if any; numeric example if available; tolerance rationale; reviewer identity; ISO approval date; and future tests-first runtime authorization status. An equivalent manual source review can satisfy these fields only when it supplies the same clause traceability and reviewer-owned interpretation as source retrieval.

| Blocker | Required resolution | Current status |
|---|---|---|
| Governing source clauses | Exact source, edition, clause/reference, interpretation note, and trace ID | open |
| Semantic choice | Reviewer selects `L/r`, `K·L/r`, or explicit `blocked-only` | open; current gate remains `blocked-only` |
| Limits and applicability | Source-backed limits, member categories, exclusions, and compression applicability | open |
| Required inputs | Member length basis, `K` if applicable, axis radii policy, bracing/end-condition assumptions, units | open |
| Accepted example | Inputs, expression, intermediate values, expected result, tolerance/rationale, trace ID, and clause | open |
| Reviewer / date | Reviewer identity and ISO approval date covering the full evidence packet | open |
| Runtime authorization | Future tests-first runtime authorization status after a complete reviewer-approved evidence packet exists | open |

Until all rows are resolved, `CHK-SLENDERNESS-001` stays `TODO_DOMAIN_VALIDATION` and blocked for source, schema, CLI, tests, reports, optimizer feasibility, and runtime behavior.

### `CHK-SLENDERNESS-001` geometric `L/r` runtime authorization packet questions

Current decision: `deferred`. The packet decision must be recorded as decision: `approved | rejected | deferred`; until approval is complete, no runtime `L/r` computation is authorized.

Required packet fields to resolve before a future tests-first runtime slice: decision; semantic choice; applicability; scalar radius policy; units; numeric example; tolerance; reviewer identity; ISO date; and future tests-first runtime authorization status.

Open resolution items:

| Packet item | Required reviewer answer | Current status |
|---|---|---|
| semantic choice | Whether a future rule may be `geometric_scalar_L_over_r_quantity_only` | deferred |
| applicability | Whether two-node member endpoint length only is acceptable for a quantity-only, non-compliance value | deferred |
| scalar radius policy | Whether existing scalar `section.radius_of_gyration` may be used as `r`, including governing-axis implications | deferred |
| units | Confirm `L in m, scalar r in m, output dimensionless` | deferred |
| formula if approved | Confirm `L/r = two-node member length / existing scalar section.radius_of_gyration` | deferred |
| numeric example | Provide reviewer-owned inputs, substitutions, and expected dimensionless value | missing |
| tolerance | Provide software-comparison tolerance and tolerance rationale | missing |
| reviewer identity | Record human reviewer identity | missing |
| ISO date | Record ISO date for the complete packet | missing |
| future tests-first runtime authorization status | Approve, reject, or defer a later tests-first runtime SDD | deferred |

Deferred reason: missing reviewer identity, ISO date, numeric example, and tolerance. This packet does not authorize runtime `L/r`, does not authorize schema expansion, does not authorize pass/fail, does not authorize ASCE limits, does not authorize effective `K·L/r`, does not authorize Euler or column capacity, does not authorize optimizer feasibility, does not authorize runtime `civil-rag`, and does not authorize final-design claims.

## Optimization Questions

| ID | Question | Blocks Implementation? | Owner / Resolution Path | Current Status |
|---|---|---|---|---|
| Q-OPT-001 | What deterministic candidate ordering should the greedy optimizer use? | Yes, for optimizer | Define in optimization design before WU5. | open |
| Q-OPT-002 | How should infeasible assignments report unresolved constraints? | Yes, for optimizer/reporting | Define required fields: group, candidate set, controlling failed checks, and assumptions. | open |
| Q-OPT-003 | What benchmark proves the optimizer reduces weight without hiding unsafe checks? | Yes, for optimizer acceptance | Add validation example with before/after weight and utilization traces. | open |
| Q-OPT-004 | Which metaheuristic papers are relevant to future tickets? | No, future only | Review `algoritmo-paper.pdf` and `algoritmo-paper2.pdf`; do not affect MVP greedy baseline. | open |

## Documentation / Governance Questions

| ID | Question | Blocks Implementation? | Owner / Resolution Path | Current Status |
|---|---|---|---|---|
| Q-GOV-001 | Who accepts the domain gate, and on what date? | Yes, before WU2 proceeds beyond scaffold | Record reviewer/date in `acceptance_gate.md`. | open |
| Q-GOV-002 | What remains allowed as `TODO_DOMAIN_VALIDATION` in the first implementation slice? | Yes, for checks/reports | List allowed gaps explicitly before implementation. | open |
| Q-GOV-003 | How will each formula-register entry link to tests and report traces? | Yes, for checks | Define source-to-test trace convention before checks. | open |

## Non-Blocking Future Questions

| ID | Question | Future Area |
|---|---|---|
| Q-FUT-001 | How will foundations use ACI 318 or other concrete/foundation references? | Future foundation module |
| Q-FUT-002 | How will seismic provisions such as AISC 341 affect future scope? | Future seismic scope |
| Q-FUT-003 | How will detailed connections or AISC 358-like references be handled? | Future connection module |
| Q-FUT-004 | How will PLS-TOWER or external tools be used for validation/interoperability? | Future validation workflow |
