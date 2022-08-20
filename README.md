# F1-game-telemetry library

Choose the telemetry data you want to record from the F1 video game.

### Example main.rs

Below is an example of how you would use this library.

```rust

use f1_game_telemetry::TelemetryBuilder;
use serde_json::Value;

#[tokio::main] //this is a test
async fn main() {
    let endpoint = "127.0.0.1:30500";
    let mut tel = TelemetryBuilder::new(endpoint.to_owned())
        .add_events_data()
        .build();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    tel.record(tx).await;
    while let Some(val) = rx.recv().await {
        let val: Value = serde_json::from_str(&val).unwrap();
        println!("RECEIVED: {}", val);
    }
}

```

### License

This library is licensed under the [MIT License](https://opensource.org/licenses/MIT)

### github

Please check out my [github](https://github.com/bwintertkb)! Thank you!
