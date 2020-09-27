pub fn find_lift_force(lift_coefficient: f64, air_density: f64, velocity: f64, wing_area: f64) -> f64 {
    lift_coefficient * air_density * velocity.powi(2) * wing_area / (2 as f64)
}