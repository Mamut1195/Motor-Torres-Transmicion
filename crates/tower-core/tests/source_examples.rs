use std::path::Path;

use serde::Deserialize;
use tower_core::analysis::TrussSolver;
use tower_core::loads::LoadCaseId;
use tower_core::model::TowerModel;

const SIMPLE_BAR_FIXTURE: &str =
    include_str!("fixtures/source_examples/example_01_simple_bar.toml");
const MATRIX_GATE_FIXTURE: &str =
    include_str!("fixtures/source_examples/example_09_self_weight_nodal_distribution_gate.toml");
const ALLOWED_TARGET: &str = "tower_core_truss_solver";

#[derive(Debug, Deserialize)]
struct SourceExample {
    id: Option<String>,
    status: Option<String>,
    trace_ids: Option<Vec<String>>,
    allowed_target: Option<String>,
    source: Option<SourceTrace>,
    approval: Option<Approval>,
    model: Option<ModelTarget>,
    #[serde(default)]
    expected: Vec<ExpectedOutput>,
    blocked_reason: Option<String>,
    #[serde(default)]
    missing_approval_fields: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SourceTrace {
    document: Option<String>,
    section: Option<String>,
    page: Option<String>,
    equation: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Approval {
    reviewer: Option<String>,
    date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ModelTarget {
    path: String,
    load_case: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedOutput {
    kind: String,
    node_id: Option<String>,
    member_id: Option<String>,
    component: String,
    value: f64,
    tolerance: Option<Tolerance>,
}

#[derive(Debug, Deserialize)]
struct Tolerance {
    absolute: f64,
    relative: f64,
    rationale: Option<String>,
}

#[derive(Debug, PartialEq)]
struct ActualOutput {
    label: String,
    value: f64,
}

fn parse_fixture(input: &str) -> Result<SourceExample, String> {
    toml::from_str(input).map_err(|error| error.to_string())
}

fn validate_metadata(example: &SourceExample) -> Result<(), String> {
    require_present(example.id.as_deref(), "id")?;
    let status = require_present(example.status.as_deref(), "status")?;
    if !matches!(
        status,
        "approved" | "candidate" | "blocked" | "TODO_DOMAIN_VALIDATION"
    ) {
        return Err(format!("status {status} is not allowed"));
    }

    let trace_ids = example.trace_ids.as_ref().ok_or("trace_ids is required")?;
    if trace_ids.is_empty() {
        return Err("trace_ids is required".to_string());
    }

    let source = example.source.as_ref().ok_or("source is required")?;
    require_present(source.document.as_deref(), "source.document")?;
    require_present(source.section.as_deref(), "source.section")?;
    require_present(source.page.as_deref(), "source.page")?;
    require_present(source.equation.as_deref(), "source.equation")?;

    if status == "approved" {
        let approval = example.approval.as_ref().ok_or("approval is required")?;
        require_present(approval.reviewer.as_deref(), "approval.reviewer")?;
        require_present(approval.date.as_deref(), "approval.date")?;
        for (index, expected) in example.expected.iter().enumerate() {
            let tolerance = expected
                .tolerance
                .as_ref()
                .ok_or_else(|| format!("expected[{index}].tolerance is required"))?;
            if !tolerance.absolute.is_finite() || !tolerance.relative.is_finite() {
                return Err(format!("expected[{index}].tolerance must be finite"));
            }
            require_present(
                tolerance.rationale.as_deref(),
                &format!("expected[{index}].tolerance.rationale"),
            )?;
        }
    }

    Ok(())
}

fn is_execution_eligible(example: &SourceExample) -> Result<bool, String> {
    validate_metadata(example)?;
    if example.status.as_deref() != Some("approved") {
        return Ok(false);
    }

    let approval = example.approval.as_ref().ok_or("approval is required")?;
    require_present(approval.reviewer.as_deref(), "approval.reviewer")?;
    require_present(approval.date.as_deref(), "approval.date")?;
    if example.expected.is_empty() {
        return Err("expected output records are required".to_string());
    }
    if example.model.is_none() {
        return Err("model is required".to_string());
    }
    match example.allowed_target.as_deref() {
        Some(ALLOWED_TARGET) => Ok(true),
        Some(_) => Err("allowed_target is not whitelisted".to_string()),
        None => Err("allowed_target is required".to_string()),
    }
}

fn execute_fixture(example: &SourceExample) -> Result<(), String> {
    let actuals = collect_actuals(example)?;
    for expected in &example.expected {
        let expected_label = expected_label(expected)?;
        let actual = actuals
            .iter()
            .find(|actual| actual.label == expected_label)
            .ok_or_else(|| format!("missing actual output {expected_label}"))?;
        assert_within_tolerance(
            actual.value,
            expected.value,
            expected.tolerance.as_ref().unwrap(),
        )?;
    }
    Ok(())
}

fn collect_actuals(example: &SourceExample) -> Result<Vec<ActualOutput>, String> {
    if !is_execution_eligible(example)? {
        return Err("fixture is not executable".to_string());
    }

    let model_target = example.model.as_ref().ok_or("model is required")?;
    let fixture_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/source_examples");
    let model_path = fixture_dir.join(&model_target.path);
    let model_text = std::fs::read_to_string(&model_path)
        .map_err(|error| format!("failed to read {}: {error}", model_path.display()))?;
    let model = TowerModel::from_toml_str(&model_text).map_err(|error| error.to_string())?;
    let result = TrussSolver::solve(&model, &LoadCaseId(model_target.load_case.clone()))
        .map_err(|error| error.to_string())?;

    let mut actuals = Vec::new();
    for expected in &example.expected {
        let value = match (expected.kind.as_str(), expected.component.as_str()) {
            ("displacement", "ux_m") => {
                result
                    .displacement(required_output_id(expected)?)
                    .ok_or_else(|| "missing displacement".to_string())?
                    .ux_m
            }
            ("reaction", "fx_kn") => {
                result
                    .reaction(required_output_id(expected)?)
                    .ok_or_else(|| "missing reaction".to_string())?
                    .fx_kn
            }
            ("member_force", "axial_kn") => {
                result
                    .member_force(required_output_id(expected)?)
                    .ok_or_else(|| "missing member force".to_string())?
                    .axial_kn
            }
            _ => {
                return Err(format!(
                    "unsupported expected output {}.{}",
                    expected.kind, expected.component
                ))
            }
        };
        actuals.push(ActualOutput {
            label: expected_label(expected)?,
            value,
        });
    }
    Ok(actuals)
}

fn assert_within_tolerance(
    actual: f64,
    expected: f64,
    tolerance: &Tolerance,
) -> Result<(), String> {
    if !actual.is_finite() || !expected.is_finite() {
        return Err("actual and expected values must be finite".to_string());
    }

    let difference = (actual - expected).abs();
    let relative_bound = tolerance.relative * expected.abs().max(1.0);
    if difference <= tolerance.absolute || difference <= relative_bound {
        Ok(())
    } else {
        Err(format!(
            "actual {actual} differs from expected {expected} beyond absolute {} or relative {}",
            tolerance.absolute, tolerance.relative
        ))
    }
}

fn expected_label(expected: &ExpectedOutput) -> Result<String, String> {
    Ok(format!(
        "{}:{}:{}",
        expected.kind,
        required_output_id(expected)?,
        expected.component
    ))
}

fn required_output_id(expected: &ExpectedOutput) -> Result<&str, String> {
    expected
        .node_id
        .as_deref()
        .or(expected.member_id.as_deref())
        .ok_or_else(|| "expected output requires node_id or member_id".to_string())
}

fn require_present<'a>(value: Option<&'a str>, field: &str) -> Result<&'a str, String> {
    let value = value.ok_or_else(|| format!("{field} is required"))?;
    if value.trim().is_empty() {
        return Err(format!("{field} is required"));
    }
    Ok(value)
}

#[test]
fn metadata_validation_rejects_missing_required_fields() {
    let cases = [
        (SIMPLE_BAR_FIXTURE.replace("trace_ids = [\"NUM-AXIAL-001\", \"NUM-AXIAL-002\", \"NUM-AXIAL-003\", \"NUM-AXIAL-004\"]\n", ""), "trace_ids"),
        (SIMPLE_BAR_FIXTURE.replace("[source]\n", "[missing_source]\n"), "source"),
        (SIMPLE_BAR_FIXTURE.replace("status = \"approved\"\n", ""), "status"),
        (SIMPLE_BAR_FIXTURE.replace("[approval]\n", "[missing_approval]\n"), "approval"),
        (SIMPLE_BAR_FIXTURE.replace("tolerance = { absolute = 1e-9, relative = 1e-7, rationale = \"WU3 closed-form axial bar displacement\" }\n", ""), "expected[0].tolerance"),
    ];

    for (fixture, missing_field) in cases {
        let example = parse_fixture(&fixture).unwrap();
        let error = validate_metadata(&example).unwrap_err();
        assert!(
            error.contains(missing_field),
            "expected missing field {missing_field}, got {error}"
        );
    }
}

#[test]
fn non_approved_fixtures_are_metadata_valid_but_not_executable() {
    for status in ["candidate", "blocked", "TODO_DOMAIN_VALIDATION"] {
        let fixture = MATRIX_GATE_FIXTURE.replace(
            "status = \"TODO_DOMAIN_VALIDATION\"",
            &format!("status = \"{status}\""),
        );
        let example = parse_fixture(&fixture).unwrap();

        validate_metadata(&example).unwrap();
        assert!(
            !is_execution_eligible(&example).unwrap(),
            "{status} must not execute"
        );
        assert_eq!(example.blocked_reason.as_deref(), Some("Self-weight nodal distribution needs manual PDF/equation/sign review before runtime authorization."));
    }
}

#[test]
fn approved_simple_bar_executes_against_whitelisted_target() {
    let example = parse_fixture(SIMPLE_BAR_FIXTURE).unwrap();
    validate_metadata(&example).unwrap();
    assert!(is_execution_eligible(&example).unwrap());

    execute_fixture(&example).unwrap();
}

#[test]
fn approved_simple_bar_execution_is_stable() {
    let example = parse_fixture(SIMPLE_BAR_FIXTURE).unwrap();

    let first = collect_actuals(&example).unwrap();
    let second = collect_actuals(&example).unwrap();

    assert_eq!(first, second);
    assert!(first
        .iter()
        .any(|actual| actual.label == "displacement:free:ux_m" && actual.value == 0.0001));
}

#[test]
fn blocked_matrix_fixture_preserves_manual_review_blockers_without_dispatch() {
    let example = parse_fixture(MATRIX_GATE_FIXTURE).unwrap();

    validate_metadata(&example).unwrap();
    assert!(!is_execution_eligible(&example).unwrap());
    assert!(example.model.is_none());
    assert!(example.expected.is_empty());
    assert!(example
        .missing_approval_fields
        .iter()
        .any(|field| field == "directions/signs"));
    assert!(example
        .blocked_reason
        .as_deref()
        .unwrap()
        .contains("manual PDF/equation/sign review"));
}

#[test]
fn incomplete_approved_fixtures_fail_before_numeric_comparison() {
    let no_reviewer = SIMPLE_BAR_FIXTURE.replace("reviewer = \"Jonnathan\"\n", "");
    let no_expected = SIMPLE_BAR_FIXTURE.replace("[[expected]]", "[[missing_expected]]");
    let unknown_target = SIMPLE_BAR_FIXTURE.replace(
        "allowed_target = \"tower_core_truss_solver\"",
        "allowed_target = \"runtime_load_generator\"",
    );

    for (fixture, expected_error) in [
        (no_reviewer, "approval.reviewer"),
        (no_expected, "expected"),
        (unknown_target, "allowed_target"),
    ] {
        let example = parse_fixture(&fixture).unwrap();
        let error = is_execution_eligible(&example).unwrap_err();
        assert!(
            error.contains(expected_error),
            "expected {expected_error}, got {error}"
        );
    }
}
