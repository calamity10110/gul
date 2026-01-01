#![allow(dead_code)]
// Math and science utilities
use std::f64::consts::{E, PI};

pub struct Math;

impl Math {
    // Basic math functions
    pub fn abs(x: f64) -> f64 {
        x.abs()
    }

    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    pub fn pow(base: f64, exp: f64) -> f64 {
        base.powf(exp)
    }

    pub fn exp(x: f64) -> f64 {
        x.exp()
    }

    pub fn ln(x: f64) -> f64 {
        x.ln()
    }

    pub fn log10(x: f64) -> f64 {
        x.log10()
    }

    pub fn log(x: f64, base: f64) -> f64 {
        x.log(base)
    }

    // Trigonometric functions
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    pub fn cos(x: f64) -> f64 {
        x.cos()
    }

    pub fn tan(x: f64) -> f64 {
        x.tan()
    }

    pub fn asin(x: f64) -> f64 {
        x.asin()
    }

    pub fn acos(x: f64) -> f64 {
        x.acos()
    }

    pub fn atan(x: f64) -> f64 {
        x.atan()
    }

    pub fn atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }

    // Hyperbolic functions
    pub fn sinh(x: f64) -> f64 {
        x.sinh()
    }

    pub fn cosh(x: f64) -> f64 {
        x.cosh()
    }

    pub fn tanh(x: f64) -> f64 {
        x.tanh()
    }

    // Rounding functions
    pub fn floor(x: f64) -> f64 {
        x.floor()
    }

    pub fn ceil(x: f64) -> f64 {
        x.ceil()
    }

    pub fn round(x: f64) -> f64 {
        x.round()
    }

    // Min/Max
    pub fn min(a: f64, b: f64) -> f64 {
        a.min(b)
    }

    pub fn max(a: f64, b: f64) -> f64 {
        a.max(b)
    }

    // Constants
    pub fn pi() -> f64 {
        PI
    }

    pub fn e() -> f64 {
        E
    }

    // Degree/Radian conversion
    pub fn deg_to_rad(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    pub fn rad_to_deg(radians: f64) -> f64 {
        radians * 180.0 / PI
    }

    // Factorial
    pub fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }

    // GCD and LCM
    pub fn gcd(a: i64, b: i64) -> i64 {
        let mut a = a.abs();
        let mut b = b.abs();
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    pub fn lcm(a: i64, b: i64) -> i64 {
        (a.abs() * b.abs()) / Self::gcd(a, b)
    }
}

pub struct Physics;

impl Physics {
    // Physical constants
    pub const SPEED_OF_LIGHT: f64 = 299_792_458.0; // m/s
    pub const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // N⋅m²/kg²
    pub const PLANCK_CONSTANT: f64 = 6.626e-34; // J⋅s
    pub const BOLTZMANN_CONSTANT: f64 = 1.381e-23; // J/K
    pub const AVOGADRO_NUMBER: f64 = 6.022e23; // mol⁻¹
    pub const GAS_CONSTANT: f64 = 8.314; // J/(mol⋅K)
    pub const ELECTRON_MASS: f64 = 9.109e-31; // kg
    pub const PROTON_MASS: f64 = 1.673e-27; // kg
    pub const ELEMENTARY_CHARGE: f64 = 1.602e-19; // C
    pub const VACUUM_PERMITTIVITY: f64 = 8.854e-12; // F/m
    pub const VACUUM_PERMEABILITY: f64 = 1.257e-6; // H/m

    // Kinematic equations
    pub fn velocity(initial_velocity: f64, acceleration: f64, time: f64) -> f64 {
        initial_velocity + acceleration * time
    }

    pub fn displacement(initial_velocity: f64, acceleration: f64, time: f64) -> f64 {
        initial_velocity * time + 0.5 * acceleration * time.powi(2)
    }

    pub fn kinetic_energy(mass: f64, velocity: f64) -> f64 {
        0.5 * mass * velocity.powi(2)
    }

    pub fn potential_energy(mass: f64, height: f64, g: f64) -> f64 {
        mass * g * height
    }

    // Force calculations
    pub fn force(mass: f64, acceleration: f64) -> f64 {
        mass * acceleration
    }

    pub fn gravitational_force(m1: f64, m2: f64, distance: f64) -> f64 {
        Self::GRAVITATIONAL_CONSTANT * m1 * m2 / distance.powi(2)
    }

    pub fn coulomb_force(q1: f64, q2: f64, distance: f64) -> f64 {
        (1.0 / (4.0 * PI * Self::VACUUM_PERMITTIVITY)) * q1 * q2 / distance.powi(2)
    }

    // Wave calculations
    pub fn wavelength(frequency: f64, speed: f64) -> f64 {
        speed / frequency
    }

    pub fn frequency(wavelength: f64, speed: f64) -> f64 {
        speed / wavelength
    }

    pub fn photon_energy(frequency: f64) -> f64 {
        Self::PLANCK_CONSTANT * frequency
    }
}

pub struct Chemistry;

impl Chemistry {
    // Atomic masses (in u - unified atomic mass units)
    pub const HYDROGEN_MASS: f64 = 1.008;
    pub const CARBON_MASS: f64 = 12.011;
    pub const NITROGEN_MASS: f64 = 14.007;
    pub const OXYGEN_MASS: f64 = 15.999;
    pub const SULFUR_MASS: f64 = 32.06;

    // Ideal gas law: PV = nRT
    pub fn ideal_gas_pressure(n: f64, temperature: f64, volume: f64) -> f64 {
        (n * Physics::GAS_CONSTANT * temperature) / volume
    }

    pub fn ideal_gas_volume(n: f64, temperature: f64, pressure: f64) -> f64 {
        (n * Physics::GAS_CONSTANT * temperature) / pressure
    }

    pub fn ideal_gas_temperature(pressure: f64, volume: f64, n: f64) -> f64 {
        (pressure * volume) / (n * Physics::GAS_CONSTANT)
    }

    pub fn ideal_gas_moles(pressure: f64, volume: f64, temperature: f64) -> f64 {
        (pressure * volume) / (Physics::GAS_CONSTANT * temperature)
    }

    // Molarity calculations
    pub fn molarity(moles: f64, volume_liters: f64) -> f64 {
        moles / volume_liters
    }

    pub fn moles_from_molarity(molarity: f64, volume_liters: f64) -> f64 {
        molarity * volume_liters
    }

    // pH calculations
    pub fn ph_from_h_concentration(h_concentration: f64) -> f64 {
        -h_concentration.log10()
    }

    pub fn h_concentration_from_ph(ph: f64) -> f64 {
        10.0_f64.powf(-ph)
    }

    // Molecular mass calculation
    pub fn molecular_mass(atoms: Vec<(f64, u32)>) -> f64 {
        atoms
            .iter()
            .map(|(mass, count)| mass * (*count as f64))
            .sum()
    }
}

pub struct UnitConverter;

impl UnitConverter {
    // Length conversions
    pub fn meters_to_feet(meters: f64) -> f64 {
        meters * 3.28084
    }

    pub fn feet_to_meters(feet: f64) -> f64 {
        feet / 3.28084
    }

    pub fn meters_to_miles(meters: f64) -> f64 {
        meters / 1609.34
    }

    pub fn miles_to_meters(miles: f64) -> f64 {
        miles * 1609.34
    }

    // Temperature conversions
    pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        celsius * 9.0 / 5.0 + 32.0
    }

    pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    pub fn celsius_to_kelvin(celsius: f64) -> f64 {
        celsius + 273.15
    }

    pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
        kelvin - 273.15
    }

    // Mass conversions
    pub fn kg_to_pounds(kg: f64) -> f64 {
        kg * 2.20462
    }

    pub fn pounds_to_kg(pounds: f64) -> f64 {
        pounds / 2.20462
    }

    // Energy conversions
    pub fn joules_to_calories(joules: f64) -> f64 {
        joules / 4.184
    }

    pub fn calories_to_joules(calories: f64) -> f64 {
        calories * 4.184
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        assert_eq!(Math::abs(-5.0), 5.0);
        assert_eq!(Math::sqrt(16.0), 4.0);
        assert_eq!(Math::pow(2.0, 3.0), 8.0);
    }

    #[test]
    fn test_trigonometry() {
        assert!((Math::sin(Math::pi() / 2.0) - 1.0).abs() < 1e-10);
        assert!((Math::cos(0.0) - 1.0).abs() < 1e-10);
        assert!((Math::tan(Math::pi() / 4.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_degree_conversion() {
        assert!((Math::deg_to_rad(180.0) - Math::pi()).abs() < 1e-10);
        assert!((Math::rad_to_deg(Math::pi()) - 180.0).abs() < 1e-10);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(Math::factorial(5), 120);
        assert_eq!(Math::factorial(0), 1);
    }

    #[test]
    fn test_gcd_lcm() {
        assert_eq!(Math::gcd(12, 8), 4);
        assert_eq!(Math::lcm(12, 8), 24);
    }

    #[test]
    fn test_physics_constants() {
        assert_eq!(Physics::SPEED_OF_LIGHT, 299_792_458.0);
        assert_eq!(Physics::GRAVITATIONAL_CONSTANT, 6.674e-11);
    }

    #[test]
    fn test_kinetic_energy() {
        let energy = Physics::kinetic_energy(10.0, 5.0);
        assert_eq!(energy, 125.0);
    }

    #[test]
    fn test_ideal_gas_law() {
        let pressure = Chemistry::ideal_gas_pressure(1.0, 273.15, 0.0224);
        assert!((pressure - 101325.0).abs() < 1000.0); // Approximately 1 atm
    }

    #[test]
    fn test_ph_calculations() {
        let ph = Chemistry::ph_from_h_concentration(1e-7);
        assert!((ph - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_unit_conversions() {
        assert!((UnitConverter::celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-10);
        assert!((UnitConverter::celsius_to_kelvin(0.0) - 273.15).abs() < 1e-10);
        assert!((UnitConverter::meters_to_feet(1.0) - 3.28084).abs() < 1e-5);
    }

    #[test]
    fn test_molecular_mass() {
        // Water: H2O = 2*1.008 + 15.999 = 18.015
        let water_mass = Chemistry::molecular_mass(vec![
            (Chemistry::HYDROGEN_MASS, 2),
            (Chemistry::OXYGEN_MASS, 1),
        ]);
        assert!((water_mass - 18.015).abs() < 0.01);
    }
}
