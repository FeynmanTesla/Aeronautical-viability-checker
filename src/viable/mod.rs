mod weight;
mod lift;
mod drag;
mod thrust;
mod air_density;

const ALTITUDE_INCREMENT: i64 = 10;
const VELOCITY_INCREMENT: i64 = 10;

fn forces_balance(weight: f64, lift: f64, thrust: f64, drag: f64) -> bool {
    lift >= weight && thrust >= drag
}

pub fn is_viable(mass_kilograms: f64, max_altitude_metres: i64, min_velocity_metres_per_second: i64, max_velocity_metres_per_second: i64, frontal_projection_area: f64, wing_area: f64, lift_coefficient: f64, drag_coefficient: f64, altitudes_thrusts: &Vec<(i64, f64)>) -> String {
    assert!(is_altitudes_thrusts_vec_in_order(altitudes_thrusts));
    assert!(altitudes_thrusts.len() > 3);
    assert!(mass_kilograms > 0 as f64);
    assert!(max_altitude_metres > 0);
    assert!(min_velocity_metres_per_second > 0);
    assert!(max_velocity_metres_per_second > 0);
    assert!(max_velocity_metres_per_second > min_velocity_metres_per_second);
    assert!(frontal_projection_area > 0 as f64);
    assert!(wing_area > 0 as f64);
    assert!(wing_area > frontal_projection_area);
    assert!(lift_coefficient > 0 as f64);
    assert!(drag_coefficient > 0 as f64);

    for altitude_metres in (0..max_altitude_metres).step_by(ALTITUDE_INCREMENT as usize) {
        let air_density: f64 = air_density::find_air_density(altitude_metres).unwrap();
        for velocity_metres_per_second in (min_velocity_metres_per_second..max_velocity_metres_per_second).step_by(VELOCITY_INCREMENT as usize) {
            let weight_force: f64 = weight::find_gravity_force_at_altitude(mass_kilograms, altitude_metres as f64);
            let lift_force: f64 = lift::find_lift_force(lift_coefficient, air_density, velocity_metres_per_second as f64, wing_area);
            let thrust_force: f64 = thrust::find_thrust_at_altitude_from_map(altitude_metres, altitudes_thrusts);
            let drag_force: f64 = drag::find_drag_force(drag_coefficient, air_density, velocity_metres_per_second as f64, frontal_projection_area);

            if !forces_balance(weight_force, lift_force, thrust_force, drag_force) {
                let mut res: String = String::from(format!("The forces didn't balance at an altitude of {} metres. The weight force was {} N. The lift force was {} N. The thrust force was {} N. The drag force was {} N.", altitude_metres, weight_force, lift_force, thrust_force, drag_force));
                if lift_force < weight_force {
                    res = res + &*format!(" The lift force is smaller than the weight force.");
                }
                if thrust_force < drag_force {
                    res = res + &*format!(" The thrust force is smaller than the drag force.");
                }
                return res;
            }
        }
    }
    String::from("The forces balanced at all altitudes. This makes the aircraft aeronautically viable.")
}

fn is_altitudes_thrusts_vec_in_order(altitudes_thrusts: &Vec<(i64, f64)>) -> bool {
    for index in 0..altitudes_thrusts.len() - 2 {
        if altitudes_thrusts.get(index).unwrap().0 >= altitudes_thrusts.get(index + 1).unwrap().0 {
            return false;
        }
    }
    true
}