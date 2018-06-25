extern crate hyper;
extern crate tokio_fs;

extern crate tokio_io;
extern crate http;
extern crate futures;

use std::io::{self, Write};
use std::path::Path;

use hyper::{Client, Method, Request, Response};
use hyper::rt::{self, Future, Stream};
use hyper::body::Body;
use self::http::status::StatusCode;
use super::read_from_path;

static NOTFOUND: &[u8] = b"Not Found";

fn get_url() -> hyper::Uri {
    "http://127.0.0.1:3000/upload_file".parse::<hyper::Uri>().unwrap()
}

pub fn push_request(req: Request<Body>) -> impl Future<Item=(), Error=()> {
    let client = Client::new();

    client
        // Fetch the url...
//        .get(url)
        .request(req)
        // And then, if we get a response back...
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            // The body is a stream, and for_each returns a new Future
            // when the stream is finished, and calls the closure on
            // each chunk of the body...
            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        // If all good, just tell the user...
        .map(|_| {
            println!("\n\nDone.");
        })
        // If there was an error, let the user know...
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}

pub fn make_request_from_file_path<P: AsRef<Path>>(uri: hyper::Uri, file_path: P) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .header("filename", file_path.as_ref().as_os_str().to_str().unwrap())
        .method(Method::POST)
//        .body(Body::from("qweqwe"))
        .body(Body::from(read_from_path(".gitignore").unwrap()))
        .unwrap()
}

type ResponseFuture = Box<Future<Item=Response<Body>, Error=io::Error> + Send>;


#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn test_push_request(){
        let mut request_builder = Request::builder();
        let req = request_builder.uri(get_url())
            .header("filename", "qq")
            .method(Method::POST)
            .body(Body::from("qweqwe"))
            .unwrap()
        ;

        rt::run(push_request(req));
    }

    # [test]
    fn test_make_request_from_file(){
        rt::run(push_request(make_request_from_file_path(get_url(), ".gitignore")));
    }
}