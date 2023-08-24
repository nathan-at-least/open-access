use crate::cli::options::Options;

/// Run the commandline tool; `open-access` executable directly delegates to this
///
/// This function initializes `tracing`, parses commandline options, and then runs them. An application wanting to embed `open-access` functionality should consider using [Options::run] instead.
pub async fn run() -> anyhow::Result<()> {
    init_tracing();
    let options = Options::parse();
    tracing::debug!(?options);
    options.run().await
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}
