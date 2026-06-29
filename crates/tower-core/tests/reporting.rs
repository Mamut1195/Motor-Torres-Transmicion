use tower_core::design_checks::{CheckResult, CheckStatus, CheckTrace, FormulaStatus};
use tower_core::errors::TowerError;
use tower_core::geometry::NodeId;
use tower_core::loads::{LoadCase, LoadCaseId, LoadProvenanceStatus, NodalLoad};
use tower_core::optimization::{BlockedOptimization, OptimizationResult};
use tower_core::reporting::{
    blocked_load_model_evidence, LoadEvidence, PreliminaryReport, ENGINEERING_DISCLAIMER,
};
use tower_core::units::ForceKilonewtons;

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

#[test]
fn load_evidence_report_lists_explicit_load_status_source_and_disclaimer() {
    let load_case = LoadCase {
        id: LoadCaseId("service_load".to_string()),
        status: LoadProvenanceStatus::ExplicitUserInput,
        source: "user input".to_string(),
        nodal_loads: vec![NodalLoad {
            node_id: NodeId("top".to_string()),
            fx: ForceKilonewtons::new(5.0).expect("valid force"),
            fy: ForceKilonewtons::new(0.0).expect("valid force"),
            fz: ForceKilonewtons::new(-2.0).expect("valid force"),
        }],
    };

    let report = PreliminaryReport::from_load_evidence(
        "minimum-load-model-v1",
        &[LoadEvidence::explicit_load_case(&load_case)],
    )
    .render();

    assert!(report.contains(ENGINEERING_DISCLAIMER));
    assert!(report.contains("not for final engineering design"));
    assert!(report.contains("Load model evidence"));
    assert!(report.contains("service_load"));
    assert!(report.contains("explicit_user_input"));
    assert!(report.contains("source: user input"));
    assert!(report.contains("nodal loads: 1 explicit user input record(s)"));
    assert!(!report.contains("code-compliant loading"));
    assert!(!report.contains("final design"));
}

#[test]
fn load_evidence_report_lists_validated_quantity_and_blocked_categories_without_generated_loads() {
    let evidence = vec![
        LoadEvidence::validated_quantity(
            "QTY-WEIGHT-001",
            "member self-weight quantity",
            "formula register and design check trace",
        ),
        LoadEvidence::todo_domain_validation(
            "wind loads",
            "requires reviewer-approved wind loading clauses and assumptions",
        ),
        LoadEvidence::todo_domain_validation(
            "load combinations and factors",
            "requires reviewer-approved combinations and factors",
        ),
    ];

    let report = PreliminaryReport::from_load_evidence("load-evidence", &evidence).render();

    assert!(report.contains("QTY-WEIGHT-001"));
    assert!(report.contains("validated_quantity"));
    assert!(report.contains("member self-weight quantity"));
    assert!(report.contains("formula register and design check trace"));
    assert!(report.contains("wind loads"));
    assert!(report.contains("load combinations and factors"));
    assert!(report.contains("TODO_DOMAIN_VALIDATION"));
    assert!(report.contains("No generated nodal loads are created from this evidence."));
    assert!(!report.contains("self-weight nodal load"));
    assert!(!report.contains("wind pressure result"));
}

#[test]
fn default_blocked_load_evidence_covers_unapproved_loading_categories() {
    let report = PreliminaryReport::from_load_evidence(
        "blocked-load-categories",
        &blocked_load_model_evidence(),
    )
    .render();

    for category in [
        "self-weight to nodal load generation",
        "wind loads",
        "conductor loads",
        "load combinations",
        "load factors",
        "displacement or design-level loading",
    ] {
        assert!(report.contains(category), "missing category {category}");
    }
    assert!(report.contains("TODO_DOMAIN_VALIDATION"));
    assert!(report.contains("not for final engineering design"));
}
