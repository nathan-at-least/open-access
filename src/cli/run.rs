use crate::cli::options::Options;
use crate::cli::Runnable;

/// Run the commandline tool; `main` delegates to this
pub async fn run() -> anyhow::Result<()> {
    let options = Options::parse();
    options.run().await
}
