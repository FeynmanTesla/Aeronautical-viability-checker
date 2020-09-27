pub fn find_drag_force(drag_coefficient: f64, air_density: f64, velocity: f64, reference_area: f64) -> f64 {
    drag_coefficient * air_density * velocity.powi(2) * reference_area / (2 as f64)
}