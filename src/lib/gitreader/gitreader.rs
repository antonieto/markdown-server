// Library used for querying a git repository and fetching markdown files from it
// Server will instance markdown repo and call a method to populate

use std::collections::HashMap;

use serde::Serialize;

use super::github_client::TreeItem;

// A handle to a github repo, exposing methods used to fetch contents of a file and stuff
// c is the lifetime of tokio http
#[allow(dead_code)]
pub struct GitReader {
    // Git handle - such as antonieto/markdown_server
}

// Should have a method to read files in a repo
impl GitReader {}

// Recursive map Type

// TODO: implement tree read
#[allow(dead_code)]
#[derive(Serialize)]
pub struct FileNode {
    pub node_type: FileType,
    pub name: String,
    pub full_path: String,
    pub children: HashMap<String, FileNode>,
}

impl FileNode {
    pub fn new(name: String, n_type: FileType) -> FileNode {
        Self {
            node_type: n_type,
            children: HashMap::new(),
            name,
            full_path: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum FileType {
    DIR,
    FILE,
}

pub enum ApiError {
    Unknown,
}

// TODO: Add tests
// Builds a tree of files from a list of TreeItem from a
pub fn build_tree(items: &Vec<TreeItem>) -> FileNode {
    // Empty name for root directory
    let mut root = FileNode::new(String::new(), FileType::DIR);

    // Sort - shorter paths are closer to root
    // items.sort_by(|a, b| a.path.len().cmp(&b.path.len()));

    for item in items {
        let split: Vec<&str> = item.path.split("/").into_iter().collect();

        let n_type = if item._type == "tree" {
            FileType::DIR
        } else {
            FileType::FILE
        };

        add_to_tree(
            &mut root,
            split.clone().as_slice(),
            FileNode::new(item.path.clone(), n_type),
        );
    }

    println!("Successfully built tree");
    root
}

//  root: reference to the root of our tree
//  path: slice of string slices
//
//  returns a refernce to the created node
fn add_to_tree<'r>(
    root: &'r mut FileNode,
    path: &[&str],
    node_to_add: FileNode,
) -> Option<&'r FileNode> {
    if path.is_empty() {
        return Some(root);
    }

    let first = path[0];
    let rest = &path[1..];

    // Use entry API to handle both cases with a single mutable borrow
    root.children.entry(String::from(first)).or_insert_with(|| {
        let n_type = if path.len() > 1 {
            FileType::DIR
        } else {
            FileType::FILE
        };
        FileNode::new(String::from(first), n_type)
    });

    // Now that we've ensured the node exists, we can safely get a mutable reference
    // We need to use if let here because of lifetime issues
    if let Some(child) = root.children.get_mut(first) {
        add_to_tree(child, rest, node_to_add)
    } else {
        // This should never happen if the insertion was successful
        None
    }
}
