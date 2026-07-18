use crate::errors::{Result, TowerError};
use crate::loads::{generated_self_weight_load_case, LoadCase, NodalLoad};
use crate::materials::MaterialId;
use crate::model::{Member, Node, TowerModel};
use crate::sections::Section;
use crate::units::ForceKilonewtons;

pub const STANDARD_GRAVITY_M_PER_S2: f64 = 9.806_65;

pub fn member_self_weight_kn(density_kg_m3: f64, area_m2: f64, length_m: f64) -> f64 {
    density_kg_m3 * area_m2 * length_m * STANDARD_GRAVITY_M_PER_S2 / 1_000.0
}

pub fn generated_self_weight_load_case_for_model(model: &TowerModel) -> Result<Option<LoadCase>> {
    if model.members.is_empty() {
        return Ok(None);
    }

    let mut nodal_loads = Vec::with_capacity(model.members.len() * 2);
    for member in &model.members {
        let section = section_for_member(model, member)?;
        let density = density_for_section(model, &section.material_id)?;
        let length_m = member_length(model, member)?;
        let total_weight_kn = member_self_weight_kn(density, section.nominal_area_m2, length_m);
        let endpoint_fz = ForceKilonewtons::new(-total_weight_kn / 2.0)?;

        nodal_loads.push(NodalLoad {
            node_id: member.start.clone(),
            fx: ForceKilonewtons::new(0.0)?,
            fy: ForceKilonewtons::new(0.0)?,
            fz: endpoint_fz,
        });
        nodal_loads.push(NodalLoad {
            node_id: member.end.clone(),
            fx: ForceKilonewtons::new(0.0)?,
            fy: ForceKilonewtons::new(0.0)?,
            fz: endpoint_fz,
        });
    }

    Ok(Some(generated_self_weight_load_case(nodal_loads)))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_self_weight_reuses_qty_weight_arithmetic() {
        assert_eq!(member_self_weight_kn(7850.0, 0.001, 2.0), 0.153_964_405);
    }
}
