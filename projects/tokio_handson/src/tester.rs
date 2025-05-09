use eyre::bail;
use rand::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use tracing::{error, info};

pub async fn run() -> eyre::Result<()> {
    loop {
        if let Err(e) = main().await {
            error!("{e}");
        }
    }
}

pub async fn main() -> eyre::Result<()> {
    info!("Starting the Tester...");
    let mut rng = StdRng::from_entropy();
    loop {
        sleep(Duration::from_secs(1)).await;
        let random_number = rng.gen_range(0..10);
        if random_number >= 5 {
            bail!("This is a test error {random_number}");
        }
    }
}
