use serde::Deserialize;

use crate::errors::Result;
use crate::materials::MaterialId;
use crate::units::{LengthMeters, UnitValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct SectionId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    pub id: SectionId,
    pub material_id: MaterialId,
    pub nominal_area_m2: f64,
    pub radius_of_gyration: LengthMeters,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawSection {
    pub id: String,
    pub material_id: String,
    pub nominal_area: UnitValue,
    pub radius_of_gyration: UnitValue,
}

impl RawSection {
    pub fn validate(&self) -> Result<Section> {
        let area = self
            .nominal_area
            .require_unit(format!("sections.{}.nominal_area", self.id), "m2")?;
        if area <= 0.0 {
            return Err(crate::errors::TowerError::NonPositiveValue {
                field: format!("sections.{}.nominal_area", self.id),
                value: area,
            });
        }
        let radius_of_gyration = self
            .radius_of_gyration
            .require_unit(format!("sections.{}.radius_of_gyration", self.id), "m")?;
        if radius_of_gyration <= 0.0 {
            return Err(crate::errors::TowerError::NonPositiveValue {
                field: format!("sections.{}.radius_of_gyration", self.id),
                value: radius_of_gyration,
            });
        }

        Ok(Section {
            id: SectionId(self.id.clone()),
            material_id: MaterialId(self.material_id.clone()),
            nominal_area_m2: area,
            radius_of_gyration: LengthMeters::new(radius_of_gyration)?,
        })
    }
}
