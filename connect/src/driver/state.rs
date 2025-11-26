use crate::itf::Value;
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub trait State<D>: PartialEq + DeserializeOwned + Debug {
    fn from_driver(driver: &D) -> Result<Self>;

    fn from_spec(value: Value) -> Result<Self> {
        Ok(Self::deserialize(value).context("Failed to deserialize specification's state")?)
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
