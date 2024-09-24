pub mod gitreader;
pub mod http_client;

pub struct RepoHandle {
    pub owner: String,
    pub name: String,
}

impl RepoHandle {
    pub fn from_uri(uri: &str) -> Result<RepoHandle, String> {
        let mut iterator = uri.split("/").skip(1);

        let owner: String = if let Some(i) = iterator.next() {
            String::from(i)
        } else {
            return Err(String::from("URI is not well formatted"));
        };

        let name = if let Some(i) = iterator.last() {
            String::from(i)
        } else {
            return Err(String::from("URI is missing repository name"));
        };

        Ok(RepoHandle { owner, name })
    }
}
