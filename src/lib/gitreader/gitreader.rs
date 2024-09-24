use super::http_client::{HttpClient, TokioHttp};

// Library used for querying a git repository and fetching markdown files from it
// Server will instance markdown repo and call a method to populate

// A handle to a github repo, exposing methods used to fetch contents of a file and stuff
// c is the lifetime of tokio http
#[allow(dead_code)]
pub struct GitReader<'c> {
    // Git handle - such as antonieto/markdown_server
    handle: String,
    http_client: &'c TokioHttp,
}

// Should have a method to read files in a repo
impl<'t> GitReader<'t> {
    // Constructor
    pub fn new(handle: String, client: &TokioHttp) -> GitReader {
        GitReader {
            handle,
            http_client: client,
        }
    }

    pub fn read(&self) -> String {
        String::from("Hello, world")
    }

    //TODO: Implement ListFiles
}

pub struct FileNode {
    node_type: FileType,
}

enum FileType {
    DIR,
    FILE,
}
