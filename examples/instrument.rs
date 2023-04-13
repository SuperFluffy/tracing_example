use tracing::{error, info, instrument};

#[instrument]
fn hello_world() {
    info!("hello world");
    error!("things went horribly wrong :(");
}

fn main() {
    tracing_subscriber::fmt().init();
    hello_world();
}
