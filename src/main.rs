use rust_socketio::{ClientBuilder, Payload};
use std::thread;
use std::time::Duration;
use log::{info, error, LevelFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter(None, LevelFilter::Debug)
        .init();

    // Build the client and set up event handlers
    let client = ClientBuilder::new("http://localhost:3000")
        .on("connect", |_, _| {
            info!("Connected to the server from callback.");
            ()
        })
        .on("error", |payload, _| {
            match payload {
                Payload::String(t) => error!("String error: {:?}", t),
                Payload::Binary(b) => error!("Binary error occurred: {:?}", b),
                Payload::Text(t) => error!("Text error: {:?}", t),
                _ => error!("Unhandled payload type"),
            }
            ()
        })
        .connect()?; // This is a synchronous call and returns once connected

    info!("Connected to the server from code (after connect returned).");

    // Emit the "osman" event after the client is connected
    client.emit("osman", "giden mesaj")?;
    info!("Emitted 'osman' event with message: giden mesaj");

    // Keep the connection alive so we can see any incoming messages
    loop {
        thread::sleep(Duration::from_secs(5));
    }
}