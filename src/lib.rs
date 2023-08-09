#![cfg_attr(doc, feature(async_fn_in_trait))]
//! A library and associated binary for retrieving, storing, and searching open access publications

mod repo;
mod url;

pub mod cli;
pub mod cmd;
pub use self::repo::Repo;
pub use self::url::Url;
