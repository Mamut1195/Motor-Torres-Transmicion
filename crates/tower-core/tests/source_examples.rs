use std::path::Path;

use serde::Deserialize;
use tower_core::analysis::{AnalysisResult, TrussSolver};
use tower_core::design_checks::{CheckEngine, CheckRule, FormulaStatus};
use tower_core::loads::LoadCaseId;
use tower_core::model::TowerModel;

const SIMPLE_BAR_FIXTURE: &str =
    include_str!("fixtures/source_examples/example_01_simple_bar.toml");
const MEMBER_WEIGHT_FIXTURE: &str =
    include_str!("fixtures/source_examples/example_05_member_weight_quantity.toml");
const MATRIX_GATE_FIXTURE: &str =
    include_str!("fixtures/source_examples/example_09_self_weight_nodal_distribution_gate.toml");
const SOLVER_TARGET: &str = "tower_core_truss_solver";
const TOTAL_WEIGHT_TARGET: &str = "tower_core_total_weight_check";
const TOTAL_WEIGHT_TRACE_ID: &str = "QTY-WEIGHT-001";
const VALIDATION_EXAMPLES_DOC: &str = include_str!("../../../docs/domain/validation_examples.md");
const FORMULAS_REGISTER_DOC: &str = include_str!("../../../docs/domain/formulas_register.md");
const ACCEPTANCE_GATE_DOC: &str = include_str!("../../../docs/domain/acceptance_gate.md");
const OPEN_QUESTIONS_DOC: &str = include_str!("../../../docs/domain/open_questions.md");
const LOAD_SW_DIST_TRACE_ID: &str = "LOAD-SW-DIST-001";
const MANDATORY_LOAD_SW_LEDGER_FIELDS: &[&str] = &[
    "source rule",
    "clause/project-rule ID",
    "reviewer interpretation",
    "assumptions",
    "target nodes",
    "signs/directions",
    "units",
    "applicability limits",
    "numeric trace",
    "tolerance rationale",
    "reviewer identity",
    "ISO review date",
    "future tests-first runtime authorization status",
];

#[derive(Debug, Deserialize)]
struct SourceExample {
    id: Option<String>,
    status: Option<String>,
    trace_ids: Option<Vec<String>>,
    allowed_target: Option<String>,
    source: Option<SourceTrace>,
    approval: Option<Approval>,
    model: Option<ModelTarget>,
    evidence_boundary: Option<String>,
    #[serde(default)]
    expected: Vec<ExpectedOutput>,
    blocked_reason: Option<String>,
    candidate: Option<CandidateMetadata>,
    #[serde(default)]
    missing_approval_fields: Vec<String>,
    #[serde(default)]
    candidate_sources: Vec<String>,
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

#[derive(Debug, Deserialize)]
struct CandidateMetadata {
    status: String,
    total_self_weight_kn: f64,
    equal_end_lump_kn: f64,
    boundary: String,
    #[serde(default)]
    unapproved_fields: Vec<String>,
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
        validate_quantity_boundary(example)?;
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
    if example.trace_ids.as_ref().is_some_and(|trace_ids| {
        trace_ids
            .iter()
            .any(|trace_id| trace_id == LOAD_SW_DIST_TRACE_ID)
    }) && !example.missing_approval_fields.is_empty()
    {
        return Err("runtime authorization is blocked until every LOAD-SW-DIST-001 approval field is complete".to_string());
    }
    if example.expected.is_empty() {
        return Err("expected output records are required".to_string());
    }
    if example.model.is_none() {
        return Err("model is required".to_string());
    }
    match example.allowed_target.as_deref() {
        Some(SOLVER_TARGET | TOTAL_WEIGHT_TARGET) => Ok(true),
        Some(_) => Err("allowed_target is not whitelisted".to_string()),
        None => Err("allowed_target is required".to_string()),
    }
}

fn validate_quantity_boundary(example: &SourceExample) -> Result<(), String> {
    if !example.trace_ids.as_ref().is_some_and(|trace_ids| {
        trace_ids
            .iter()
            .any(|trace_id| trace_id == TOTAL_WEIGHT_TRACE_ID)
    }) {
        return Ok(());
    }

    if example.allowed_target.as_deref() == Some(SOLVER_TARGET) {
        return Ok(());
    }

    let boundary = require_present(example.evidence_boundary.as_deref(), "evidence_boundary")?;
    let boundary_lower = boundary.to_ascii_lowercase();
    if !boundary_lower.contains("quantity-only") {
        return Err("evidence_boundary must state quantity-only approval".to_string());
    }

    for forbidden in ["approves", "authorizes", "approved"] {
        if boundary_lower.contains(forbidden) {
            return Err(
                "evidence_boundary must not approve distribution or runtime generation claims"
                    .to_string(),
            );
        }
    }

    Ok(())
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
    let solver_result = if example.allowed_target.as_deref() == Some(SOLVER_TARGET) {
        Some(
            TrussSolver::solve(&model, &LoadCaseId(model_target.load_case.clone()))
                .map_err(|error| error.to_string())?,
        )
    } else {
        None
    };
    let design_check = if example.allowed_target.as_deref() == Some(TOTAL_WEIGHT_TARGET) {
        let analysis = AnalysisResult {
            displacements: Vec::new(),
            reactions: Vec::new(),
            member_forces: Vec::new(),
        };
        let results = CheckEngine::run(
            &model,
            &analysis,
            &[CheckRule::TotalWeight {
                trace_id: TOTAL_WEIGHT_TRACE_ID,
                formula_status: FormulaStatus::Validated,
            }],
        )
        .map_err(|error| error.to_string())?;
        Some(
            results
                .into_iter()
                .next()
                .ok_or_else(|| "missing total weight check result".to_string())?,
        )
    } else {
        None
    };

    let mut actuals = Vec::new();
    for expected in &example.expected {
        let value = match (expected.kind.as_str(), expected.component.as_str()) {
            ("displacement", "ux_m") => {
                solver_result
                    .as_ref()
                    .ok_or_else(|| "solver result is required".to_string())?
                    .displacement(required_output_id(expected)?)
                    .ok_or_else(|| "missing displacement".to_string())?
                    .ux_m
            }
            ("reaction", "fx_kn") => {
                solver_result
                    .as_ref()
                    .ok_or_else(|| "solver result is required".to_string())?
                    .reaction(required_output_id(expected)?)
                    .ok_or_else(|| "missing reaction".to_string())?
                    .fx_kn
            }
            ("member_force", "axial_kn") => {
                solver_result
                    .as_ref()
                    .ok_or_else(|| "solver result is required".to_string())?
                    .member_force(required_output_id(expected)?)
                    .ok_or_else(|| "missing member force".to_string())?
                    .axial_kn
            }
            ("design_check", "total_weight_kn") => design_check
                .as_ref()
                .and_then(|result| result.value)
                .ok_or_else(|| "missing total weight check value".to_string())?,
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
    if expected.kind == "design_check" && expected.component == "total_weight_kn" {
        return Ok(format!(
            "design_check:{TOTAL_WEIGHT_TRACE_ID}:{}",
            expected.component
        ));
    }

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

fn assert_doc_contains_all_fields(doc_name: &str, doc: &str) {
    assert!(
        doc.contains(LOAD_SW_DIST_TRACE_ID),
        "{doc_name} must trace {LOAD_SW_DIST_TRACE_ID}"
    );
    for field in MANDATORY_LOAD_SW_LEDGER_FIELDS {
        assert!(
            doc.contains(field),
            "{doc_name} must document mandatory ledger field {field}"
        );
    }
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
    assert!(example.allowed_target.is_none());
    assert!(example.approval.is_none());
    assert!(example.model.is_none());
    assert!(example.expected.is_empty());
    assert!(example
        .missing_approval_fields
        .iter()
        .any(|field| field == "signs/directions"));
    assert!(example
        .blocked_reason
        .as_deref()
        .unwrap()
        .contains("manual PDF/equation/sign review"));
}

#[test]
fn blocked_matrix_fixture_records_candidate_arithmetic_without_runtime_approval() {
    let example = parse_fixture(MATRIX_GATE_FIXTURE).unwrap();
    let candidate = example
        .candidate
        .as_ref()
        .expect("example_09 must record candidate-only arithmetic metadata");

    assert_eq!(example.status.as_deref(), Some("TODO_DOMAIN_VALIDATION"));
    assert_eq!(candidate.status, "candidate_only_unapproved");
    assert_eq!(candidate.total_self_weight_kn, 0.153_964_405);
    assert_eq!(candidate.equal_end_lump_kn, 0.076_982_202_5);
    assert!(candidate.boundary.contains("candidate only"));
    assert!(candidate.boundary.contains("not approved"));
    assert!(candidate.boundary.contains("runtime authorization"));
    for field in [
        "axis/sign",
        "target nodes",
        "distribution factors",
        "tolerance rationale",
        "reviewer/date",
        "runtime authorization",
    ] {
        assert!(
            candidate
                .unapproved_fields
                .iter()
                .any(|unapproved| unapproved == field),
            "candidate metadata must keep {field} unapproved"
        );
    }
}

#[test]
fn docs_record_candidate_arithmetic_and_preserve_runtime_blockers() {
    for required in [
        "total self-weight `0.153964405 kN`",
        "equal-end candidate value `0.0769822025 kN`",
        "review-only arithmetic",
        "axis/sign, target nodes, distribution factors, tolerance rationale, reviewer/date, and runtime authorization remain unapproved",
    ] {
        assert!(
            VALIDATION_EXAMPLES_DOC.contains(required),
            "validation_examples.md must contain {required}"
        );
    }

    for required in [
        "candidate review values: total `0.153964405 kN`; equal-end lumping candidate `0.0769822025 kN` per end",
        "not an approved formula, nodal distribution rule, load-generation rule, or runtime authorization",
        "axis/sign, target nodes, distribution factors, tolerance rationale, reviewer/date, and runtime authorization",
    ] {
        assert!(
            FORMULAS_REGISTER_DOC.contains(required),
            "formulas_register.md must contain {required}"
        );
    }

    for required in [
        "candidate arithmetic does not authorize schema, CLI, runtime, reports, optimizer, examples, or executable tests",
        "total `0.153964405 kN` and equal-end candidate `0.0769822025 kN` per end",
        "no target-node, axis/sign, tolerance, reviewer/date, or runtime authorization may be inferred",
    ] {
        assert!(
            ACCEPTANCE_GATE_DOC.contains(required),
            "acceptance_gate.md must contain {required}"
        );
    }
}

#[test]
fn docs_define_complete_load_sw_dist_non_runtime_approval_packet_ledger() {
    assert_doc_contains_all_fields("formulas_register.md", FORMULAS_REGISTER_DOC);
    assert_doc_contains_all_fields("validation_examples.md", VALIDATION_EXAMPLES_DOC);
    assert_doc_contains_all_fields("acceptance_gate.md", ACCEPTANCE_GATE_DOC);
    assert_doc_contains_all_fields("open_questions.md", OPEN_QUESTIONS_DOC);

    for doc in [
        FORMULAS_REGISTER_DOC,
        VALIDATION_EXAMPLES_DOC,
        ACCEPTANCE_GATE_DOC,
        OPEN_QUESTIONS_DOC,
    ] {
        let doc_lower = doc.to_ascii_lowercase();
        assert!(doc_lower
            .contains("candidate inventory/arithmetic is not approved engineering evidence"));
        assert!(doc_lower.contains("does not authorize runtime execution"));
    }
}

#[test]
fn self_weight_packet_missing_reviewer_interpretation_remains_candidate_only() {
    let fixture = MATRIX_GATE_FIXTURE.replace("    \"reviewer interpretation\",\n", "");
    let example = parse_fixture(&fixture).unwrap();

    validate_metadata(&example).unwrap();
    assert!(!is_execution_eligible(&example).unwrap());
    assert!(!example
        .missing_approval_fields
        .iter()
        .any(|field| field == "reviewer interpretation"));
    assert!(example.candidate.is_some());
    assert!(collect_actuals(&example)
        .unwrap_err()
        .contains("not executable"));
}

#[test]
fn reviewer_identity_and_date_without_future_authorization_stay_non_executable() {
    let fixture = format!(
        "{}\n[approval]\nreviewer = \"Domain Reviewer\"\ndate = \"2026-06-30\"\n",
        MATRIX_GATE_FIXTURE.replace(
            "status = \"TODO_DOMAIN_VALIDATION\"",
            "status = \"approved\"\nallowed_target = \"tower_core_truss_solver\"",
        )
    );
    let example = parse_fixture(&fixture).unwrap();

    let error = is_execution_eligible(&example).unwrap_err();

    assert!(error.contains("runtime authorization"), "got {error}");
    assert!(example.expected.is_empty());
    assert!(example.model.is_none());
}

#[test]
fn matrix_gate_fixture_records_every_approval_blocker_and_no_executable_fields() {
    let example = parse_fixture(MATRIX_GATE_FIXTURE).unwrap();

    assert_eq!(example.status.as_deref(), Some("TODO_DOMAIN_VALIDATION"));
    assert!(example.trace_ids.as_ref().is_some_and(|trace_ids| trace_ids
        .iter()
        .any(|trace_id| trace_id == LOAD_SW_DIST_TRACE_ID)));
    assert!(example.allowed_target.is_none());
    assert!(example.approval.is_none());
    assert!(example.model.is_none());
    assert!(example.expected.is_empty());
    assert_eq!(
        example.missing_approval_fields,
        MANDATORY_LOAD_SW_LEDGER_FIELDS
            .iter()
            .map(|field| field.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        example.candidate_sources,
        vec![
            "SRC-MATRIX-CH5-LOADS-BETWEEN-NODES".to_string(),
            "SRC-MATRIX-CH5-FIXED-END-EQUIVALENT-LOADS".to_string(),
            "SRC-MATRIX-CH7-WORK-EQUIVALENT-LOADS".to_string(),
        ]
    );
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

#[test]
fn approved_member_weight_quantity_executes_total_weight_check_and_is_stable() {
    let example = parse_fixture(MEMBER_WEIGHT_FIXTURE).unwrap();
    validate_metadata(&example).unwrap();
    assert!(is_execution_eligible(&example).unwrap());

    let first = collect_actuals(&example).unwrap();
    let second = collect_actuals(&example).unwrap();

    assert_eq!(first, second);
    assert_eq!(
        first,
        vec![ActualOutput {
            label: "design_check:QTY-WEIGHT-001:total_weight_kn".to_string(),
            value: 0.153_964_405,
        }]
    );
    execute_fixture(&example).unwrap();
}

#[test]
fn incomplete_member_weight_quantity_approval_fails_closed_before_execution() {
    let cases = [
        (
            MEMBER_WEIGHT_FIXTURE.replace("reviewer = \"Jonnathan\"\n", ""),
            "approval.reviewer",
        ),
        (
            MEMBER_WEIGHT_FIXTURE.replace("date = \"2026-06-29\"\n", ""),
            "approval.date",
        ),
        (
            MEMBER_WEIGHT_FIXTURE.replace("[source]\n", "[missing_source]\n"),
            "source",
        ),
        (
            MEMBER_WEIGHT_FIXTURE.replace("[model]\n", "[missing_model]\n"),
            "model",
        ),
        (
            MEMBER_WEIGHT_FIXTURE
                .replace("allowed_target = \"tower_core_total_weight_check\"\n", ""),
            "allowed_target",
        ),
        (
            MEMBER_WEIGHT_FIXTURE.replace(
                "rationale = \"QTY-WEIGHT-001 deterministic member self-weight quantity\"",
                "rationale = \"\"",
            ),
            "tolerance.rationale",
        ),
    ];

    for (fixture, expected_error) in cases {
        let example = parse_fixture(&fixture).unwrap();
        let error = is_execution_eligible(&example).unwrap_err();
        assert!(
            error.contains(expected_error),
            "expected {expected_error}, got {error}"
        );
    }
}

#[test]
fn member_weight_quantity_rejects_distribution_or_runtime_generation_claims() {
    for forbidden_claim in [
        "approves nodal distribution",
        "approves target nodes",
        "approves signs",
        "approves distribution factors",
        "authorizes runtime self-weight generation",
    ] {
        let fixture = MEMBER_WEIGHT_FIXTURE.replace(
            "quantity-only; no nodal distribution, target nodes, signs, distribution factors, or runtime self-weight generation",
            forbidden_claim,
        );
        let example = parse_fixture(&fixture).unwrap();

        let error = validate_metadata(&example).unwrap_err();

        assert!(
            error.contains("evidence_boundary"),
            "expected boundary rejection for {forbidden_claim}, got {error}"
        );
    }
}
