use bio::alphabets::dna;

pub struct DnaTools;

impl DnaTools {
    pub fn reverse_complement(sequence: &str) -> String {
        String::from_utf8(dna::revcomp(sequence.as_bytes())).unwrap()
    }

    pub fn is_valid(sequence: &str) -> bool {
        sequence.bytes().all(|b| {
            matches!(
                b,
                b'A' | b'C' | b'G' | b'T' | b'N' | b'a' | b'c' | b'g' | b't' | b'n'
            )
        })
    }
}

pub struct ProteinTools;

impl ProteinTools {
    pub fn mass(sequence: &str) -> f64 {
        // Mock mass calculation
        sequence.len() as f64 * 110.0
    }
}
