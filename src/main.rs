
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use rand::{Rng, thread_rng};

#[derive(Debug, Serialize, Deserialize)]
struct JsonRequestSend {
    jsonrpc: String,
    id: String,
    method: String,
    params: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRequestSend2 {
    jsonrpc: String,
    id: String,
    method: String,
    params: Option<Vec<String>>,
}

#[tokio::main]
async fn main() {
    let url = "wss://echo.websocket.events";
    println!("Connecting to {url}");

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to Agent Network");

    let (mut write, mut read) = ws_stream.split();


    let params = vec![
        "no1".to_string(),
        "no2".to_string()
    ];

    let random_int_string_10_chars: String = thread_rng()
                .sample_iter(&rand::distributions::Uniform::new_inclusive(b'0', b'9'))
                .take(10)
                .map(|c| c as char)
                .collect();

    let body = JsonRequestSend {
        jsonrpc: "2.0".to_string(),
        id: random_int_string_10_chars.clone(),
        method: "send".to_string(),
        params: params,
    };

    let body_json = serde_json::to_string(&body).expect("Failed to serialize body to JSON");

    let msg = Message::Text(r#body_json.to_string().into());
    
    write.send(msg).await.expect("Failed to send message");

    loop {
        if let Some(message) = read.next().await {
            if let Ok(text) = message.expect("Failed to read message").into_text() {
                if let Ok(json_request) = serde_json::from_str::<JsonRequestSend>(text.as_str()) {
                    if json_request.id == random_int_string_10_chars {
                        println!("Received response with id: {}", json_request.id);
                        break;
                    }
                } else {
                    println!("Error parsing JSON");
                }
            }
        }
    }
    
}
