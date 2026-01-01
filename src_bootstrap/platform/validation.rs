// Package Manifest Validation
// Validates package.toml schema and dependencies

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    pub package: PackageMetadata,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
    #[serde(default, rename = "dev-dependencies")]
    pub dev_dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    pub features: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencySpec {
    Simple(String),
    Detailed {
        version: String,
        #[serde(default)]
        optional: bool,
        #[serde(default)]
        features: Vec<String>,
    },
}

impl DependencySpec {
    pub fn version(&self) -> &str {
        match self {
            DependencySpec::Simple(v) => v,
            DependencySpec::Detailed { version, .. } => version,
        }
    }
}

pub struct ManifestValidator;

impl ManifestValidator {
    /// Validate package manifest
    pub fn validate(manifest: &PackageManifest) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Validate package name
        if !Self::is_valid_package_name(&manifest.package.name) {
            errors.push(ValidationError::InvalidPackageName(
                manifest.package.name.clone(),
            ));
        }

        // Validate version (semver)
        if !Self::is_valid_semver(&manifest.package.version) {
            errors.push(ValidationError::InvalidVersion(
                manifest.package.version.clone(),
            ));
        }

        // Validate dependencies
        for (name, spec) in &manifest.dependencies {
            if !Self::is_valid_package_name(name) {
                errors.push(ValidationError::InvalidDependencyName(name.clone()));
            }

            if !Self::is_valid_version_spec(spec.version()) {
                errors.push(ValidationError::InvalidDependencyVersion {
                    package: name.clone(),
                    version: spec.version().to_string(),
                });
            }
        }

        // Validate features
        for (feature, deps) in &manifest.features {
            for dep in deps {
                if !manifest.dependencies.contains_key(dep)
                    && !manifest.dev_dependencies.contains_key(dep)
                {
                    errors.push(ValidationError::UnknownFeatureDependency {
                        feature: feature.clone(),
                        dependency: dep.clone(),
                    });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn is_valid_package_name(name: &str) -> bool {
        // Package name rules:
        // - lowercase letters, numbers, hyphens
        // - must start with letter
        // - 2-64 characters
        if name.len() < 2 || name.len() > 64 {
            return false;
        }

        let chars: Vec<char> = name.chars().collect();
        if !chars[0].is_ascii_lowercase() {
            return false;
        }

        chars
            .iter()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
    }

    fn is_valid_semver(version: &str) -> bool {
        // Basic semver validation: X.Y.Z
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        parts.iter().all(|p| p.parse::<u32>().is_ok())
    }

    fn is_valid_version_spec(spec: &str) -> bool {
        // Support: "1.0.0", "^1.0.0", ">=1.0.0", "~1.0.0", "*"
        if spec == "*" {
            return true;
        }

        let version = spec
            .trim_start_matches('^')
            .trim_start_matches('~')
            .trim_start_matches(">=")
            .trim_start_matches('>')
            .trim_start_matches("<=")
            .trim_start_matches('<');

        Self::is_valid_semver(version)
    }
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidPackageName(String),
    InvalidVersion(String),
    InvalidDependencyName(String),
    InvalidDependencyVersion { package: String, version: String },
    UnknownFeatureDependency { feature: String, dependency: String },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidPackageName(name) => {
                write!(f, "Invalid package name: {}", name)
            }
            ValidationError::InvalidVersion(version) => {
                write!(f, "Invalid version: {}", version)
            }
            ValidationError::InvalidDependencyName(name) => {
                write!(f, "Invalid dependency name: {}", name)
            }
            ValidationError::InvalidDependencyVersion { package, version } => {
                write!(
                    f,
                    "Invalid version '{}' for dependency '{}'",
                    version, package
                )
            }
            ValidationError::UnknownFeatureDependency {
                feature,
                dependency,
            } => {
                write!(
                    f,
                    "Feature '{}' references unknown dependency '{}'",
                    feature, dependency
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_manifest() {
        let manifest = PackageManifest {
            package: PackageMetadata {
                name: "my-package".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Test package".to_string()),
                authors: vec!["Author".to_string()],
                license: Some("MIT".to_string()),
                repository: None,
                homepage: None,
                keywords: vec![],
            },
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            features: HashMap::new(),
        };

        assert!(ManifestValidator::validate(&manifest).is_ok());
    }

    #[test]
    fn test_invalid_package_name() {
        let manifest = PackageManifest {
            package: PackageMetadata {
                name: "Invalid_Name".to_string(), // Uppercase and underscore
                version: "1.0.0".to_string(),
                description: None,
                authors: vec![],
                license: None,
                repository: None,
                homepage: None,
                keywords: vec![],
            },
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            features: HashMap::new(),
        };

        assert!(ManifestValidator::validate(&manifest).is_err());
    }

    #[test]
    fn test_invalid_version() {
        let manifest = PackageManifest {
            package: PackageMetadata {
                name: "my-package".to_string(),
                version: "invalid".to_string(),
                description: None,
                authors: vec![],
                license: None,
                repository: None,
                homepage: None,
                keywords: vec![],
            },
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            features: HashMap::new(),
        };

        assert!(ManifestValidator::validate(&manifest).is_err());
    }

    #[test]
    fn test_valid_dependency_specs() {
        assert!(ManifestValidator::is_valid_version_spec("1.0.0"));
        assert!(ManifestValidator::is_valid_version_spec("^1.0.0"));
        assert!(ManifestValidator::is_valid_version_spec("~1.0.0"));
        assert!(ManifestValidator::is_valid_version_spec(">=1.0.0"));
        assert!(ManifestValidator::is_valid_version_spec("*"));
    }

    #[test]
    fn test_package_name_validation() {
        assert!(ManifestValidator::is_valid_package_name("my-package"));
        assert!(ManifestValidator::is_valid_package_name("package123"));
        assert!(!ManifestValidator::is_valid_package_name("Package")); // Uppercase
        assert!(!ManifestValidator::is_valid_package_name("my_package")); // Underscore
        assert!(!ManifestValidator::is_valid_package_name("a")); // Too short
    }
}
