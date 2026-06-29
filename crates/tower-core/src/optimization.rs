use std::collections::BTreeSet;

use crate::design_checks::{CheckStatus, FormulaStatus};
use crate::errors::{Result, TowerError};
use crate::sections::SectionId;

pub fn optimization_not_available() -> Result<()> {
    Err(TowerError::BlockedDomainFeature {
        feature: "section optimization",
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateCheck {
    pub trace_id: &'static str,
    pub status: CheckStatus,
    pub validation_status: FormulaStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateSection {
    pub section_id: SectionId,
    pub weight_kg_per_m: f64,
    pub checks: Vec<CandidateCheck>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeasibleAssignment {
    pub section_id: SectionId,
    pub weight_kg_per_m: f64,
    pub controlling_checks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfeasibleOptimization {
    pub failed_candidates: Vec<String>,
    pub unresolved_constraints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockedOptimization {
    pub blocked_candidates: Vec<String>,
    pub unresolved_constraints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationResult {
    Feasible(FeasibleAssignment),
    Infeasible(InfeasibleOptimization),
    Blocked(BlockedOptimization),
}

pub struct GreedyOptimizer;

impl GreedyOptimizer {
    pub fn select(candidates: &[CandidateSection]) -> OptimizationResult {
        if let Some(selected) = candidates
            .iter()
            .filter(|candidate| {
                !candidate.checks.is_empty()
                    && candidate.checks.iter().all(is_passing_validated_check)
            })
            .min_by(|left, right| {
                left.weight_kg_per_m
                    .total_cmp(&right.weight_kg_per_m)
                    .then_with(|| left.section_id.0.cmp(&right.section_id.0))
            })
        {
            return OptimizationResult::Feasible(FeasibleAssignment {
                section_id: selected.section_id.clone(),
                weight_kg_per_m: selected.weight_kg_per_m,
                controlling_checks: selected
                    .checks
                    .iter()
                    .map(|check| check.trace_id.to_string())
                    .collect(),
            });
        }

        let blocked_candidates = candidates
            .iter()
            .filter(|candidate| candidate.checks.iter().any(is_unresolved_check))
            .map(|candidate| candidate.section_id.0.clone())
            .collect::<Vec<_>>();
        if !blocked_candidates.is_empty() {
            return OptimizationResult::Blocked(BlockedOptimization {
                blocked_candidates,
                unresolved_constraints: unresolved_constraints(candidates),
            });
        }

        OptimizationResult::Infeasible(InfeasibleOptimization {
            failed_candidates: candidates
                .iter()
                .map(|candidate| candidate.section_id.0.clone())
                .collect(),
            unresolved_constraints: Vec::new(),
        })
    }
}

fn is_passing_validated_check(check: &CandidateCheck) -> bool {
    has_trace_id(check)
        && check.validation_status == FormulaStatus::Validated
        && check.status == CheckStatus::Pass
}

fn is_unresolved_check(check: &CandidateCheck) -> bool {
    !has_trace_id(check)
        || check.validation_status != FormulaStatus::Validated
        || matches!(
            check.status,
            CheckStatus::Blocked | CheckStatus::TodoDomainValidation
        )
}

fn has_trace_id(check: &CandidateCheck) -> bool {
    !check.trace_id.trim().is_empty()
}

fn unresolved_constraints(candidates: &[CandidateSection]) -> Vec<String> {
    candidates
        .iter()
        .flat_map(|candidate| candidate.checks.iter())
        .filter(|check| is_unresolved_check(check))
        .map(|check| {
            if has_trace_id(check) {
                check.trace_id.to_string()
            } else {
                "<missing trace>".to_string()
            }
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
