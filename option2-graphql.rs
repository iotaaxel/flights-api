use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use async_graphql::{graphql, Context, Object, Schema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define the GraphQL schema
struct FlightSchema;

#[Object]
impl FlightSchema {
    async fn flights(&self, ctx: &Context<'_>) -> Vec<Flight> {
        // Return a list of flights
        vec![
            Flight {
                source: "SFO".to_string(),
                destination: "EWR".to_string(),
            },
            Flight {
                source: "ATL".to_string(),
                destination: "EWR".to_string(),
            },
            Flight {
                source: "SFO".to_string(),
                destination: "ATL".to_string(),
            },
        ]
    }

    async fn flight_path(&self, ctx: &Context<'_>, flights: Vec<Flight>) -> FlightPath {
        // Calculate the flight path
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

        FlightPath { path }
    }
}

// Define the GraphQL resolvers
async fn flights_resolver(_ctx: &Context<'_>) -> Vec<Flight> {
    // Return a list of flights
    vec![
        Flight {
            source: "SFO".to_string(),
            destination: "EWR".to_string(),
        },
        Flight {
            source: "ATL".to_string(),
            destination: "EWR".to_string(),
        },
        Flight {
            source: "SFO".to_string(),
            destination: "ATL".to_string(),
        },
    ]
}

async fn flight_path_resolver(_ctx: &Context<'_>, flights: Vec<Flight>) -> FlightPath {
    // Calculate the flight path
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

    FlightPath { path }
}

// Define the GraphQL schema and resolvers
async fn schema() -> Schema<FlightSchema, Context> {
    let schema = Schema::new(FlightSchema, Context);
    schema.register_query("flights", flights_resolver);
    schema.register_query("flightPath", flight_path_resolver);
    schema
}

// Define the API endpoint
async fn graphql_endpoint(req: web::Json<GraphQLRequest>) -> impl Responder {
    let schema = schema().await;
    let result = graphql(&schema, &req.query, &req.variables).await;
    HttpResponse::Ok().json(result)
}

// Define the GraphQL request struct
#[derive(Deserialize)]
struct GraphQLRequest {
    query: String,
    variables: HashMap<String, String>,
}

// Define the main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_endpoint))
                    .route(web::get().to(|| async { "Hello, world!" })),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
