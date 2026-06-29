use std::collections::HashMap;

use crate::errors::{Result, TowerError};
use crate::geometry::{MemberId, NodeId};
use crate::loads::{LoadCase, LoadCaseId};
use crate::model::{Member, Node, Support, TowerModel};
use crate::sections::SectionId;

pub fn solve_not_available() -> Result<()> {
    Err(TowerError::BlockedDomainFeature {
        feature: "3D truss solver",
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodalDisplacement {
    pub node_id: NodeId,
    pub ux_m: f64,
    pub uy_m: f64,
    pub uz_m: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportReaction {
    pub node_id: NodeId,
    pub fx_kn: f64,
    pub fy_kn: f64,
    pub fz_kn: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberAxialForce {
    pub member_id: MemberId,
    pub axial_kn: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisResult {
    pub displacements: Vec<NodalDisplacement>,
    pub reactions: Vec<SupportReaction>,
    pub member_forces: Vec<MemberAxialForce>,
}

impl AnalysisResult {
    pub fn displacement(&self, node_id: &str) -> Option<&NodalDisplacement> {
        self.displacements
            .iter()
            .find(|displacement| displacement.node_id.0 == node_id)
    }

    pub fn reaction(&self, node_id: &str) -> Option<&SupportReaction> {
        self.reactions
            .iter()
            .find(|reaction| reaction.node_id.0 == node_id)
    }

    pub fn member_force(&self, member_id: &str) -> Option<&MemberAxialForce> {
        self.member_forces
            .iter()
            .find(|force| force.member_id.0 == member_id)
    }
}

pub struct TrussSolver;

impl TrussSolver {
    pub fn solve(model: &TowerModel, load_case_id: &LoadCaseId) -> Result<AnalysisResult> {
        let load_case = model
            .load_cases
            .iter()
            .find(|load_case| load_case.id == *load_case_id)
            .ok_or_else(|| TowerError::UnknownReference {
                field: "load_case_id".to_string(),
                id: load_case_id.0.clone(),
            })?;

        let node_index = model
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.id.0.as_str(), index))
            .collect::<HashMap<_, _>>();
        let dof_count = model.nodes.len() * 3;
        let mut stiffness = vec![vec![0.0; dof_count]; dof_count];
        let mut loads = vec![0.0; dof_count];

        assemble_global_stiffness(model, &node_index, &mut stiffness)?;
        assemble_loads(load_case, &node_index, &mut loads);

        let restrained = restrained_dofs(model, &node_index, dof_count);
        let active_dofs = (0..dof_count)
            .filter(|dof| !restrained[*dof])
            .collect::<Vec<_>>();
        if active_dofs.is_empty() {
            return Err(TowerError::UnstableModel {
                reason: "no active DOF remains after support application".to_string(),
            });
        }

        let reduced_stiffness = active_dofs
            .iter()
            .map(|row| {
                active_dofs
                    .iter()
                    .map(|column| stiffness[*row][*column])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let reduced_loads = active_dofs
            .iter()
            .map(|dof| loads[*dof])
            .collect::<Vec<_>>();

        let reduced_displacements =
            solve_linear_system(reduced_stiffness, reduced_loads, |index| {
                dof_label(model, active_dofs[index])
            })?;
        let mut displacements = vec![0.0; dof_count];
        for (reduced_index, dof) in active_dofs.iter().enumerate() {
            displacements[*dof] = reduced_displacements[reduced_index];
        }

        let reactions = multiply_matrix_vector(&stiffness, &displacements)
            .into_iter()
            .zip(loads.iter())
            .map(|(internal_force, applied_load)| internal_force - applied_load)
            .collect::<Vec<_>>();

        Ok(AnalysisResult {
            displacements: model
                .nodes
                .iter()
                .enumerate()
                .map(|(index, node)| NodalDisplacement {
                    node_id: node.id.clone(),
                    ux_m: displacements[dof(index, 0)],
                    uy_m: displacements[dof(index, 1)],
                    uz_m: displacements[dof(index, 2)],
                })
                .collect(),
            reactions: model
                .supports
                .iter()
                .map(|support| {
                    let index = node_index[support.node_id.0.as_str()];
                    SupportReaction {
                        node_id: support.node_id.clone(),
                        fx_kn: reactions[dof(index, 0)],
                        fy_kn: reactions[dof(index, 1)],
                        fz_kn: reactions[dof(index, 2)],
                    }
                })
                .collect(),
            member_forces: model
                .members
                .iter()
                .map(|member| member_axial_force(model, member, &node_index, &displacements))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

fn assemble_global_stiffness(
    model: &TowerModel,
    node_index: &HashMap<&str, usize>,
    stiffness: &mut [Vec<f64>],
) -> Result<()> {
    for member in &model.members {
        let start_index = node_index[member.start.0.as_str()];
        let end_index = node_index[member.end.0.as_str()];
        let (length, direction) =
            member_geometry(&model.nodes[start_index], &model.nodes[end_index])?;
        let axial_stiffness = member_axial_stiffness(model, &member.section_id, length)?;

        for i in 0..3 {
            for j in 0..3 {
                let value = axial_stiffness * direction[i] * direction[j];
                stiffness[dof(start_index, i)][dof(start_index, j)] += value;
                stiffness[dof(end_index, i)][dof(end_index, j)] += value;
                stiffness[dof(start_index, i)][dof(end_index, j)] -= value;
                stiffness[dof(end_index, i)][dof(start_index, j)] -= value;
            }
        }
    }

    Ok(())
}

fn assemble_loads(load_case: &LoadCase, node_index: &HashMap<&str, usize>, loads: &mut [f64]) {
    for nodal_load in &load_case.nodal_loads {
        let index = node_index[nodal_load.node_id.0.as_str()];
        loads[dof(index, 0)] += nodal_load.fx.get();
        loads[dof(index, 1)] += nodal_load.fy.get();
        loads[dof(index, 2)] += nodal_load.fz.get();
    }
}

fn restrained_dofs(
    model: &TowerModel,
    node_index: &HashMap<&str, usize>,
    dof_count: usize,
) -> Vec<bool> {
    let mut restrained = vec![false; dof_count];
    for Support {
        node_id,
        ux,
        uy,
        uz,
    } in &model.supports
    {
        let index = node_index[node_id.0.as_str()];
        restrained[dof(index, 0)] = *ux;
        restrained[dof(index, 1)] = *uy;
        restrained[dof(index, 2)] = *uz;
    }
    restrained
}

fn member_geometry(start: &Node, end: &Node) -> Result<(f64, [f64; 3])> {
    let dx = end.point.x.get() - start.point.x.get();
    let dy = end.point.y.get() - start.point.y.get();
    let dz = end.point.z.get() - start.point.z.get();
    let length = (dx.mul_add(dx, dy.mul_add(dy, dz * dz))).sqrt();
    if length <= 1.0e-12 {
        return Err(TowerError::InvalidAnalysisModel {
            reason: format!("zero-length member between {} and {}", start.id.0, end.id.0),
        });
    }

    Ok((length, [dx / length, dy / length, dz / length]))
}

fn member_axial_stiffness(
    model: &TowerModel,
    section_id: &SectionId,
    length_m: f64,
) -> Result<f64> {
    let section = model
        .sections
        .iter()
        .find(|section| section.id == *section_id)
        .ok_or_else(|| TowerError::UnknownReference {
            field: "member.section_id".to_string(),
            id: section_id.0.clone(),
        })?;
    let material = model
        .materials
        .iter()
        .find(|material| material.id == section.material_id)
        .ok_or_else(|| TowerError::UnknownReference {
            field: "section.material_id".to_string(),
            id: section.material_id.0.clone(),
        })?;
    let elastic_modulus =
        material
            .elastic_modulus_kn_per_m2
            .ok_or_else(|| TowerError::InvalidAnalysisModel {
                reason: format!(
                    "material {} requires elastic_modulus for 3D truss analysis",
                    material.id.0
                ),
            })?;

    Ok(elastic_modulus * section.nominal_area_m2 / length_m)
}

fn member_axial_force(
    model: &TowerModel,
    member: &Member,
    node_index: &HashMap<&str, usize>,
    displacements: &[f64],
) -> Result<MemberAxialForce> {
    let start_index = node_index[member.start.0.as_str()];
    let end_index = node_index[member.end.0.as_str()];
    let (length, direction) = member_geometry(&model.nodes[start_index], &model.nodes[end_index])?;
    let axial_stiffness = member_axial_stiffness(model, &member.section_id, length)?;
    let axial_extension = (0..3)
        .map(|axis| {
            (displacements[dof(end_index, axis)] - displacements[dof(start_index, axis)])
                * direction[axis]
        })
        .sum::<f64>();

    Ok(MemberAxialForce {
        member_id: member.id.clone(),
        axial_kn: axial_stiffness * axial_extension,
    })
}

fn solve_linear_system(
    mut matrix: Vec<Vec<f64>>,
    mut right_hand_side: Vec<f64>,
    dof_label: impl Fn(usize) -> String,
) -> Result<Vec<f64>> {
    let size = right_hand_side.len();
    let singular_threshold = 1.0e-12;

    for pivot in 0..size {
        let max_row = (pivot..size)
            .max_by(|a, b| matrix[*a][pivot].abs().total_cmp(&matrix[*b][pivot].abs()))
            .unwrap();
        if matrix[max_row][pivot].abs() <= singular_threshold {
            return Err(TowerError::UnstableModel {
                reason: format!(
                    "singular or near-singular stiffness matrix at active DOF {}",
                    dof_label(pivot)
                ),
            });
        }
        matrix.swap(pivot, max_row);
        right_hand_side.swap(pivot, max_row);

        let pivot_row = matrix[pivot].clone();
        for row in (pivot + 1)..size {
            let factor = matrix[row][pivot] / matrix[pivot][pivot];
            for (column, value) in matrix[row].iter_mut().enumerate().skip(pivot) {
                *value -= factor * pivot_row[column];
            }
            right_hand_side[row] -= factor * right_hand_side[pivot];
        }
    }

    let mut solution = vec![0.0; size];
    for row in (0..size).rev() {
        let known = ((row + 1)..size)
            .map(|column| matrix[row][column] * solution[column])
            .sum::<f64>();
        solution[row] = (right_hand_side[row] - known) / matrix[row][row];
    }

    Ok(solution)
}

fn multiply_matrix_vector(matrix: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
    matrix
        .iter()
        .map(|row| row.iter().zip(vector).map(|(a, b)| a * b).sum())
        .collect()
}

fn dof(node_index: usize, axis: usize) -> usize {
    node_index * 3 + axis
}

fn dof_label(model: &TowerModel, dof: usize) -> String {
    let node_index = dof / 3;
    let axis = match dof % 3 {
        0 => "ux",
        1 => "uy",
        _ => "uz",
    };
    format!("{}.{}", model.nodes[node_index].id.0, axis)
}
