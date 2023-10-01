extern crate error_chain;
extern crate futures;
extern crate swagger;
extern crate tokio;
extern crate transportation_api;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use log::info;
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use transportation_api::server::MakeService;

use db::Db;

use crate::server::Server;
mod conversion;
mod db;
mod errors;
mod model;
mod schema;
mod server;

pub async fn start_server() {
    let addr = "0.0.0.0:8090"
        .parse()
        .expect("Failed to parse bind address");

    info!("Start listening {}", addr);
    let manager = ConnectionManager::<SqliteConnection>::new("/tmp/Transportation.db");
    let database_pool = Box::leak(Box::new(
        Pool::builder()
            .build(manager)
            .expect("Failed to create DB pool."),
    ));
    let db = Box::leak(Box::new(Db::new(database_pool)));

    let server = Server::new(Box::leak(Box::new(*db)));

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        transportation_api::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    // Using HTTP
    info!("Starting server");
    hyper::server::Server::bind(&addr)
        .serve(service)
        .await
        .unwrap()
}

#[tokio::main]
async fn main() {
    env_logger::init();
    start_server().await;
}

