use tracing::{event, span, Level};

fn hello_world() {
    let span = span!(Level::INFO, "hello_world");
    let _enter = span.enter();
    event!(Level::INFO, "hello world");
    event!(Level::ERROR, "things went horribly wrong :(");
}

fn main() {
    tracing_subscriber::fmt().init();
    hello_world();
}
