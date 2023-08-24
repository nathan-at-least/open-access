//! High level commands below the [cli](crate::cli) layer
use crate::{Repo, Url};

/// Fetch a publication at the given [Url] along with associated metadata
///
/// The resulting publication and metadata are stored in the repository.
#[tracing::instrument]
pub async fn fetch(repo: &Repo, url: &Url) -> anyhow::Result<()> {
    use tokio::fs::File;

    let repopath = repo.get_url_path(url);

    tracing::debug!(?repopath, "creating parent directory");
    tokio::fs::create_dir_all(repopath.parent().unwrap()).await?;

    let client = reqwest::Client::new();
    let mut response = client.get(url.as_str()).send().await?.error_for_status()?;

    let mut f = File::create(repopath).await?;
    while let Some(chunk) = response.chunk().await? {
        use tokio::io::AsyncWriteExt;

        f.write_all(&chunk).await?;
    }

    Ok(())
}
