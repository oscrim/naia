
use naia_derive::State;
use naia_shared::{State, Property};

use super::Protocol;

#[derive(State, Clone)]
#[type_name = "Protocol"]
pub struct Auth {
    pub username: Property<String>,
    pub password: Property<String>,
}

impl Auth {
    fn is_guaranteed() -> bool {
        false
    }

    pub fn new(username: &str, password: &str) -> Auth {
        return Auth::state_new_complete(username.to_string(), password.to_string());
    }
}
