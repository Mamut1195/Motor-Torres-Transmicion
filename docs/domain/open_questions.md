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
