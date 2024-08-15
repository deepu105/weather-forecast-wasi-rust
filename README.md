# WASI Weather Forecast Server

A simple Warp server that fetches and returns weather forecasts for given coordinates. Built with Rust for WebAssembly System Interface (WASI). Meant to be run using [WasmEdge](https://wasmedge.org/docs/start/install) or on Kubernetes.

## Create a Rust WASM app

```bash
rustup target add wasm32-wasi
cargo new weather_forecast
```

## Add dependencies

```toml
warp_wasi = "0.3"
reqwest_wasi = { version = "0.11", features = ["json"] }
tokio_wasi = { version = "1.21", features = ["full"] }
serde = "1.0"
serde_derive = "1.0"
serde_json = "1.0"
```

## Add code

Refer to the `src/main.rs` file in this repository.

## Build for WASI

```bash
cargo build --target wasm32-wasi --release
```

## Test locally using WasmEdge

```bash
# Install WasmEdge -> https://wasmedge.org/docs/start/install
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
source $HOME/.wasmedge/env

# Run the server
wasmedge target/wasm32-wasi/release/weather_forecast.wasm


# Open another terminal and test the server -> You should see the weather forecast as JSON
curl "http://localhost:8090?lat=52.52&long=11.2"
```

## Run on Docker

```bash
# Build the Docker image
docker buildx build --provenance=false --platform wasi/wasm -t deepu105/weather_forecast_wasi .
docker push deepu105/weather_forecast_wasi

# Requires Docker Desktop with the containerd image store feature in your Docker Desktop settings enabled.
docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm deepu105/weather_forecast_wasi:latest
```

## Run on Kubernetes

### Make an OCI image

```bash
# Install buildah for creating OCI images -> https://github.com/containers/buildah/blob/main/install.md
# Build OIC image
buildah build --annotation "module.wasm.image/variant=compat-smart" -t weather_forecast Dockerfile_OCI
# Push the image to Docker Hub
buildah push --authfile ~/.docker/config.json weather_forecast docker://docker.io/deepu105/weather_forecast:latest
```

### Setup KinD cluster

```bash
# Create a "KinD" Cluster
kind create cluster
# Enable WasmEdge support using KWasm
kubectl apply -f https://raw.githubusercontent.com/KWasm/kwasm-node-installer/main/example/daemonset.yaml
```

### Deploy the app on Kubernetes

```bash
kubectl apply -f k8s-manifest.yaml

# Open a tunnel to the service
kubectl port-forward service/word-generator-service 8080

# Test the server
curl "http://localhost:8080?lat=52.52&long=11.2"
```
