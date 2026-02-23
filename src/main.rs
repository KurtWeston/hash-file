use anyhow::Result;
use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::path::PathBuf;

mod hasher;
mod verifier;

use hasher::{HashAlgorithm, Hasher};
use verifier::Verifier;

#[derive(Parser)]
#[command(name = "hash-file")]
#[command(about = "Fast CLI tool to calculate and verify cryptographic hashes")]
struct Cli {
    #[arg(help = "Files or directories to hash")]
    paths: Vec<PathBuf>,

    #[arg(short, long, value_enum, default_value = "sha256")]
    algorithm: Algorithm,

    #[arg(short, long, help = "Verify against checksum (hash or file)")]
    verify: Option<String>,

    #[arg(short, long, help = "Recursive directory processing")]
    recursive: bool,

    #[arg(short = 'q', long, help = "Output only hash value")]
    quiet: bool,

    #[arg(short, long, value_enum, default_value = "plain")]
    format: OutputFormat,

    #[arg(long, help = "Find duplicate files")]
    duplicates: bool,

    #[arg(long, help = "Read file list from stdin")]
    stdin: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Algorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Blake3,
}

impl From<Algorithm> for HashAlgorithm {
    fn from(alg: Algorithm) -> Self {
        match alg {
            Algorithm::Md5 => HashAlgorithm::Md5,
            Algorithm::Sha1 => HashAlgorithm::Sha1,
            Algorithm::Sha256 => HashAlgorithm::Sha256,
            Algorithm::Sha512 => HashAlgorithm::Sha512,
            Algorithm::Blake3 => HashAlgorithm::Blake3,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Plain,
    Bsd,
    Gnu,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let hasher = Hasher::new(cli.algorithm.into());

    if let Some(checksum) = cli.verify {
        let verifier = Verifier::new(hasher);
        if cli.paths.is_empty() {
            anyhow::bail!("No files specified for verification");
        }
        
        for path in &cli.paths {
            match verifier.verify_file(path, &checksum) {
                Ok(true) => println!("{}: {}", path.display(), "OK".green()),
                Ok(false) => println!("{}: {}", path.display(), "FAILED".red()),
                Err(e) => println!("{}: {} - {}", path.display(), "ERROR".red(), e),
            }
        }
        return Ok(());
    }

    let mut files = Vec::new();
    
    if cli.stdin {
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            files.push(PathBuf::from(line?));
        }
    } else {
        for path in &cli.paths {
            if path.is_dir() && cli.recursive {
                for entry in walkdir::WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        files.push(entry.path().to_path_buf());
                    }
                }
            } else if path.is_file() {
                files.push(path.clone());
            }
        }
    }

    if cli.duplicates {
        let duplicates = hasher.find_duplicates(&files)?;
        for (hash, paths) in duplicates {
            if paths.len() > 1 {
                println!("\n{} ({})", "Duplicate files:".yellow(), hash);
                for path in paths {
                    println!("  {}", path.display());
                }
            }
        }
        return Ok(());
    }

    for path in files {
        match hasher.hash_file(&path) {
            Ok(hash) => {
                if cli.quiet {
                    println!("{}", hash);
                } else {
                    match cli.format {
                        OutputFormat::Plain => println!("{} {}", hash, path.display()),
                        OutputFormat::Bsd => println!("{}({}) = {}", 
                            format!("{:?}", cli.algorithm).to_uppercase(), 
                            path.display(), 
                            hash
                        ),
                        OutputFormat::Gnu => println!("{} *{}", hash, path.display()),
                    }
                }
            }
            Err(e) => eprintln!("{}: {}", path.display(), e.to_string().red()),
        }
    }

    Ok(())
}