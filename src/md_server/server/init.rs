use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use rustdown::{element, lexer};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use base64::{engine::general_purpose, Engine as _};

use crate::gitreader::gitreader::build_tree;
use crate::gitreader::{
    github_client::{Github, GithubClient},
    RepoHandle,
};
use crate::util::split_url_at_nth_slash;

// Routes requests
async fn route_service(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let gh_client = GithubClient::new();

    let mut response_content = String::new();

    if let Some((left, right)) = split_url_at_nth_slash(req.uri().path(), 3) {
        let handle = RepoHandle::from_uri(left).unwrap();
        // TODO use for side navigation
        let _tree_json = match &gh_client.get_tree(handle.clone()).await {
            Ok(res) => {
                // TODO: Use this to build nav bar
                let constructed_tree = build_tree(&res.tree);

                let serialized = serde_json::to_string_pretty(&constructed_tree).unwrap();

                serialized
            }
            Err(_) => String::new(),
        };

        // Build
        match &gh_client
            .get_file_content(&handle, String::from(right))
            .await
        {
            Ok(content) => {
                let cleaned_input: String = content
                    .content
                    .as_str()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                let bytes = general_purpose::STANDARD.decode(cleaned_input).unwrap();
                let decoded = String::from_utf8(bytes).unwrap();

                let mut html_root =
                    element::Element::Complex(element::ElementWithChildren::new("body"));

                let lexer = lexer::Lexer::new(decoded.as_str());

                for (tok, _) in lexer {
                    let (tag, content) = tok.as_tag();
                    let text_element = element::TextElement::new(tag, content);
                    let element = element::Element::Text(text_element);

                    html_root.add_child(element);
                }

                response_content = html_root.render();
            }
            _err => (),
        };

        // Build response using this file content
    } else {
        return Ok(Response::new(Full::new(Bytes::from(format!(
            "Not working yet"
        )))));
    }

    return Ok(Response::new(Full::new(Bytes::from(format!(
        "{}",
        response_content
    )))));
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
