mod status;
mod step;

pub mod nondet;

pub use status::Status;
pub use step::Step;

use serde::de::DeserializeOwned;

pub trait Driver {
    type State: DeserializeOwned;
    fn step(&mut self, step: &Step) -> Status;
}
