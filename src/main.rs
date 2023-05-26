/// Core crates
use tokio;
use env_logger;
use env_logger::Env;

/// Custom crates;

mod config;
mod controllers;
mod helpers;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let routes = routes::themes::routes();
    warp::serve(routes).run(([127,0,0,1], 8000)).await;
    println!("Game engine service running");
}