pub mod temperature {
    pub fn celsius_to_fahrenheit(c: f64) -> f64 {
        c * 1.8 + 32.0
    }

    pub fn fahrenheit_to_celsius(f: f64) -> f64 {
        (f - 32.0) / 1.8
    }
}

pub mod length {
    pub fn meters_to_feet(m: f64) -> f64 {
        m * 3.28084
    }

    pub fn feet_to_meters(f: f64) -> f64 {
        f / 3.28084
    }
}

pub mod weight {
    pub fn kg_to_lbs(kg: f64) -> f64 {
        kg * 2.20462
    }
}
