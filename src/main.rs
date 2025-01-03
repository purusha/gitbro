#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::File;
use axum::{routing::get, Router};

mod repository;
use repository::*;

mod abc;

use rand::{distributions::Alphanumeric, Rng};

#[tokio::main]
async fn main() {
    let app_name: &str = env!("CARGO_PKG_NAME");
    let file_log: Result<File, std::io::Error> = File::create(format!("{}.log", app_name));

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info, Config::default(), file_log.unwrap()
        ),
    ]).unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/commit", get(get_commit));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_commit() -> String {
    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");

    resolve();

    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(42)
        .map(char::from)
        .collect();

    return s;
}