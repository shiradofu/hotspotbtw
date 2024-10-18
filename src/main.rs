#[tokio::main]
async fn main() -> Result<(), ()> {
    hotspotbtw::run().await?;
    Ok(())
}
