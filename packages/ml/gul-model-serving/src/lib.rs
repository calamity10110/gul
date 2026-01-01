use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use gul_ml::{Inference, Model};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub inference: Arc<Mutex<Option<Inference>>>,
}

#[derive(Deserialize)]
pub struct PredictRequest {
    pub inputs: Vec<f32>,
}

#[derive(Serialize)]
pub struct PredictResponse {
    pub outputs: Vec<f32>,
}

pub struct ModelServer;

impl ModelServer {
    pub async fn run(model: Model, port: u16) {
        let inference = Inference::new(model);
        let state = AppState {
            inference: Arc::new(Mutex::new(Some(inference))),
        };

        let app = Router::new()
            .route("/health", get(health_check))
            .route("/predict", post(predict))
            .with_state(state);

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        println!("Model Server listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

async fn predict(
    State(state): State<AppState>,
    Json(payload): Json<PredictRequest>,
) -> Result<Json<PredictResponse>, StatusCode> {
    let inference = state.inference.lock().unwrap();

    if let Some(inf) = &*inference {
        match inf.predict(&payload.inputs) {
            Ok(outputs) => Ok(Json(PredictResponse { outputs })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
