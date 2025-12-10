use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

// In-memory mock DB for MVP
struct AppState {
    packages: Mutex<HashMap<String, Package>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
    description: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Shared state
    let shared_state = Arc::new(AppState {
        packages: Mutex::new(HashMap::new()),
    });

    // Seed with a test package
    {
        let mut pkgs = shared_state.packages.lock().unwrap();
        pkgs.insert(
            "std.math".to_string(),
            Package {
                name: "std.math".to_string(),
                version: "1.0.0".to_string(),
                description: "Standard math library".to_string(),
            },
        );
    }

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/packages/:name", get(get_package))
        .route("/api/v1/publish", post(publish_package))
        .with_state(shared_state);

    let parsed_addr = "127.0.0.1:3000".parse::<SocketAddr>().unwrap();
    println!("Registry listening on {}", parsed_addr);
    let listener = tokio::net::TcpListener::bind(parsed_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

// Handler: Get Package Metadata
async fn get_package(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<Package>, StatusCode> {
    let pkgs = state.packages.lock().unwrap();
    if let Some(pkg) = pkgs.get(&name) {
        Ok(Json(pkg.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
struct PublishRequest {
    name: String,
    version: String,
    description: String,
}

// Handler: Publish Package (Mock)
// In real impl, this would handle multipart upload for the .gtar file
async fn publish_package(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PublishRequest>,
) -> (StatusCode, Json<Package>) {
    let mut pkgs = state.packages.lock().unwrap();
    let pkg = Package {
        name: payload.name.clone(),
        version: payload.version.clone(),
        description: payload.description,
    };
    pkgs.insert(payload.name, pkg.clone());
    (StatusCode::CREATED, Json(pkg))
}
