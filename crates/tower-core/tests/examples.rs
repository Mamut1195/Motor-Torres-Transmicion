use tower_core::model::TowerModel;
use tower_core::reporting::ENGINEERING_DISCLAIMER;

fn load_example(input: &str) -> TowerModel {
    TowerModel::from_toml_str(input).unwrap()
}

#[test]
fn deterministic_examples_load_with_expected_shape() {
    let simple_bar = load_example(include_str!("../../../examples/simple_bar.toml"));
    let simple_3d_truss = load_example(include_str!("../../../examples/simple_3d_truss.toml"));
    let tower_69kv = load_example(include_str!("../../../examples/69kv_tower.toml"));
    let optimization_demo = load_example(include_str!("../../../examples/optimization_demo.toml"));

    assert_eq!(simple_bar.metadata.name, "example_01_simple_bar");
    assert_eq!(simple_bar.metadata.disclaimer, ENGINEERING_DISCLAIMER);
    assert_eq!(simple_bar.nodes.len(), 2);
    assert_eq!(simple_bar.members.len(), 1);
    assert_eq!(simple_bar.load_cases[0].id.0, "axial");

    assert_eq!(
        simple_3d_truss.metadata.name,
        "example_02_simple_3d_truss_star"
    );
    assert_eq!(simple_3d_truss.metadata.disclaimer, ENGINEERING_DISCLAIMER);
    assert_eq!(simple_3d_truss.nodes.len(), 4);
    assert_eq!(simple_3d_truss.members.len(), 3);
    assert_eq!(simple_3d_truss.load_cases[0].id.0, "wind");

    assert_eq!(
        tower_69kv.metadata.name,
        "example_03_69kv_tower_placeholder"
    );
    assert_eq!(tower_69kv.metadata.disclaimer, ENGINEERING_DISCLAIMER);
    assert_eq!(tower_69kv.nodes.len(), 6);
    assert_eq!(tower_69kv.members.len(), 5);
    assert_eq!(tower_69kv.load_cases[0].id.0, "placeholder_wind");

    assert_eq!(
        optimization_demo.metadata.name,
        "example_04_optimization_demo"
    );
    assert_eq!(
        optimization_demo.metadata.disclaimer,
        ENGINEERING_DISCLAIMER
    );
    assert_eq!(optimization_demo.sections.len(), 2);
    assert_eq!(optimization_demo.members.len(), 1);
}

#[test]
fn tower_placeholder_uses_explicit_non_normative_disclaimer() {
    let input = include_str!("../../../examples/69kv_tower.toml");
    let model = load_example(input);

    assert_eq!(model.metadata.name, "example_03_69kv_tower_placeholder");
    assert_eq!(model.metadata.disclaimer, ENGINEERING_DISCLAIMER);
    assert!(input.contains("placeholder"));
    assert!(input.contains("non-normative"));
}
