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
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for flight in flights.iter() {
        graph.entry(flight.source.clone()).or_insert(Vec::new()).push(flight.destination.clone());
    }
    let mut visited: HashMap<String, bool> = HashMap::new();
    let mut path: Vec<String> = Vec::new();
    for airport in graph.keys() {
        if !visited.contains_key(airport) {
            dfs(&graph, airport, &mut visited, &mut path);
        }
    }
    HttpResponse::Ok().json(FlightPath { path })
}

fn dfs(graph: &HashMap<String, Vec<String>>, airport: &str, visited: &mut HashMap<String, bool>, path: &mut Vec<String>) {
    visited.insert(airport.to_string(), true);
    path.push(airport.to_string());
    if let Some(destinations) = graph.get(airport) {
        for destination in destinations {
            if !visited.contains_key(destination) {
                dfs(graph, destination, visited, path);
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/calculate")
                .route(web::post().to(calculate_flight_path))
                .route(web::get().to(|| async { "Welcome to the flight path tracker!" })),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
