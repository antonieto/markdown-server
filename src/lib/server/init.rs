use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::gitreader::RepoHandle;

async fn route_service(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let handle = match RepoHandle::from_uri(req.uri().path()) {
        Ok(h) => h,
        Err(e) => {
            let body = Full::new(Bytes::from(e));
            let res = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap();
            return Ok(res);
        }
    };

    Ok(Response::new(Full::new(Bytes::from(format!(
        "Owner: {} \nName: {}",
        handle.owner, handle.name
    )))))
}

// Init function -- server entry point
#[tokio::main]
pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(route_service))
                .await
            {
                eprintln!("Error serving connection {:?}", err);
            }
        });
    }
}
