use actix_web::{HttpRequest, Responder};

use askama_actix::{Template, TemplateToResponse};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    messages: Vec<&'static str>,
}

pub async fn root(_req: HttpRequest) -> impl Responder {
    HomeTemplate {
        messages: vec!["hello world!", "from antiokh!!"],
    }
    .to_response()
}
