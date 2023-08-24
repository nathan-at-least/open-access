use ::url::{ParseError as InnerParseError, Url as InnerUrl};
use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
pub struct Url {
    inner: InnerUrl,
}

#[derive(Clone, Debug, thiserror::Error)]
#[error("malformed url {input:?}: {reason}")]
pub struct ParseError {
    input: String,
    reason: ParseErrorReason,
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ParseErrorReason {
    #[error("unsupported scheme {0:?}")]
    UnsupportedScheme(String),
    #[error("{0}")]
    Inner(#[from] InnerParseError),
}

impl Url {
    fn parse_str(s: &str) -> Result<Self, ParseErrorReason> {
        let inner = <InnerUrl as FromStr>::from_str(s)?;
        match inner.scheme() {
            "http" | "https" | "ftp" => Ok(()),
            other => Err(ParseErrorReason::UnsupportedScheme(other.to_string())),
        }?;
        Ok(Url { inner })
    }

    pub fn scheme(&self) -> &str {
        self.inner.scheme()
    }

    pub fn port(&self) -> Option<u16> {
        self.inner.port()
    }

    pub fn host_str(&self) -> Option<&str> {
        self.inner.host_str()
    }

    pub fn path_segments(&self) -> impl Iterator<Item = &str> {
        self.inner.path_segments().into_iter().flatten()
    }

    pub(crate) fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Debug for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Url({:?})", self.inner.as_str())
    }
}

impl FromStr for Url {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::parse_str(s).map_err(|reason| {
            let input = s.to_string();
            ParseError { input, reason }
        })
    }
}

impl<'a> TryFrom<&'a str> for Url {
    type Error = ParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
