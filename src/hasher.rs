use anyhow::{Context, Result};
use md5::{Md5, Digest as Md5Digest};
use sha1::{Sha1, Digest as Sha1Digest};
use sha2::{Sha256, Sha512, Digest as Sha2Digest};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Blake3,
}

pub struct Hasher {
    algorithm: HashAlgorithm,
}

impl Hasher {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        Self { algorithm }
    }

    pub fn hash_file<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let file = File::open(path.as_ref())
            .context("Failed to open file")?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0u8; 8192];

        match self.algorithm {
            HashAlgorithm::Md5 => {
                let mut hasher = Md5::new();
                loop {
                    let n = reader.read(&mut buffer)?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                }
                Ok(hex::encode(hasher.finalize()))
            }
            HashAlgorithm::Sha1 => {
                let mut hasher = Sha1::new();
                loop {
                    let n = reader.read(&mut buffer)?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                }
                Ok(hex::encode(hasher.finalize()))
            }
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                loop {
                    let n = reader.read(&mut buffer)?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                }
                Ok(hex::encode(hasher.finalize()))
            }
            HashAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                loop {
                    let n = reader.read(&mut buffer)?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                }
                Ok(hex::encode(hasher.finalize()))
            }
            HashAlgorithm::Blake3 => {
                let mut hasher = blake3::Hasher::new();
                loop {
                    let n = reader.read(&mut buffer)?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                }
                Ok(hasher.finalize().to_hex().to_string())
            }
        }
    }

    pub fn find_duplicates<P: AsRef<Path>>(&self, paths: &[P]) -> Result<HashMap<String, Vec<String>>> {
        let results: Vec<_> = paths.par_iter()
            .filter_map(|path| {
                let path_str = path.as_ref().display().to_string();
                self.hash_file(path).ok().map(|hash| (hash, path_str))
            })
            .collect();

        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for (hash, path) in results {
            map.entry(hash).or_insert_with(Vec::new).push(path);
        }
        
        Ok(map.into_iter().filter(|(_, v)| v.len() > 1).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_hash_file_sha256() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "hello world").unwrap();
        
        let hasher = Hasher::new(HashAlgorithm::Sha256);
        let hash = hasher.hash_file(file.path()).unwrap();
        
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_hash_file_md5() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test").unwrap();
        
        let hasher = Hasher::new(HashAlgorithm::Md5);
        let hash = hasher.hash_file(file.path()).unwrap();
        
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hash_file_blake3() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "blake3 test").unwrap();
        
        let hasher = Hasher::new(HashAlgorithm::Blake3);
        let hash = hasher.hash_file(file.path()).unwrap();
        
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_hash_file_nonexistent() {
        let hasher = Hasher::new(HashAlgorithm::Sha256);
        let result = hasher.hash_file("/nonexistent/file.txt");
        
        assert!(result.is_err());
    }

    #[test]
    fn test_find_duplicates() {
        let mut file1 = NamedTempFile::new().unwrap();
        let mut file2 = NamedTempFile::new().unwrap();
        let mut file3 = NamedTempFile::new().unwrap();
        
        writeln!(file1, "same content").unwrap();
        writeln!(file2, "same content").unwrap();
        writeln!(file3, "different").unwrap();
        
        let hasher = Hasher::new(HashAlgorithm::Sha256);
        let paths = vec![file1.path(), file2.path(), file3.path()];
        let duplicates = hasher.find_duplicates(&paths).unwrap();
        
        assert_eq!(duplicates.len(), 1);
        let dup_files = duplicates.values().next().unwrap();
        assert_eq!(dup_files.len(), 2);
    }

    #[test]
    fn test_find_duplicates_no_duplicates() {
        let mut file1 = NamedTempFile::new().unwrap();
        let mut file2 = NamedTempFile::new().unwrap();
        
        writeln!(file1, "unique1").unwrap();
        writeln!(file2, "unique2").unwrap();
        
        let hasher = Hasher::new(HashAlgorithm::Sha256);
        let paths = vec![file1.path(), file2.path()];
        let duplicates = hasher.find_duplicates(&paths).unwrap();
        
        assert_eq!(duplicates.len(), 0);
    }
}