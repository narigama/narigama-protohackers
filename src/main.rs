mod prime_time;
mod smoke_test;
mod util;

use clap::Parser;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // parse args
    let args = util::Args::parse();

    // setup logging
    tracing_subscriber::fmt::init();

    // spawn tasks
    tracing::info!("spawning workers");
    let tasks = vec![
        tokio::spawn(smoke_test::main(args.host.clone(), args.port)),
        tokio::spawn(prime_time::main(args.host.clone(), args.port + 1)),
    ];

    // now join all
    futures::future::join_all(tasks).await;

    // all good
    Ok(())
}
