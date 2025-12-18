#!/usr/bin/env python3
"""
Script to add new package registrations to package_support.rs
This ensures proper formatting and structure.
"""

# Rust packages to add
rust_packages = [
    ("actix-web", "4.5", "Powerful, pragmatic, and extremely fast web framework", 
     ["tokio", "actix-rt"], ["http_server", "routing", "middleware", "websockets"],
     ["Web", "Async"], "MIT OR Apache-2.0", "https://github.com/actix/actix-web", "https://actix.rs",
     ["web", "http", "server", "async"]),
    
    ("sqlx", "0.7", "Async SQL toolkit with compile-time checked queries",
     ["tokio"], ["async_sql", "compile_time_checked", "postgresql", "mysql", "sqlite"],
     ["Database", "Async"], "MIT OR Apache-2.0", "https://github.com/launchbadge/sqlx", "https://github.com/launchbadge/sqlx",
     ["sql", "async", "database", "orm"]),
    
    ("clap", "4.5", "Command Line Argument Parser for Rust",
     [], ["cli_parsing", "derive_macros", "subcommands", "validation"],
     ["CLI"], "MIT OR Apache-2.0", "https://github.com/clap-rs/clap", "https://docs.rs/clap",
     ["cli", "argument", "parser", "command"]),
    
    ("regex", "1.10", "Regular expressions for Rust",
     [], ["pattern_matching", "unicode", "performance", "captures"],
     ["Utility"], "MIT OR Apache-2.0", "https://github.com/rust-lang/regex", "https://docs.rs/regex",
     ["regex", "pattern", "matching", "text"]),
    
    ("rayon", "1.8", "Data parallelism library for Rust",
     [], ["data_parallelism", "work_stealing", "thread_pools", "parallel_iterators"],
     ["Async"], "MIT OR Apache-2.0", "https://github.com/rayon-rs/rayon", "https://docs.rs/rayon",
     ["parallel", "concurrency", "threading", "performance"]),
    
    ("reqwest", "0.11", "Higher level HTTP client library",
     ["tokio"], ["http_client", "async", "json", "cookies"],
     ["Networking", "Async"], "MIT OR Apache-2.0", "https://github.com/seanmonstar/reqwest", "https://docs.rs/reqwest",
     ["http", "client", "async", "request"]),
    
    ("diesel", "2.1", "Safe, extensible ORM and Query Builder",
     [], ["orm", "query_builder", "migrations", "type_safe"],
     ["Database"], "MIT OR Apache-2.0", "https://github.com/diesel-rs/diesel", "https://diesel.rs",
     ["orm", "database", "sql", "query"]),
    
    ("warp", "0.3", "Composable web framework with filters",
     ["tokio"], ["web_framework", "filters", "async", "composable"],
     ["Web", "Async"], "MIT", "https://github.com/seanmonstar/warp", "https://docs.rs/warp",
     ["web", "framework", "async", "http"]),
    
    ("tracing", "0.1", "Application-level tracing for Rust",
     [], ["structured_logging", "async_aware", "spans", "events"],
     ["Utility"], "MIT", "https://github.com/tokio-rs/tracing", "https://docs.rs/tracing",
     ["logging", "tracing", "diagnostics", "async"]),
    
    ("anyhow", "1.0", "Flexible concrete Error type built on std::error::Error",
     [], ["error_handling", "context", "backtrace", "downcasting"],
     ["Utility"], "MIT OR Apache-2.0", "https://github.com/dtolnay/anyhow", "https://docs.rs/anyhow",
     ["error", "handling", "result", "context"]),
]

# JavaScript packages to add
js_packages = [
    ("next.js", "14.1", "javascript", "The React Framework for Production",
     ["react"], ["ssr", "ssg", "routing", "api_routes"],
     ["Web"], "MIT", "https://github.com/vercel/next.js", "https://nextjs.org",
     ["react", "ssr", "framework", "web"]),
    
    ("svelte", "4.2", "javascript", "Cybernetically enhanced web apps",
     [], ["reactive", "compiler", "no_virtual_dom", "components"],
     ["Web", "GUI"], "MIT", "https://github.com/sveltejs/svelte", "https://svelte.dev",
     ["ui", "framework", "reactive", "compiler"]),
    
    ("prisma", "5.9", "typescript", "Next-generation ORM for Node.js and TypeScript",
     [], ["orm", "type_safe", "migrations", "prisma_studio"],
     ["Database"], "Apache-2.0", "https://github.com/prisma/prisma", "https://www.prisma.io",
     ["orm", "database", "typescript", "sql"]),
    
    ("jest", "29.7", "javascript", "Delightful JavaScript Testing Framework",
     [], ["testing", "mocking", "snapshots", "coverage"],
     ["Testing"], "MIT", "https://github.com/jestjs/jest", "https://jestjs.io",
     ["testing", "jest", "javascript", "test"]),
    
    ("webpack", "5.90", "javascript", "A bundler for javascript and friends",
     [], ["bundling", "code_splitting", "loaders", "plugins"],
     ["Utility"], "MIT", "https://github.com/webpack/webpack", "https://webpack.js.org",
     ["bundler", "webpack", "build", "module"]),
    
    ("tailwindcss", "3.4", "javascript", "A utility-first CSS framework",
     [], ["utility_first", "jit", "responsive", "customizable"],
     ["Web"], "MIT", "https://github.com/tailwindlabs/tailwindcss", "https://tailwindcss.com",
     ["css", "utility", "framework", "styling"]),
    
    ("axios", "1.6", "javascript", "Promise based HTTP client for the browser and node.js",
     [], ["http_client", "promises", "interceptors", "cancellation"],
     ["Networking"], "MIT", "https://github.com/axios/axios", "https://axios-http.com",
     ["http", "client", "promise", "ajax"]),
    
    ("socket.io", "4.6", "javascript", "Realtime application framework (Node.JS server)",
     ["nodejs"], ["websocket", "realtime", "fallbacks", "rooms"],
     ["Networking"], "MIT", "https://github.com/socketio/socket.io", "https://socket.io",
     ["websocket", "realtime", "socket", "io"]),
    
    ("graphql", "16.8", "javascript", "A query language for APIs and a runtime for fulfilling those queries",
     [], ["query_language", "schema", "resolvers", "introspection"],
     ["Web"], "MIT", "https://github.com/graphql/graphql-js", "https://graphql.org",
     ["graphql", "query", "api", "schema"]),
    
    ("nestjs", "10.3", "typescript", "A progressive Node.js framework for building efficient and scalable server-side applications",
     ["nodejs", "typescript"], ["backend_framework", "dependency_injection", "modules", "decorators"],
     ["Web"], "MIT", "https://github.com/nestjs/nest", "https://nestjs.com",
     ["backend", "framework", "typescript", "server"]),
]

def generate_rust_function(name, version, desc, deps, features, categories, license, repo, homepage, keywords):
    safe_name = name.replace(".", "_").replace("-", "_")
    deps_str = ", ".join([f'"{d}".to_string()' for d in deps])
    features_str = ",\n                    ".join([f'"{f}".to_string()' for f in features])
    cats_str = ", ".join([f"PackageCategory::{c}" for c in categories])
    keywords_str = ", ".join([f'"{k}".to_string()' for k in keywords])
    
    return f'''    fn register_{safe_name}(&mut self) {{
        self.packages.insert(
            "{name}".to_string(),
            Package {{
                name: "{name}".to_string(),
                language: "rust".to_string(),
                version: "{version}".to_string(),
                description: "{desc}".to_string(),
                dependencies: vec![{deps_str}],
                features: vec![
                    {features_str}
                ],
                categories: vec![{cats_str}],
                license: "{license}".to_string(),
                repository: Some("{repo}".to_string()),
                homepage: Some("{homepage}".to_string()),
                keywords: vec![{keywords_str}],
            }},
        );
    }}
'''

def generate_js_function(name, version, lang, desc, deps, features, categories, license, repo, homepage, keywords):
    safe_name = name.replace(".", "").replace("-", "")
    deps_str = ", ".join([f'"{d}".to_string()' for d in deps])
    features_str = ",\n                    ".join([f'"{f}".to_string()' for f in features])
    cats_str = ", ".join([f"PackageCategory::{c}" for c in categories])
    keywords_str = ", ".join([f'"{k}".to_string()' for k in keywords])
    
    return f'''    fn register_{safe_name}(&mut self) {{
        self.packages.insert(
            "{name}".to_string(),
            Package {{
                name: "{name}".to_string(),
                language: "{lang}".to_string(),
                version: "{version}".to_string(),
                description: "{desc}".to_string(),
                dependencies: vec![{deps_str}],
                features: vec![
                    {features_str}
                ],
                categories: vec![{cats_str}],
                license: "{license}".to_string(),
                repository: Some("{repo}".to_string()),
                homepage: Some("{homepage}".to_string()),
                keywords: vec![{keywords_str}],
            }},
        );
    }}
'''

# Generate all functions
print("    // New Rust Packages\n")
for pkg in rust_packages:
    print(generate_rust_function(*pkg))

print("\n    // New JavaScript/TypeScript Packages\n")
for pkg in js_packages:
    print(generate_js_function(*pkg))

# Generate registration calls for register_default_packages
print("\n// Add these calls to register_default_packages():")
print("        // New Rust packages")
for pkg in rust_packages:
    safe_name = pkg[0].replace(".", "_").replace("-", "_")
    print(f"        self.register_{safe_name}();")

print("\n        // New JavaScript packages")
for pkg in js_packages:
    safe_name = pkg[0].replace(".", "").replace("-", "")
    print(f"        self.register_{safe_name}();")
