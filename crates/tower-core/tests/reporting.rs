use tower_core::design_checks::{CheckResult, CheckStatus, CheckTrace, FormulaStatus};
use tower_core::errors::TowerError;
use tower_core::optimization::{BlockedOptimization, OptimizationResult};
use tower_core::reporting::{PreliminaryReport, ENGINEERING_DISCLAIMER};

const TRACE_TENSION: &str = "CHK-TENSION-001";
const TRACE_COMPRESSION: &str = "CHK-COMPRESSION-001";

fn blocked_check() -> CheckResult {
    CheckResult {
        rule_id: TRACE_TENSION.to_string(),
        status: CheckStatus::TodoDomainValidation,
        value: None,
        trace: CheckTrace {
            formula_id: TRACE_TENSION.to_string(),
            validation_status: FormulaStatus::TodoDomainValidation,
            inputs: Vec::new(),
        },
        message: "CHK-TENSION-001 is blocked until formula-register validation is complete"
            .to_string(),
    }
}

fn compression_stress_check() -> CheckResult {
    CheckResult {
        rule_id: TRACE_COMPRESSION.to_string(),
        status: CheckStatus::Pass,
        value: Some(0.04),
        trace: CheckTrace {
            formula_id: TRACE_COMPRESSION.to_string(),
            validation_status: FormulaStatus::Validated,
            inputs: vec![
                "bar-x.axial_force_kN".to_string(),
                "bar.nominal_area_m2".to_string(),
                "steel.yield_stress_MPa".to_string(),
                "stress_mpa = abs(kN) / m2 / 1000".to_string(),
            ],
        },
        message: "compression axial stress utilization for bar-x is 0.040000".to_string(),
    }
}

#[test]
fn report_includes_disclaimer_and_validation_gaps() {
    let report =
        PreliminaryReport::from_checks("example_02_simple_3d_truss_star", &[blocked_check()], None)
            .render();

    assert!(report.contains(ENGINEERING_DISCLAIMER));
    assert!(report.contains("Validation gaps"));
    assert!(report.contains("TODO_DOMAIN_VALIDATION"));
    assert!(report.contains(TRACE_TENSION));
    assert!(report.contains("does not establish code compliance"));
}

#[test]
fn failed_reports_explain_invalid_and_singular_outcomes() {
    let invalid = PreliminaryReport::from_error(
        "invalid-toml",
        &TowerError::Parse {
            message: "expected a table".to_string(),
        },
    )
    .render();
    let singular = PreliminaryReport::from_error(
        "singular-model",
        &TowerError::UnstableModel {
            reason: "singular or near-singular stiffness matrix at active DOF free.uy".to_string(),
        },
    )
    .render();

    assert!(invalid.contains(ENGINEERING_DISCLAIMER));
    assert!(invalid.contains("Failed run"));
    assert!(invalid.contains("input parsing failed"));
    assert!(singular.contains("Failed run"));
    assert!(singular.contains("singular or near-singular stiffness matrix"));
    assert!(singular.contains("not final-design approval"));
}

#[test]
fn blocked_optimization_report_lists_unresolved_constraints() {
    let optimization = OptimizationResult::Blocked(BlockedOptimization {
        blocked_candidates: vec!["L45x45x5".to_string()],
        unresolved_constraints: vec![TRACE_TENSION.to_string()],
    });

    let report =
        PreliminaryReport::from_checks("optimization_demo", &[], Some(&optimization)).render();

    assert!(report.contains("Optimization status: blocked"));
    assert!(report.contains("L45x45x5"));
    assert!(report.contains(TRACE_TENSION));
    assert!(report.contains("unresolved checks remain visible"));
}

#[test]
fn report_lists_axial_stress_utilization_trace_inputs_and_preserves_wording_boundary() {
    let report = PreliminaryReport::from_checks(
        "example_07_compression_axial_stress_utilization",
        &[compression_stress_check(), blocked_check()],
        None,
    )
    .render();

    assert!(report.contains(ENGINEERING_DISCLAIMER));
    assert!(report.contains(TRACE_COMPRESSION));
    assert!(report.contains("value: 0.04"));
    assert!(report.contains("bar-x.axial_force_kN"));
    assert!(report.contains("bar.nominal_area_m2"));
    assert!(report.contains("steel.yield_stress_MPa"));
    assert!(report.contains("stress_mpa = abs(kN) / m2 / 1000"));
    assert!(report.contains("axial stress utilization"));
    assert!(!report.contains("compression capacity"));
    assert!(!report.contains("column capacity"));
    assert!(!report.contains("buckling"));
    assert!(report.contains("Validation gaps"));
}
