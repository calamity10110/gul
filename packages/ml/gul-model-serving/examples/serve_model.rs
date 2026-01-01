use gul_ml::{Activation, Layer, Model};
use gul_model_serving::ModelServer;

#[tokio::main]
async fn main() {
    // 1. Create a dummy trained model
    let mut model = Model::new("production-model");
    model.add_layer(Layer::Dense {
        input_size: 10,
        output_size: 1,
        activation: Activation::Linear,
    });

    // 2. Start Serving
    println!("Starting Inference Server...");
    ModelServer::run(model, 8080).await;
}
