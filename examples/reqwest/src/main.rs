use reqwest::blocking::Client;
use serde_json::Value;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let client = Client::new();
    let res = client
        .post("https://eth.llamarpc.com")
        .header("Content-Type", "application/json")
        .body(
            r#"
            {
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 1
            }
            "#,
        )
        .send()?;

    let response_text = res.text()?;

    let json_value: Value = serde_json::from_str(&response_text)?;

    let result = json_value.get("result").unwrap();
    println!("{}", result.as_str().unwrap());
    let pretty_json = serde_json::to_string_pretty(&json_value)?;

    println!("{}", pretty_json);

    Ok(())
}
