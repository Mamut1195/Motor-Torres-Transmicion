//! Core domain boundaries for MAMUT Tower Engine.
//!
//! This crate is currently a safe scaffold. It includes explicit unit types,
//! domain identifiers, basic geometry, deterministic input validation, and the
//! WU3 linear 3D truss solver, WU4 preliminary check trace boundaries, and the
//! WU5 deterministic greedy optimizer core. It intentionally does not implement
//! normative member strength checks, load combinations, reporting expansion, or
//! final engineering design claims.

pub mod analysis;
pub mod design_checks;
pub mod errors;
pub mod geometry;
pub mod loads;
pub mod materials;
pub mod model;
pub mod optimization;
pub mod reporting;
pub mod sections;
pub mod self_weight;
pub mod units;

pub use errors::{Result, TowerError};
