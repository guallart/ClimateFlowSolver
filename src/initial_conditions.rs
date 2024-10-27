use crate::math;
use crate::mesh::geometry::Vector;
use crate::mesh::mesher::Mesh;

const DENSITY_LAPSE_RATE: f64 = -0.0013;
const TEMPERATURE_LAPSE_RATE: f64 = -0.0065;
const UNIVERSAL_GAS_CONSTANT: f64 = 8.31432;
const GRAVITY: f64 = 9.80665;
const AIR_MOLAR_MASS: f64 = 0.0289644;
const PRESSURE_SEA_LEVEL: f64 = 101325.0;
const TEMPERATURE_SEA_LEVEL: f64 = 20.0 + 273.15;

pub fn define_initial_conditions_naive(
    mesh: &mut Mesh,
    z_ref: f64,
    speed_ref: f64,
    density_ref: f64,
    direction: f64,
    shear: f64,
    temperature: f64,
) {
    // pub velocity: Vector,
    // pub pressure: f64,
    // pub temperature: f64,
    // pub density: f64,
    // pub energy: f64,

    let press_power = -GRAVITY * AIR_MOLAR_MASS / (UNIVERSAL_GAS_CONSTANT * TEMPERATURE_LAPSE_RATE);
    for cell in mesh.cells.iter_mut() {
        let z = cell.center.z;
        let delta_z = z - z_ref;
        let height = z - cell.ground_height;

        cell.physics.density = density_ref * delta_z * DENSITY_LAPSE_RATE;
        cell.physics.pressure = PRESSURE_SEA_LEVEL
            * (1.0 + TEMPERATURE_LAPSE_RATE / TEMPERATURE_SEA_LEVEL * height).powf(press_power);
        cell.physics.temperature = temperature;

        let u_ref = speed_ref * math::as_rads(direction).cos();
        let v_ref = speed_ref * math::as_rads(direction).sin();

        let u = u_ref * (z / z_ref).powf(shear);
        let v = v_ref * (z / z_ref).powf(shear);

        cell.physics.velocity = Vector::new(u, v, 0.0);
    }
}
