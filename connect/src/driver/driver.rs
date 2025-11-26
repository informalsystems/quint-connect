use crate::driver::Step;
use serde::de::DeserializeOwned;

pub type Status = anyhow::Result<()>;

pub trait Driver {
    type State: DeserializeOwned;

    fn step(&mut self, step: &Step) -> Status;
}
