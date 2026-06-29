use tower_core::design_checks::{CheckStatus, FormulaStatus};
use tower_core::optimization::{
    CandidateCheck, CandidateSection, GreedyOptimizer, OptimizationResult,
};
use tower_core::sections::SectionId;

const TRACE_WEIGHT: &str = "QTY-WEIGHT-001";
const TRACE_TENSION: &str = "CHK-TENSION-001";

fn candidate(id: &str, weight_kg_per_m: f64, checks: Vec<CandidateCheck>) -> CandidateSection {
    CandidateSection {
        section_id: SectionId(id.to_string()),
        weight_kg_per_m,
        checks,
    }
}

fn check(
    trace_id: &'static str,
    status: CheckStatus,
    validation_status: FormulaStatus,
) -> CandidateCheck {
    CandidateCheck {
        trace_id,
        status,
        validation_status,
    }
}

fn passing_weight_check() -> CandidateCheck {
    check(TRACE_WEIGHT, CheckStatus::Pass, FormulaStatus::Validated)
}

#[test]
fn chooses_lightest_safe_section_with_deterministic_id_tie_break() {
    let result = GreedyOptimizer::select(&[
        candidate("S-B", 12.0, vec![passing_weight_check()]),
        candidate("S-C", 8.0, vec![passing_weight_check()]),
        candidate("S-A", 8.0, vec![passing_weight_check()]),
    ]);

    match result {
        OptimizationResult::Feasible(assignment) => {
            assert_eq!(assignment.section_id, SectionId("S-A".to_string()));
            assert_eq!(assignment.weight_kg_per_m, 8.0);
            assert_eq!(
                assignment.controlling_checks,
                vec![TRACE_WEIGHT.to_string()]
            );
        }
        other => panic!("expected feasible assignment, got {other:?}"),
    }
}

#[test]
fn selects_heavier_safe_section_over_lighter_failed_section() {
    let result = GreedyOptimizer::select(&[
        candidate(
            "LIGHT-FAILED",
            4.0,
            vec![check(
                TRACE_TENSION,
                CheckStatus::Fail,
                FormulaStatus::Validated,
            )],
        ),
        candidate("HEAVY-SAFE", 10.0, vec![passing_weight_check()]),
    ]);

    match result {
        OptimizationResult::Feasible(assignment) => {
            assert_eq!(assignment.section_id, SectionId("HEAVY-SAFE".to_string()));
            assert_eq!(assignment.weight_kg_per_m, 10.0);
            assert_eq!(
                assignment.controlling_checks,
                vec![TRACE_WEIGHT.to_string()]
            );
        }
        other => panic!("expected safety-first feasible assignment, got {other:?}"),
    }
}

#[test]
fn returns_infeasible_when_all_validated_candidates_fail() {
    let result = GreedyOptimizer::select(&[
        candidate(
            "S-1",
            5.0,
            vec![check(
                TRACE_TENSION,
                CheckStatus::Fail,
                FormulaStatus::Validated,
            )],
        ),
        candidate(
            "S-2",
            7.0,
            vec![check(
                TRACE_TENSION,
                CheckStatus::Fail,
                FormulaStatus::Validated,
            )],
        ),
    ]);

    match result {
        OptimizationResult::Infeasible(infeasible) => {
            assert_eq!(
                infeasible.failed_candidates,
                vec!["S-1".to_string(), "S-2".to_string()]
            );
            assert_eq!(infeasible.unresolved_constraints, Vec::<String>::new());
        }
        other => panic!("expected infeasible optimization, got {other:?}"),
    }
}

#[test]
fn returns_infeasible_when_candidates_have_no_required_checks() {
    let result = GreedyOptimizer::select(&[candidate("UNCHECKED", 1.0, Vec::new())]);

    match result {
        OptimizationResult::Infeasible(infeasible) => {
            assert_eq!(infeasible.failed_candidates, vec!["UNCHECKED".to_string()]);
            assert_eq!(infeasible.unresolved_constraints, Vec::<String>::new());
        }
        other => panic!("expected unchecked candidate to be infeasible, got {other:?}"),
    }
}

#[test]
fn does_not_select_passing_checks_without_validated_formula_status() {
    let result = GreedyOptimizer::select(&[
        candidate(
            "PENDING-PASS",
            1.0,
            vec![check(
                TRACE_WEIGHT,
                CheckStatus::Pass,
                FormulaStatus::Pending,
            )],
        ),
        candidate(
            "PROVISIONAL-PASS",
            2.0,
            vec![check(
                TRACE_TENSION,
                CheckStatus::Pass,
                FormulaStatus::Provisional,
            )],
        ),
        candidate(
            "TODO-PASS",
            3.0,
            vec![check(
                "CHK-SLENDERNESS-001",
                CheckStatus::Pass,
                FormulaStatus::TodoDomainValidation,
            )],
        ),
    ]);

    match result {
        OptimizationResult::Blocked(blocked) => {
            assert_eq!(
                blocked.blocked_candidates,
                vec![
                    "PENDING-PASS".to_string(),
                    "PROVISIONAL-PASS".to_string(),
                    "TODO-PASS".to_string(),
                ]
            );
            assert_eq!(
                blocked.unresolved_constraints,
                vec![
                    "CHK-SLENDERNESS-001".to_string(),
                    TRACE_TENSION.to_string(),
                    TRACE_WEIGHT.to_string(),
                ]
            );
        }
        other => {
            panic!("expected non-validated passing checks to block optimization, got {other:?}")
        }
    }
}

#[test]
fn returns_blocked_when_constraints_are_unresolved() {
    let result = GreedyOptimizer::select(&[candidate(
        "S-BLOCKED",
        6.0,
        vec![check(
            TRACE_TENSION,
            CheckStatus::TodoDomainValidation,
            FormulaStatus::TodoDomainValidation,
        )],
    )]);

    match result {
        OptimizationResult::Blocked(blocked) => {
            assert_eq!(blocked.blocked_candidates, vec!["S-BLOCKED".to_string()]);
            assert_eq!(
                blocked.unresolved_constraints,
                vec![TRACE_TENSION.to_string()]
            );
        }
        other => panic!("expected blocked optimization, got {other:?}"),
    }
}

#[test]
fn returns_blocked_when_candidate_check_status_is_blocked() {
    let result = GreedyOptimizer::select(&[candidate(
        "EXPLICITLY-BLOCKED",
        5.0,
        vec![check(
            TRACE_TENSION,
            CheckStatus::Blocked,
            FormulaStatus::Validated,
        )],
    )]);

    match result {
        OptimizationResult::Blocked(blocked) => {
            assert_eq!(
                blocked.blocked_candidates,
                vec!["EXPLICITLY-BLOCKED".to_string()]
            );
            assert_eq!(
                blocked.unresolved_constraints,
                vec![TRACE_TENSION.to_string()]
            );
        }
        other => panic!("expected blocked check status to block optimization, got {other:?}"),
    }
}

#[test]
fn rejects_passing_validated_candidate_with_missing_trace_id() {
    let result = GreedyOptimizer::select(&[candidate(
        "MISSING-TRACE",
        5.0,
        vec![check("", CheckStatus::Pass, FormulaStatus::Validated)],
    )]);

    match result {
        OptimizationResult::Blocked(blocked) => {
            assert_eq!(
                blocked.blocked_candidates,
                vec!["MISSING-TRACE".to_string()]
            );
            assert_eq!(
                blocked.unresolved_constraints,
                vec!["<missing trace>".to_string()]
            );
        }
        other => panic!("expected missing trace to block optimization, got {other:?}"),
    }
}
