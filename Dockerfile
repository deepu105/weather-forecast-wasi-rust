FROM --platform=$BUILDPLATFORM rust:1.80 AS buildbase
WORKDIR /src
RUN <<EOT bash
    set -ex
    apt-get update
    apt-get install -y \
        git \
        clang
    rustup target add wasm32-wasi
EOT

FROM buildbase AS build
COPY Cargo.toml .
COPY src ./src
# Build the WASM binary
RUN cargo build --target wasm32-wasi --release

FROM scratch
ENTRYPOINT [ "weather_forecast.wasm" ]
COPY --link --from=build /src/target/wasm32-wasi/release/weather_forecast.wasm /weather_forecast.wasm