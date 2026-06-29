use tower_core::analysis::TrussSolver;
use tower_core::errors::TowerError;
use tower_core::loads::LoadCaseId;
use tower_core::model::TowerModel;

const SIMPLE_BAR_FIXTURE_ID: &str = "example_01_simple_bar";
const TRUSS_STAR_FIXTURE_ID: &str = "example_02_simple_3d_truss_star";
const SINGULAR_FIXTURE_ID: &str = "example_03_singular_unstable_model";
const TRACE_AXIAL_STIFFNESS: &str = "NUM-AXIAL-001";
const TRACE_AXIAL_DISPLACEMENT: &str = "NUM-AXIAL-002";
const TRACE_REACTION_EQUILIBRIUM: &str = "NUM-AXIAL-003";
const TRACE_AXIAL_FORCE: &str = "NUM-AXIAL-004";

fn solve_fixture(
    input: &str,
    load_case_id: &str,
) -> tower_core::Result<tower_core::analysis::AnalysisResult> {
    let model = TowerModel::from_toml_str(input).unwrap();
    TrussSolver::solve(&model, &LoadCaseId(load_case_id.to_string()))
}

fn assert_close(actual: f64, expected: f64, absolute_tolerance: f64, relative_tolerance: f64) {
    let absolute_error = (actual - expected).abs();
    let relative_error = if expected == 0.0 {
        absolute_error
    } else {
        absolute_error / expected.abs()
    };

    assert!(
        absolute_error <= absolute_tolerance || relative_error <= relative_tolerance,
        "expected {actual} to be within absolute tolerance {absolute_tolerance} or relative tolerance {relative_tolerance} of {expected}"
    );
}

fn simple_bar_toml() -> &'static str {
    r#"
[metadata]
name = "example_01_simple_bar"
disclaimer = "not for final engineering design"

[[nodes]]
id = "fixed"
point.x = { value = 0.0, unit = "m" }
point.y = { value = 0.0, unit = "m" }
point.z = { value = 0.0, unit = "m" }

[[nodes]]
id = "free"
point.x = { value = 2.0, unit = "m" }
point.y = { value = 0.0, unit = "m" }
point.z = { value = 0.0, unit = "m" }

[[materials]]
id = "steel"
density = { value = 7850.0, unit = "kg/m3" }
yield_stress = { value = 250.0, unit = "MPa" }
elastic_modulus = { value = 200000000.0, unit = "kN/m2" }

[[sections]]
id = "bar"
material_id = "steel"
nominal_area = { value = 0.001, unit = "m2" }
radius_of_gyration = { value = 0.02, unit = "m" }

[[members]]
id = "bar-x"
start = "fixed"
end = "free"
section_id = "bar"

[[supports]]
node_id = "fixed"
ux = true
uy = true
uz = true

[[supports]]
node_id = "free"
ux = false
uy = true
uz = true

[[load_cases]]
id = "axial"

[[load_cases.nodal_loads]]
node_id = "free"
fx = { value = 10.0, unit = "kN" }
fy = { value = 0.0, unit = "kN" }
fz = { value = 0.0, unit = "kN" }
"#
}

fn truss_star_toml() -> &'static str {
    r#"
[metadata]
name = "example_02_simple_3d_truss_star"
disclaimer = "not for final engineering design"

[[nodes]]
id = "support-x"
point.x = { value = -1.0, unit = "m" }
point.y = { value = 0.0, unit = "m" }
point.z = { value = 0.0, unit = "m" }

[[nodes]]
id = "support-y"
point.x = { value = 0.0, unit = "m" }
point.y = { value = -1.0, unit = "m" }
point.z = { value = 0.0, unit = "m" }

[[nodes]]
id = "support-z"
point.x = { value = 0.0, unit = "m" }
point.y = { value = 0.0, unit = "m" }
point.z = { value = -1.0, unit = "m" }

[[nodes]]
id = "free"
point.x = { value = 0.0, unit = "m" }
point.y = { value = 0.0, unit = "m" }
point.z = { value = 0.0, unit = "m" }

[[materials]]
id = "steel"
density = { value = 7850.0, unit = "kg/m3" }
yield_stress = { value = 250.0, unit = "MPa" }
elastic_modulus = { value = 200000000.0, unit = "kN/m2" }

[[sections]]
id = "bar"
material_id = "steel"
nominal_area = { value = 0.001, unit = "m2" }
radius_of_gyration = { value = 0.02, unit = "m" }

[[members]]
id = "x-bar"
start = "support-x"
end = "free"
section_id = "bar"

[[members]]
id = "y-bar"
start = "support-y"
end = "free"
section_id = "bar"

[[members]]
id = "z-bar"
start = "support-z"
end = "free"
section_id = "bar"

[[supports]]
node_id = "support-x"
ux = true
uy = true
uz = true

[[supports]]
node_id = "support-y"
ux = true
uy = true
uz = true

[[supports]]
node_id = "support-z"
ux = true
uy = true
uz = true

[[load_cases]]
id = "wind"

[[load_cases.nodal_loads]]
node_id = "free"
fx = { value = 10.0, unit = "kN" }
fy = { value = 20.0, unit = "kN" }
fz = { value = 30.0, unit = "kN" }
"#
}

#[test]
fn solves_accepted_simple_bar_reference_fixture() {
    assert_eq!(SIMPLE_BAR_FIXTURE_ID, "example_01_simple_bar");
    assert_eq!(TRACE_AXIAL_STIFFNESS, "NUM-AXIAL-001");
    assert_eq!(TRACE_AXIAL_DISPLACEMENT, "NUM-AXIAL-002");
    assert_eq!(TRACE_REACTION_EQUILIBRIUM, "NUM-AXIAL-003");
    assert_eq!(TRACE_AXIAL_FORCE, "NUM-AXIAL-004");

    let result = solve_fixture(simple_bar_toml(), "axial").unwrap();

    let free_displacement = result.displacement("free").unwrap();
    assert_close(free_displacement.ux_m, 0.0001, 1.0e-9, 1.0e-7);

    let fixed_reaction = result.reaction("fixed").unwrap();
    assert_close(fixed_reaction.fx_kn, -10.0, 1.0e-9, 1.0e-7);

    let bar_force = result.member_force("bar-x").unwrap();
    assert_close(bar_force.axial_kn, 10.0, 1.0e-9, 1.0e-7);
}

#[test]
fn solves_accepted_3d_truss_star_reference_fixture() {
    assert_eq!(TRUSS_STAR_FIXTURE_ID, "example_02_simple_3d_truss_star");
    assert_eq!(TRACE_AXIAL_STIFFNESS, "NUM-AXIAL-001");
    assert_eq!(TRACE_AXIAL_DISPLACEMENT, "NUM-AXIAL-002");
    assert_eq!(TRACE_REACTION_EQUILIBRIUM, "NUM-AXIAL-003");
    assert_eq!(TRACE_AXIAL_FORCE, "NUM-AXIAL-004");

    let result = solve_fixture(truss_star_toml(), "wind").unwrap();

    let free_displacement = result.displacement("free").unwrap();
    assert_close(free_displacement.ux_m, 0.00005, 1.0e-9, 1.0e-7);
    assert_close(free_displacement.uy_m, 0.00010, 1.0e-9, 1.0e-7);
    assert_close(free_displacement.uz_m, 0.00015, 1.0e-9, 1.0e-7);

    assert_close(
        result.reaction("support-x").unwrap().fx_kn,
        -10.0,
        1.0e-9,
        1.0e-7,
    );
    assert_close(
        result.reaction("support-y").unwrap().fy_kn,
        -20.0,
        1.0e-9,
        1.0e-7,
    );
    assert_close(
        result.reaction("support-z").unwrap().fz_kn,
        -30.0,
        1.0e-9,
        1.0e-7,
    );

    assert_close(
        result.member_force("x-bar").unwrap().axial_kn,
        10.0,
        1.0e-9,
        1.0e-7,
    );
    assert_close(
        result.member_force("y-bar").unwrap().axial_kn,
        20.0,
        1.0e-9,
        1.0e-7,
    );
    assert_close(
        result.member_force("z-bar").unwrap().axial_kn,
        30.0,
        1.0e-9,
        1.0e-7,
    );
}

#[test]
fn rejects_singular_unstable_model_deterministically() {
    assert_eq!(SINGULAR_FIXTURE_ID, "example_03_singular_unstable_model");

    let mut input = simple_bar_toml().to_string();
    input = input.replace(
        "[[supports]]\nnode_id = \"free\"\nux = false\nuy = true\nuz = true\n",
        "[[supports]]\nnode_id = \"free\"\nux = false\nuy = false\nuz = true\n",
    );

    let err = solve_fixture(&input, "axial").unwrap_err();

    assert_eq!(
        err,
        TowerError::UnstableModel {
            reason: "singular or near-singular stiffness matrix at active DOF free.uy".to_string()
        }
    );
}
