mod biometrics;
mod blockchain;
mod database;
mod identity;
mod mesh;
mod models;
mod sync;

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .build();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting D-HADS Backend...");

    // Initialize the local database
    let db = database::LocalStore::new("./data")?;
    info!("Local database initialized");

    // Initialize the P2P mesh network
    let mesh = mesh::Network::new()?;
    info!("P2P network initialized");

    // Initialize the blockchain client
    let blockchain = blockchain::Client::new()?;
    info!("Blockchain client initialized");

    // Start the synchronization service
    let sync_service = sync::Service::new(db.clone(), mesh.clone())?;
    tokio::spawn(async move {
        if let Err(e) = sync_service.run().await {
            tracing::error!("Sync service error: {}", e);
        }
    });

    // Start the HTTP API server
    let api = api::Server::new(db, mesh, blockchain);
    api.run().await?;

    Ok(())
}

mod api {
    use super::*;
    use axum::{
        routing::{get, post},
        Router,
    };

    pub struct Server {
        db: database::LocalStore,
        mesh: mesh::Network,
        blockchain: blockchain::Client,
    }

    impl Server {
        pub fn new(
            db: database::LocalStore,
            mesh: mesh::Network,
            blockchain: blockchain::Client,
        ) -> Self {
            Self {
                db,
                mesh,
                blockchain,
            }
        }

        pub async fn run(self) -> Result<()> {
            let app = Router::new()
                .route("/health", get(|| async { "OK" }))
                .route("/recipients", post(self.register_recipient));

            let addr = "[::]:3000".parse()?;
            info!("API server listening on {}", addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await?;

            Ok(())
        }

        async fn register_recipient(
            // Implementation will be added later
        ) {
            // TODO: Implement recipient registration
        }
    }
} 