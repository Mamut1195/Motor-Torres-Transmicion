use serde::Deserialize;

use crate::errors::Result;
use crate::geometry::NodeId;
use crate::units::{ForceKilonewtons, UnitValue};

fn default_load_provenance_status() -> LoadProvenanceStatus {
    LoadProvenanceStatus::ExplicitUserInput
}

fn default_load_source() -> String {
    "user input".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct LoadCaseId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct NodalLoad {
    pub node_id: NodeId,
    pub fx: ForceKilonewtons,
    pub fy: ForceKilonewtons,
    pub fz: ForceKilonewtons,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum LoadProvenanceStatus {
    #[serde(rename = "explicit_user_input")]
    ExplicitUserInput,
    #[serde(rename = "validated_quantity")]
    ValidatedQuantity,
    #[serde(rename = "candidate_provisional")]
    CandidateProvisional,
    #[serde(rename = "TODO_DOMAIN_VALIDATION")]
    TodoDomainValidation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadCase {
    pub id: LoadCaseId,
    pub nodal_loads: Vec<NodalLoad>,
    pub status: LoadProvenanceStatus,
    pub source: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawLoadCase {
    pub id: String,
    #[serde(default = "default_load_provenance_status")]
    pub status: LoadProvenanceStatus,
    #[serde(default = "default_load_source")]
    pub source: String,
    #[serde(default)]
    pub nodal_loads: Vec<RawNodalLoad>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawNodalLoad {
    pub node_id: String,
    pub fx: UnitValue,
    pub fy: UnitValue,
    pub fz: UnitValue,
}

impl RawLoadCase {
    pub fn validate(&self) -> Result<Option<LoadCase>> {
        if self.status != LoadProvenanceStatus::ExplicitUserInput {
            return Ok(None);
        }

        Ok(Some(LoadCase {
            id: LoadCaseId(self.id.clone()),
            status: self.status,
            source: self.source.clone(),
            nodal_loads: self
                .nodal_loads
                .iter()
                .map(|load| load.validate(&self.id))
                .collect::<Result<Vec<_>>>()?,
        }))
    }
}

impl RawNodalLoad {
    fn validate(&self, case_id: &str) -> Result<NodalLoad> {
        Ok(NodalLoad {
            node_id: NodeId(self.node_id.clone()),
            fx: ForceKilonewtons::new(
                self.fx
                    .require_unit(format!("load_cases.{case_id}.{}.fx", self.node_id), "kN")?,
            )?,
            fy: ForceKilonewtons::new(
                self.fy
                    .require_unit(format!("load_cases.{case_id}.{}.fy", self.node_id), "kN")?,
            )?,
            fz: ForceKilonewtons::new(
                self.fz
                    .require_unit(format!("load_cases.{case_id}.{}.fz", self.node_id), "kN")?,
            )?,
        })
    }
}
