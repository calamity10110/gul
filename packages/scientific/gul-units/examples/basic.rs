use gul_units::{length, temperature, weight};

fn main() {
    let t = 25.0;
    println!("{} C = {} F", t, temperature::celsius_to_fahrenheit(t));

    let dist = 100.0;
    println!("{} m = {} ft", dist, length::meters_to_feet(dist));

    let w = 70.0;
    println!("{} kg = {} lbs", w, weight::kg_to_lbs(w));
}
