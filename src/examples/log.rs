use std::time::Duration;

use log::*;

use krossbar_log_lib::init_logger;

#[tokio::main]
async fn main() {
    let logger = init_logger("com.examples.logging", LevelFilter::Trace, true).await;

    tokio::spawn(logger.run());

    loop {
        error!("Error message");
        warn!("Warning message");
        info!("Info message");
        debug!("Debug message");
        trace!("Trace message");

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
