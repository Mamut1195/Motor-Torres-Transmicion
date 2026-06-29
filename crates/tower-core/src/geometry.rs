use serde::Deserialize;

use crate::errors::Result;
use crate::units::{LengthMeters, UnitValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct MemberId(pub String);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: LengthMeters,
    pub y: LengthMeters,
    pub z: LengthMeters,
}

impl Point3 {
    pub fn new(x: LengthMeters, y: LengthMeters, z: LengthMeters) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Self) -> Result<LengthMeters> {
        let dx = other.x.get() - self.x.get();
        let dy = other.y.get() - self.y.get();
        let dz = other.z.get() - self.z.get();
        LengthMeters::new((dx.mul_add(dx, dy.mul_add(dy, dz * dz))).sqrt())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawPoint3 {
    pub x: UnitValue,
    pub y: UnitValue,
    pub z: UnitValue,
}

impl RawPoint3 {
    pub fn validate(&self, field: &str) -> Result<Point3> {
        Ok(Point3::new(
            LengthMeters::new(self.x.require_unit(format!("{field}.x"), "m")?)?,
            LengthMeters::new(self.y.require_unit(format!("{field}.y"), "m")?)?,
            LengthMeters::new(self.z.require_unit(format!("{field}.z"), "m")?)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_member_length_in_meters() {
        let a = Point3::new(
            LengthMeters::new(0.1).unwrap(),
            LengthMeters::new(0.1).unwrap(),
            LengthMeters::new(0.1).unwrap(),
        );
        let b = Point3::new(
            LengthMeters::new(3.1).unwrap(),
            LengthMeters::new(4.1).unwrap(),
            LengthMeters::new(0.1).unwrap(),
        );

        assert_eq!(a.distance_to(&b).unwrap().get(), 5.0);
    }
}
