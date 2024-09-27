use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Error, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::gitreader::{
    github_client::{Github, GithubClient},
    RepoHandle,
};

// Rotues requests
async fn route_service(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let handle = RepoHandle::from_uri(req.uri().path()).unwrap();

    let gh_client = GithubClient::new();

    match gh_client.get_tree(handle.clone()).await {
        Ok(tree) => {
            println!("SHA: {}", tree.sha);
        }
        Err(e) => {
            eprint!("{}", e.to_string());
        }
    }

    Ok(Response::new(Full::new(Bytes::from(format!(
        "Not working yet"
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
