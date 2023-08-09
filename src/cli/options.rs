//! Commandline options and parsing
use crate::{Repo, Url};
use clap::{Args, Parser, Subcommand};

/// Manage open access research papers
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[clap(short, long, default_value_t)]
    pub repo: Repo,

    #[command(subcommand)]
    pub command: Command,
}

impl Options {
    /// Parse the commandline arguments; errors display help to the user and then exits
    pub fn parse() -> Self {
        // This just wraps the trait method so that consumers do not need the trait in scope:
        <Self as Parser>::parse()
    }

    /// Run the cli behavior with the given options in `self`
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Fetch(opts) => crate::cmd::fetch(&self.repo, &opts.url).await,
        }
    }
}

/// The command to execute
#[derive(Debug, Subcommand)]
pub enum Command {
    Fetch(FetchOptions),
}

/// Fetch a publication and insert it into the repository
#[derive(Debug, Args)]
pub struct FetchOptions {
    pub url: Url,
}
