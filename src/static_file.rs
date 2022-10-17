use std::borrow::Cow;

use actix_web::{
    guard::{Guard, GuardContext},
    HttpRequest, HttpResponse,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
pub struct StaticFile;
// TODO is there a way to compile in static assets in such a way as to include their hash?
// And also to get a handle to this hash-included path?

pub async fn get_static_file(req: HttpRequest) -> HttpResponse {
    let path = std::path::Path::new(req.match_info().query("filename"));

    if let Some(path_str) = path.to_str() {
        if let Some(static_data) = StaticFile::get(path_str) {
            let mut response = HttpResponse::Ok();

            if let Some(os_str_extension) = path.extension() {
                if let Some(extension) = os_str_extension.to_str() {
                    let mimetype = actix_files::file_extension_to_mime(extension);
                    response.content_type(mimetype);
                }
            }

            return match static_data.data {
                Cow::Borrowed(content) => response.body(content),
                Cow::Owned(content) => response.body(content),
            };
        }
    }

    HttpResponse::NotFound().finish()
}

pub struct StaticFileGuard;
impl Guard for StaticFileGuard {
    fn check(&self, req: &GuardContext<'_>) -> bool {
        let mut path = req.head().uri.path();

        if path.starts_with('/') {
            path = &path[1..];
        }

        matches!(StaticFile::get(path), Some(_))
    }
}

#[cfg(test)]
mod test {
    use super::get_static_file;

    use actix_web::{
        http::{header, StatusCode},
        test,
    };

    #[actix_web::test]
    async fn test_get_static_asset_favicon() {
        let req = test::TestRequest::default()
            .param("filename", "favicon.ico")
            .uri("http://localhost/favicon.ico")
            .to_http_request();
        let resp = get_static_file(req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "image/x-icon"
        );
    }

    #[actix_web::test]
    async fn test_get_static_asset_json() {
        let req = test::TestRequest::default()
            .param("filename", "manifest.json")
            .uri("http://localhost/manifest.json")
            .to_http_request();
        let resp = get_static_file(req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
    }

    #[actix_web::test]
    async fn test_get_static_asset_not_found() {
        let req = test::TestRequest::default()
            .param("filename", "foo.txt")
            .uri("http://localhost/foo.txt")
            .to_http_request();
        let resp = get_static_file(req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        assert!(resp.headers().get(header::CONTENT_TYPE).is_none());
    }
}
