use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Flight {
    source: String,
    destination: String,
}

#[derive(Serialize, Deserialize)]
struct FlightPath {
    path: Vec<String>,
}

async fn calculate_flight_path(flights: web::Json<Vec<Flight>>) -> impl Responder {
    let mut flight_map: HashMap<String, String> = HashMap::new();
    for flight in flights.iter() {
        flight_map.insert(flight.destination.clone(), flight.source.clone());
    }

    let mut path = Vec::new();
    let mut current_airport = flights[0].source.clone();
    loop {
        path.push(current_airport.clone());
        if let Some(next_airport) = flight_map.get(&current_airport) {
            current_airport = next_airport.clone();
        } else {
            break;
        }
    }

    HttpResponse::Ok().json(FlightPath { path })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/calculate")
                .route(web::post().to(calculate_flight_path))
                .route(web::get().to(|| async { "Hello, world!" })),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
