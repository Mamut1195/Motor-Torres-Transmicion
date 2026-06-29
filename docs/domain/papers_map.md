# Papers and References Map

This map inventories optimization and numerical-analysis references. It does not validate algorithms or results by itself. Do not copy paper text into the repository; record only metadata, extracted variables/objectives/constraints after review, and validation status.

## Status Values

| Status | Meaning |
|---|---|
| `inventory-only` | Known need or file exists, but not reviewed. |
| `pending-review` | Candidate source should be reviewed before use. |
| `source-candidate` | Likely useful for algorithm design or validation. |
| `validated-example-needed` | Concept is accepted, but needs a reproducible test case. |

## Optimization Source Inventory

| ID | Source | Location | Algorithm Family | Possible Engine Use | Review Notes | Status |
|---|---|---|---|---|---|---|
| PAPER-OPT-001 | Optimization paper, title pending review | `docs/domain/algoritmo-paper.pdf` | Unknown until reviewed | Candidate for section sizing or tower optimization benchmark | Do not infer variables/objective/results from filename. | pending-review |
| PAPER-OPT-002 | Optimization paper, title pending review | `docs/domain/algoritmo-paper2.pdf` | Unknown until reviewed | Candidate for section sizing, metaheuristic, or tower-family optimization mapping | Do not infer variables/objective/results from filename. | pending-review |

## Optimization Topics to Classify

| Topic | What to Extract During Review | MVP Use | Status |
|---|---|---|---|
| Greedy discrete section sizing | Variables, candidate ordering, objective, constraints, infeasible handling | Required MVP optimizer baseline | validated-example-needed |
| Simulated annealing | Design variables, move strategy, temperature schedule, constraints, benchmark tower | Future optimizer ticket | inventory-only |
| Genetic algorithms | Encoding, population, crossover/mutation, constraints, benchmark tower | Future optimizer ticket | inventory-only |
| NSGA-II / multiobjective | Objectives, Pareto metrics, constraints, decision variables | Future multiobjective optimization | inventory-only |
| Shape optimization | Geometry variables, member grouping, constructability constraints | Future tower geometry optimization | inventory-only |
| Topology optimization | Member existence variables, ground structure, constraints | Future/non-MVP; high scope risk | inventory-only |
| Tower-family optimization | Shared variables across tower family, cost/weight tradeoffs | Future product direction | inventory-only |
| PLS-TOWER integration with external algorithms | Data exchange, validation outputs, comparison workflow | Future validation/interoperability only | inventory-only |
| Cost-based optimization | Steel, fabrication, galvanizing, transport, erection, reactions/foundation penalties | Future objective beyond MVP weight minimization | inventory-only |

## Numerical Analysis Reference Needs

| Topic | Reference Need | MVP Use | Status |
|---|---|---|---|
| Linear 3D truss stiffness | Analytical derivation and benchmark examples | Required for element stiffness tests | validated-example-needed |
| Global stiffness assembly | DOF indexing, support application, reaction recovery | Required for solver tests | validated-example-needed |
| Boundary conditions and singularity | Numerical stability, rank/singularity detection, threshold policy | Required before accepting solver output | validated-example-needed |
| Floating-point tolerances | Deterministic comparison policy for tests/reports | Required for regression tests | validated-example-needed |
| Validation examples | Simple bar, simple 3D truss, tower-like benchmark | Required for solver acceptance | validated-example-needed |

## Acceptance

Every implemented numerical method must have at least one source-backed derivation or validation example before code is merged. Every implemented optimization algorithm must record objective, variables, constraints, infeasible handling, and deterministic test evidence. Future metaheuristics must not enter the MVP unless explicitly approved by a new ADR.
