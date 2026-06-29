use serde::Deserialize;

use crate::errors::Result;
use crate::geometry::NodeId;
use crate::units::{ForceKilonewtons, UnitValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct LoadCaseId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct NodalLoad {
    pub node_id: NodeId,
    pub fx: ForceKilonewtons,
    pub fy: ForceKilonewtons,
    pub fz: ForceKilonewtons,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadCase {
    pub id: LoadCaseId,
    pub nodal_loads: Vec<NodalLoad>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawLoadCase {
    pub id: String,
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
    pub fn validate(&self) -> Result<LoadCase> {
        Ok(LoadCase {
            id: LoadCaseId(self.id.clone()),
            nodal_loads: self
                .nodal_loads
                .iter()
                .map(|load| load.validate(&self.id))
                .collect::<Result<Vec<_>>>()?,
        })
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
