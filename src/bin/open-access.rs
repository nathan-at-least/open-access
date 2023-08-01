#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    open_access::cli::run().await
}
