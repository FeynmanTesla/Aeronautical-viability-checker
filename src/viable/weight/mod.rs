const AVERAGE_EARTH_RADIUS: i64 = 6371000;
// metres, 4sf
const GRAVITY_CONSTANT_EARTH_MASS_PRODUCT: i64 = 398589405760000; // kilograms metres cubed per second squared, 5sf

fn find_acceleration_due_to_gravity_at_altitude(mass_kilograms: f64, altitude_metres: f64) -> f64 {
    ((GRAVITY_CONSTANT_EARTH_MASS_PRODUCT as f64) * mass_kilograms) / ((AVERAGE_EARTH_RADIUS as f64 + altitude_metres).powi(2))
}

pub fn find_gravity_force_at_altitude(mass_kilograms: f64, altitude_metres: f64) -> f64 {
    mass_kilograms * find_acceleration_due_to_gravity_at_altitude(mass_kilograms, altitude_metres)
}