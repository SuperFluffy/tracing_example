use std::fmt::Display;

use tracing::{info, instrument};
use tracing_error::{ErrorLayer, InstrumentResult, TracedError};
use tracing_subscriber::prelude::*;

use uuid::Uuid;

struct Person {
    name: String,
    _password: String,
}

#[derive(Debug)]
struct NoName;

impl Display for NoName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("person had no name")
    }
}

impl std::error::Error for NoName {}

#[derive(Debug)]
struct Error {
    source: TracedError<NoName>,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.source()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.source, f)
    }
}

impl From<NoName> for Error {
    fn from(value: NoName) -> Self {
        Self {
            source: value.into(),
        }
    }
}

impl Person {
    #[instrument(fields(name = self.name), skip(self))]
    fn make_greeting(&self, salutation: &str) -> Result<String, NoName> {
        info!("greeting requested");
        if self.name.is_empty() {
            return Err(NoName);
        }
        Ok(format!("{salutation}, {}", self.name))
    }
}

#[instrument(fields(id = %id.as_simple()), skip_all)]
fn hello(id: Uuid, salutation: &str, person: &Person) -> Result<(), TracedError<NoName>> {
    let greeting = person.make_greeting(salutation).in_current_span()?;
    info!(greeting, "greeting created");
    Ok(())
}

fn print_spantrace(error: &(dyn std::error::Error + 'static)) {
    let mut error = Some(error);
    let mut ind = 0;

    eprintln!("Error:");

    while let Some(err) = error {
        eprintln!("{:>4}: {}", ind, err);
        error = err.source();
        ind += 1;
    }
}

fn main() {
    tracing_subscriber::Registry::default()
        .with(ErrorLayer::default())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let michael = Person {
        name: "".into(),
        _password: "1234".into(),
    };
    let id = Uuid::new_v4();
    if let Err(e) = hello(id, "good day", &michael) {
        print_spantrace(&e);
    }
}
