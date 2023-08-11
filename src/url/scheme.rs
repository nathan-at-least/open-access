use std::fmt;
use std::str::FromStr;

/// The subset of supported URL schemes
#[derive(Copy, Clone, Debug)]
pub enum Scheme {
    Http,
    Https,
    Ftp,
}

impl Scheme {
    pub fn as_str(self) -> &'static str {
        use Scheme::*;

        match self {
            Http => "http",
            Https => "https",
            Ftp => "ftp",
        }
    }
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Clone, Debug, thiserror::Error)]
#[error("unsupported scheme: {0:?}")]
pub struct UnsupportedScheme(pub String);

impl FromStr for Scheme {
    type Err = UnsupportedScheme;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Scheme::*;

        match s {
            "http" => Ok(Http),
            "https" => Ok(Https),
            "ftp" => Ok(Ftp),
            other => Err(UnsupportedScheme(other.to_string())),
        }
    }
}
