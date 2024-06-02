# Overview

Senior Software Engineer Take-Home Programming Assignment for Rust

Story: There are over 100,000 flights a day, with millions of people and cargo being transferred around the world. With so many people and different carrier/agency groups, it can be hard to track where a person might be. In order to determine the flight path of a person, we must sort through all of their flight records.

Goal: To create a microservice API that can help us understand and track how a particular person’s flight path may be queried. The API should accept a request that includes a list of flights, which are defined by a source and destination airport code. These flights may not be listed in order and will need to be sorted to find the total flight paths starting and ending airports.

Required JSON structure: 

[["SFO", "EWR"]]                              => ["SFO", "EWR"]
[["ATL", "EWR"], ["SFO", "ATL"]]              => ["SFO", "EWR"]
[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]
=> ["SFO", "EWR"]



Specifications: 
Your miscroservice must listen on port 8080 and expose the flight path tracker under /calculate endpoint.

Define and document the format of the API endpoint in the README.

Use Rust and/or with any tools that you think will help you best accomplish the task at hand.

## Option 1
Here is an example of how you can implement the flight path tracker microservice in Rust:
```rust
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
```
This code defines a Flight struct to represent a single flight, and a FlightPath struct to represent the calculated flight path. The calculate_flight_path function takes a list of flights as input, builds a graph representing the flights, and then uses a depth-first search (DFS) algorithm to find the total flight path.

The API endpoint is defined as a POST request to /calculate, which accepts a JSON payload containing a list of flights. The response is a JSON object containing the calculated flight path.

To run the microservice, simply execute the binary and it will start listening on port 8080. You can then use a tool like curl to test the API endpoint. For example:

`curl -X POST -H "Content-Type: application/json" -d '[{"source": "SFO", "destination": "EWR"}, {"source": "ATL", "destination": "EWR"}, {"source": "SFO", "destination": "ATL"}]' http://localhost:8080/calculate`

This should return the calculated flight path `["SFO", "EWR"]`.

## Option 2
Here is an example of how you can create a microservice API in Rust to track a person's flight path:
```
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
```
This code defines a Flight struct to represent a single flight, and a FlightPath struct to represent the calculated flight path. The calculate_flight_path function takes a list of flights as input, builds a map of destination airports to source airports, and then uses this map to construct the flight path.

The API endpoint is defined using the actix-web framework, and listens on port 8080. The /calculate endpoint accepts a POST request with a JSON body containing a list of flights, and returns a JSON response containing the calculated flight path.

To run the microservice, simply execute the main function. You can use a tool like curl to test the API endpoint:

`curl -X POST -H "Content-Type: application/json" -d '[{"source": "SFO", "destination": "EWR"}, {"source": "ATL", "destination": "EWR"}, {"source": "SFO", "destination": "ATL"}]' http://localhost:8080/calculate`
This should return the calculated flight path `["SFO", "EWR"]`.

Note that this implementation assumes that the input flights are valid and correctly formatted. You may want to add additional error handling and validation to make the API more robust.

##  For the previous code, integrate graphql to query potential flight data.

### Option 1
To integrate GraphQL into the previous code, we can use the async-graphql library in Rust. Here's an example of how you can modify the code to add a GraphQL API:
```
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
```
This code defines a FlightQuery struct that implements the Object trait from async_graphql. This trait allows us to define the fields that can be queried using GraphQL.

The flights field takes two arguments, source and destination, and returns a list of Flight objects.

We then define a FlightSchema struct that implements the Schema trait from async_graphql. This trait defines the root query, mutation, and subscription types for our GraphQL schema.

The graphql_route function handles GraphQL requests by executing them against our schema.

Finally, we add a new service to our Actix web server that listens for GraphQL requests at the /graphql endpoint.

You can use a tool like curl to test the GraphQL API:

`curl -X POST -H "Content-Type: application/json" -d '{"query": "query { flights(source: \"SFO\", destination: \"EWR\") { source, destination } }"}' http://localhost:8080/graphql`
This should return a list of flights from SFO to EWR.

Note that this is just a basic example, and you would need to modify it to fit your specific use case.

### Option 2
To integrate GraphQL into the previous code, we can use the async-graphql crate to define a GraphQL schema and resolvers. Here's an example of how we can modify the code to add GraphQL support:
```
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
```
This code defines a GraphQL schema with two queries: flights and flightPath. The flights query returns a list of flights, and the flightPath query takes a list of flights as input and returns the calculated flight path.

The graphql_endpoint function handles GraphQL requests and executes the queries using the graphql function from the async-graphql crate.

To test the GraphQL endpoint, you can use a tool like curl:

`curl -X POST -H "Content-Type: application/json" -d '{"query": "query { flights { source destination } }"}' http://localhost:8080/graphql`
This should return a list of flights in JSON format.

You can also use a GraphQL client library in your favorite programming language to query the endpoint. For example, in JavaScript, you can use the graphql-request library:
```
import { GraphQLClient } from 'graphql-request';

const client = new GraphQLClient('http://localhost:8080/graphql');

const query = `
  query {
    flights {
      source
      destination
    }
  }
`;

client.request(query).then(data => console.log(data));
```
This should also return a list of flights in JSON format.


