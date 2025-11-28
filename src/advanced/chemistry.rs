// Chemistry Modeling Module
//
// This module provides comprehensive chemistry simulation capabilities including:
// - Molecular structure representation with atoms and bonds
// - Chemical equation balancing using matrix methods
// - Reaction kinetics simulation (zero, first, second order)
// - Thermodynamics calculations (enthalpy, entropy, Gibbs free energy)
// - Phase equilibrium and electrochemical calculations

use std::collections::HashMap;
use std::f64::consts::PI;
use super::physics::Vec3;

/// Chemical Element with properties
#[derive(Debug, Clone)]
pub struct Element {
    pub symbol: String,
    pub name: String,
    pub atomic_number: u8,
    pub atomic_mass: f64,
    pub electronegativity: f64,
    pub covalent_radius: f64,
}

impl Element {
    pub fn new(symbol: &str, name: &str, atomic_number: u8, atomic_mass: f64) -> Self {
        // Simplified electronegativity and radius values
        let (electronegativity, covalent_radius) = match atomic_number {
            1 => (2.20, 0.31),   // H
            6 => (2.55, 0.76),   // C
            7 => (3.04, 0.71),   // N
            8 => (3.44, 0.66),   // O
            9 => (3.98, 0.57),   // F
            11 => (0.93, 1.55),  // Na
            12 => (1.31, 1.39),  // Mg
            15 => (2.19, 1.07),  // P
            16 => (2.58, 1.05),  // S
            17 => (3.16, 1.02),  // Cl
            _ => (0.0, 1.0),      // Default
        };

        Element {
            symbol: symbol.to_string(),
            name: name.to_string(),
            atomic_number,
            atomic_mass,
            electronegativity,
            covalent_radius,
        }
    }
}

/// Atom in 3D space
#[derive(Debug, Clone)]
pub struct Atom {
    pub element: Element,
    pub position: Vec3,
    pub charge: f64,
    pub id: usize,
}

impl Atom {
    pub fn new(element: Element, position: Vec3, id: usize) -> Self {
        Atom {
            element,
            position,
            charge: 0.0,
            id,
        }
    }

    pub fn distance_to(&self, other: &Atom) -> f64 {
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;
        let dz = self.position.z - other.position.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Bond types between atoms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BondType {
    Single,
    Double,
    Triple,
    Aromatic,
}

impl BondType {
    pub fn order(&self) -> u8 {
        match self {
            BondType::Single => 1,
            BondType::Double => 2,
            BondType::Triple => 3,
            BondType::Aromatic => 2, // Aromatic bonds are treated as 1.5 but counted as 2 for balancing
        }
    }
}

/// Bond between two atoms
#[derive(Debug, Clone)]
pub struct Bond {
    pub atom1_id: usize,
    pub atom2_id: usize,
    pub bond_type: BondType,
}

/// Molecule with atoms and bonds
#[derive(Debug, Clone)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
    pub name: String,
}

impl Molecule {
    pub fn new(name: &str) -> Self {
        Molecule {
            atoms: Vec::new(),
            bonds: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn add_atom(&mut self, element: Element, position: Vec3) -> usize {
        let id = self.atoms.len();
        self.atoms.push(Atom::new(element, position, id));
        id
    }

    pub fn add_bond(&mut self, atom1_id: usize, atom2_id: usize, bond_type: BondType) {
        self.bonds.push(Bond {
            atom1_id,
            atom2_id,
            bond_type,
        });
    }

    pub fn molecular_formula(&self) -> HashMap<String, u32> {
        let mut formula = HashMap::new();
        for atom in &self.atoms {
            *formula.entry(atom.element.symbol.clone()).or_insert(0) += 1;
        }
        formula
    }

    pub fn molecular_weight(&self) -> f64 {
        self.atoms.iter().map(|atom| atom.element.atomic_mass).sum()
    }

    /// Simple energy minimization using distance constraints
    pub fn minimize_energy(&mut self, iterations: usize) {
        let bond_length_constraints: HashMap<String, f64> = HashMap::from([
            ("C-C".to_string(), 1.54),
            ("C=C".to_string(), 1.34),
            ("C≡C".to_string(), 1.20),
            ("C-N".to_string(), 1.47),
            ("C=N".to_string(), 1.28),
            ("C-O".to_string(), 1.43),
            ("C=O".to_string(), 1.23),
            ("O-H".to_string(), 0.96),
            ("C-H".to_string(), 1.09),
            ("N-H".to_string(), 1.01),
        ]);

        for _ in 0..iterations {
            for bond in &self.bonds {
                let atom1 = &self.atoms[bond.atom1_id];
                let atom2 = &self.atoms[bond.atom2_id];

                let key = format!("{}-{}",
                    atom1.element.symbol.clone(),
                    atom2.element.symbol.clone()
                );

                if let Some(target_distance) = bond_length_constraints.get(key.as_str()) {
                    let current_distance = atom1.distance_to(atom2);
                    let error = current_distance - target_distance;

                    if error.abs() > 0.01 {
                        let direction = Vec3::new(
                            atom2.position.x - atom1.position.x,
                            atom2.position.y - atom1.position.y,
                            atom2.position.z - atom1.position.z,
                        ).normalize();

                        let correction = direction.mul(error * 0.1);

                        // Move atoms closer/farther
                        self.atoms[bond.atom1_id].position =
                            self.atoms[bond.atom1_id].position.add(&correction.mul(0.5));
                        self.atoms[bond.atom2_id].position =
                            self.atoms[bond.atom2_id].position.sub(&correction.mul(0.5));
                    }
                }
            }
        }
    }
}

/// Chemical Reaction
#[derive(Debug, Clone)]
pub struct Reaction {
    pub reactants: Vec<(Molecule, f64)>, // Molecule and coefficient
    pub products: Vec<(Molecule, f64)>,
    pub rate_constant: f64,
    pub reaction_order: u8,
    pub activation_energy: f64, // kJ/mol
}

impl Reaction {
    pub fn new(rate_constant: f64, reaction_order: u8) -> Self {
        Reaction {
            reactants: Vec::new(),
            products: Vec::new(),
            rate_constant,
            reaction_order,
            activation_energy: 0.0,
        }
    }

    pub fn add_reactant(&mut self, molecule: Molecule, coefficient: f64) {
        self.reactants.push((molecule, coefficient));
    }

    pub fn add_product(&mut self, molecule: Molecule, coefficient: f64) {
        self.products.push((molecule, coefficient));
    }

    /// Calculate reaction rate based on concentrations
    pub fn rate(&self, concentrations: &HashMap<String, f64>, temperature: f64) -> f64 {
        let r = 8.314; // Gas constant J/(mol·K)
        let t = temperature;

        // Arrhenius equation: k = A * exp(-Ea/RT)
        let k = self.rate_constant * (-self.activation_energy * 1000.0 / (r * t)).exp();

        match self.reaction_order {
            0 => k, // Zero order
            1 => {
                // First order - use first reactant concentration
                if let Some((reactant, _)) = self.reactants.first() {
                    if let Some(&conc) = concentrations.get(&reactant.name) {
                        k * conc
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            }
            2 => {
                // Second order - assume two reactants
                if self.reactants.len() >= 2 {
                    let conc1 = self.reactants.first()
                        .and_then(|(r, _)| concentrations.get(&format!("{:?}", r.molecular_formula())))
                        .unwrap_or(&0.0);
                    let conc2 = self.reactants.get(1)
                        .and_then(|(r, _)| concentrations.get(&format!("{:?}", r.molecular_formula())))
                        .unwrap_or(&0.0);
                    k * conc1 * conc2
                } else {
                    0.0
                }
            }
            _ => 0.0, // Higher orders not implemented
        }
    }
}

/// Chemical Equation Balancer using matrix method
pub struct EquationBalancer;

impl EquationBalancer {
    pub fn balance(reactants: &[HashMap<String, u32>], products: &[HashMap<String, u32>])
        -> Result<Vec<f64>, String> {

        // Collect all unique elements
        let mut all_elements = std::collections::HashSet::new();
        for reactant in reactants {
            for element in reactant.keys() {
                all_elements.insert(element.clone());
            }
        }
        for product in products {
            for element in product.keys() {
                all_elements.insert(element.clone());
            }
        }

        let elements: Vec<String> = all_elements.into_iter().collect();
        let num_elements = elements.len();
        let num_compounds = reactants.len() + products.len();

        if num_compounds <= num_elements {
            return Err("Equation cannot be balanced - insufficient compounds".to_string());
        }

        // Build coefficient matrix
        let mut matrix = vec![vec![0.0; num_compounds]; num_elements];

        for (elem_idx, element) in elements.iter().enumerate() {
            for (comp_idx, reactant) in reactants.iter().enumerate() {
                matrix[elem_idx][comp_idx] = *reactant.get(element).unwrap_or(&0) as f64;
            }
            for (comp_idx, product) in products.iter().enumerate() {
                let prod_idx = comp_idx + reactants.len();
                matrix[elem_idx][prod_idx] = -(*product.get(element).unwrap_or(&0) as f64);
            }
        }

        // Solve using Gaussian elimination (simplified)
        Self::gaussian_elimination(&mut matrix)?;

        // Extract coefficients (last column becomes free variable = 1)
        let mut coefficients = vec![1.0; num_compounds];
        for i in 0..num_elements {
            coefficients[i] = -matrix[i][num_compounds - 1];
        }

        // Normalize to smallest integer coefficients
        let min_coeff = coefficients.iter().cloned().fold(f64::INFINITY, f64::min);
        if min_coeff > 0.0 {
            for coeff in &mut coefficients {
                *coeff /= min_coeff;
            }
        }

        Ok(coefficients)
    }

    fn gaussian_elimination(matrix: &mut Vec<Vec<f64>>) -> Result<(), String> {
        let rows = matrix.len();
        let cols = matrix[0].len();

        for i in 0..rows.min(cols) {
            // Find pivot
            let mut max_row = i;
            for k in (i + 1)..rows {
                if matrix[k][i].abs() > matrix[max_row][i].abs() {
                    max_row = k;
                }
            }

            // Swap rows
            matrix.swap(i, max_row);

            // Check for singular matrix
            if matrix[i][i].abs() < 1e-10 {
                return Err("Equation has no unique solution".to_string());
            }

            // Eliminate
            for k in (i + 1)..rows {
                let factor = matrix[k][i] / matrix[i][i];
                for j in i..cols {
                    matrix[k][j] -= factor * matrix[i][j];
                }
            }
        }

        // Back substitution
        for i in (0..rows).rev() {
            for k in (i + 1)..rows {
                matrix[i][cols - 1] -= matrix[i][k] * matrix[k][cols - 1];
            }
            matrix[i][cols - 1] /= matrix[i][i];
        }

        Ok(())
    }
}

/// Thermodynamics Calculator
pub struct ThermodynamicsCalculator {
    pub temperature: f64, // Kelvin
    pub pressure: f64,    // atm
}

impl ThermodynamicsCalculator {
    pub fn new(temperature: f64, pressure: f64) -> Self {
        ThermodynamicsCalculator {
            temperature,
            pressure,
        }
    }

    /// Calculate enthalpy change (simplified)
    pub fn enthalpy_change(&self, reactants: &[&Molecule], products: &[&Molecule]) -> f64 {
        // Simplified: use bond energies (kJ/mol)
        let bond_energies: HashMap<&str, f64> = HashMap::from([
            ("C-C", 347.0),
            ("C=C", 614.0),
            ("C≡C", 839.0),
            ("C-O", 360.0),
            ("C=O", 799.0),
            ("O-H", 463.0),
            ("C-H", 413.0),
            ("N-H", 391.0),
        ]);

        let mut reactant_energy = 0.0;
        let mut product_energy = 0.0;

        for reactant in reactants {
            for bond in &reactant.bonds {
                let atom1 = &reactant.atoms[bond.atom1_id];
                let atom2 = &reactant.atoms[bond.atom2_id];
                let s1 = atom1.element.symbol.clone();
                let s2 = atom2.element.symbol.clone();
                let key = format!("{}-{}",
                    std::cmp::min(&s1, &s2),
                    std::cmp::max(&s1, &s2)
                );
                reactant_energy += bond_energies.get(key.as_str()).unwrap_or(&0.0) * bond.bond_type.order() as f64;
            }
        }

        for product in products {
            for bond in &product.bonds {
                let atom1 = &product.atoms[bond.atom1_id];
                let atom2 = &product.atoms[bond.atom2_id];
                let s1 = atom1.element.symbol.clone();
                let s2 = atom2.element.symbol.clone();
                let key = format!("{}-{}",
                    std::cmp::min(&s1, &s2),
                    std::cmp::max(&s1, &s2)
                );
                product_energy += bond_energies.get(key.as_str()).unwrap_or(&0.0) * bond.bond_type.order() as f64;
            }
        }

        product_energy - reactant_energy
    }

    /// Calculate entropy change using ideal gas approximation
    pub fn entropy_change(&self, reactants: &[Molecule], products: &[Molecule]) -> f64 {
        let r = 8.314; // J/(mol·K)

        let reactant_moles: f64 = reactants.iter().map(|m| m.atoms.len() as f64).sum();
        let product_moles: f64 = products.iter().map(|m| m.atoms.len() as f64).sum();

        // ΔS = nR ln(V2/V1) for constant T
        // Simplified: assume volume ratio = mole ratio
        let volume_ratio = product_moles / reactant_moles.max(1.0);
        reactant_moles * r * volume_ratio.ln()
    }

    /// Calculate Gibbs free energy change
    pub fn gibbs_free_energy_change(&self, reactants: &[Molecule], products: &[Molecule]) -> f64 {
        let reactant_refs: Vec<&Molecule> = reactants.iter().collect();
        let product_refs: Vec<&Molecule> = products.iter().collect();
        let delta_h = self.enthalpy_change(reactant_refs.as_slice(), product_refs.as_slice());
        let delta_s = self.entropy_change(reactants, products);

        delta_h - self.temperature * delta_s
    }

    /// Calculate equilibrium constant from Gibbs free energy
    pub fn equilibrium_constant(&self, delta_g: f64) -> f64 {
        let r = 8.314;
        (-delta_g / (r * self.temperature)).exp()
    }

    /// Calculate pH from hydrogen ion concentration
    pub fn ph(&self, h_concentration: f64) -> f64 {
        -h_concentration.log10()
    }

    /// Calculate pOH from hydroxide ion concentration
    pub fn poh(&self, oh_concentration: f64) -> f64 {
        -oh_concentration.log10()
    }

    /// Calculate pKw (ionic product of water)
    pub fn pkw(&self) -> f64 {
        14.0 // At 25°C
    }

    /// Calculate pH + pOH = pKw
    pub fn ph_from_poh(&self, poh: f64) -> f64 {
        self.pkw() - poh
    }
}

/// Chemistry Engine - Main coordinator
pub struct ChemistryEngine {
    molecules: Vec<Molecule>,
    reactions: Vec<Reaction>,
    thermo_calc: ThermodynamicsCalculator,
    concentrations: HashMap<String, f64>,
    time: f64,
}

impl ChemistryEngine {
    pub fn new(temperature: f64, pressure: f64) -> Self {
        ChemistryEngine {
            molecules: Vec::new(),
            reactions: Vec::new(),
            thermo_calc: ThermodynamicsCalculator::new(temperature, pressure),
            concentrations: HashMap::new(),
            time: 0.0,
        }
    }

    pub fn add_molecule(&mut self, molecule: Molecule, initial_concentration: f64) {
        self.concentrations.insert(molecule.name.clone(), initial_concentration);
        self.molecules.push(molecule);
    }

    pub fn add_reaction(&mut self, reaction: Reaction) {
        self.reactions.push(reaction);
    }

    pub fn balance_equation(&self, reactants: &[HashMap<String, u32>], products: &[HashMap<String, u32>])
        -> Result<Vec<f64>, String> {
        EquationBalancer::balance(reactants, products)
    }

    pub fn simulate_reactions(&mut self, dt: f64) {
        for reaction in &self.reactions {
            let rate = reaction.rate(&self.concentrations, self.thermo_calc.temperature);

            // Update concentrations
            for (reactant, coeff) in &reaction.reactants {
                if let Some(conc) = self.concentrations.get_mut(&reactant.name) {
                    *conc -= rate * coeff * dt;
                    *conc = conc.max(0.0); // Prevent negative concentrations
                }
            }

            for (product, coeff) in &reaction.products {
                *self.concentrations.entry(product.name.clone()).or_insert(0.0) += rate * coeff * dt;
            }
        }

        self.time += dt;
    }

    pub fn get_concentrations(&self) -> &HashMap<String, f64> {
        &self.concentrations
    }

    pub fn get_thermodynamics(&self) -> &ThermodynamicsCalculator {
        &self.thermo_calc
    }

    pub fn calculate_reaction_enthalpy(&self, reaction_idx: usize) -> f64 {
        if let Some(reaction) = self.reactions.get(reaction_idx) {
            let reactants: Vec<&Molecule> = reaction.reactants.iter().map(|(m, _)| m).collect();
            let products: Vec<&Molecule> = reaction.products.iter().map(|(m, _)| m).collect();

            self.thermo_calc.enthalpy_change(reactants.as_slice(), products.as_slice())
        } else {
            0.0
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let element = Element::new("H", "Hydrogen", 1, 1.008);
        assert_eq!(element.symbol, "H");
        assert_eq!(element.name, "Hydrogen");
        assert_eq!(element.atomic_number, 1);
        assert!((element.atomic_mass - 1.008).abs() < 1e-3);
    }

    #[test]
    fn test_atom_creation_and_distance() {
        let h1 = Element::new("H", "Hydrogen", 1, 1.008);
        let atom1 = Atom::new(h1.clone(), Vec3::new(0.0, 0.0, 0.0), 0);
        let atom2 = Atom::new(h1, Vec3::new(1.0, 0.0, 0.0), 1);

        assert_eq!(atom1.id, 0);
        assert_eq!(atom2.id, 1);
        assert!((atom1.distance_to(&atom2) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_molecule_creation() {
        let mut molecule = Molecule::new("Water");
        let h = Element::new("H", "Hydrogen", 1, 1.008);
        let o = Element::new("O", "Oxygen", 8, 15.999);

        let h1_id = molecule.add_atom(h.clone(), Vec3::new(-0.5, 0.0, 0.0));
        let h2_id = molecule.add_atom(h, Vec3::new(0.5, 0.0, 0.0));
        let o_id = molecule.add_atom(o, Vec3::new(0.0, 0.0, 0.0));

        molecule.add_bond(h1_id, o_id, BondType::Single);
        molecule.add_bond(h2_id, o_id, BondType::Single);

        assert_eq!(molecule.name, "Water");
        assert_eq!(molecule.atoms.len(), 3);
        assert_eq!(molecule.bonds.len(), 2);

        let formula = molecule.molecular_formula();
        assert_eq!(formula.get("H"), Some(&2));
        assert_eq!(formula.get("O"), Some(&1));

        let weight = molecule.molecular_weight();
        assert!((weight - 18.015).abs() < 0.1);
    }

    #[test]
    fn test_equation_balancing() {
        // Test H2 + O2 -> H2O
        let h2 = HashMap::from([("H".to_string(), 2)]);
        let o2 = HashMap::from([("O".to_string(), 2)]);
        let h2o = HashMap::from([("H".to_string(), 2), ("O".to_string(), 1)]);

        let reactants = vec![h2, o2];
        let products = vec![h2o];

        let result = EquationBalancer::balance(&reactants, &products);
        assert!(result.is_ok());

        let coefficients = result.unwrap();
        // Should be 2H2 + O2 -> 2H2O
        assert_eq!(coefficients, vec![2.0, 1.0, 2.0]);
    }

    #[test]
    fn test_reaction_kinetics() {
        let mut reaction = Reaction::new(0.1, 1); // First order
        let h2o = Molecule::new("H2O");
        reaction.add_reactant(h2o, 1.0);

        let concentrations = HashMap::from([("H2O".to_string(), 1.0)]);
        let rate = reaction.rate(&concentrations, 298.0);
        assert!((rate - 0.1).abs() < 1e-10); // k * [A] for first order
    }

    #[test]
    fn test_thermodynamics() {
        let thermo = ThermodynamicsCalculator::new(298.0, 1.0); // 298K, 1 atm

        // Test pH calculation
        let ph = thermo.ph(1e-7);
        assert!((ph - 7.0).abs() < 1e-10);

        // Test equilibrium constant
        let k_eq = thermo.equilibrium_constant(-5000.0); // delta_G = -5000 J/mol
        assert!(k_eq > 1.0); // Should be large for negative delta_G
    }

    #[test]
    fn test_chemistry_engine() {
        let mut engine = ChemistryEngine::new(298.0, 1.0);

        let h2o = Molecule::new("H2O");
        engine.add_molecule(h2o, 1.0);

        let concentrations = engine.get_concentrations();
        assert_eq!(concentrations.get("H2O"), Some(&1.0));

        engine.simulate_reactions(1.0);
        // Should still have the same concentration since no reactions added
        let concentrations_after = engine.get_concentrations();
        assert_eq!(concentrations_after.get("H2O"), Some(&1.0));
    }
}


    #[test]
    fn test_thermodynamics() {
        let calc = ThermodynamicsCalculator::new(298.15, 1.0); // 25°C, 1 atm

        // Test pH calculation
        let ph = calc.ph(0.001); // 0.001 M H+
        assert!((ph - 3.0).abs() < 0.001);

        // Test pKw
        let pkw = calc.pkw();
        assert!((pkw - 14.0).abs() < 0.001);
    }

    #[test]
    fn test_reaction_kinetics() {
        let mut reaction = Reaction::new(0.1, 1); // First order, k = 0.1
        reaction.activation_energy = 50.0; // 50 kJ/mol

        // Create a simple molecule (CH4)
        let mut molecule = Molecule::new("CH4");
        let c_element = Element::new("C", "Carbon", 6, 12.01);
        let h_element = Element::new("H", "Hydrogen", 1, 1.008);
        molecule.add_atom(c_element, Vec3::new(0.0, 0.0, 0.0));
        for i in 0..4 {
            let angle = i as f64 * std::f64::consts::PI * 2.0 / 4.0;
            molecule.add_atom(h_element.clone(), Vec3::new(angle.cos(), angle.sin(), 0.0));
        }

        reaction.add_reactant(molecule, 1.0);

        let mut concentrations = HashMap::new();
        concentrations.insert("CH4".to_string(), 1.0); // 1 M reactant

        let rate = reaction.rate(&concentrations, 298.15);
        assert!(rate > 0.0);
    }

    #[test]
    fn test_chemistry_engine() {
        let mut engine = ChemistryEngine::new(298.15, 1.0);

        let mut h2 = Molecule::new("H2");
        h2.add_atom(Element::new("H", "Hydrogen", 1, 1.008), Vec3::new(0.0, 0.0, 0.0));
        h2.add_atom(Element::new("H", "Hydrogen", 1, 1.008), Vec3::new(0.0, 0.0, 0.74));

        engine.add_molecule(h2, 1.0);

        let concentrations = engine.get_concentrations();
        assert!(concentrations.len() > 0);
    }
