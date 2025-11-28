use anyhow::{Context, Result};
use itf::Value;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub trait State<D>: PartialEq + DeserializeOwned + Debug {
    fn from_driver(driver: &D) -> Result<Self>;

    #[doc(hidden)] // internal use only
    fn from_spec(value: Value) -> Result<Self> {
        Self::deserialize(value).context(
            "Failed to deserialize specification's state.\n\
             Please check the crate docs for tips and tricks on state deserialization.",
        )
    }
}

impl<D> State<D> for () {
    fn from_driver(_driver: &D) -> Result<Self> {
        Ok(())
    }

    fn from_spec(_value: Value) -> Result<Self> {
        Ok(())
    }
}
