# Dependency Notes

This workspace keeps dependencies minimal for WU2.

| Crate | Used by | Justification |
|---|---|---|
| `serde` | `tower-core` input structs | Provides stable, explicit deserialization boundaries for TOML-ready model scaffolding without owning domain semantics. |
| `toml` | `TowerModel::from_toml_str` | Supports the accepted primary input format while keeping parsing separate from validation. |
| `thiserror` | `TowerError` | Gives deterministic, typed validation errors with clear messages and minimal boilerplate. |

No linear algebra, solver, optimizer, API, UI, database, cloud, Python runtime, or runtime AI dependencies are introduced in WU2.
