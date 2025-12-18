// Helper macro for package registration
// This file contains a template for updating all package registrations

// Template for package registration with all fields:
/*
self.packages.insert(
    "package_name".to_string(),
    Package {
        name: "package_name".to_string(),
        language: "rust|python|javascript|etc".to_string(),
        version: "x.y.z".to_string(),
        description: "Description here".to_string(),
        dependencies: vec![],
        features: vec![],
        categories: vec![PackageCategory::Web],  // NEW
        license: "MIT|Apache-2.0|etc".to_string(),  // NEW
        repository: Some("https://github.com/...".to_string()),  // NEW
        homepage: Some("https://...".to_string()),  // NEW
        keywords: vec!["keyword1".to_string()],  // NEW
    },
);
*/

// Common package metadata by ecosystem:

// Rust packages - typically MIT or Apache-2.0
// Python packages - typically MIT, BSD, or Apache-2.0
// JavaScript packages - typically MIT
// Databases - various (PostgreSQL License, MySQL GPL, etc.)

// Common categories:
// - Web: web frameworks, HTTP servers
// - DataScience: data analysis, scientific computing
// - MachineLearning: ML frameworks
// - Database: database drivers, ORMs
// - Testing: test frameworks
// - CLI: command-line tools
// - GUI: graphical interfaces
// - Networking: network protocols
// - Async: async runtimes
// - Serialization: data serialization
// - Utility: general utilities
