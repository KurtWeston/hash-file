use anyhow::Result;
use std::path::Path;
use crate::hasher::Hasher;

pub struct Verifier {
    hasher: Hasher,
}

impl Verifier {
    pub fn new(hasher: Hasher) -> Self {
        Self { hasher }
    }

    pub fn verify_file<P: AsRef<Path>>(&self, path: P, expected: &str) -> Result<bool> {
        let calculated = self.hasher.hash_file(path)?;
        Ok(calculated.eq_ignore_ascii_case(expected.trim()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hasher::HashAlgorithm;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_verify_file_success() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        
        let hasher = Hasher::new(HashAlgorithm::Sha256);
        let hash = hasher.hash_file(file.path()).unwrap();
        
        let verifier = Verifier::new(hasher);
        let result = verifier.verify_file(file.path(), &hash).unwrap();
        
        assert!(result);
    }

    #[test]
    fn test_verify_file_failure() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        
        let hasher = Hasher::new(HashAlgorithm::Sha256);
        let verifier = Verifier::new(hasher);
        let result = verifier.verify_file(file.path(), "wronghash123").unwrap();
        
        assert!(!result);
    }

    #[test]
    fn test_verify_file_case_insensitive() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "case test").unwrap();
        
        let hasher = Hasher::new(HashAlgorithm::Sha256);
        let hash = hasher.hash_file(file.path()).unwrap();
        let uppercase_hash = hash.to_uppercase();
        
        let verifier = Verifier::new(hasher);
        let result = verifier.verify_file(file.path(), &uppercase_hash).unwrap();
        
        assert!(result);
    }

    #[test]
    fn test_verify_file_with_whitespace() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "whitespace test").unwrap();
        
        let hasher = Hasher::new(HashAlgorithm::Sha256);
        let hash = hasher.hash_file(file.path()).unwrap();
        let hash_with_spaces = format!("  {}  ", hash);
        
        let verifier = Verifier::new(hasher);
        let result = verifier.verify_file(file.path(), &hash_with_spaces).unwrap();
        
        assert!(result);
    }

    #[test]
    fn test_verify_nonexistent_file() {
        let hasher = Hasher::new(HashAlgorithm::Sha256);
        let verifier = Verifier::new(hasher);
        let result = verifier.verify_file("/nonexistent/file.txt", "somehash");
        
        assert!(result.is_err());
    }
}