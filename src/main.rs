use lib::gitreader::http_client;

fn main() {
    // let _ = init::start_server();

    let _ = http_client::tokio_main();
}
