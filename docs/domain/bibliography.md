# Bibliography

This register inventories candidate domain sources for MAMUT Tower Engine. It does **not** validate formulas by itself. Do not copy protected standards, books, or papers into the repository; store only bibliographic metadata, source mapping, short summaries, and validation status.

## Status Values

| Status | Meaning |
|---|---|
| `inventory-only` | Source is known but has not been reviewed. |
| `pending-review` | Source appears relevant and needs technical review. |
| `source-candidate` | Source is likely useful for future formula/check validation. |
| `future/non-mvp` | Source is useful later but not required for MVP core. |
| `blocked/unreadable` | Source could not be opened or parsed. |

## Standards and Codes

| ID | Source | Location / URL | MVP Use | Status |
|---|---|---|---|---|
| STD-ASCE-10-15 | ASCE/SEI 10-15 — Design of Latticed Steel Transmission Structures | `docs/domain/ASCE 10-15 Design of latticed steel transmission structures -- ASCE American Society of Civil Engineers -- American Society of Civil Engineers_; ASCE -- isbn13 9780784413760.pdf` | Primary candidate for latticed transmission tower member/check context. | pending-review |
| STD-ASCE-MOP-74 | ASCE Manual of Practice No. 74 — Guidelines for Electrical Transmission Line Structural Loading | `docs/domain/Guidelines for Electrical Transmission Line Structural -- Frank Agnew, P_E_ -- Manuals and Reports on Engineering Practice Ser, v_74, -- American -- isbn13 9780784415566 -- 644f5736623ab428335bd4e7c9ff6c30 -- A.pdf` | Primary candidate for transmission line loading context. | pending-review |
| STD-IEC-60826 | IEC 60826 — Design criteria of overhead transmission lines | `docs/domain/Design criteria of overhead transmission lines -- 4_0 -- e4216313b03dd32cb0f4ef0f9743868a.pdf` | Primary candidate for overhead line design/loading context. | pending-review |
| STD-ASCE-7 | ASCE 7 | `https://www.asce.org/asce-7` | Supporting/context for general loads; exact MVP use must be validated against transmission-specific references. | source-candidate |
| STD-AISC-360-22 | AISC 360-22 — Specification for Structural Steel Buildings | `https://www.aisc.org/globalassets/aisc/publications/standards/a360-22w.pdf` | Supporting/context for steel member behavior; not transmission-tower-specific. | source-candidate |
| STD-AISC-341-22 | AISC 341-22 — Seismic Provisions for Structural Steel Buildings | `https://www.aisc.org/globalassets/aisc/publications/standards/a341-22w.pdf` | Future seismic context; not MVP truss core. | future/non-mvp |
| STD-AISC-358-22 | AISC 358-22 — Prequalified Connections for Special and Intermediate Steel Moment Frames | `https://www.aisc.org/globalassets/aisc/publications/standards/a358-22w.pdf` | Future connection/seismic context; out of MVP scope. | future/non-mvp |
| STD-ACI-318 | ACI 318 | `https://www.concrete.org/` | Future concrete/foundation context; out of MVP scope. | future/non-mvp |

## Books and Technical References

| ID | Source | Location / URL | MVP Use | Status |
|---|---|---|---|---|
| REF-TIMOSHENKO-GERE-STABILITY | Timoshenko & Gere — Theory of Elastic Stability | `docs/domain/Theory of Elastic Stability (Dover Civil and Mechanical Engineering) (Gere, James M.  Timoshenko, Stephen P.).pdf` | Supporting theory for compression, buckling, and slenderness concepts. | source-candidate |
| REF-CONCEPTUAL-TOWER-DESIGN | Diseño conceptual de torres | `docs/domain/Diseno-conceptual-de-torres.pdf` | Candidate reference for tower geometry/domain vocabulary; needs review. | pending-review |
| REF-MATRIX-STRUCTURAL | Matrix structural analysis textbook/reference | Not provided yet | 3D truss stiffness validation and numerical examples. | inventory-only |
| REF-RUST-NUMERICS | Rust linear algebra crate docs selected by ADR | Not selected yet | Solver implementation behavior after dependency ADR. | inventory-only |

## Optimization Papers

| ID | Source | Location / URL | MVP Use | Status |
|---|---|---|---|---|
| PAPER-OPT-001 | Optimization paper, title pending review | `docs/domain/algoritmo-paper.pdf` | Candidate for optimization algorithm mapping; do not infer results until reviewed. | pending-review |
| PAPER-OPT-002 | Optimization paper, title pending review | `docs/domain/algoritmo-paper2.pdf` | Candidate for optimization algorithm mapping; do not infer results until reviewed. | pending-review |

## Rule

Use citations and short summaries only. If a rule cannot be source-backed, mark it `TODO_DOMAIN_VALIDATION` and block implementation for that rule. Source presence is not validation: each implemented formula or check must later map to a formula-register entry, source location, units, limitations, and at least one validation/regression test.
