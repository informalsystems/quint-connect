use anyhow::{Context, Result};
use itf::Value;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

/// Trait for extracting and comparing state between a Quint specification and a Rust
/// implementation.
///
/// This trait enables the framework to extract state from both the driver implementation
/// (via [`from_driver`](State::from_driver)) and the Quint specification (via internal
/// deserialization), then compare them for equality. The state type must be deserializable
/// from the [ITF](itf) format used by Quint traces.
///
/// See the [Quick Start](crate#quick-start) and [Examples](crate#examples) sections in the
/// crate docs for usage examples.
///
/// # Trait Bounds
///
/// - [`PartialEq`]: Required to compare implementation state with specification state
/// - [`DeserializeOwned`]: Required to deserialize state from Quint traces
/// - [`Debug`]: Required for error reporting when states diverge
///
/// # Stateless Drivers
///
/// For drivers that don't need state validation, use the unit type `()` which has a
/// default implementation of this trait.
///
/// # Deserialization Tips
///
/// See the [Tips and Tricks](crate#tips-and-tricks) section in the crate docs for
/// guidance on deserializing Quint types (enums, optional fields, etc.).
pub trait State<D>: PartialEq + DeserializeOwned + Debug {
    /// Extracts the state from a driver implementation.
    ///
    /// This method converts the driver's internal state into the state type that can be
    /// compared with the specification. Implementations should extract relevant fields
    /// from the driver and construct the state representation that matches the Quint
    /// specification.
    ///
    /// # Parameters
    ///
    /// - `driver`: Reference to the driver implementation
    ///
    /// # Returns
    ///
    /// The extracted state, or an error if state extraction fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the state cannot be extracted from the driver.
    fn from_driver(driver: &D) -> Result<Self>;

    #[doc(hidden)] // internal use only
    fn from_spec(value: Value) -> Result<Self> {
        Self::deserialize(value).context(
            "Failed to deserialize specification's state.\n\
             Please check the crate docs for tips and tricks on state deserialization.",
        )
    }
}

/// Implements [State] for the unit type, effectively disabling state checking for
/// the given test driver.
impl<D> State<D> for () {
    fn from_driver(_driver: &D) -> Result<Self> {
        Ok(())
    }

    fn from_spec(_value: Value) -> Result<Self> {
        Ok(())
    }
}
