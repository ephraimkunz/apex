use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header};
use rocket::{Request, Response};

static HEADERS: &[(&str, &str)] = &[
    ("x-xss-protection", "1; mode=block"),
    ("strict-transport-security", "max-age=63072000"),
    ("x-content-type-options", "nosniff"),
    (
        "referrer-policy",
        "no-referrer, strict-origin-when-cross-origin",
    ),
];

static HEADER_CSP_NORMAL: &str =
    "default-src 'self'; frame-ancestors 'self'; img-src 'self'; frame-src https://www.google.com https://www.youtube.com";
static HEADER_CSP_SVG: &str =
    "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'none'";

pub(crate) struct InjectHeaders;

#[rocket::async_trait]
impl Fairing for InjectHeaders {
    fn info(&self) -> Info {
        Info {
            name: "HTTP headers injector",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, response: &mut Response<'r>) {
        for (key, value) in HEADERS {
            response.set_header(Header::new(*key, *value));
        }
        let csp = if response.content_type() == Some(ContentType::SVG) {
            // SVGs adhere to Content Security Policy, and they often include inline styles.
            // This uses a custom CSP when the Content-Type is SVG.
            HEADER_CSP_SVG
        } else {
            HEADER_CSP_NORMAL
        };
        response.set_header(Header::new("content-security-policy", csp));
    }
}
