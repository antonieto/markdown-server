use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::client::conn::http1::SendRequest;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

#[allow(async_fn_in_trait)]
pub trait HttpClient {
    // TODO: figure out how to deal with json responses
    async fn get(&mut self) -> Result<String, String>;
}

pub struct TokioHttp {
    base_url: String,
    hyper_client: SendRequest<Empty<Bytes>>,
    authority_str: String,
}

impl TokioHttp {
    fn new(&mut self, base_url: String) {
        self.base_url = base_url;
        let _ = self.connect();
    }

    // Starts connection and initializes hyper client
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Parse our URL...
        let url = self.base_url.as_str().parse::<hyper::Uri>()?;

        // Store authority str
        self.authority_str = String::from(url.authority().unwrap().clone().as_str());

        // Get the host and the port
        let host = url.host().expect("uri has no host");
        let port = url.port_u16().unwrap_or(80);

        let address = format!("{}:{}", host, port);

        // Open a TCP connection to the remote host
        let stream = TcpStream::connect(address).await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Create the Hyper client
        let (sender, _) = hyper::client::conn::http1::handshake(io).await?;

        self.hyper_client = sender;
        Ok(())
    }
}

impl HttpClient for TokioHttp {
    // We'll see how to strongly type for JSON
    async fn get(&mut self) -> Result<String, String> {
        if self.hyper_client.is_ready() {
            return Err(String::from("http client is not ready"));
        }
        let req = Request::builder()
            .uri(self.base_url.clone())
            .header(hyper::header::HOST, self.authority_str.clone())
            .body(Empty::<Bytes>::new())
            .map_err(|_| String::from("Could not build request"))?;

        let res = self
            .hyper_client
            .send_request(req)
            .await
            .map_err(|_| String::from("Failed to send request"))?;

        println!("Status: {}", res.status());

        Ok(String::from("work in progress"))
    }
}
