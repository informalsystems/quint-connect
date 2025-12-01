mod nondet;
mod state;
mod step;

pub use state::State;
pub use step::Step;

/// A convenience type alias for [`anyhow::Result`] used throughout this crate.
///
/// This type defaults to `Result<()>` when no type parameter is provided, making it
/// ergonomic for functions that return success/failure without a value.
pub type Result<A = ()> = anyhow::Result<A>;

/// A static path used to navigate nested structures in Quint specifications.
///
/// Paths are represented as static string slices and are used in [`Config`] to specify
/// where to find state and nondeterministic picks within the specification's state space.
///
/// # Examples
///
/// ```rust
/// use quint_connect::Config;
///
/// let config = Config {
///     state: &["global_var", "nested_record", "my_state"],
///     nondet: &["nondet_choices"],
/// };
/// ```
pub type Path = &'static [&'static str];

/// Configuration for a [`Driver`] that specifies where to find state and nondeterministic
/// picks within a Quint specification.
///
/// By default, both paths are empty (`&[]`). Empty paths indicate that:
/// - State is extracted from the top level of the specification's state space
/// - Nondeterministic picks are extracted from Quint's builtin `mbt::actionTaken` and
///   `mbt::nondetPicks` variables
///
/// Override these paths when your specification nests the relevant state within a larger
/// structure, or when tracking nondeterminism manually rather than using Quint's builtin
/// variables.
///
/// # Examples
///
/// Specifying custom paths for nested state:
///
/// ```rust
/// use quint_connect::{Driver, Config};
/// # use quint_connect::{Step, Result, State};
/// #
/// # #[derive(Debug, PartialEq, serde::Deserialize)]
/// # struct MyState;
/// #
/// # impl State<MyDriver> for MyState {
/// #     fn from_driver(driver: &MyDriver) -> Result<Self> { Ok(MyState) }
/// # }
/// #
/// # struct MyDriver;
///
/// impl Driver for MyDriver {
///     type State = MyState;
///
///     fn config() -> Config {
///         Config {
///             state: &["global_var", "nested_record", "my_state"],
///             nondet: &["global_var", "nondet_choices"],
///         }
///     }
///
///     fn step(&mut self, step: &Step) -> Result {
///         // ...
/// #       Ok(())
///     }
/// }
/// ```
#[derive(Default)]
pub struct Config {
    /// Path to the state within the Quint specification's state space.
    ///
    /// An empty path (`&[]`) indicates the state is at the top level.
    pub state: Path,

    /// Path to nondeterministic picks within the Quint specification's state space.
    ///
    /// An empty path (`&[]`) uses Quint's builtin `mbt::actionTaken` and `mbt::nondetPicks`
    /// variables.
    pub nondet: Path,
}

/// Core trait for connecting Rust implementations to Quint specifications.
///
/// Implementations of this trait define how to execute steps from a Quint trace against
/// a Rust implementation, enabling model-based testing. The framework automatically
/// generates traces from your Quint specification and replays them through your driver,
/// verifying that the implementation state matches the specification state after each step.
///
/// See the [Quick Start](crate#quick-start) and [Examples](crate#examples) sections in the
/// crate docs for examples.
///
/// # Associated Types
///
/// - [`State`]: The state type that can be extracted from both the driver implementation
///   and the Quint specification for comparison.
///
/// # Required Methods
///
/// - [`step`](Driver::step): Processes a single step from the trace, typically by
///   pattern-matching on the action name and nondeterministic picks, then executing the
///   corresponding implementation code.
///
/// # Optional Methods
///
/// - [`config`](Driver::config): Returns configuration specifying where to find state
///   and nondeterministic picks in the specification. Defaults to top-level paths.
pub trait Driver: Sized {
    /// The state type that can be extracted from both the driver and the specification.
    ///
    /// This type must implement [`State`] to provide state extraction and comparison logic.
    /// Note that stateless drivers can set `State = ()` to disable state checking.
    type State: State<Self>;

    /// Processes a single step from a Quint trace.
    ///
    /// This method is called for each step in a generated trace. Implementations typically
    /// use the [`switch!`](crate::switch) macro to pattern-match on the action name and
    /// execute the corresponding implementation code.
    fn step(&mut self, step: &Step) -> Result;

    /// Returns configuration for this driver.
    ///
    /// Override this method to specify custom paths for extracting state and
    /// nondeterministic picks from nested structures in the specification.
    fn config() -> Config {
        Config::default()
    }
}
