mod caching;
mod headers;

use std::env;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;

use rocket_dyn_templates::Template;

use caching::{Cached, Caching};
use lazy_static::lazy_static;
use serde::Serialize;

use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};

lazy_static! {
    static ref ASSETS: AssetFiles = {
        AssetFiles {
            css: CSSFiles {
                app: "/static/styles/app_packaged.css".into(),
                fonts: "/static/styles/fonts_packaged.css".into(),
                vendor: "/static/styles/vendor_packaged.css".into(),
            },
        }
    };
    static ref ROBOTS_TXT_DISALLOW_ALL: bool = env::var("ROBOTS_TXT_DISALLOW_ALL").is_ok();
}

#[derive(Serialize)]
struct Context<T: ::serde::Serialize> {
    page: String,
    title: String,
    parent: &'static str,
    data: T,
    assets: &'static AssetFiles,
}

impl<T: ::serde::Serialize> Context<T> {
    fn new(page: String, title: &str, data: T) -> Self {
        let title = if title.is_empty() {
            "".into()
        } else {
            title.into()
        };
        Self {
            page,
            title,
            parent: LAYOUT,
            data,
            assets: &ASSETS,
        }
    }
}

#[derive(Clone, Serialize)]
struct CSSFiles {
    app: String,
    fonts: String,
    vendor: String,
}

#[derive(Clone, Serialize)]
struct AssetFiles {
    css: CSSFiles,
}

static LAYOUT: &str = "components/layout";

#[get("/static/<file..>", rank = 1)]
async fn files(file: PathBuf) -> Option<Cached<NamedFile>> {
    let max_age_header = format!("max-age={}", 3600);
    NamedFile::open(Path::new("static/").join(file))
        .await
        .ok()
        .map(|file| file.cached(vec![max_age_header]))
}

#[get("/robots.txt", rank = 1)]
fn robots_txt() -> Option<&'static str> {
    if *ROBOTS_TXT_DISALLOW_ALL {
        Some("User-agent: *\nDisallow: /")
    } else {
        None
    }
}

#[get("/")]
fn index() -> Template {
    render_simple_template("index")
}

#[get("/services")]
fn services() -> Template {
    render_simple_template("services")
}

#[get("/tips-and-tricks")]
fn tips_and_tricks() -> Template {
    render_simple_template("tips-and-tricks")
}

#[get("/about-us")]
fn about_us() -> Template {
    render_simple_template("about-us")
}

#[get("/contact")]
fn contact() -> Template {
    render_simple_template("contact")
}

#[catch(404)]
fn not_found() -> Template {
    not_found_html()
}

fn not_found_html() -> Template {
    let page = "404";
    let context = Context::new("404".into(), "error404-page-title", ());
    Template::render(page, &context)
}

#[catch(500)]
fn catch_error() -> Template {
    not_found_html()
}

fn render_simple_template(name: impl Into<String>) -> Template {
    let page = name.into();
    let context = Context::new(page.clone(), &page, ());

    Template::render(page, &context)
}

#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    let rocket = rocket::build()
        .attach(Template::fairing())
        .attach(headers::InjectHeaders)
        .mount(
            "/",
            routes![
                index,
                services,
                tips_and_tricks,
                about_us,
                contact,
                files,
                robots_txt,
            ],
        )
        .register("/", catchers![not_found, catch_error]);

    if is_running_on_lambda() {
        // Launch on AWS Lambda
        launch_rocket_on_lambda(rocket).await?;
    } else {
        // Launch local server
        rocket.launch().await?;
    }
    Ok(())
}
