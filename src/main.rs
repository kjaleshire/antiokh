#![feature(let_chains)]

mod pages;
mod static_asset;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use static_asset::StaticAssetGuard;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/{filename:.*}").guard(StaticAssetGuard).route(web::get().to(static_asset::get_static_file)))
            .service(web::resource("/").route(web::get().to(pages::home::root)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
