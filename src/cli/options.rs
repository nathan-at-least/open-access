use clap::{Args, Parser, Subcommand};
use reqwest::Url;

/// Manage open access research papers
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub command: Command,
}

impl Options {
    /// Parse the commandline arguments; errors display help to the user and then exits
    pub fn parse() -> Self {
        // This just wraps the trait method so that consumers do not need the trait in scope:
        <Self as Parser>::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Fetch(FetchOptions),
}

/// Manage open access research papers
#[derive(Debug, Args)]
pub struct FetchOptions {
    pub url: Url,
}
