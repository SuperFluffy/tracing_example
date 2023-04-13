use tracing::{info, instrument};
use uuid::Uuid;

struct Person {
    name: String,
    _password: String,
}

impl Person {
    #[instrument(fields(name = self.name), skip(self))]
    fn make_greeting(&self, salutation: &str) -> String {
        info!("greeting requested");
        format!("{salutation}, {}", self.name)
    }
}

#[instrument(fields(id = %id.as_simple()), skip_all)]
fn hello(id: Uuid, salutation: &str, person: &Person) {
    let greeting = person.make_greeting(salutation);
    info!(greeting, "greeting created");
}

fn main() {
    tracing_subscriber::fmt().init();
    let michael = Person {
        name: "Michael".into(),
        _password: "1234".into(),
    };
    let id = Uuid::new_v4();
    hello(id, "good day", &michael);
}
