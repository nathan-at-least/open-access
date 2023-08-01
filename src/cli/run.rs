use crate::cli::options::Options;

/// Run the commandline tool; `main` delegates to this
pub async fn run() -> anyhow::Result<()> {
    let options = Options::parse();
    todo!("{options:#?}");
}
