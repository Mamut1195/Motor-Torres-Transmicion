# Acceptance Criteria

## WU1 / Phase 0

- [x] Planning docs exist under `docs/`.
- [x] Domain gate docs exist under `docs/domain/`.
- [x] ADRs 0001 through 0008 exist under `docs/adr/`.
- [x] Tasks and tickets explicitly block Rust solver work until the domain gate is accepted.
- [x] Scope excludes API, UI, database, cloud, runtime AI, and final-design claims.
- [x] Relevant docs include the exact disclaimer `not for final engineering design`.

## Gate Before Rust Implementation

- [ ] Minimum formulas in `docs/domain/formulas_register.md` are reviewed and statused.
- [ ] At least one validation example exists for each implemented formula/check.
- [ ] Numerical tolerance and singularity policies are approved.
- [ ] Every implemented check has source-to-test traceability.
- [ ] Failed-run reporting examples are planned for validation, instability, unsupported checks, and infeasibility.
