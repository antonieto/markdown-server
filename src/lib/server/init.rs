use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use serde::Serialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::gitreader::gitreader::{build_tree, FileNode};
use crate::gitreader::{
    github_client::{Github, GithubClient},
    RepoHandle,
};

// Routes requests
async fn route_service(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let handle = RepoHandle::from_uri(req.uri().path()).unwrap();

    let gh_client = GithubClient::new();

    match &gh_client.get_tree(handle.clone()).await {
        Ok(res) => {
            let constructed_tree = build_tree(&res.tree);

            let serialized = serde_json::to_string_pretty(&constructed_tree).unwrap();
           

            println!("Built tree: {}", serialized);
        }
        Err(e) => {
            eprint!("{}", e.to_string());
        }
    }

    // TODO: Add rendered html to this response
    let _ = Response::builder().header("Content-Type", "text/html");

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
