# Overview

**Inspiration:** There are over 100,000 flights a day, with millions of people and cargo being transferred around the world. With so many people and different carrier/agency groups, it can be hard to track where a person might be. To determine the flight path of a person, we must sort through all of their flight records.

**Goal:** Create a microservice API that can help us understand and track how a particular person’s flight path may be queried. The API should accept a request that includes a list of flights, which are defined by a source and destination airport code. These flights may not be listed in order and will need to be sorted to find the total flight paths starting and ending airports.

Required JSON structure: 
```json
[["SFO", "EWR"]]                              => ["SFO", "EWR"]
[["ATL", "EWR"], ["SFO", "ATL"]]              => ["SFO", "EWR"]
[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]
=> ["SFO", "EWR"]
```

## Option 1

- This code defines a `Flight` struct to represent a single flight, and a `FlightPath` struct to represent the calculated flight path.
- The `calculate_flight_path` function takes a list of flights as input, builds a graph representing the flights, and then uses a depth-first search (DFS) algorithm to find the total flight path.

- The API endpoint is defined as a POST request to `/calculate`, which accepts a JSON payload containing a list of flights. The response is a JSON object containing the calculated flight path.

### Usage:
To run the microservice, simply execute the binary and it will start listening on port 8080. You can then use a tool like curl to test the API endpoint. 

```
curl -X POST -H "Content-Type: application/json" -d '[{"source": "SFO", "destination": "EWR"}, {"source": "ATL", "destination": "EWR"}, {"source": "SFO", "destination": "ATL"}]' http://localhost:8080/calculate
```

This should return the calculated flight path `["SFO", "EWR"]`.

## Option 2

- This code defines a `Flight` struct to represent a single flight, and a `FlightPath` struct to represent the calculated flight path.
- The `calculate_flight_path` function takes a list of flights as input, builds a map of destination airports to source airports, and then uses this map to construct the flight path.
- The API endpoint is defined using the `actix-web` framework, and listens on port 8080. The /calculate endpoint accepts a POST request with a JSON body containing a list of flights, and returns a JSON response containing the calculated flight path.

### Usage:
To run the microservice, simply execute the main function. You can use a tool like curl to test the API endpoint:

```
curl -X POST -H "Content-Type: application/json" -d '[{"source": "SFO", "destination": "EWR"}, {"source": "ATL", "destination": "EWR"}, {"source": "SFO", "destination": "ATL"}]' http://localhost:8080/calculate
```

This should return the calculated flight path `["SFO", "EWR"]`.

Note that this implementation assumes that the input flights are valid and correctly formatted. You may want to add additional error handling and validation to make the API more robust.

##  For the previous code, integrate graphql to query potential flight data.

### Option 1 (GraphQL)
- We can use the `async-graphql` crate.

- This code defines a `FlightQuery` struct that implements the `Object` trait from `async_graphql`. This trait allows us to define the fields that can be queried using GraphQL.

- The `flights` field takes two arguments, `source` and `destination`, and returns a list of `Flight` objects.

- We then define a `FlightSchema` struct that implements the `Schema` trait from `async_graphql`. This trait defines the `root query`, `mutation`, and `subscription` types for our GraphQL schema.

- The `graphql_route` function handles GraphQL requests by executing them against our schema.

- Finally, we add a new service to our `Actix` web server that listens for GraphQL requests at the `/graphql` endpoint.

### Usage

You can use a tool like curl to test the GraphQL API:

```
curl -X POST -H "Content-Type: application/json" -d '{"query": "query { flights(source: \"SFO\", destination: \"EWR\") { source, destination } }"}' http://localhost:8080/graphql
```

This should return a list of flights from SFO to EWR.

### Option 2 (GraphQL)
- To integrate GraphQL into the previous code, we can use the `async-graphql` crate to define a GraphQL schema and resolvers. 

- This code defines a GraphQL schema with two queries: flights and flightPath. The flights query returns a list of flights, and the flightPath query takes a list of flights as input and returns the calculated flight path.

- The `graphql_endpoint` function handles GraphQL requests and executes the queries using the graphql function from the `async-graphql` crate.

### Usage

To test the GraphQL endpoint, you can use a tool like curl:

```
curl -X POST -H "Content-Type: application/json" -d '{"query": "query { flights { source destination } }"}' http://localhost:8080/graphql
```

This should return a list of flights in JSON format.

## Suggestion: Consider using a GraphQL client library

You can also use a GraphQL client library in your favorite programming language to query the endpoint. For example, in JavaScript, you can use the `graphql-request` library:
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

## Future Work
- Error handling
- Consider concurrency, fault tolerance, and general networking troubleshooting to sustain this API deployed in the wild
- Use versioning
- Make a user-friendly API. There are so many examples online of cool projects. 

