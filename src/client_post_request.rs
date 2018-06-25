extern crate hyper;
extern crate tokio_fs;

extern crate tokio_io;
extern crate http;
extern crate futures;

use std::io::{self, Write};

use hyper::{Client, Method, Request, Response};
use hyper::rt::{self, Future, Stream};
use hyper::body::Body;
use self::http::status::StatusCode;

static NOTFOUND: &[u8] = b"Not Found";

pub fn post_request(body: Body, uri: hyper::Uri) {



    // Run the runtime with the future trying to fetch and print this URL.
    //
    // Note that in more complicated use cases, the runtime should probably
    // run on its own, and futures should just be spawned into it.
//    rt::run(fetch_url(url));
    rt::run(fetch_url(body, uri));
}

fn get_url() -> hyper::Uri {
    "http://127.0.0.1:3000/upload_file".parse::<hyper::Uri>().unwrap()
}

pub fn fetch_url(body: Body, url: hyper::Uri) -> impl Future<Item=(), Error=()> {
    let client = Client::new();

    let mut request_builder = Request::builder();
    let req = request_builder.uri(url)
        .header("filename", "qq")
        .method(Method::POST)
        .body(Body::from("qweqwe"))
        .unwrap()
    ;
//    let mut req = Request::new(Body::from("qwe"));
//    let mut req = Request::new(body);
//    let mut req = Request::new(Body::wrap_stream(stream));
//    *req.method_mut() = Method::POST;
//    *req.uri_mut() = url;
//    *req.headers_mut() =
//    *req.uri_mut() = get_url();
//    *req.body_mut() = Body::from("rty");

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

type ResponseFuture = Box<Future<Item=Response<Body>, Error=io::Error> + Send>;


#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn test_fetch_url(){
        rt::run(fetch_url(Body::empty(), get_url()));
    }
}