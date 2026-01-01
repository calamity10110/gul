use gul_bio::{DnaTools, ProteinTools};

fn main() {
    let seq = "ACGT";
    println!("Original: {}", seq);
    println!("RevComp:  {}", DnaTools::reverse_complement(seq));

    let protein = "MKWVTFISLLLLFSSAYSRGVFRR";
    println!("Protein Mass: {}", ProteinTools::mass(protein));
}
