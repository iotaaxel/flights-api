use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Object, Schema, Subscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Flight {
    source: String,
    destination: String,
}

struct FlightQuery;

#[Object]
impl FlightQuery {
    async fn flights(&self, source: String, destination: String) -> Vec<Flight> {
        // Here you would fetch the flights from your data source
        // For this example, we'll just return a hardcoded list of flights
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
}

struct FlightSchema {
    query: FlightQuery,
    subscription: EmptySubscription,
}

impl Schema for FlightSchema {
    type Query = FlightQuery;
    type Mutation = EmptyMutation;
    type Subscription = EmptySubscription;
}

async fn graphql_route(req: GraphQLRequest) -> GraphQLResponse {
    let schema = FlightSchema {
        query: FlightQuery,
        subscription: EmptySubscription,
    };
    req.execute(&schema).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(
                        web::get()
                            .to(|| async { GraphiQLSource::build().endpoint("/graphql").finish() }),
                    ),
            )
            .service(
                web::resource("/calculate")
                    .route(web::post().to(calculate_flight_path))
                    .route(web::get().to(|| async { "Hello, world!" })),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
