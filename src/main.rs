use f1_game_telemetry::TelemetryBuilder;
use serde_json::Value;

#[tokio::main] //this is a test
async fn main() {
    let endpoint = "127.0.0.1:2995";
    let mut tel = TelemetryBuilder::new(endpoint.to_owned())
        .add_all_data()
        .build();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    tel.record(tx).await;
    while let Some(val) = rx.recv().await {
        let val: Value = serde_json::from_str(&val).unwrap();
        println!("RECEIVED: {}", val);
    }
}
