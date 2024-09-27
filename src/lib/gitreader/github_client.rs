use hyper_util::client::legacy;

static GITHUB_API_URL: &'static str = "https://api.github.com";
// Github HTTP Client wrapper
use serde::{Deserialize, Serialize};

use super::RepoHandle;

// Concrete instance and configuration
pub struct GithubClient {
    base_url: String,
}

impl GithubClient {
    pub fn new() -> GithubClient {
        let instance = GithubClient {
            base_url: String::from(GITHUB_API_URL),
        };
        instance
    }
}

// Module for consuming Github API
#[derive(Serialize, Deserialize)]
pub struct GetTreeResponse {
    pub sha: String,
    pub url: String,
    pub tree: Vec<TreeItem>,
}

#[derive(Serialize, Deserialize)]
pub struct TreeItem {
    path: String,
    mode: String,
    #[serde(rename = "type")]
    _type: String,
    sha: String,
    size: Option<u32>,
    url: String,
}

pub trait Github {
    async fn get_tree(
        &self,
        handle: RepoHandle,
    ) -> Result<GetTreeResponse, Box<dyn std::error::Error>>;
}

impl Github for GithubClient {
    async fn get_tree(
        &self,
        handle: RepoHandle,
    ) -> Result<GetTreeResponse, Box<dyn std::error::Error>> {
        let cl = reqwest::Client::new();
        let url = format!(
            "{}/repos/{}/{}/git/trees/main?recursive=1",
            self.base_url, handle.owner, handle.name
        );

        // TODO: refactor this into a middleware or something so every request includes header
        let res = cl.get(url).header("User-Agent", "rust-cli").send().await?;

        let text = res.text().await?;

        let tree: GetTreeResponse = serde_json::from_str(text.as_str())?;

        println!("tree: {}", tree.sha);

        return Ok(tree);
    }
}
