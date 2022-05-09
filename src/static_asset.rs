use std::borrow::Cow;

use actix_web::{
    guard::{Guard, GuardContext},
    HttpRequest, HttpResponse, Responder,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
pub struct StaticAsset;
// is there a way to compile in static assets in such a way as to include their hash?
// And also to get a handle to this hash-included path?

pub async fn get_static_file(req: HttpRequest) -> impl Responder {
    let path = std::path::Path::new(req.match_info().query("filename"));

    if let Some(path_str) = path.to_str() && let Some(static_data) = StaticAsset::get(path_str) {
        let mut response = HttpResponse::Ok();

        if let Some(os_str_extension) = path.extension() && let Some(extension) = os_str_extension.to_str() {
            let mimetype = actix_files::file_extension_to_mime(extension);
            response.content_type(mimetype);
        }

        return match static_data.data {
            Cow::Borrowed(content) => response.body(content),
            Cow::Owned(content) => response.body(content),
        }
    }

    HttpResponse::NotFound().finish()
}

pub struct StaticAssetGuard;
impl Guard for StaticAssetGuard {
    fn check(&self, req: &GuardContext) -> bool {
        let mut path = req.head().uri.path();

        if path.starts_with("/") {
            path = &path[1..];
        }

        if let Some(_) = StaticAsset::get(path) {
            true
        } else {
            false
        }
    }
}
