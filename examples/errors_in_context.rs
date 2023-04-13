use std::{error::Error, fmt::Display};

use tracing::{error, info, instrument};
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

impl Error for NoName {}

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
fn hello(id: Uuid, salutation: &str, person: &Person) {
    match person.make_greeting(salutation) {
        Ok(greeting) => info!(greeting, "greeting created"),
        Err(e) => error!(error = ?e, "failed creating a greeting"),
    }
}

fn main() {
    tracing_subscriber::fmt().init();
    let michael = Person {
        name: "".into(),
        _password: "1234".into(),
    };
    let id = Uuid::new_v4();
    hello(id, "good day", &michael);
}
