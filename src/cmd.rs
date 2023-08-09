//! High level commands below the [cli](crate::cli) layer
use crate::{Repo, Url};

/// Fetch a publication at the given [Url] along with associated metadata
///
/// The resulting publication and metadata are stored in the repository.
pub async fn fetch(repo: &Repo, url: &Url) -> anyhow::Result<()> {
    dbg!(repo, url);
    todo!();
}
