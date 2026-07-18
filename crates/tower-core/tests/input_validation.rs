use tower_core::errors::TowerError;
use tower_core::loads::{LoadProvenanceStatus, SELF_WEIGHT_LOAD_CASE_ID};
use tower_core::model::TowerModel;

fn valid_model_toml() -> String {
    r#"
[metadata]
name = "simple scaffold"
disclaimer = "not for final engineering design"

[[nodes]]
id = "n1"
point.x = { value = 0.1, unit = "m" }
point.y = { value = 0.1, unit = "m" }
point.z = { value = 0.1, unit = "m" }

[[nodes]]
id = "n2"
point.x = { value = 3.1, unit = "m" }
point.y = { value = 4.1, unit = "m" }
point.z = { value = 0.1, unit = "m" }

[[materials]]
id = "steel"
density = { value = 7850.0, unit = "kg/m3" }
yield_stress = { value = 250.0, unit = "MPa" }

[[sections]]
id = "L1"
material_id = "steel"
nominal_area = { value = 0.001, unit = "m2" }
radius_of_gyration = { value = 0.02, unit = "m" }

[[members]]
id = "m1"
start = "n1"
end = "n2"
section_id = "L1"

[[supports]]
node_id = "n1"
ux = true
uy = true
uz = true

[[load_cases]]
id = "wind"

[[load_cases.nodal_loads]]
node_id = "n2"
fx = { value = 1.0, unit = "kN" }
fy = { value = 0.0, unit = "kN" }
fz = { value = 0.0, unit = "kN" }
"#
    .to_string()
}

fn explicit_wind_case(model: &TowerModel) -> &tower_core::loads::LoadCase {
    model
        .load_cases
        .iter()
        .find(|load_case| load_case.id.0 == "wind")
        .expect("explicit wind load case must be preserved")
}

fn generated_self_weight_case(model: &TowerModel) -> &tower_core::loads::LoadCase {
    model
        .load_cases
        .iter()
        .find(|load_case| load_case.id.0 == SELF_WEIGHT_LOAD_CASE_ID)
        .expect("generated self-weight load case must be appended")
}

fn assert_unknown_reference(input: String, expected_field: &str, expected_id: &str) {
    let err = TowerModel::from_toml_str(&input).unwrap_err();

    assert_eq!(
        err,
        TowerError::UnknownReference {
            field: expected_field.to_string(),
            id: expected_id.to_string(),
        }
    );
}

#[test]
fn rejects_missing_coordinate_unit() {
    let err = TowerModel::from_toml_str(
        r#"
[metadata]
name = "missing unit"
disclaimer = "not for final engineering design"

[[nodes]]
id = "n1"
point.x = { value = 0.0 }
point.y = { value = 0.0, unit = "m" }
point.z = { value = 1.0, unit = "m" }
"#,
    )
    .unwrap_err();

    assert!(matches!(err, TowerError::MissingUnit { .. }));
}

#[test]
fn accepts_minimal_model_with_explicit_units() {
    let model = TowerModel::from_toml_str(&valid_model_toml()).unwrap();

    assert_eq!(model.nodes.len(), 2);
    assert_eq!(model.members.len(), 1);
}

#[test]
fn defaults_existing_nodal_load_cases_to_explicit_user_input() {
    let model = TowerModel::from_toml_str(&valid_model_toml()).unwrap();

    assert_eq!(model.load_cases.len(), 2);
    let explicit = explicit_wind_case(&model);
    let generated = generated_self_weight_case(&model);
    assert_eq!(explicit.id.0, "wind");
    assert_eq!(explicit.status, LoadProvenanceStatus::ExplicitUserInput);
    assert_eq!(explicit.source, "user input");
    assert_eq!(explicit.nodal_loads.len(), 1);
    assert_eq!(explicit.nodal_loads[0].node_id.0, "n2");
    assert_eq!(explicit.nodal_loads[0].fx.get(), 1.0);
    assert_eq!(explicit.nodal_loads[0].fy.get(), 0.0);
    assert_eq!(explicit.nodal_loads[0].fz.get(), 0.0);
    assert_eq!(generated.status, LoadProvenanceStatus::ValidatedQuantity);
}

#[test]
fn preserves_explicit_load_case_provenance_when_provided() {
    let input = valid_model_toml().replace(
        "id = \"wind\"",
        "id = \"wind\"\nstatus = \"explicit_user_input\"\nsource = \"field survey import\"",
    );

    let model = TowerModel::from_toml_str(&input).unwrap();

    assert_eq!(model.load_cases.len(), 2);
    let explicit = explicit_wind_case(&model);
    assert_eq!(explicit.status, LoadProvenanceStatus::ExplicitUserInput);
    assert_eq!(explicit.source, "field survey import");
    assert_eq!(explicit.nodal_loads[0].node_id.0, "n2");
    assert_eq!(explicit.nodal_loads[0].fx.get(), 1.0);
    assert_eq!(generated_self_weight_case(&model).nodal_loads.len(), 2);
}

#[test]
fn keeps_non_explicit_load_statuses_out_of_solver_load_cases() {
    let mut input = valid_model_toml();
    input.push_str(
        r#"
[[load_cases]]
id = "self-weight-quantity"
status = "validated_quantity"
source = "QTY-WEIGHT-001"

[[load_cases.nodal_loads]]
node_id = "n2"
fx = { value = 9.0, unit = "kN" }
fy = { value = 0.0, unit = "kN" }
fz = { value = 0.0, unit = "kN" }

[[load_cases]]
id = "placeholder-wind"
status = "candidate_provisional"
source = "demo placeholder"

[[load_cases.nodal_loads]]
node_id = "n2"
fx = { value = 8.0, unit = "kN" }
fy = { value = 0.0, unit = "kN" }
fz = { value = 0.0, unit = "kN" }

[[load_cases]]
id = "blocked-conductor"
status = "TODO_DOMAIN_VALIDATION"
source = "pending reviewer-approved evidence"

[[load_cases.nodal_loads]]
node_id = "n2"
fx = { value = 7.0, unit = "kN" }
fy = { value = 0.0, unit = "kN" }
fz = { value = 0.0, unit = "kN" }
"#,
    );

    let model = TowerModel::from_toml_str(&input).unwrap();

    assert_eq!(model.load_cases.len(), 2);
    let explicit = explicit_wind_case(&model);
    assert_eq!(explicit.id.0, "wind");
    assert_eq!(explicit.status, LoadProvenanceStatus::ExplicitUserInput);
    assert_eq!(explicit.nodal_loads[0].fx.get(), 1.0);
    assert_eq!(
        generated_self_weight_case(&model).status,
        LoadProvenanceStatus::ValidatedQuantity
    );
}

#[test]
fn rejects_duplicate_node_ids() {
    let mut input = valid_model_toml();
    input.push_str(
        r#"
[[nodes]]
id = "n1"
point.x = { value = 1.0, unit = "m" }
point.y = { value = 1.0, unit = "m" }
point.z = { value = 1.0, unit = "m" }
"#,
    );

    let err = TowerModel::from_toml_str(&input).unwrap_err();

    assert_eq!(
        err,
        TowerError::DuplicateId {
            collection: "nodes".to_string(),
            id: "n1".to_string(),
        }
    );
}

#[test]
fn rejects_duplicate_load_case_ids() {
    let mut input = valid_model_toml();
    input.push_str(
        r#"
[[load_cases]]
id = "wind"
"#,
    );

    let err = TowerModel::from_toml_str(&input).unwrap_err();

    assert_eq!(
        err,
        TowerError::DuplicateId {
            collection: "load_cases".to_string(),
            id: "wind".to_string(),
        }
    );
}

#[test]
fn rejects_member_with_unknown_start_node() {
    assert_unknown_reference(
        valid_model_toml().replace("start = \"n1\"", "start = \"missing-node\""),
        "members.m1.start",
        "missing-node",
    );
}

#[test]
fn rejects_member_with_unknown_end_node() {
    assert_unknown_reference(
        valid_model_toml().replace("end = \"n2\"", "end = \"missing-node\""),
        "members.m1.end",
        "missing-node",
    );
}

#[test]
fn rejects_member_with_unknown_section() {
    assert_unknown_reference(
        valid_model_toml().replace("section_id = \"L1\"", "section_id = \"missing-section\""),
        "members.m1.section_id",
        "missing-section",
    );
}

#[test]
fn rejects_section_with_unknown_material() {
    assert_unknown_reference(
        valid_model_toml().replace(
            "material_id = \"steel\"",
            "material_id = \"missing-material\"",
        ),
        "sections.L1.material_id",
        "missing-material",
    );
}

#[test]
fn rejects_support_with_unknown_node() {
    assert_unknown_reference(
        valid_model_toml().replace("node_id = \"n1\"", "node_id = \"missing-support-node\""),
        "supports.missing-support-node.node_id",
        "missing-support-node",
    );
}

#[test]
fn rejects_load_with_unknown_node() {
    assert_unknown_reference(
        valid_model_toml().replace("node_id = \"n2\"", "node_id = \"missing-load-node\""),
        "load_cases.wind.missing-load-node.node_id",
        "missing-load-node",
    );
}
