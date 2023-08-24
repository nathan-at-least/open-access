use crate::Url;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

/// A path to the repository
///
/// This wraps [PathBuf] to provide an appropriate [Default] impl.
#[derive(Clone, Debug, derive_more::From)]
pub struct Repo(PathBuf);

impl Repo {
    pub fn get_url_path(&self, url: &Url) -> PathBuf {
        let mut pb = self
            .0
            .join(format!(
                "{}{}",
                url.scheme(),
                url.port()
                    .map(|p| format!("_{p}"))
                    .unwrap_or("".to_string())
            ))
            .join(url.host_str().unwrap_or("_no_host_"));

        for segment in url.path_segments() {
            pb.push(segment);
        }

        pb
    }
}

impl Default for Repo {
    fn default() -> Self {
        Repo(
            directories_next::ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
                .unwrap()
                .data_local_dir()
                .join("repo"),
        )
    }
}

impl FromStr for Repo {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PathBuf::from_str(s).map(Repo)
    }
}

impl Display for Repo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.display().fmt(f)
    }
}

#[cfg(test)]
mod tests;
