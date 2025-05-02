// src/main.rs
mod models;
mod scanner;
mod db;
mod api;
mod tmdb;

use actix_web::{web, App, HttpServer};
use db::init_db;
use std::path::PathBuf;
use clap::Parser;
use dotenv::dotenv;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to media library
    #[arg(short, long, default_value = ".")]
    media_path: PathBuf,

    /// Rescan library on startup
    #[arg(short, long, action)]
    rescan: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Carica variabili d'ambiente
    dotenv().ok();
    
    // Configura logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Parsa gli argomenti CLI
    let args = Args::parse();

    log::info!("Starting Media Manager v{}", env!("CARGO_PKG_VERSION"));

    // Inizializza database
    let db_pool = init_db().await.expect("Failed to initialize database");
    log::info!("Database initialized");

    // Scanner iniziale se richiesto
    if args.rescan {
        log::info!("Starting media library scan...");
        scanner::scan_library(&args.media_path, &db_pool)
            .await
            .expect("Library scan failed");
        log::info!("Library scan completed");
    }

    // Configura server HTTP
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(args.media_path.clone()))
            .service(api::movies::get_movies)
            .service(api::movies::search_movies)
            .service(api::tv::get_shows)
            .service(api::tv::get_episodes)
            .service(api::scan::start_scan)
    })
    .bind("127.0.0.1:8080")?;

    log::info!("Server running on http://127.0.0.1:8080");
    server.run().await
}
