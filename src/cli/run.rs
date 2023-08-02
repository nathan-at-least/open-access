use crate::cli::options::Options;

/// Run the commandline tool; `open-access` executable directly delegates to this
pub async fn run() -> anyhow::Result<()> {
    let options = Options::parse();
    options.run().await
}
