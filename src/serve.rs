extern crate futures;
extern crate hyper;
extern crate tokio_fs;
extern crate tokio_io;

use self::futures::{future, Future};
use self::hyper::{Body, Method, Request, Response, Server, StatusCode};
use self::hyper::service::service_fn;
use tokio::runtime::Runtime;

use std::io;

static NOTFOUND: &[u8] = b"Not Found";

pub fn run(rt: &mut Runtime) {
    let addr = "0.0.0.0:51497".parse().unwrap();
    let server = Server::bind(&addr)
        .serve(|| service_fn(routes))
        .map_err(|e| eprintln!("server error: {}", e));

    rt.spawn(server);

    println!("Listening on http://{}", addr);
}

type ResponseFuture = Box<Future<Item=Response<Body>, Error=io::Error> + Send>;

fn routes(req: Request<Body>) -> ResponseFuture {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/0") => {
            simple_file_send("/home/yuval/Videos/0")
        },
        _ => {
            Box::new(future::ok(Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::empty())
                                .unwrap()))
        }
    }

}

fn simple_file_send(f: &str) -> ResponseFuture {
    let filename = f.to_string();
    Box::new(tokio_fs::file::File::open(filename)
        .and_then(|file| {
            let buf: Vec<u8> = Vec::new();
            tokio_io::io::read_to_end(file, buf)
                .and_then(|item| {
                    Ok(Response::new(item.1.into()))
                })
                .or_else(|_| {
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap())
                })
        })
        .or_else(|_| {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }))
}
