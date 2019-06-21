use bufflib::registry;
use quicli::prelude::*;

pub fn execute(email: &str, password: &str) {
  trace!("Logging in with email {}", email);
  registry::login(email, password);
  info!("Logged in successfully!")
}
