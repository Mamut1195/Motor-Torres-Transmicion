use crate::analysis::AnalysisResult;
use crate::errors::{Result, TowerError};
use crate::materials::MaterialId;
use crate::model::{Member, Node, TowerModel};
use crate::sections::Section;

pub fn design_checks_not_available() -> Result<()> {
    Err(TowerError::BlockedDomainFeature {
        feature: "normative design checks",
    })
}

const STANDARD_GRAVITY_M_PER_S2: f64 = 9.806_65;
const VALIDATED_TOTAL_WEIGHT_TRACE_ID: &str = "QTY-WEIGHT-001";
const VALIDATED_TENSION_TRACE_ID: &str = "CHK-TENSION-001";
const VALIDATED_COMPRESSION_TRACE_ID: &str = "CHK-COMPRESSION-001";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormulaStatus {
    Validated,
    Pending,
    Provisional,
    TodoDomainValidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    Pass,
    Fail,
    Blocked,
    TodoDomainValidation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckTrace {
    pub formula_id: String,
    pub validation_status: FormulaStatus,
    pub inputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub rule_id: String,
    pub status: CheckStatus,
    pub value: Option<f64>,
    pub trace: CheckTrace,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckRule {
    TotalWeight {
        trace_id: &'static str,
        formula_status: FormulaStatus,
    },
    Tension {
        trace_id: &'static str,
        formula_status: FormulaStatus,
    },
    Compression {
        trace_id: &'static str,
        formula_status: FormulaStatus,
    },
    Slenderness {
        trace_id: &'static str,
        formula_status: FormulaStatus,
    },
    Displacement {
        trace_id: &'static str,
        formula_status: FormulaStatus,
    },
}

pub struct CheckEngine;

impl CheckEngine {
    pub fn run(
        model: &TowerModel,
        analysis: &AnalysisResult,
        rules: &[CheckRule],
    ) -> Result<Vec<CheckResult>> {
        rules
            .iter()
            .map(|rule| match rule {
                CheckRule::TotalWeight {
                    trace_id,
                    formula_status,
                } => total_weight_check(model, trace_id, *formula_status),
                CheckRule::Tension {
                    trace_id,
                    formula_status,
                } => axial_stress_check(
                    model,
                    analysis,
                    trace_id,
                    *formula_status,
                    AxialClassification::Tension,
                ),
                CheckRule::Compression {
                    trace_id,
                    formula_status,
                } => axial_stress_check(
                    model,
                    analysis,
                    trace_id,
                    *formula_status,
                    AxialClassification::Compression,
                ),
                CheckRule::Slenderness {
                    trace_id,
                    formula_status,
                }
                | CheckRule::Displacement {
                    trace_id,
                    formula_status,
                } => Ok(blocked_normative_check(trace_id, *formula_status)),
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AxialClassification {
    Tension,
    Compression,
}

impl AxialClassification {
    fn validated_trace_id(self) -> &'static str {
        match self {
            Self::Tension => VALIDATED_TENSION_TRACE_ID,
            Self::Compression => VALIDATED_COMPRESSION_TRACE_ID,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Tension => "tension axial stress utilization",
            Self::Compression => "compression axial stress utilization",
        }
    }

    fn example_id(self) -> &'static str {
        match self {
            Self::Tension => "example_06_tension_axial_stress_utilization",
            Self::Compression => "example_07_compression_axial_stress_utilization",
        }
    }

    fn is_active(self, axial_kn: f64) -> bool {
        match self {
            Self::Tension => axial_kn >= 0.0,
            Self::Compression => axial_kn < 0.0,
        }
    }
}

fn total_weight_check(
    model: &TowerModel,
    trace_id: &str,
    formula_status: FormulaStatus,
) -> Result<CheckResult> {
    if formula_status != FormulaStatus::Validated {
        return Ok(blocked_check(trace_id, formula_status));
    }
    if trace_id != VALIDATED_TOTAL_WEIGHT_TRACE_ID {
        return Ok(blocked_total_weight_trace_check(trace_id, formula_status));
    }

    let mut total_weight_kn = 0.0;
    let mut inputs = Vec::new();
    for member in &model.members {
        let section = section_for_member(model, member)?;
        let density = density_for_section(model, &section.material_id)?;
        let length_m = member_length(model, member)?;
        total_weight_kn +=
            density * section.nominal_area_m2 * length_m * STANDARD_GRAVITY_M_PER_S2 / 1_000.0;
        inputs.push(format!("{}.length_m", member.id.0));
        inputs.push(format!("{}.nominal_area_m2", section.id.0));
        inputs.push(format!("{}.density_kg_per_m3", section.material_id.0));
    }

    Ok(CheckResult {
        rule_id: trace_id.to_string(),
        status: CheckStatus::Pass,
        value: Some(total_weight_kn),
        trace: CheckTrace {
            formula_id: trace_id.to_string(),
            validation_status: formula_status,
            inputs,
        },
        message: format!("{trace_id} computed total member self-weight in kN"),
    })
}

fn axial_stress_check(
    model: &TowerModel,
    analysis: &AnalysisResult,
    trace_id: &str,
    formula_status: FormulaStatus,
    classification: AxialClassification,
) -> Result<CheckResult> {
    if formula_status != FormulaStatus::Validated {
        return Ok(blocked_check(trace_id, formula_status));
    }
    if trace_id != classification.validated_trace_id() {
        return Ok(blocked_axial_trace_check(
            trace_id,
            formula_status,
            classification,
        ));
    }

    let mut controlling_utilization = 0.0;
    let mut active_members = Vec::new();
    let mut inputs = Vec::new();

    for member in &model.members {
        let Some(force) = analysis.member_force(&member.id.0) else {
            return Ok(blocked_missing_force_check(
                trace_id,
                formula_status,
                &member.id.0,
            ));
        };

        if !classification.is_active(force.axial_kn) {
            continue;
        }

        let section = section_for_member(model, member)?;
        let yield_stress_mpa = yield_stress_for_section(model, &section.material_id)?;
        let utilization =
            axial_stress_utilization(force.axial_kn, section.nominal_area_m2, yield_stress_mpa);
        if utilization > controlling_utilization {
            controlling_utilization = utilization;
        }
        active_members.push(member.id.0.clone());
        inputs.push(format!("{}.axial_force_kN", member.id.0));
        inputs.push(format!("{}.nominal_area_m2", section.id.0));
        inputs.push(format!("{}.yield_stress_MPa", section.material_id.0));
    }

    inputs.push("stress_mpa = abs(kN) / m2 / 1000".to_string());

    let message = if active_members.is_empty() {
        format!(
            "{trace_id} has no active {} demand; utilization is 0.000000",
            classification.label()
        )
    } else {
        format!(
            "{trace_id} {} for {} is {:.6} using {}",
            classification.label(),
            active_members.join(", "),
            controlling_utilization,
            classification.example_id()
        )
    };

    Ok(CheckResult {
        rule_id: trace_id.to_string(),
        status: if controlling_utilization <= 1.0 {
            CheckStatus::Pass
        } else {
            CheckStatus::Fail
        },
        value: Some(controlling_utilization),
        trace: CheckTrace {
            formula_id: trace_id.to_string(),
            validation_status: formula_status,
            inputs,
        },
        message,
    })
}

fn axial_stress_utilization(axial_kn: f64, nominal_area_m2: f64, yield_stress_mpa: f64) -> f64 {
    (axial_kn.abs() / nominal_area_m2 / 1000.0) / yield_stress_mpa
}

fn blocked_normative_check(trace_id: &str, formula_status: FormulaStatus) -> CheckResult {
    CheckResult {
        rule_id: trace_id.to_string(),
        status: CheckStatus::TodoDomainValidation,
        value: None,
        trace: CheckTrace {
            formula_id: trace_id.to_string(),
            validation_status: formula_status,
            inputs: Vec::new(),
        },
        message: format!("{trace_id} is blocked until formula-register validation is complete"),
    }
}

fn blocked_check(trace_id: &str, formula_status: FormulaStatus) -> CheckResult {
    CheckResult {
        rule_id: trace_id.to_string(),
        status: CheckStatus::Blocked,
        value: None,
        trace: CheckTrace {
            formula_id: trace_id.to_string(),
            validation_status: formula_status,
            inputs: Vec::new(),
        },
        message: format!("{trace_id} is blocked until formula-register validation is complete"),
    }
}

fn blocked_total_weight_trace_check(trace_id: &str, formula_status: FormulaStatus) -> CheckResult {
    CheckResult {
        rule_id: trace_id.to_string(),
        status: CheckStatus::Blocked,
        value: None,
        trace: CheckTrace {
            formula_id: trace_id.to_string(),
            validation_status: formula_status,
            inputs: Vec::new(),
        },
        message: format!(
            "{trace_id} is blocked because only {VALIDATED_TOTAL_WEIGHT_TRACE_ID} is validated for total member self-weight"
        ),
    }
}

fn blocked_axial_trace_check(
    trace_id: &str,
    formula_status: FormulaStatus,
    classification: AxialClassification,
) -> CheckResult {
    CheckResult {
        rule_id: trace_id.to_string(),
        status: CheckStatus::Blocked,
        value: None,
        trace: CheckTrace {
            formula_id: trace_id.to_string(),
            validation_status: formula_status,
            inputs: Vec::new(),
        },
        message: format!(
            "{trace_id} is blocked because only {} is validated for {}",
            classification.validated_trace_id(),
            classification.label()
        ),
    }
}

fn blocked_missing_force_check(
    trace_id: &str,
    formula_status: FormulaStatus,
    member_id: &str,
) -> CheckResult {
    CheckResult {
        rule_id: trace_id.to_string(),
        status: CheckStatus::Blocked,
        value: None,
        trace: CheckTrace {
            formula_id: trace_id.to_string(),
            validation_status: formula_status,
            inputs: Vec::new(),
        },
        message: format!(
            "{trace_id} is blocked because member {member_id} has missing axial force"
        ),
    }
}

fn section_for_member<'a>(model: &'a TowerModel, member: &Member) -> Result<&'a Section> {
    model
        .sections
        .iter()
        .find(|section| section.id == member.section_id)
        .ok_or_else(|| TowerError::UnknownReference {
            field: "member.section_id".to_string(),
            id: member.section_id.0.clone(),
        })
}

fn density_for_section(model: &TowerModel, material_id: &MaterialId) -> Result<f64> {
    model
        .materials
        .iter()
        .find(|material| material.id == *material_id)
        .map(|material| material.density.get())
        .ok_or_else(|| TowerError::UnknownReference {
            field: "section.material_id".to_string(),
            id: material_id.0.clone(),
        })
}

fn yield_stress_for_section(model: &TowerModel, material_id: &MaterialId) -> Result<f64> {
    model
        .materials
        .iter()
        .find(|material| material.id == *material_id)
        .map(|material| material.yield_stress.get())
        .ok_or_else(|| TowerError::UnknownReference {
            field: "section.material_id".to_string(),
            id: material_id.0.clone(),
        })
}

fn member_length(model: &TowerModel, member: &Member) -> Result<f64> {
    let start = node_by_id(model, &member.start)?;
    let end = node_by_id(model, &member.end)?;

    Ok(start.point.distance_to(&end.point)?.get())
}

fn node_by_id<'a>(model: &'a TowerModel, id: &crate::geometry::NodeId) -> Result<&'a Node> {
    model
        .nodes
        .iter()
        .find(|node| node.id == *id)
        .ok_or_else(|| TowerError::UnknownReference {
            field: "member.node_id".to_string(),
            id: id.0.clone(),
        })
}
