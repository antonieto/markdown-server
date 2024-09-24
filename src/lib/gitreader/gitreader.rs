// Library used for querying a git repository and fetching markdown files from it
// Server will instance markdown repo and call a method to populate

// A handle to a github repo, exposing methods used to fetch contents of a file and stuff
#[allow(dead_code)]
pub struct GitReader {
    // Git handle - such as antonieto/markdown_server
    handle: String,
}

// Should have a method to read files in a repo
impl GitReader {
    // Constructor
    pub fn new(handle: String) -> GitReader {
        GitReader { handle }
    }

    pub fn read(&self) -> String {
        String::from("Hello, world")
    }
}
