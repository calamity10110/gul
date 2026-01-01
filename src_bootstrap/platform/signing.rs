// Package Signing and Verification
// Provides cryptographic signing using Ed25519

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSignature {
    pub algorithm: String,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
    pub timestamp: u64,
    pub package_hash: String,
}

pub struct PackageSigner {
    // In production, this would use ed25519_dalek
    // For now, we'll use a simplified version
    private_key: Vec<u8>,
    public_key: Vec<u8>,
}

impl PackageSigner {
    pub fn new() -> Self {
        // In production: generate actual Ed25519 keypair
        // let keypair = Keypair::generate(&mut OsRng);
        Self {
            private_key: vec![0u8; 32], // Placeholder
            public_key: vec![0u8; 32],  // Placeholder
        }
    }

    pub fn from_keys(private_key: Vec<u8>, public_key: Vec<u8>) -> Self {
        Self {
            private_key,
            public_key,
        }
    }

    /// Sign a package by hashing its content and creating a signature
    pub fn sign_package(&self, package_data: &[u8]) -> Result<PackageSignature, String> {
        // Create SHA-256 hash of package
        let mut hasher = Sha256::new();
        hasher.update(package_data);
        let hash = hasher.finalize();
        let package_hash = hex::encode(hash);

        // In production: Sign the hash with Ed25519
        // let signature = self.keypair.sign(&hash);

        // Placeholder signature
        let signature = self.create_signature(&hash);

        Ok(PackageSignature {
            algorithm: "Ed25519".to_string(),
            signature,
            public_key: self.public_key.clone(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            package_hash,
        })
    }

    /// Verify a package signature
    pub fn verify_signature(&self, package_data: &[u8], signature: &PackageSignature) -> bool {
        // Verify hash matches
        let mut hasher = Sha256::new();
        hasher.update(package_data);
        let hash = hasher.finalize();
        let computed_hash = hex::encode(hash);

        if computed_hash != signature.package_hash {
            return false;
        }

        // In production: Verify Ed25519 signature
        // let public_key = PublicKey::from_bytes(&signature.public_key).unwrap();
        // let sig = Signature::from_bytes(&signature.signature).unwrap();
        // public_key.verify(&hash, &sig).is_ok()

        // Placeholder verification
        self.verify_signature_internal(&hash, &signature.signature)
    }

    // Placeholder methods - replace with actual Ed25519 in production
    fn create_signature(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.private_key);
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    fn verify_signature_internal(&self, data: &[u8], signature: &[u8]) -> bool {
        let expected = self.create_signature(data);
        expected == signature
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }
}

impl Default for PackageSigner {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to compute package checksum
pub fn compute_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let signer = PackageSigner::new();
        let package_data = b"test package content";

        let signature = signer.sign_package(package_data).unwrap();

        assert_eq!(signature.algorithm, "Ed25519");
        assert!(!signature.signature.is_empty());
        assert!(!signature.package_hash.is_empty());

        // Verify signature
        let is_valid = signer.verify_signature(package_data, &signature);
        assert!(is_valid);
    }

    #[test]
    fn test_verify_fails_with_wrong_data() {
        let signer = PackageSigner::new();
        let package_data = b"test package content";
        let wrong_data = b"wrong content";

        let signature = signer.sign_package(package_data).unwrap();

        // Should fail with different data
        let is_valid = signer.verify_signature(wrong_data, &signature);
        assert!(!is_valid);
    }

    #[test]
    fn test_compute_checksum() {
        let data = b"test data";
        let checksum = compute_checksum(data);

        assert_eq!(checksum.len(), 64); // SHA-256 produces 64 hex chars

        // Same data should produce same checksum
        let checksum2 = compute_checksum(data);
        assert_eq!(checksum, checksum2);
    }
}
