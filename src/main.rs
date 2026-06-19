// src/main.rs
pub mod config;
pub mod docs;
pub mod initiator;
pub mod internal;

use initiator::AppInitiator;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    AppInitiator::initiate().await
}
