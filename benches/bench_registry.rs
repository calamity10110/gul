use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    println!("Benchmarking resolve_dependencies allocations...");
    bench_resolve_dependencies();

    println!("Benchmarking search_by_keywords allocations...");
    bench_search_by_keywords();
}

#[derive(Clone)]
struct Package {
    dependencies: HashMap<String, String>,
    keywords: Vec<String>,
}

fn bench_resolve_dependencies() {
    let mut packages = HashMap::new();

    // Create a deep dependency tree
    let mut root_deps = HashMap::new();
    for i in 0..10 {
        root_deps.insert(format!("dep{}", i), "1.0".to_string());
    }

    packages.insert(
        "root@1.0".to_string(),
        Package {
            dependencies: root_deps,
            keywords: vec![],
        },
    );

    for i in 0..10 {
        let mut deps = HashMap::new();
        for j in 0..10 {
            deps.insert(format!("subdep{}_{}", i, j), "1.0".to_string());
        }
        packages.insert(
            format!("dep{}@1.0", i),
            Package {
                dependencies: deps,
                keywords: vec![],
            },
        );

        for j in 0..10 {
            packages.insert(
                format!("subdep{}_{}@1.0", i, j),
                Package {
                    dependencies: HashMap::new(),
                    keywords: vec![],
                },
            );
        }
    }

    // Original algorithm
    let start = Instant::now();
    for _ in 0..1000 {
        let mut resolved = Vec::new();
        let mut visited = HashSet::new();
        let mut to_resolve: Vec<(&str, &str)> = packages
            .get("root@1.0")
            .unwrap()
            .dependencies
            .iter()
            .map(|(name, ver)| (name.as_str(), ver.as_str()))
            .collect();

        while let Some((name, ver)) = to_resolve.pop() {
            let dep_key = format!("{}@{}", name, ver);

            if visited.insert(dep_key.clone()) {
                resolved.push(dep_key.clone());

                if let Some(dep_pkg) = packages.get(&dep_key) {
                    for (dep_name, dep_ver) in &dep_pkg.dependencies {
                        to_resolve.push((dep_name.as_str(), dep_ver.as_str()));
                    }
                }
            }
        }
    }
    let orig_time = start.elapsed();

    // Optimized algorithm
    let start = Instant::now();
    for _ in 0..1000 {
        let mut resolved = Vec::new();
        let mut visited: HashSet<(&str, &str)> = HashSet::new();
        let mut to_resolve: Vec<(&str, &str)> = packages
            .get("root@1.0")
            .unwrap()
            .dependencies
            .iter()
            .map(|(name, ver)| (name.as_str(), ver.as_str()))
            .collect();

        while let Some((name, ver)) = to_resolve.pop() {
            if visited.insert((name, ver)) {
                let dep_key = format!("{}@{}", name, ver);

                if let Some(dep_pkg) = packages.get(&dep_key) {
                    for (dep_name, dep_ver) in &dep_pkg.dependencies {
                        to_resolve.push((dep_name.as_str(), dep_ver.as_str()));
                    }
                }

                resolved.push(dep_key);
            }
        }
    }
    let opt_time = start.elapsed();

    println!("Original resolve_dependencies: {:?}", orig_time);
    println!("Optimized resolve_dependencies: {:?}", opt_time);
    println!(
        "Speedup: {:.2}x",
        orig_time.as_secs_f64() / opt_time.as_secs_f64()
    );
}

fn bench_search_by_keywords() {
    let mut packages = HashMap::new();

    for i in 0..1000 {
        packages.insert(
            format!("pkg{}", i),
            Package {
                dependencies: HashMap::new(),
                keywords: vec![
                    "Web".to_string(),
                    "Framework".to_string(),
                    "Rust".to_string(),
                    format!("kw{}", i),
                ],
            },
        );
    }

    let keywords = vec!["web".to_string(), "rust".to_string()];

    // Original algorithm
    let start = Instant::now();
    for _ in 0..1000 {
        let _res: Vec<_> = packages
            .values()
            .filter(|pkg| {
                keywords.iter().any(|kw| {
                    pkg.keywords
                        .iter()
                        .any(|pkg_kw| pkg_kw.to_lowercase() == kw.to_lowercase())
                })
            })
            .collect();
    }
    let orig_time = start.elapsed();

    // Optimized algorithm
    let start = Instant::now();
    for _ in 0..1000 {
        let _res: Vec<_> = packages
            .values()
            .filter(|pkg| {
                keywords.iter().any(|kw| {
                    pkg.keywords
                        .iter()
                        .any(|pkg_kw| pkg_kw.eq_ignore_ascii_case(kw))
                })
            })
            .collect();
    }
    let opt_time = start.elapsed();

    println!("Original search_by_keywords: {:?}", orig_time);
    println!("Optimized search_by_keywords: {:?}", opt_time);
    println!(
        "Speedup: {:.2}x",
        orig_time.as_secs_f64() / opt_time.as_secs_f64()
    );
}
