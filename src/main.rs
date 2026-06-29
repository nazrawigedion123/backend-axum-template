// src/main.rs
pub mod config;
pub mod docs;
pub mod initiator;
pub mod internal;
pub mod platform;


use initiator::AppInitiator;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    AppInitiator::initiate().await
}
