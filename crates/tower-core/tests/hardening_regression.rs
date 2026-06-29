use tower_core::analysis::TrussSolver;
use tower_core::design_checks::{CheckEngine, CheckRule, CheckStatus, FormulaStatus};
use tower_core::errors::TowerError;
use tower_core::loads::LoadCaseId;
use tower_core::model::TowerModel;
use tower_core::optimization::{
    CandidateCheck, CandidateSection, GreedyOptimizer, OptimizationResult,
};
use tower_core::reporting::{PreliminaryReport, ENGINEERING_DISCLAIMER};
use tower_core::sections::SectionId;

const INVALID_TOML: &str = include_str!("../../../tests/fixtures/invalid_model.toml");
const SINGULAR_MODEL: &str = include_str!("../../../tests/fixtures/singular_model.toml");

#[test]
fn fixture_failures_render_preliminary_failed_reports() {
    let invalid_error = TowerModel::from_toml_str(INVALID_TOML).unwrap_err();
    assert!(matches!(invalid_error, TowerError::Parse { .. }));

    let invalid_report = PreliminaryReport::from_error("invalid fixture", &invalid_error).render();
    assert!(invalid_report.contains(ENGINEERING_DISCLAIMER));
    assert!(invalid_report.contains("Failed run"));
    assert!(invalid_report.contains("input parsing failed"));

    let singular_model = TowerModel::from_toml_str(SINGULAR_MODEL).unwrap();
    let singular_error =
        TrussSolver::solve(&singular_model, &LoadCaseId("lateral".to_string())).unwrap_err();
    assert!(matches!(singular_error, TowerError::UnstableModel { .. }));

    let singular_report =
        PreliminaryReport::from_error("singular fixture", &singular_error).render();
    assert!(singular_report.contains(ENGINEERING_DISCLAIMER));
    assert!(singular_report.contains("model is unstable"));
    assert!(singular_report.contains("not final-design approval"));
}

#[test]
fn unsupported_checks_and_infeasible_optimization_stay_visible_in_reports() {
    let model =
        TowerModel::from_toml_str(include_str!("../../../examples/simple_bar.toml")).unwrap();
    let analysis = TrussSolver::solve(&model, &LoadCaseId("axial".to_string())).unwrap();
    let checks = CheckEngine::run(
        &model,
        &analysis,
        &[
            CheckRule::Tension {
                trace_id: "CHK-TENSION-001",
                formula_status: FormulaStatus::TodoDomainValidation,
            },
            CheckRule::Slenderness {
                trace_id: "CHK-SLENDERNESS-001",
                formula_status: FormulaStatus::Pending,
            },
        ],
    )
    .unwrap();

    assert_eq!(checks.len(), 2);
    assert!(checks.iter().all(|check| matches!(
        check.status,
        CheckStatus::Blocked | CheckStatus::TodoDomainValidation
    )));

    let optimization = GreedyOptimizer::select(&[CandidateSection {
        section_id: SectionId("FAILS-TENSION".to_string()),
        weight_kg_per_m: 1.0,
        checks: vec![CandidateCheck {
            trace_id: "CHK-TENSION-001",
            status: CheckStatus::Fail,
            validation_status: FormulaStatus::Validated,
        }],
    }]);
    assert!(matches!(optimization, OptimizationResult::Infeasible(_)));

    let report =
        PreliminaryReport::from_checks("hardening regression", &checks, Some(&optimization))
            .render();
    assert!(report.contains("Validation gaps"));
    assert!(report.contains("CHK-TENSION-001"));
    assert!(report.contains("CHK-SLENDERNESS-001"));
    assert!(report.contains("Optimization status: infeasible"));
    assert!(report.contains("FAILS-TENSION"));
    assert!(report.contains("not for final engineering design"));
}

#[test]
fn deterministic_examples_keep_regression_metadata() {
    let simple_bar =
        TowerModel::from_toml_str(include_str!("../../../examples/simple_bar.toml")).unwrap();
    let tower_69kv =
        TowerModel::from_toml_str(include_str!("../../../examples/69kv_tower.toml")).unwrap();
    let optimization_demo =
        TowerModel::from_toml_str(include_str!("../../../examples/optimization_demo.toml"))
            .unwrap();

    assert_eq!(simple_bar.metadata.name, "example_01_simple_bar");
    assert_eq!(simple_bar.metadata.disclaimer, ENGINEERING_DISCLAIMER);
    assert_eq!(
        tower_69kv.metadata.name,
        "example_03_69kv_tower_placeholder"
    );
    assert_eq!(tower_69kv.metadata.disclaimer, ENGINEERING_DISCLAIMER);
    assert_eq!(
        optimization_demo.metadata.name,
        "example_04_optimization_demo"
    );
    assert_eq!(
        optimization_demo.metadata.disclaimer,
        ENGINEERING_DISCLAIMER
    );
}
