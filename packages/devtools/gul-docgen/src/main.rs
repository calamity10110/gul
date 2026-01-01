use clap::Parser;
use walkdir::WalkDir;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(author, version, about = "GUL Auto-Documentation Tool")]
struct Args {
    #[arg(short, long, default_value = ".")]
    root: String,
}

#[derive(serde::Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(serde::Deserialize)]
struct Package {
    name: String,
    description: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Scanning workspace at: {}", args.root);

    for entry in WalkDir::new(&args.root).min_depth(1).max_depth(4) {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml" {
             process_package(entry.path())?;
        }
    }
    
    Ok(())
}

fn process_package(toml_path: &Path) -> Result<()> {
    let content = fs::read_to_string(toml_path)?;
    // Simple parsing to reuse the struct
    let cargo: CargoToml = toml::from_str(&content).context(format!("Failed to parse {:?}", toml_path))?;
    
    let dir = toml_path.parent().unwrap();
    let readme_path = dir.join("README.md");
    
    if !readme_path.exists() {
        println!("generating README for {}", cargo.package.name);
        let desc = cargo.package.description.unwrap_or_else(|| "No description provided.".to_string());
        
        let readme = format!(
            "# {}\n\n{}\n\n## Usage\nAdd this to your `Cargo.toml`:\n```toml\n[dependencies]\n{} = \"0.1.0\"\n```\n",
            cargo.package.name, desc, cargo.package.name
        );
        
        fs::write(readme_path, readme)?;
    } else {
        // println!("Skipping existing README for {}", cargo.package.name);
    }
    
    Ok(())
}
