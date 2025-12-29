"""
GUL Biology
Bioinformatics utilities (DNA/RNA).

Status: ✅ Implemented
Priority: Low
"""

__version__ = "0.1.0"
__all__ = ['transcribe', 'translate', 'gc_content', 'reverse_complement']

CODON_TABLE = {
    'ATA':'I', 'ATC':'I', 'ATT':'I', 'ATG':'M',
    'ACA':'T', 'ACC':'T', 'ACG':'T', 'ACT':'T',
    'AAC':'N', 'AAT':'N', 'AAA':'K', 'AAG':'K',
    'AGC':'S', 'AGT':'S', 'AGA':'R', 'AGG':'R',
    'CTA':'L', 'CTC':'L', 'CTG':'L', 'CTT':'L',
    'CCA':'P', 'CCC':'P', 'CCG':'P', 'CCT':'P',
    'CAC':'H', 'CAT':'H', 'CAA':'Q', 'CAG':'Q',
    'CGA':'R', 'CGC':'R', 'CGG':'R', 'CGT':'R',
    'GTA':'V', 'GTC':'V', 'GTG':'V', 'GTT':'V',
    'GCA':'A', 'GCC':'A', 'GCG':'A', 'GCT':'A',
    'GAC':'D', 'GAT':'D', 'GAA':'E', 'GAG':'E',
    'GGA':'G', 'GGC':'G', 'GGG':'G', 'GGT':'G',
    'TCA':'S', 'TCC':'S', 'TCG':'S', 'TCT':'S',
    'TTC':'F', 'TTT':'F', 'TTA':'L', 'TTG':'L',
    'TAC':'Y', 'TAT':'Y', 'TAA':'_', 'TAG':'_',
    'TGC':'C', 'TGT':'C', 'TGA':'_', 'TGG':'W',
}

def transcribe(dna: str) -> str:
    """DNA to RNA"""
    return dna.replace('T', 'U')

def translate(seq: str) -> str:
    """DNA/RNA to Protein"""
    seq = seq.replace('U', 'T') # Standardize to DNA for lookup
    protein = []
    if len(seq) % 3 == 0:
        for i in range(0, len(seq), 3):
            codon = seq[i:i+3]
            protein.append(CODON_TABLE.get(codon, 'X'))
    return "".join(protein)

def gc_content(seq: str) -> float:
    """Calculate GC content percentage"""
    if not seq: return 0.0
    g = seq.count('G') + seq.count('g')
    c = seq.count('C') + seq.count('c')
    return (g + c) / len(seq) * 100

def reverse_complement(dna: str) -> str:
    """Reverse complement of DNA"""
    comp = {'A':'T', 'T':'A', 'G':'C', 'C':'G', 'a':'t', 't':'a', 'g':'c', 'c':'g'}
    return "".join(comp.get(base, base) for base in reversed(dna))
