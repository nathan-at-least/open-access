mod scheme;

use ::url::{ParseError as InnerParseError, Url as InnerUrl};
use std::fmt;
use std::str::FromStr;

pub use self::scheme::{Scheme, UnsupportedScheme};

#[derive(Clone, Debug)]
pub struct Url {
    inner: InnerUrl,
    scheme: Scheme,
}

impl Url {
    pub fn scheme(&self) -> Scheme {
        self.scheme
    }

    pub fn port(&self) -> Option<u16> {
        self.inner.port()
    }

    pub fn host_str(&self) -> Option<&str> {
        self.inner.host_str()
    }

    pub fn path_segments(&self) -> impl Iterator<Item=&str> {
        self.inner.path_segments().into_iter().flatten()
    }
}

impl TryFrom<InnerUrl> for Url {
    type Error = UnsupportedScheme;

    fn try_from(inner: InnerUrl) -> Result<Self, Self::Error> {
        let scheme = inner.scheme().parse()?;
        Ok(Url { inner, scheme })
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unsupported scheme")]
    UnsupportedScheme(#[from] UnsupportedScheme),
    #[error("malformed url")]
    Inner(#[from] InnerParseError),
}

impl FromStr for Url {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = <InnerUrl as FromStr>::from_str(s)?;
        let this = Url::try_from(inner)?;
        Ok(this)
    }
}

impl<'a> TryFrom<&'a str> for Url {
    type Error = ParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
