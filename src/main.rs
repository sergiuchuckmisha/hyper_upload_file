
/**
basicaly use code from
https://github.com/hyperium/hyper/blob/master/examples/echo.rs
*/
extern crate futures;
extern crate hyper;


use futures::future;
use hyper::rt::{Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

mod config;
mod files_io_api;
mod client_post_request;
use files_io_api::*;
use config::*;


/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

static MISSING: &[u8] = b"Missing header \"filename\"";
static SUCCESS: &[u8] = b"Success";

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }

        // Simply echo the body back to the client.
        (&Method::GET, "/echo") => {
            *response.body_mut() = req.into_body();
        }

        // Simply echo the body back to the client.
        (&Method::POST, "/upload_file") => {

            if req.headers().get("filename").is_none() {
                *response.body_mut() = Body::from(MISSING);
            } else {
                println!("req.headers:{:?}", req.headers());
                println!("req.headers.get(\"filename\"):{:?}", req.headers().get("filename"));
                let filename = req.headers().get("filename").unwrap().to_str().unwrap().to_owned();
                println!("req.headers.get(\"filename\"):{:?}", filename);
                let file_write_result = req.into_body().concat2().map(move |chunk| {
                    let body = chunk.iter().cloned().collect::<Vec<u8>>();
                    write_to_file(make_path_from_file_name_and_folder_path(filename, SERVER_TEMPORARY_FOLDER_PATH), &body);//todo handle possible errors
                    *response.body_mut() = Body::from(SUCCESS);
                    response
                });
                return Box::new(file_write_result);
//                println!("file_content:{:?}", file_content);

//                let filename = String::from_utf8_lossy(filename_header.unwrap().as_ref());
//                write_to_file("q", file_content);
//                *response.body_mut() = req.into_body();
            }
        }

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => {
//            *response.body_mut() = req.into_body();
            println!("req.headers:{:?}", req.headers());
            println!("req.headers.get(\"filename\"):{:?}", req.headers().get("filename"));
            let filename = req.headers().get("filename").unwrap().to_str().unwrap().to_owned();
            println!("req.headers.get(\"filename\"):{:?}", filename);
            let reversed = req.into_body().concat2().map(move |chunk| {
                let body = chunk.iter().cloned().collect::<Vec<u8>>();
                println!("req.body:{:?}", String::from_utf8(body.clone()));
                write_to_file(filename, String::from_utf8(body.clone()).unwrap());
                *response.body_mut() = Body::from(body);
                response
            });

            return Box::new(reversed);
        }

        // Convert to uppercase before sending back to client.
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req.into_body().map(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });

            *response.body_mut() = Body::wrap_stream(mapping);
        }

        // Reverse the entire body before sending back to the client.
        //
        // Since we don't know the end yet, we can't simply stream
        // the chunks as they arrive. So, this returns a different
        // future, waiting on concatenating the full body, so that
        // it can be reversed. Only then can we return a `Response`.
        (&Method::POST, "/echo/reversed") => {
            let reversed = req.into_body().concat2().map(move |chunk| {
                let body = chunk.iter().rev().cloned().collect::<Vec<u8>>();
                *response.body_mut() = Body::from(body);
                response
            });

            return Box::new(reversed);
        }

        // The 404 Not Found route...
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}