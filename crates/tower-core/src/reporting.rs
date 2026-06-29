use crate::design_checks::{CheckResult, CheckStatus, FormulaStatus};
use crate::errors::TowerError;
use crate::loads::{LoadCase, LoadProvenanceStatus};
use crate::optimization::OptimizationResult;

pub const ENGINEERING_DISCLAIMER: &str = "not for final engineering design";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreliminaryReport {
    title: String,
    lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadEvidence {
    id: String,
    label: String,
    status: LoadProvenanceStatus,
    source: String,
    note: String,
}

impl LoadEvidence {
    pub fn explicit_load_case(load_case: &LoadCase) -> Self {
        Self {
            id: load_case.id.0.clone(),
            label: "explicit nodal load case".to_string(),
            status: load_case.status,
            source: load_case.source.clone(),
            note: format!(
                "nodal loads: {} explicit user input record(s)",
                load_case.nodal_loads.len()
            ),
        }
    }

    pub fn validated_quantity(
        id: impl Into<String>,
        label: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            status: LoadProvenanceStatus::ValidatedQuantity,
            source: source.into(),
            note: "No generated nodal loads are created from this evidence.".to_string(),
        }
    }

    pub fn todo_domain_validation(label: impl Into<String>, reason: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            id: label.clone(),
            label,
            status: LoadProvenanceStatus::TodoDomainValidation,
            source: "blocked pending reviewer-approved evidence".to_string(),
            note: reason.into(),
        }
    }
}

pub fn blocked_load_model_evidence() -> Vec<LoadEvidence> {
    vec![
        LoadEvidence::todo_domain_validation(
            "self-weight to nodal load generation",
            "blocked until a reviewer-approved lumping/modeling assumption exists",
        ),
        LoadEvidence::todo_domain_validation(
            "wind loads",
            "blocked until reviewer-approved wind loading clauses and assumptions exist",
        ),
        LoadEvidence::todo_domain_validation(
            "conductor loads",
            "blocked until reviewer-approved conductor loading clauses and assumptions exist",
        ),
        LoadEvidence::todo_domain_validation(
            "load combinations",
            "blocked until reviewer-approved load combination rules exist",
        ),
        LoadEvidence::todo_domain_validation(
            "load factors",
            "blocked until reviewer-approved load factor rules exist",
        ),
        LoadEvidence::todo_domain_validation(
            "displacement or design-level loading",
            "blocked until reviewer-approved serviceability/design loading evidence exists",
        ),
    ]
}

impl PreliminaryReport {
    pub fn from_checks(
        title: impl Into<String>,
        checks: &[CheckResult],
        optimization: Option<&OptimizationResult>,
    ) -> Self {
        let mut lines = base_lines(title.into(), "Completed preliminary run");
        lines.push("This report does not establish code compliance.".to_string());
        lines.push("Checks:".to_string());
        if checks.is_empty() {
            lines.push("- No checks were executed in this report scope.".to_string());
        } else {
            for check in checks {
                let value = check
                    .value
                    .map(|value| format!("; value: {value}"))
                    .unwrap_or_default();
                let inputs = if check.trace.inputs.is_empty() {
                    String::new()
                } else {
                    format!("; inputs: {}", check.trace.inputs.join(", "))
                };
                lines.push(format!(
                    "- {}: {} [{}] — {}",
                    check.rule_id,
                    check_status_label(check.status),
                    formula_status_label(check.trace.validation_status),
                    format_args!("{}{}{}", check.message, value, inputs)
                ));
            }
        }

        let validation_gaps = checks
            .iter()
            .filter(|check| is_validation_gap(check))
            .collect::<Vec<_>>();
        if !validation_gaps.is_empty() {
            lines.push("Validation gaps:".to_string());
            for check in validation_gaps {
                lines.push(format!(
                    "- {} remains {} with formula status {}",
                    check.trace.formula_id,
                    check_status_label(check.status),
                    formula_status_label(check.trace.validation_status)
                ));
            }
        }

        append_optimization(&mut lines, optimization);

        Self {
            title: lines[0].clone(),
            lines,
        }
    }

    pub fn from_error(title: impl Into<String>, error: &TowerError) -> Self {
        let mut lines = base_lines(title.into(), "Failed run");
        lines.push("Failure details:".to_string());
        lines.push(format!("- {error}"));
        lines.push("This is not final-design approval.".to_string());
        Self {
            title: lines[0].clone(),
            lines,
        }
    }

    pub fn from_load_evidence(title: impl Into<String>, evidence: &[LoadEvidence]) -> Self {
        let mut lines = base_lines(title.into(), "Completed load evidence report");
        lines.push("This report does not establish code compliance.".to_string());
        lines.push("Load model evidence:".to_string());
        if evidence.is_empty() {
            lines.push("- No load evidence records were provided.".to_string());
        } else {
            for record in evidence {
                lines.push(format!(
                    "- {}: {} [{}]; source: {}; {}",
                    record.id,
                    record.label,
                    load_status_label(record.status),
                    record.source,
                    record.note
                ));
            }
        }

        Self {
            title: lines[0].clone(),
            lines,
        }
    }

    pub fn render(&self) -> String {
        self.lines.join("\n")
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

fn base_lines(title: String, outcome: &str) -> Vec<String> {
    vec![
        format!("Report: {title}"),
        format!("Disclaimer: {ENGINEERING_DISCLAIMER}"),
        format!("Outcome: {outcome}"),
        "Boundary: preliminary trace report; validation gaps and unresolved checks remain visible."
            .to_string(),
    ]
}

fn append_optimization(lines: &mut Vec<String>, optimization: Option<&OptimizationResult>) {
    match optimization {
        Some(OptimizationResult::Feasible(assignment)) => {
            lines.push("Optimization status: feasible".to_string());
            lines.push(format!("- selected section: {}", assignment.section_id.0));
            lines.push(format!("- weight_kg_per_m: {}", assignment.weight_kg_per_m));
            lines.push(format!(
                "- controlling checks: {}",
                assignment.controlling_checks.join(", ")
            ));
        }
        Some(OptimizationResult::Infeasible(infeasible)) => {
            lines.push("Optimization status: infeasible".to_string());
            lines.push(format!(
                "- failed candidates: {}",
                infeasible.failed_candidates.join(", ")
            ));
            lines.push("- unresolved checks remain visible if present.".to_string());
            append_unresolved(lines, &infeasible.unresolved_constraints);
        }
        Some(OptimizationResult::Blocked(blocked)) => {
            lines.push("Optimization status: blocked".to_string());
            lines.push(format!(
                "- blocked candidates: {}",
                blocked.blocked_candidates.join(", ")
            ));
            lines.push("- unresolved checks remain visible.".to_string());
            append_unresolved(lines, &blocked.unresolved_constraints);
        }
        None => lines.push("Optimization status: not run".to_string()),
    }
}

fn append_unresolved(lines: &mut Vec<String>, unresolved_constraints: &[String]) {
    if unresolved_constraints.is_empty() {
        lines.push("- unresolved constraints: none".to_string());
    } else {
        lines.push(format!(
            "- unresolved constraints: {}",
            unresolved_constraints.join(", ")
        ));
    }
}

fn is_validation_gap(check: &CheckResult) -> bool {
    check.trace.validation_status != FormulaStatus::Validated
        || matches!(
            check.status,
            CheckStatus::Blocked | CheckStatus::TodoDomainValidation
        )
}

fn check_status_label(status: CheckStatus) -> &'static str {
    match status {
        CheckStatus::Pass => "PASS",
        CheckStatus::Fail => "FAIL",
        CheckStatus::Blocked => "BLOCKED",
        CheckStatus::TodoDomainValidation => "TODO_DOMAIN_VALIDATION",
    }
}

fn formula_status_label(status: FormulaStatus) -> &'static str {
    match status {
        FormulaStatus::Validated => "VALIDATED",
        FormulaStatus::Pending => "PENDING",
        FormulaStatus::Provisional => "PROVISIONAL",
        FormulaStatus::TodoDomainValidation => "TODO_DOMAIN_VALIDATION",
    }
}

fn load_status_label(status: LoadProvenanceStatus) -> &'static str {
    match status {
        LoadProvenanceStatus::ExplicitUserInput => "explicit_user_input",
        LoadProvenanceStatus::ValidatedQuantity => "validated_quantity",
        LoadProvenanceStatus::CandidateProvisional => "candidate_provisional",
        LoadProvenanceStatus::TodoDomainValidation => "TODO_DOMAIN_VALIDATION",
    }
}
