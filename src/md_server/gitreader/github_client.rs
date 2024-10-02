static GITHUB_API_URL: &'static str = "https://api.github.com";
// Github HTTP Client wrapper
use serde::{Deserialize, Serialize};

use super::RepoHandle;

// Concrete instance and configuration
pub struct GithubClient {
    base_url: String,
    client: reqwest::Client,
}

impl GithubClient {
    pub fn new() -> GithubClient {
        let cl = reqwest::Client::builder()
            .user_agent("cli")
            .build()
            .unwrap();

        GithubClient {
            base_url: String::from(GITHUB_API_URL),
            client: cl,
        }
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
    pub path: String,
    mode: String,
    #[serde(rename = "type")]
    pub _type: String,
    sha: String,
    size: Option<u32>,
    url: String,
}

pub trait Github {
    #[allow(async_fn_in_trait)]
    async fn get_tree(
        &self,
        handle: RepoHandle,
    ) -> Result<GetTreeResponse, Box<dyn std::error::Error>>;

    #[allow(async_fn_in_trait)]
    async fn get_file_content(
        &self,
        handle: &RepoHandle,
        path: String,
    ) -> Result<GetFileContentResponse, Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize)]
pub struct GetFileContentResponse {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u32,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub content: String,
    pub encoding: String,
    pub _links: FileLinks,
}

#[derive(Serialize, Deserialize)]
pub struct FileLinks {
    #[serde(rename = "self")]
    pub _self: String,
    pub git: String,
    pub html: String,
}

impl Github for GithubClient {
    async fn get_tree(
        &self,
        handle: RepoHandle,
    ) -> Result<GetTreeResponse, Box<dyn std::error::Error>> {
        // TODO: either include branch in route or
        let url = format!(
            "{}/repos/{}/{}/git/trees/main?recursive=1",
            self.base_url, handle.owner, handle.name
        );

        let res = self.client.get(url).send().await?;

        let text = res.text().await?;

        let mut tree: GetTreeResponse = serde_json::from_str(text.as_str())?;

        // Sort here
        tree.tree.sort_by(|a, b| a.path.len().cmp(&b.path.len()));

        return Ok(tree);
    }

    async fn get_file_content(
        &self,
        handle: &RepoHandle,
        path: String,
    ) -> Result<GetFileContentResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}",
            self.base_url, handle.owner, handle.name, path
        );

        let res = self.client.get(url).send().await?;

        let text = res.text().await?;

        let content: GetFileContentResponse = serde_json::from_str(text.as_str())?;

        Ok(content)
    }
}
