use std::collections::HashSet;

use serde::Deserialize;

use crate::errors::{Result, TowerError};
use crate::geometry::{MemberId, NodeId, Point3, RawPoint3};
use crate::loads::{LoadCase, LoadCaseId, RawLoadCase};
use crate::materials::{Material, MaterialId, RawMaterial};
use crate::sections::{RawSection, Section, SectionId};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub point: Point3,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub id: MemberId,
    pub start: NodeId,
    pub end: NodeId,
    pub section_id: SectionId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Support {
    pub node_id: NodeId,
    pub ux: bool,
    pub uy: bool,
    pub uz: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TowerModel {
    pub metadata: ModelMetadata,
    pub nodes: Vec<Node>,
    pub members: Vec<Member>,
    pub supports: Vec<Support>,
    pub materials: Vec<Material>,
    pub sections: Vec<Section>,
    pub load_cases: Vec<LoadCase>,
}

impl TowerModel {
    pub fn from_toml_str(input: &str) -> Result<Self> {
        let raw: RawTowerModel = toml::from_str(input).map_err(|error| TowerError::Parse {
            message: error.to_string(),
        })?;
        raw.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ModelMetadata {
    pub name: String,
    pub disclaimer: String,
}

#[derive(Debug, Deserialize)]
struct RawTowerModel {
    metadata: ModelMetadata,
    #[serde(default)]
    nodes: Vec<RawNode>,
    #[serde(default)]
    members: Vec<RawMember>,
    #[serde(default)]
    supports: Vec<Support>,
    #[serde(default)]
    materials: Vec<RawMaterial>,
    #[serde(default)]
    sections: Vec<RawSection>,
    #[serde(default)]
    load_cases: Vec<RawLoadCase>,
}

impl RawTowerModel {
    fn validate(self) -> Result<TowerModel> {
        let nodes = self
            .nodes
            .iter()
            .map(RawNode::validate)
            .collect::<Result<Vec<_>>>()?;
        ensure_unique(nodes.iter().map(|node| node.id.0.as_str()), "nodes")?;

        let materials = self
            .materials
            .iter()
            .map(RawMaterial::validate)
            .collect::<Result<Vec<_>>>()?;
        ensure_unique(
            materials.iter().map(|material| material.id.0.as_str()),
            "materials",
        )?;

        let sections = self
            .sections
            .iter()
            .map(RawSection::validate)
            .collect::<Result<Vec<_>>>()?;
        ensure_unique(
            sections.iter().map(|section| section.id.0.as_str()),
            "sections",
        )?;

        let members = self
            .members
            .iter()
            .map(RawMember::validate)
            .collect::<Result<Vec<_>>>()?;
        ensure_unique(members.iter().map(|member| member.id.0.as_str()), "members")?;

        let load_cases = self
            .load_cases
            .iter()
            .map(RawLoadCase::validate)
            .collect::<Result<Vec<_>>>()?;
        ensure_unique(
            load_cases.iter().map(|load_case| load_case.id.0.as_str()),
            "load_cases",
        )?;

        validate_references(
            &nodes,
            &materials,
            &sections,
            &members,
            &self.supports,
            &load_cases,
        )?;

        Ok(TowerModel {
            metadata: self.metadata,
            nodes,
            members,
            supports: self.supports,
            materials,
            sections,
            load_cases,
        })
    }
}

fn validate_references(
    nodes: &[Node],
    materials: &[Material],
    sections: &[Section],
    members: &[Member],
    supports: &[Support],
    load_cases: &[LoadCase],
) -> Result<()> {
    let node_ids: HashSet<&NodeId> = nodes.iter().map(|node| &node.id).collect();
    let material_ids: HashSet<&MaterialId> =
        materials.iter().map(|material| &material.id).collect();
    let section_ids: HashSet<&SectionId> = sections.iter().map(|section| &section.id).collect();

    for section in sections {
        ensure_known_reference(
            material_ids.contains(&section.material_id),
            format!("sections.{}.material_id", section.id.0),
            &section.material_id.0,
        )?;
    }

    for member in members {
        ensure_known_reference(
            node_ids.contains(&member.start),
            format!("members.{}.start", member.id.0),
            &member.start.0,
        )?;
        ensure_known_reference(
            node_ids.contains(&member.end),
            format!("members.{}.end", member.id.0),
            &member.end.0,
        )?;
        ensure_known_reference(
            section_ids.contains(&member.section_id),
            format!("members.{}.section_id", member.id.0),
            &member.section_id.0,
        )?;
    }

    for support in supports {
        ensure_known_reference(
            node_ids.contains(&support.node_id),
            format!("supports.{}.node_id", support.node_id.0),
            &support.node_id.0,
        )?;
    }

    for LoadCase { id, nodal_loads } in load_cases {
        validate_load_references(id, nodal_loads, &node_ids)?;
    }

    Ok(())
}

fn validate_load_references(
    load_case_id: &LoadCaseId,
    nodal_loads: &[crate::loads::NodalLoad],
    node_ids: &HashSet<&NodeId>,
) -> Result<()> {
    for load in nodal_loads {
        ensure_known_reference(
            node_ids.contains(&load.node_id),
            format!("load_cases.{}.{}.node_id", load_case_id.0, load.node_id.0),
            &load.node_id.0,
        )?;
    }

    Ok(())
}

fn ensure_known_reference(is_known: bool, field: String, id: &str) -> Result<()> {
    if is_known {
        Ok(())
    } else {
        Err(TowerError::UnknownReference {
            field,
            id: id.to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawNode {
    id: String,
    point: RawPoint3,
}

impl RawNode {
    fn validate(&self) -> Result<Node> {
        Ok(Node {
            id: NodeId(self.id.clone()),
            point: self.point.validate(&format!("nodes.{}", self.id))?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawMember {
    id: String,
    start: String,
    end: String,
    section_id: String,
}

impl RawMember {
    fn validate(&self) -> Result<Member> {
        Ok(Member {
            id: MemberId(self.id.clone()),
            start: NodeId(self.start.clone()),
            end: NodeId(self.end.clone()),
            section_id: SectionId(self.section_id.clone()),
        })
    }
}

impl<'de> Deserialize<'de> for Support {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawSupport {
            node_id: String,
            ux: bool,
            uy: bool,
            uz: bool,
        }

        let raw = RawSupport::deserialize(deserializer)?;
        Ok(Self {
            node_id: NodeId(raw.node_id),
            ux: raw.ux,
            uy: raw.uy,
            uz: raw.uz,
        })
    }
}

fn ensure_unique<'a>(ids: impl Iterator<Item = &'a str>, collection: &str) -> Result<()> {
    let mut seen = HashSet::new();
    for id in ids {
        if !seen.insert(id) {
            return Err(TowerError::DuplicateId {
                collection: collection.to_string(),
                id: id.to_string(),
            });
        }
    }
    Ok(())
}
