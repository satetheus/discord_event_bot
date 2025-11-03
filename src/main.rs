use std::env;

use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    message: String,
    url: Option<String>,
}

async fn function_handler(event: LambdaEvent<Value>) -> Result<String, Error> {
    let payload: Message = serde_json::from_value(event.payload)?;

    let url = match payload.url {
        Some(n) => n,
        None => env::var("ANNOUNCEMENTS_HOOK")?
    };

    let client = reqwest::Client::new();
    let params = [("content", payload.message)];

    let _res = client.post(url).form(&params).send().await?;

    Ok("Success".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = run(service_fn(function_handler)).await;

    Ok(())
}
