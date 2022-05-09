use actix_web::{HttpRequest, Responder};

use askama_actix::{Template, TemplateToResponse};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    messages: Vec<String>,
}

pub async fn root(_req: HttpRequest) -> impl Responder {
    log::info!("get root");

    HomeTemplate {
        messages: vec![String::from("hello world!"), String::from("from antiokh!!")],
    }
    .to_response()
}
