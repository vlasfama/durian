use blockchain;
use blockchain::blockchain::Blockchain;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::{convert::Infallible, net::SocketAddr};

async fn gas_Price(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("{id:1,jsonrpc:2.0,result:0x0}")))
}

#[tokio::main]
pub async fn Start() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(gas_Price)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
