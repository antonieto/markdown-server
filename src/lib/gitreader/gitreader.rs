// Library used for querying a git repository and fetching markdown files from it
// Server will instance markdown repo and call a method to populate

// A handle to a github repo, exposing methods used to fetch contents of a file and stuff
// c is the lifetime of tokio http
#[allow(dead_code)]
pub struct GitReader {
    // Git handle - such as antonieto/markdown_server
}

// Should have a method to read files in a repo
impl GitReader {
}

pub struct FileNode {
    node_type: FileType,
    children: Vec<FileNode>,
}

impl FileNode {
    pub fn new(n_type: FileType) -> FileNode {
        Self {
            node_type: n_type,
            children: Vec::new(),
        }
    }
}

enum FileType {
    DIR,
    FILE,
}

enum ApiError {
    Unknown,
}
