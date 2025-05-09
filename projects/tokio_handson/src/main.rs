use tokio::task::JoinSet;
use tracing::error;
mod api;
mod tester;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Set Tracing
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut join_set = JoinSet::new();
    join_set.spawn(api::run());
    join_set.spawn(tester::run());

    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok(_)) => (),
            Ok(Err(err)) => error!("Task Error: {}", err),
            Err(e) => error!("{e:?}"),
        }
    }

    eyre::Ok(())
}
