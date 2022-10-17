#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(dead_code)]

mod pages;
mod settings;
mod static_file;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use settings::SETTINGS;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(
                web::resource("/{filename:.*}")
                    .guard(static_file::StaticFileGuard)
                    .route(web::get().to(static_file::get_static_file)),
            )
            .service(web::resource("/").route(web::get().to(pages::home::root)))
    })
    .workers(SETTINGS.workers)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
