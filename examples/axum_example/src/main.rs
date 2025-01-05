//! Docs: https://docs.rs/axum/0.7.7/axum/index.html
//!
//! This is a simple API using axum.
//! To get the counter go to localhost:3000.
/* To modify the counter with the api running:
  curl -X POST http://localhost:3000/modify_counter \
 -H "Content-Type: application/json" \
 -d '{"operation": "Sum", "number": 5}'
*/
//!
use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
struct AppState {
    counter: Mutex<u64>,
}

#[tokio::main]
async fn main() {
    let shared_counter_state = Arc::new(AppState {
        counter: Mutex::new(0),
    });
    let app = Router::new()
        .route("/", get(get_counter))
        .route("/counter", get(get_counter))
        .route("/modify_counter", post(modify_counter))
        .with_state(shared_counter_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_counter(State(state): State<Arc<AppState>>) -> Json<Value> {
    let counter_value = *state.counter.lock().unwrap();
    Json(json!({ "counter": counter_value }))
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Operation {
    Sum,
    Sub,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct CounterPayload {
    operation: Operation,
    number: u64,
}

async fn modify_counter(State(state): State<Arc<AppState>>, Json(payload): Json<CounterPayload>) {
    let mut counter = state.counter.lock().unwrap();
    match payload.operation {
        Operation::Sum => *counter = counter.saturating_add(payload.number),
        Operation::Sub => *counter = counter.saturating_sub(payload.number),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_operation_serialization() {
        let payload = CounterPayload {
            operation: Operation::Sum,
            number: 5,
        };

        // Test serialization
        assert_tokens(
            &payload,
            &[
                Token::Map { len: Some(2) },
                Token::Str("operation"),
                Token::Str("Sum"),
                Token::Str("number"),
                Token::U64(5),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn test_operation_deserialization() {
        let json_data = r#"{"operation": "Sub", "number": 3}"#;

        // Test deserialization
        let expected_payload = CounterPayload {
            operation: Operation::Sub,
            number: 3,
        };

        let deserialized_payload: CounterPayload = serde_json::from_str(json_data).unwrap();
        assert_eq!(deserialized_payload, expected_payload);
    }
}
