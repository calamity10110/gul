# AI & Machine Learning Guide

GUL is an "AI-First" language. It provides built-in tools for both training models and serving them in production.

## Overview

- **`gul-ml`**: The core library for defining and training models.
- **`gul-model-serving`**: A microservice framework to serve models via standard REST APIs.

## 1. Defining a Model

Models are defined using `gul-ml` structs.

```gul
imp gul_ml

let model = gul_ml.Model.new("my_model")
model.add(gul_ml.Layer.Dense(128, activation="relu"))
model.add(gul_ml.Layer.Dense(10, activation="softmax"))
```

## 2. Training

Use the `Trainer` class to train your model.

```gul
let trainer = gul_ml.Trainer.new(model)
trainer.train(x_train, y_train, epochs=10)
model.save("model.json")
```

## 3. Serving

Once saved, you can serve the model using `gul-model-serving`.

### Running the Server

```bash
gul serve model.json --port 8080
```

### Making Requests

The server exposes a `/predict` endpoint.

```bash
curl -X POST http://localhost:8080/predict -d '{"inputs": [...]}'
```

## 4. Integration in Code

You can also use the `@ai` decorator for quick LLM integration.

```gul
@ai(model="gpt-4")
fn summarize(text: str) -> str
```
