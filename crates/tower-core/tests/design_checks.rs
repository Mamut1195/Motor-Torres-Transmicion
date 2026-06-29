use tower_core::analysis::{AnalysisResult, MemberAxialForce, TrussSolver};
use tower_core::design_checks::{CheckEngine, CheckRule, CheckStatus, FormulaStatus};
use tower_core::errors::TowerError;
use tower_core::geometry::MemberId;
use tower_core::loads::LoadCaseId;
use tower_core::model::TowerModel;

const WEIGHT_FIXTURE_ID: &str = "example_05_member_weight_quantity";
const TRACE_TOTAL_WEIGHT: &str = "QTY-WEIGHT-001";
const TRACE_TENSION: &str = "CHK-TENSION-001";
const TRACE_COMPRESSION: &str = "CHK-COMPRESSION-001";
const TRACE_SLENDERNESS: &str = "CHK-SLENDERNESS-001";
const TRACE_DISPLACEMENT: &str = "CHK-DISPLACEMENT-001";
const TRACE_BUCKLING: &str = "CHK-BUCKLING-001";
const TRACE_LOAD_COMBINATION: &str = "CHK-LOAD-COMBINATION-001";

fn simple_bar_toml() -> &'static str {
    r#"
[metadata]
name = "example_05_member_weight_quantity"
disclaimer = "not for final engineering design"

[[nodes]]
id = "fixed"
point.x = { value = 0.0, unit = "m" }
point.y = { value = 0.0, unit = "m" }
point.z = { value = 0.0, unit = "m" }

[[nodes]]
id = "free"
point.x = { value = 2.0, unit = "m" }
point.y = { value = 0.0, unit = "m" }
point.z = { value = 0.0, unit = "m" }

[[materials]]
id = "steel"
density = { value = 7850.0, unit = "kg/m3" }
yield_stress = { value = 250.0, unit = "MPa" }
elastic_modulus = { value = 200000000.0, unit = "kN/m2" }

[[sections]]
id = "bar"
material_id = "steel"
nominal_area = { value = 0.001, unit = "m2" }
radius_of_gyration = { value = 0.02, unit = "m" }

[[members]]
id = "bar-x"
start = "fixed"
end = "free"
section_id = "bar"

[[supports]]
node_id = "fixed"
ux = true
uy = true
uz = true

[[supports]]
node_id = "free"
ux = false
uy = true
uz = true

[[load_cases]]
id = "axial"

[[load_cases.nodal_loads]]
node_id = "free"
fx = { value = 10.0, unit = "kN" }
fy = { value = 0.0, unit = "kN" }
fz = { value = 0.0, unit = "kN" }
"#
}

fn analysis_with_member_force(axial_kn: f64) -> AnalysisResult {
    AnalysisResult {
        displacements: Vec::new(),
        reactions: Vec::new(),
        member_forces: vec![MemberAxialForce {
            member_id: MemberId("bar-x".to_string()),
            axial_kn,
        }],
    }
}

fn model_with_area(area_m2: f64) -> TowerModel {
    TowerModel::from_toml_str(&simple_bar_toml().replace("0.001", &area_m2.to_string())).unwrap()
}

fn assert_parse_error_contains(input: &str, expected_message: &str) {
    let error = TowerModel::from_toml_str(input).unwrap_err();

    match error {
        TowerError::Parse { message } => assert!(
            message.contains(expected_message),
            "expected parse error to contain {expected_message:?}, got {message:?}"
        ),
        other => panic!("expected parse validation boundary, got {other:?}"),
    }
}

fn assert_close(actual: f64, expected: f64, absolute_tolerance: f64, relative_tolerance: f64) {
    let absolute_error = (actual - expected).abs();
    let relative_error = if expected == 0.0 {
        absolute_error
    } else {
        absolute_error / expected.abs()
    };

    assert!(
        absolute_error <= absolute_tolerance || relative_error <= relative_tolerance,
        "expected {actual} to be within absolute tolerance {absolute_tolerance} or relative tolerance {relative_tolerance} of {expected}"
    );
}

#[test]
fn computes_total_weight_only_with_validated_quantity_trace() {
    assert_eq!(WEIGHT_FIXTURE_ID, "example_05_member_weight_quantity");
    assert_eq!(TRACE_TOTAL_WEIGHT, "QTY-WEIGHT-001");

    let model = TowerModel::from_toml_str(simple_bar_toml()).unwrap();
    let analysis = TrussSolver::solve(&model, &LoadCaseId("axial".to_string())).unwrap();

    let results = CheckEngine::run(
        &model,
        &analysis,
        &[CheckRule::TotalWeight {
            trace_id: TRACE_TOTAL_WEIGHT,
            formula_status: FormulaStatus::Validated,
        }],
    )
    .unwrap();

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.rule_id, TRACE_TOTAL_WEIGHT);
    assert_eq!(result.status, CheckStatus::Pass);
    assert_eq!(result.trace.formula_id, TRACE_TOTAL_WEIGHT);
    assert_eq!(result.trace.validation_status, FormulaStatus::Validated);
    assert_eq!(
        result.trace.inputs,
        vec![
            "bar-x.length_m",
            "bar.nominal_area_m2",
            "steel.density_kg_per_m3"
        ]
    );
    assert_close(result.value.unwrap(), 0.153_964_405, 1.0e-10, 1.0e-7);
}

#[test]
fn blocks_unvalidated_normative_member_checks_with_deterministic_status() {
    assert_eq!(TRACE_TENSION, "CHK-TENSION-001");
    assert_eq!(TRACE_COMPRESSION, "CHK-COMPRESSION-001");
    assert_eq!(TRACE_SLENDERNESS, "CHK-SLENDERNESS-001");
    assert_eq!(TRACE_DISPLACEMENT, "CHK-DISPLACEMENT-001");

    let model = TowerModel::from_toml_str(simple_bar_toml()).unwrap();
    let analysis = TrussSolver::solve(&model, &LoadCaseId("axial".to_string())).unwrap();

    let results = CheckEngine::run(
        &model,
        &analysis,
        &[
            CheckRule::Tension {
                trace_id: TRACE_TENSION,
                formula_status: FormulaStatus::TodoDomainValidation,
            },
            CheckRule::Compression {
                trace_id: TRACE_COMPRESSION,
                formula_status: FormulaStatus::TodoDomainValidation,
            },
            CheckRule::Slenderness {
                trace_id: TRACE_SLENDERNESS,
                formula_status: FormulaStatus::TodoDomainValidation,
            },
            CheckRule::Displacement {
                trace_id: TRACE_DISPLACEMENT,
                formula_status: FormulaStatus::TodoDomainValidation,
            },
        ],
    )
    .unwrap();

    assert_eq!(results.len(), 4);
    let expected_rule_ids = [
        TRACE_TENSION,
        TRACE_COMPRESSION,
        TRACE_SLENDERNESS,
        TRACE_DISPLACEMENT,
    ];
    for (result, expected_rule_id) in results.iter().zip(expected_rule_ids) {
        assert_eq!(result.rule_id, expected_rule_id);
        assert_eq!(result.trace.formula_id, expected_rule_id);
        assert!(
            matches!(
                result.status,
                CheckStatus::Blocked | CheckStatus::TodoDomainValidation
            ),
            "unexpected status for {}: {:?}",
            result.rule_id,
            result.status
        );
        assert_eq!(
            result.trace.validation_status,
            FormulaStatus::TodoDomainValidation
        );
        assert_eq!(result.value, None);
        assert!(
            result
                .message
                .contains("blocked until formula-register validation is complete"),
            "unexpected message: {}",
            result.message
        );
    }
}

#[test]
fn blocks_total_weight_when_validated_trace_id_is_not_qty_weight_001() {
    let model = TowerModel::from_toml_str(simple_bar_toml()).unwrap();
    let analysis = TrussSolver::solve(&model, &LoadCaseId("axial".to_string())).unwrap();

    let results = CheckEngine::run(
        &model,
        &analysis,
        &[CheckRule::TotalWeight {
            trace_id: "QTY-WEIGHT-999",
            formula_status: FormulaStatus::Validated,
        }],
    )
    .unwrap();

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.rule_id, "QTY-WEIGHT-999");
    assert_eq!(result.status, CheckStatus::Blocked);
    assert_eq!(result.trace.formula_id, "QTY-WEIGHT-999");
    assert_eq!(result.trace.validation_status, FormulaStatus::Validated);
    assert_eq!(result.value, None);
    assert_eq!(
        result.message,
        "QTY-WEIGHT-999 is blocked because only QTY-WEIGHT-001 is validated for total member self-weight"
    );
}

#[test]
fn blocks_total_weight_when_trace_is_not_validated() {
    let model = TowerModel::from_toml_str(simple_bar_toml()).unwrap();
    let analysis = TrussSolver::solve(&model, &LoadCaseId("axial".to_string())).unwrap();

    let results = CheckEngine::run(
        &model,
        &analysis,
        &[CheckRule::TotalWeight {
            trace_id: TRACE_TOTAL_WEIGHT,
            formula_status: FormulaStatus::Pending,
        }],
    )
    .unwrap();

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.status, CheckStatus::Blocked);
    assert_eq!(result.trace.formula_id, TRACE_TOTAL_WEIGHT);
    assert_eq!(result.trace.validation_status, FormulaStatus::Pending);
    assert_eq!(result.value, None);
    assert_eq!(
        result.message,
        "QTY-WEIGHT-001 is blocked until formula-register validation is complete"
    );
}

#[test]
fn validated_tension_axial_stress_utilization_passes_and_fails_with_traceable_inputs() {
    assert_eq!(TRACE_TENSION, "CHK-TENSION-001");
    let pass_model = model_with_area(0.001);
    let fail_model = model_with_area(0.00001);

    let pass_results = CheckEngine::run(
        &pass_model,
        &analysis_with_member_force(10.0),
        &[CheckRule::Tension {
            trace_id: TRACE_TENSION,
            formula_status: FormulaStatus::Validated,
        }],
    )
    .unwrap();
    let fail_results = CheckEngine::run(
        &fail_model,
        &analysis_with_member_force(10.0),
        &[CheckRule::Tension {
            trace_id: TRACE_TENSION,
            formula_status: FormulaStatus::Validated,
        }],
    )
    .unwrap();

    let pass = &pass_results[0];
    assert_eq!(pass.rule_id, TRACE_TENSION);
    assert_eq!(pass.status, CheckStatus::Pass);
    assert_close(pass.value.unwrap(), 0.04, 1.0e-12, 1.0e-12);
    assert_eq!(pass.trace.formula_id, TRACE_TENSION);
    assert_eq!(pass.trace.validation_status, FormulaStatus::Validated);
    assert_eq!(
        pass.trace.inputs,
        vec![
            "bar-x.axial_force_kN",
            "bar.nominal_area_m2",
            "steel.yield_stress_MPa",
            "stress_mpa = abs(kN) / m2 / 1000"
        ]
    );
    assert!(pass.message.contains("tension axial stress utilization"));
    assert!(pass
        .message
        .contains("example_06_tension_axial_stress_utilization"));

    let fail = &fail_results[0];
    assert_eq!(fail.status, CheckStatus::Fail);
    assert_close(fail.value.unwrap(), 4.0, 1.0e-12, 1.0e-12);
    assert!(fail.message.contains("tension axial stress utilization"));
}

#[test]
fn validated_compression_axial_stress_utilization_passes_and_fails_without_buckling_claims() {
    assert_eq!(TRACE_COMPRESSION, "CHK-COMPRESSION-001");
    let pass_model = model_with_area(0.001);
    let fail_model = model_with_area(0.00001);

    let pass_results = CheckEngine::run(
        &pass_model,
        &analysis_with_member_force(-10.0),
        &[CheckRule::Compression {
            trace_id: TRACE_COMPRESSION,
            formula_status: FormulaStatus::Validated,
        }],
    )
    .unwrap();
    let fail_results = CheckEngine::run(
        &fail_model,
        &analysis_with_member_force(-10.0),
        &[CheckRule::Compression {
            trace_id: TRACE_COMPRESSION,
            formula_status: FormulaStatus::Validated,
        }],
    )
    .unwrap();

    let pass = &pass_results[0];
    assert_eq!(pass.status, CheckStatus::Pass);
    assert_close(pass.value.unwrap(), 0.04, 1.0e-12, 1.0e-12);
    assert!(pass
        .message
        .contains("compression axial stress utilization"));
    assert!(pass
        .message
        .contains("example_07_compression_axial_stress_utilization"));
    assert!(!pass.message.contains("compression capacity"));
    assert!(!pass.message.contains("column capacity"));
    assert!(!pass.message.contains("buckling"));

    let fail = &fail_results[0];
    assert_eq!(fail.status, CheckStatus::Fail);
    assert_close(fail.value.unwrap(), 4.0, 1.0e-12, 1.0e-12);
    assert!(!fail.message.contains("compression capacity"));
    assert!(!fail.message.contains("column capacity"));
    assert!(!fail.message.contains("buckling"));
}

#[test]
fn missing_force_blocks_and_zero_force_has_one_deterministic_active_classification() {
    let model = model_with_area(0.001);
    let missing_force = AnalysisResult {
        displacements: Vec::new(),
        reactions: Vec::new(),
        member_forces: Vec::new(),
    };

    let missing_results = CheckEngine::run(
        &model,
        &missing_force,
        &[CheckRule::Tension {
            trace_id: TRACE_TENSION,
            formula_status: FormulaStatus::Validated,
        }],
    )
    .unwrap();
    assert_eq!(missing_results[0].status, CheckStatus::Blocked);
    assert_eq!(missing_results[0].value, None);
    assert!(missing_results[0].message.contains("missing axial force"));

    let zero_results = CheckEngine::run(
        &model,
        &analysis_with_member_force(0.0),
        &[
            CheckRule::Tension {
                trace_id: TRACE_TENSION,
                formula_status: FormulaStatus::Validated,
            },
            CheckRule::Compression {
                trace_id: TRACE_COMPRESSION,
                formula_status: FormulaStatus::Validated,
            },
        ],
    )
    .unwrap();

    assert_eq!(zero_results.len(), 2);
    assert_eq!(zero_results[0].status, CheckStatus::Pass);
    assert_eq!(zero_results[0].value, Some(0.0));
    assert!(zero_results[0]
        .message
        .contains("tension axial stress utilization"));
    assert_eq!(zero_results[1].status, CheckStatus::Pass);
    assert_eq!(zero_results[1].value, Some(0.0));
    assert!(zero_results[1]
        .message
        .contains("no active compression axial stress utilization demand"));
    let active_outputs = zero_results
        .iter()
        .filter(|result| !result.message.contains("no active"))
        .count();
    assert_eq!(active_outputs, 1);
}

#[test]
fn missing_or_invalid_area_is_rejected_before_design_check_execution() {
    let missing_area_model =
        simple_bar_toml().replace("nominal_area = { value = 0.001, unit = \"m2\" }\n", "");
    assert_parse_error_contains(&missing_area_model, "missing field `nominal_area`");

    let invalid_area_error =
        TowerModel::from_toml_str(&simple_bar_toml().replace("0.001", "0.0")).unwrap_err();
    assert_eq!(
        invalid_area_error,
        TowerError::NonPositiveValue {
            field: "sections.bar.nominal_area".to_string(),
            value: 0.0,
        }
    );
}

#[test]
fn missing_yield_stress_is_rejected_before_design_check_execution() {
    let missing_yield_stress_model =
        simple_bar_toml().replace("yield_stress = { value = 250.0, unit = \"MPa\" }\n", "");

    assert_parse_error_contains(&missing_yield_stress_model, "missing field `yield_stress`");
}

#[test]
fn blocks_wrong_trace_and_out_of_scope_or_non_validated_member_checks() {
    let model = model_with_area(0.001);
    let analysis = analysis_with_member_force(10.0);
    let results = CheckEngine::run(
        &model,
        &analysis,
        &[
            CheckRule::Tension {
                trace_id: "CHK-TENSION-999",
                formula_status: FormulaStatus::Validated,
            },
            CheckRule::Compression {
                trace_id: TRACE_BUCKLING,
                formula_status: FormulaStatus::Validated,
            },
            CheckRule::Slenderness {
                trace_id: TRACE_SLENDERNESS,
                formula_status: FormulaStatus::TodoDomainValidation,
            },
            CheckRule::Displacement {
                trace_id: TRACE_DISPLACEMENT,
                formula_status: FormulaStatus::TodoDomainValidation,
            },
            CheckRule::Tension {
                trace_id: TRACE_LOAD_COMBINATION,
                formula_status: FormulaStatus::Provisional,
            },
            CheckRule::Tension {
                trace_id: TRACE_TENSION,
                formula_status: FormulaStatus::Provisional,
            },
        ],
    )
    .unwrap();

    assert_eq!(results.len(), 6);
    for result in results {
        assert!(
            matches!(
                result.status,
                CheckStatus::Blocked | CheckStatus::TodoDomainValidation
            ),
            "{} unexpectedly emitted pass/fail",
            result.rule_id
        );
        assert_eq!(result.value, None);
        assert!(result.message.contains("blocked"));
    }
}
