use crate::cli::options::{Command, FetchOptions, Options};
use async_trait::async_trait;

/// An abstraction for running cli logic for different options and commands
#[cfg_attr(not(doc), async_trait)]
pub trait Runnable {
    async fn run(&self) -> anyhow::Result<()>;
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for Options {
    async fn run(&self) -> anyhow::Result<()> {
        self.command.run().await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for Command {
    async fn run(&self) -> anyhow::Result<()> {
        match self {
            Command::Fetch(x) => x.run().await,
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for FetchOptions {
    async fn run(&self) -> anyhow::Result<()> {
        crate::cmd::fetch(&self.url).await
    }
}
