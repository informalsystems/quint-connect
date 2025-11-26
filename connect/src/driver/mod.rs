mod step;

pub mod nondet;
pub use step::Step;

use serde::de::DeserializeOwned;

pub type Result = anyhow::Result<()>;

pub trait Driver {
    type State: DeserializeOwned;

    fn step(&mut self, step: &Step) -> Result;
}
