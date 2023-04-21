use rand::random;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_float() -> f64 {
    random::<f64>()
}

pub fn random_floatmx(min: f64, max: f64) -> f64 {
    min + (max - min) * random::<f64>()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
