use serde::Deserialize;

use crate::errors::{Result, TowerError};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LengthMeters(f64);

impl LengthMeters {
    pub fn new(value: f64) -> Result<Self> {
        ensure_finite("length_m", value)?;
        Ok(Self(value))
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ForceKilonewtons(f64);

impl ForceKilonewtons {
    pub fn new(value: f64) -> Result<Self> {
        ensure_finite("force_kn", value)?;
        Ok(Self(value))
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct StressMPa(f64);

impl StressMPa {
    pub fn new(value: f64) -> Result<Self> {
        ensure_positive("stress_mpa", value)?;
        Ok(Self(value))
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MassKilograms(f64);

impl MassKilograms {
    pub fn new(value: f64) -> Result<Self> {
        ensure_non_negative("mass_kg", value)?;
        Ok(Self(value))
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DensityKgPerCubicMeter(f64);

impl DensityKgPerCubicMeter {
    pub fn new(value: f64) -> Result<Self> {
        ensure_positive("density_kg_per_m3", value)?;
        Ok(Self(value))
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnitValue {
    pub value: f64,
    pub unit: Option<String>,
}

impl UnitValue {
    pub fn require_unit(&self, field: impl Into<String>, expected: &str) -> Result<f64> {
        let field = field.into();
        ensure_finite(&field, self.value)?;

        match self.unit.as_deref() {
            None | Some("") => Err(TowerError::MissingUnit {
                field,
                expected: expected.to_string(),
            }),
            Some(found) if found == expected => Ok(self.value),
            Some(found) => Err(TowerError::AmbiguousUnit {
                field,
                found: found.to_string(),
                expected: expected.to_string(),
            }),
        }
    }
}

fn ensure_finite(field: &str, value: f64) -> Result<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(TowerError::NonFiniteValue {
            field: field.to_string(),
            value,
        })
    }
}

fn ensure_positive(field: &str, value: f64) -> Result<()> {
    ensure_finite(field, value)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(TowerError::NonPositiveValue {
            field: field.to_string(),
            value,
        })
    }
}

fn ensure_non_negative(field: &str, value: f64) -> Result<()> {
    ensure_finite(field, value)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(TowerError::NegativeValue {
            field: field.to_string(),
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_unit() {
        let value = UnitValue {
            value: 1.0,
            unit: None,
        };

        assert!(matches!(
            value.require_unit("node.x", "m"),
            Err(TowerError::MissingUnit { .. })
        ));
    }

    #[test]
    fn rejects_ambiguous_unit() {
        let value = UnitValue {
            value: 1.0,
            unit: Some("cm".to_string()),
        };

        assert!(matches!(
            value.require_unit("node.x", "m"),
            Err(TowerError::AmbiguousUnit { .. })
        ));
    }
}
