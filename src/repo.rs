use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

/// A path to the repository
///
/// This wraps [PathBuf] to provide an appropriate [Default] impl.
#[derive(Clone, Debug)]
pub struct Repo(PathBuf);

impl Default for Repo {
    fn default() -> Self {
        Repo(
            dirs::data_local_dir()
                .unwrap()
                .join(env!("CARGO_PKG_NAME"))
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
