use crate::cli::options::{Command, FetchOptions, Options};
use async_trait::async_trait;

#[async_trait]
pub trait Runnable {
    async fn run(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl Runnable for Options {
    async fn run(&self) -> anyhow::Result<()> {
        self.command.run().await
    }
}

#[async_trait]
impl Runnable for Command {
    async fn run(&self) -> anyhow::Result<()> {
        match self {
            Command::Fetch(x) => x.run().await,
        }
    }
}

#[async_trait]
impl Runnable for FetchOptions {
    async fn run(&self) -> anyhow::Result<()> {
        todo!("{:#?}", self.url);
    }
}
