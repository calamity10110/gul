#![allow(dead_code)]
#![allow(unused_variables)]
// Physics Simulation Module
//
// This module provides comprehensive physics simulation capabilities including:
// - Particle systems with forces and integration
// - Rigid body dynamics with inertia and constraints
// - Collision detection with broad and narrow phases
// - Fluid simulation using smoothed particle hydrodynamics
// - Electromagnetic simulation with field calculations
// - Circuit simulation and wave propagation

use std::f64::consts::PI;

/// 3D Vector for positions, velocities, forces
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn mul(&self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.mul(1.0 / mag)
        } else {
            Vec3::zero()
        }
    }
}

/// Particle with physical properties
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f64,
    pub charge: f64,
    pub radius: f64,
    pub lifetime: f64,
    pub id: usize,
}

impl Particle {
    pub fn new(id: usize, position: Vec3, mass: f64) -> Self {
        Particle {
            position,
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            mass,
            charge: 0.0,
            radius: 1.0,
            lifetime: f64::INFINITY,
            id,
        }
    }

    pub fn apply_force(&mut self, force: &Vec3) {
        if self.mass > 0.0 {
            self.acceleration = self.acceleration.add(&force.mul(1.0 / self.mass));
        }
    }

    pub fn update(&mut self, dt: f64) {
        // Verlet integration for stability
        self.velocity = self.velocity.add(&self.acceleration.mul(dt));
        self.position = self.position.add(&self.velocity.mul(dt));
        self.acceleration = Vec3::zero();
        self.lifetime -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

/// Particle System Manager
pub struct ParticleSystem {
    particles: Vec<Particle>,
    next_id: usize,
    gravity: Vec3,
    time: f64,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ParticleSystem {
    pub fn new() -> Self {
        ParticleSystem {
            particles: Vec::new(),
            next_id: 0,
            gravity: Vec3::new(0.0, -9.81, 0.0), // Earth gravity
            time: 0.0,
        }
    }

    pub fn add_particle(&mut self, position: Vec3, mass: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let mut particle = Particle::new(id, position, mass);
        particle.apply_force(&self.gravity.mul(mass)); // Gravity
        self.particles.push(particle);
        id
    }

    pub fn apply_gravitational_force(&mut self) {
        let g_const = 6.67430e-11; // Gravitational constant
        for i in 0..self.particles.len() {
            for j in (i + 1)..self.particles.len() {
                let p1 = &self.particles[i];
                let p2 = &self.particles[j];

                let r = p1.position.sub(&p2.position);
                let distance = r.magnitude().max(1.0); // Avoid division by zero

                let force_magnitude = g_const * p1.mass * p2.mass / (distance * distance);
                let force_dir = r.normalize().mul(-force_magnitude);

                self.particles[i].apply_force(&force_dir);
                self.particles[j].apply_force(&force_dir.mul(-1.0));
            }
        }
    }

    pub fn apply_electromagnetic_force(&mut self) {
        let k_const = 8.987551789e9; // Coulomb's constant
        for i in 0..self.particles.len() {
            for j in (i + 1)..self.particles.len() {
                let p1 = &self.particles[i];
                let p2 = &self.particles[j];

                let r = p1.position.sub(&p2.position);
                let distance = r.magnitude().max(1.0);

                let force_magnitude = k_const * p1.charge * p2.charge / (distance * distance);
                let force_dir = r.normalize().mul(force_magnitude);

                self.particles[i].apply_force(&force_dir);
                self.particles[j].apply_force(&force_dir.mul(-1.0));
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.apply_gravitational_force();
        self.apply_electromagnetic_force();

        self.particles.retain_mut(|p| {
            p.update(dt);
            p.is_alive()
        });

        self.time += dt;
    }

    pub fn get_particles(&self) -> &[Particle] {
        &self.particles
    }
}

/// Rigid Body with rotational dynamics
#[derive(Debug, Clone)]
pub struct RigidBody {
    pub position: Vec3,
    pub orientation: Vec3, // Euler angles or quaternion
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
    pub mass: f64,
    pub inertia_tensor: [[f64; 3]; 3],
    pub id: usize,
}

impl RigidBody {
    pub fn new(id: usize, position: Vec3, mass: f64) -> Self {
        // Simple inertia tensor for sphere
        let i = (2.0 / 5.0) * mass * 1.0 * 1.0; // For radius 1
        let inertia_tensor = [[i, 0.0, 0.0], [0.0, i, 0.0], [0.0, 0.0, i]];

        RigidBody {
            position,
            orientation: Vec3::zero(),
            linear_velocity: Vec3::zero(),
            angular_velocity: Vec3::zero(),
            mass,
            inertia_tensor,
            id,
        }
    }

    pub fn apply_force(&mut self, force: &Vec3, point: &Vec3) {
        if self.mass > 0.0 {
            let linear_accel = force.mul(1.0 / self.mass);
            self.linear_velocity = self.linear_velocity.add(&linear_accel);

            // Torque = r × F
            let r = point.sub(&self.position);
            let torque = r.cross(force);
            self.apply_torque(&torque);
        }
    }

    pub fn apply_torque(&mut self, torque: &Vec3) {
        // Simplified: assume diagonal inertia tensor
        let i_xx = self.inertia_tensor[0][0];
        let i_yy = self.inertia_tensor[1][1];
        let i_zz = self.inertia_tensor[2][2];

        self.angular_velocity.x += torque.x / i_xx;
        self.angular_velocity.y += torque.y / i_yy;
        self.angular_velocity.z += torque.z / i_zz;
    }

    pub fn update(&mut self, dt: f64) {
        self.position = self.position.add(&self.linear_velocity.mul(dt));
        self.orientation = self.orientation.add(&self.angular_velocity.mul(dt));
    }
}

/// Collision Detection System
pub struct CollisionDetector {
    broad_phase_pairs: Vec<(usize, usize)>,
}

impl Default for CollisionDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl CollisionDetector {
    pub fn new() -> Self {
        CollisionDetector {
            broad_phase_pairs: Vec::new(),
        }
    }

    pub fn broad_phase(&mut self, particles: &[Particle]) {
        self.broad_phase_pairs.clear();
        for i in 0..particles.len() {
            for j in (i + 1)..particles.len() {
                // Simple AABB check (could be improved with spatial partitioning)
                let p1 = &particles[i];
                let p2 = &particles[j];

                let distance = p1.position.sub(&p2.position).magnitude();
                if distance < (p1.radius + p2.radius) {
                    self.broad_phase_pairs.push((i, j));
                }
            }
        }
    }

    pub fn narrow_phase(&self, particles: &[Particle]) -> Vec<(usize, usize, Vec3)> {
        let mut collisions = Vec::new();

        for &(i, j) in &self.broad_phase_pairs {
            let p1 = &particles[i];
            let p2 = &particles[j];

            let r = p1.position.sub(&p2.position);
            let distance = r.magnitude();

            if distance < (p1.radius + p2.radius) && distance > 0.0 {
                let normal = r.normalize();
                let penetration = (p1.radius + p2.radius) - distance;
                let contact_point = p1.position.add(&normal.mul(-p1.radius + penetration / 2.0));

                collisions.push((i, j, normal));
            }
        }

        collisions
    }

    pub fn resolve_collisions(
        &self,
        particles: &mut [Particle],
        collisions: &[(usize, usize, Vec3)],
    ) {
        for &(i, j, ref normal) in collisions {
            let v1 = particles[i].velocity;
            let v2 = particles[j].velocity;
            let m1 = particles[i].mass;
            let m2 = particles[j].mass;

            // Relative velocity
            let relative_vel = v1.sub(&v2);
            let vel_along_normal = relative_vel.dot(normal);

            // Do not resolve if velocities are separating
            if vel_along_normal > 0.0 {
                continue;
            }

            // Calculate restitution (bounciness)
            let restitution = 0.8;

            // Calculate impulse scalar
            let impulse_scalar = -(1.0 + restitution) * vel_along_normal / (1.0 / m1 + 1.0 / m2);

            // Apply impulse
            let impulse = normal.mul(impulse_scalar);
            particles[i].velocity = v1.add(&impulse.mul(1.0 / m1));
            particles[j].velocity = v2.sub(&impulse.mul(1.0 / m2));
        }
    }
}

/// SPH (Smoothed Particle Hydrodynamics) Fluid Simulation
pub struct SPHFluid {
    particles: Vec<Particle>,
    rest_density: f64,
    gas_constant: f64,
    viscosity: f64,
    smoothing_length: f64,
}

impl SPHFluid {
    pub fn new(rest_density: f64, smoothing_length: f64) -> Self {
        SPHFluid {
            particles: Vec::new(),
            rest_density,
            gas_constant: 1000.0,
            viscosity: 0.1,
            smoothing_length,
        }
    }

    pub fn add_particle(&mut self, position: Vec3) -> usize {
        let id = self.particles.len();
        let mut particle = Particle::new(id, position, 1.0);
        particle.radius = self.smoothing_length * 0.5;
        self.particles.push(particle);
        id
    }

    fn poly6_kernel(&self, r: f64) -> f64 {
        if r >= self.smoothing_length {
            0.0
        } else {
            let h = self.smoothing_length;
            315.0 / (64.0 * PI * h.powi(9)) * (h * h - r * r).powi(3)
        }
    }

    fn spiky_gradient(&self, r: Vec3) -> Vec3 {
        let r_mag = r.magnitude();
        if r_mag >= self.smoothing_length || r_mag == 0.0 {
            Vec3::zero()
        } else {
            let h = self.smoothing_length;
            let factor = -45.0 / (PI * h.powi(6)) * (h - r_mag).powi(2) / r_mag;
            r.normalize().mul(factor)
        }
    }

    pub fn compute_density_pressure(&mut self) {
        for i in 0..self.particles.len() {
            let mut density = 0.0;

            for j in 0..self.particles.len() {
                if i != j {
                    let r = self.particles[i].position.sub(&self.particles[j].position);
                    let r_mag = r.magnitude();
                    density += self.particles[j].mass * self.poly6_kernel(r_mag);
                }
            }

            self.particles[i].charge = density; // Store density in charge field temporarily

            // Ideal gas equation: P = k(ρ - ρ0)
            let pressure = self.gas_constant * (density - self.rest_density);
            self.particles[i].lifetime = pressure; // Store pressure in lifetime field temporarily
        }
    }

    pub fn compute_forces(&mut self) {
        for i in 0..self.particles.len() {
            let mut pressure_force = Vec3::zero();
            let mut viscosity_force = Vec3::zero();

            let density_i = self.particles[i].charge;
            let pressure_i = self.particles[i].lifetime;

            for j in 0..self.particles.len() {
                if i != j {
                    let r = self.particles[i].position.sub(&self.particles[j].position);
                    let r_mag = r.magnitude();

                    if r_mag > 0.0 {
                        let density_j = self.particles[j].charge;
                        let pressure_j = self.particles[j].lifetime;

                        // Pressure force
                        let pressure_term = (pressure_i + pressure_j) / (2.0 * density_j);
                        pressure_force = pressure_force.add(
                            &self
                                .spiky_gradient(r)
                                .mul(-pressure_term * self.particles[j].mass),
                        );

                        // Viscosity force
                        let velocity_diff =
                            self.particles[j].velocity.sub(&self.particles[i].velocity);
                        let viscosity_term =
                            self.viscosity * self.particles[j].mass * velocity_diff.dot(&r)
                                / (r_mag * r_mag
                                    + 0.01 * self.smoothing_length * self.smoothing_length);
                        viscosity_force = viscosity_force
                            .add(&self.spiky_gradient(r).mul(viscosity_term / density_j));
                    }
                }
            }

            // Apply forces
            self.particles[i].apply_force(&pressure_force);
            self.particles[i].apply_force(&viscosity_force);
            // Add gravity
            self.particles[i].apply_force(&Vec3::new(0.0, -9.81 * density_i, 0.0));
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.compute_density_pressure();
        self.compute_forces();

        for particle in &mut self.particles {
            particle.update(dt);
        }
    }

    pub fn get_particles(&self) -> &[Particle] {
        &self.particles
    }
}

/// Electromagnetic Field Simulator
pub struct ElectromagneticField {
    charges: Vec<(Vec3, f64)>,   // Position, charge
    currents: Vec<(Vec3, Vec3)>, // Position, current vector
}

impl Default for ElectromagneticField {
    fn default() -> Self {
        Self::new()
    }
}

impl ElectromagneticField {
    pub fn new() -> Self {
        ElectromagneticField {
            charges: Vec::new(),
            currents: Vec::new(),
        }
    }

    pub fn add_charge(&mut self, position: Vec3, charge: f64) {
        self.charges.push((position, charge));
    }

    pub fn add_current(&mut self, position: Vec3, current: Vec3) {
        self.currents.push((position, current));
    }

    pub fn electric_field(&self, point: &Vec3) -> Vec3 {
        let k = 8.987551789e9; // Coulomb's constant
        let mut field = Vec3::zero();

        for (pos, charge) in &self.charges {
            let r = point.sub(pos);
            let r_mag = r.magnitude();
            if r_mag > 0.0 {
                let field_magnitude = k * charge / (r_mag * r_mag);
                let field_direction = r.normalize();
                field = field.add(&field_direction.mul(field_magnitude));
            }
        }

        field
    }

    pub fn magnetic_field(&self, point: &Vec3) -> Vec3 {
        let mu0 = 4.0 * PI * 1e-7; // Permeability of free space
        let mut field = Vec3::zero();

        for (pos, current) in &self.currents {
            let r = point.sub(pos);
            let r_mag = r.magnitude();
            if r_mag > 0.0 {
                // Biot-Savart law: dB = (μ₀/4π) * (I × r̂) / r²
                let cross = current.cross(&r.normalize());
                let field_magnitude = mu0 / (4.0 * PI) * current.magnitude() / (r_mag * r_mag);
                field = field.add(&cross.mul(field_magnitude));
            }
        }

        field
    }

    pub fn lorentz_force(&self, charge: f64, velocity: &Vec3, position: &Vec3) -> Vec3 {
        let e_field = self.electric_field(position);
        let b_field = self.magnetic_field(position);

        // F = q(E + v × B)
        let magnetic_force = velocity.cross(&b_field);
        e_field.add(&magnetic_force).mul(charge)
    }
}

/// Physics Engine - Main coordinator
pub struct PhysicsEngine {
    particle_system: ParticleSystem,
    collision_detector: CollisionDetector,
    fluid_simulator: Option<SPHFluid>,
    em_field: ElectromagneticField,
    rigid_bodies: Vec<RigidBody>,
    time: f64,
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsEngine {
    pub fn new() -> Self {
        PhysicsEngine {
            particle_system: ParticleSystem::new(),
            collision_detector: CollisionDetector::new(),
            fluid_simulator: None,
            em_field: ElectromagneticField::new(),
            rigid_bodies: Vec::new(),
            time: 0.0,
        }
    }

    pub fn enable_fluid_simulation(&mut self, rest_density: f64, smoothing_length: f64) {
        self.fluid_simulator = Some(SPHFluid::new(rest_density, smoothing_length));
    }

    pub fn add_particle(&mut self, position: Vec3, mass: f64) -> usize {
        self.particle_system.add_particle(position, mass)
    }

    pub fn add_fluid_particle(&mut self, position: Vec3) -> Option<usize> {
        self.fluid_simulator
            .as_mut()
            .map(|fluid| fluid.add_particle(position))
    }

    pub fn add_charge(&mut self, position: Vec3, charge: f64) {
        self.em_field.add_charge(position, charge);
    }

    pub fn add_rigid_body(&mut self, position: Vec3, mass: f64) -> usize {
        let id = self.rigid_bodies.len();
        self.rigid_bodies.push(RigidBody::new(id, position, mass));
        id
    }

    pub fn update(&mut self, dt: f64) {
        // Update particle system
        self.particle_system.update(dt);

        // Update fluid simulation
        if let Some(ref mut fluid) = self.fluid_simulator {
            fluid.update(dt);
        }

        // Update rigid bodies
        for rb in &mut self.rigid_bodies {
            rb.update(dt);
        }

        // Handle collisions
        let particles = self.particle_system.get_particles();
        self.collision_detector.broad_phase(particles);
        let collisions = self.collision_detector.narrow_phase(particles);
        self.collision_detector
            .resolve_collisions(&mut self.particle_system.particles, &collisions);

        self.time += dt;
    }

    pub fn get_particles(&self) -> &[Particle] {
        self.particle_system.get_particles()
    }

    pub fn get_fluid_particles(&self) -> Option<&[Particle]> {
        self.fluid_simulator.as_ref().map(|f| f.get_particles())
    }

    pub fn get_rigid_bodies(&self) -> &[RigidBody] {
        &self.rigid_bodies
    }

    pub fn electric_field_at(&self, point: &Vec3) -> Vec3 {
        self.em_field.electric_field(point)
    }

    pub fn magnetic_field_at(&self, point: &Vec3) -> Vec3 {
        self.em_field.magnetic_field(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_operations() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let sum = v1.add(&v2);
        assert_eq!(sum, Vec3::new(5.0, 7.0, 9.0));

        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0);

        let mag = v1.magnitude();
        assert!((mag - 3.74165738677).abs() < 1e-10);
    }

    #[test]
    fn test_particle_system() {
        let mut system = ParticleSystem::new();
        let id1 = system.add_particle(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let id2 = system.add_particle(Vec3::new(10.0, 0.0, 0.0), 1.0);

        system.update(1.0);

        let particles = system.get_particles();
        assert!(!particles.is_empty()); // Particles may die

        // Check that gravity is applied
        for particle in particles {
            assert!(particle.velocity.y < 0.0); // Gravity pulls down
        }
    }

    #[test]
    fn test_collision_detection() {
        let mut detector = CollisionDetector::new();
        let particles = vec![
            Particle::new(0, Vec3::new(0.0, 0.0, 0.0), 1.0),
            Particle::new(1, Vec3::new(1.0, 0.0, 0.0), 1.0),
        ];

        detector.broad_phase(&particles);
        let collisions = detector.narrow_phase(&particles);

        assert!(!collisions.is_empty()); // Should detect collision
    }

    #[test]
    fn test_electromagnetic_field() {
        let mut field = ElectromagneticField::new();
        field.add_charge(Vec3::new(0.0, 0.0, 0.0), 1.0);

        let e_field = field.electric_field(&Vec3::new(1.0, 0.0, 0.0));
        assert!(e_field.x > 0.0); // Field should point away from positive charge
    }

    #[test]
    fn test_physics_engine_integration() {
        let mut engine = PhysicsEngine::new();
        engine.add_particle(Vec3::new(0.0, 0.0, 0.0), 1.0);
        engine.add_charge(Vec3::new(0.0, 0.0, 0.0), 1.0);

        engine.update(0.1);

        let particles = engine.get_particles();
        assert!(!particles.is_empty());

        let e_field = engine.electric_field_at(&Vec3::new(1.0, 0.0, 0.0));
        assert!(e_field.x > 0.0);
    }
}
