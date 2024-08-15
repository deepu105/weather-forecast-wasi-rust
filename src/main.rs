use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, convert::Infallible};
use warp::Filter;

#[derive(Deserialize, Debug)]
struct QueryParams {
    lat: f32,
    long: f32,
}

#[derive(Deserialize, Serialize, Debug)]
struct WeatherResponse {
    latitude: f32,
    longitude: f32,
    timezone: String,
    current_units: HashMap<String, String>,
    current: CurrentWeather,
}

#[derive(Deserialize, Serialize, Debug)]
struct CurrentWeather {
    time: String,
    interval: i32,
    temperature_2m: f32,
    wind_speed_10m: f32,
    relative_humidity_2m: f32,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let routes = warp::get()
        .and(warp::query::<QueryParams>())
        .and_then(facts);

    println!("Listening on http://0.0.0.0:8090");
    warp::serve(routes).run(([0, 0, 0, 0], 8090)).await;
}

async fn facts(q: QueryParams) -> Result<impl warp::Reply, Infallible> {
    let url = format!("http://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m,relative_humidity_2m", q.lat, q.long);

    let out = match reqwest::get(url).await {
        Ok(resp) => match resp.json::<WeatherResponse>().await {
            Ok(resp) => serde_json::to_string_pretty(&resp).unwrap(),
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    };

    Ok(out)
}
