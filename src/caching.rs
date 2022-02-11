use rocket::http::Header;
use rocket::request::Request;
use rocket::response::{self, Responder};

pub struct Cached<R> {
    inner: R,
    directives: Vec<String>,
}

impl<'r, 'o: 'r, R> Responder<'r, 'o> for Cached<R>
where
    R: Responder<'r, 'o>,
{
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let Cached { inner, directives } = self;
        inner.respond_to(req).map(|mut res| {
            for directive in directives {
                res.set_header(Header::new("Cache-Control", directive));
            }
            res
        })
    }
}

pub trait Caching
where
    Self: Sized,
{
    fn cached(self, directives: Vec<String>) -> Cached<Self>;
}

impl<'r, 'o: 'r, R> Caching for R
where
    R: Responder<'r, 'o>,
{
    fn cached(self, directives: Vec<String>) -> Cached<Self> {
        Cached {
            inner: self,
            directives,
        }
    }
}
