use tracing::{error, info, instrument};

#[instrument]
fn hello_world(who: &str) {
    info!("hello {who}");
    error!("things went horribly wrong :(");
}

fn main() {
    tracing_subscriber::fmt().init();
    hello_world("world");
}
