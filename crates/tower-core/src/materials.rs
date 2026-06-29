use serde::Deserialize;

use crate::errors::Result;
use crate::units::{DensityKgPerCubicMeter, StressMPa, UnitValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct MaterialId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub id: MaterialId,
    pub density: DensityKgPerCubicMeter,
    pub yield_stress: StressMPa,
    pub elastic_modulus_kn_per_m2: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawMaterial {
    pub id: String,
    pub density: UnitValue,
    pub yield_stress: UnitValue,
    pub elastic_modulus: Option<UnitValue>,
}

impl RawMaterial {
    pub fn validate(&self) -> Result<Material> {
        Ok(Material {
            id: MaterialId(self.id.clone()),
            density: DensityKgPerCubicMeter::new(
                self.density
                    .require_unit(format!("materials.{}.density", self.id), "kg/m3")?,
            )?,
            yield_stress: StressMPa::new(
                self.yield_stress
                    .require_unit(format!("materials.{}.yield_stress", self.id), "MPa")?,
            )?,
            elastic_modulus_kn_per_m2: self
                .elastic_modulus
                .as_ref()
                .map(|elastic_modulus| {
                    elastic_modulus
                        .require_unit(format!("materials.{}.elastic_modulus", self.id), "kN/m2")
                })
                .transpose()?,
        })
    }
}
