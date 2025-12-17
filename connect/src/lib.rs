#![doc = include_str!("../README.md")]

mod driver;
mod logger;
mod trace;
mod value;

// public for macro use
#[doc(hidden)]
pub mod runner;

pub use driver::{Config, Driver, Path, Result, State, Step};

/// Generates a test that runs multiple random traces by simulating a Quint specification.
///
/// This attribute macro transforms a function into a Rust test that generates traces using
/// `quint run` in simulation mode. The function should return a [`Driver`] implementation
/// that will be used to replay the generated traces.
///
/// # Attributes
///
/// - **`spec`** (required): Path to the Quint specification file
/// - **`main`**: Name of the main module to run (defaults to Quint's default)
/// - **`init`**: Name of the init action (defaults to Quint's default)
/// - **`step`**: Name of the step action (defaults to Quint's default)
/// - **`max_samples`**: Maximum number of traces to generate (defaults to 100)
/// - **`max_steps`**: Maximum number of steps per trace (defaults to Quint's default)
/// - **`seed`**: Random seed for reproducibility (defaults to random)
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use quint_connect::*;
/// # #[derive(Default)]
/// # struct MyDriver;
/// # impl Driver for MyDriver {
/// #     type State = ();
/// #     fn step(&mut self, _step: &Step) -> Result { Ok(()) }
/// # }
///
/// #[quint_run(spec = "spec.qnt")]
/// fn test_simulation() -> impl Driver {
///     MyDriver::default()
/// }
/// ```
///
/// With custom configuration:
///
/// ```rust
/// use quint_connect::*;
/// # #[derive(Default)]
/// # struct MyDriver;
/// # impl Driver for MyDriver {
/// #     type State = ();
/// #     fn step(&mut self, _step: &Step) -> Result { Ok(()) }
/// # }
///
/// #[quint_run(
///     spec = "spec.qnt",
///     main = "simulation",
///     init = "myInit",
///     max_samples = 50,
///     max_steps = 100
/// )]
/// fn test_custom() -> impl Driver {
///     MyDriver::default()
/// }
/// ```
pub use quint_connect_macros::quint_run;

/// Generates a test that runs traces from a specific Quint test.
///
/// This attribute macro transforms a function into a Rust test that generates traces using
/// `quint test`. The function should return a [`Driver`] implementation that will be used
/// to replay the generated traces.
///
/// # Attributes
///
/// - **`spec`** (required): Path to the Quint specification file
/// - **`test`** (required): Name of the Quint test to run
/// - **`main`**: Name of the main module containing the test (defaults to Quint's default)
/// - **`max_samples`**: Maximum number of test runs (defaults to 100)
/// - **`seed`**: Random seed for reproducibility (defaults to random)
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use quint_connect::*;
/// # #[derive(Default)]
/// # struct MyDriver;
/// # impl Driver for MyDriver {
/// #     type State = ();
/// #     fn step(&mut self, _step: &Step) -> Result { Ok(()) }
/// # }
///
/// #[quint_test(spec = "spec.qnt", test = "myTest")]
/// fn test_my_test() -> impl Driver {
///     MyDriver::default()
/// }
/// ```
///
/// With custom configuration:
///
/// ```rust
/// use quint_connect::*;
/// # #[derive(Default)]
/// # struct MyDriver;
/// # impl Driver for MyDriver {
/// #     type State = ();
/// #     fn step(&mut self, _step: &Step) -> Result { Ok(()) }
/// # }
///
/// #[quint_test(
///     spec = "spec.qnt",
///     test = "happyPathTest",
///     main = "tests",
///     max_samples = 10
/// )]
/// fn test_happy_path() -> impl Driver {
///     MyDriver::default()
/// }
/// ```
pub use quint_connect_macros::quint_test;

/// Pattern-matches on action names and extracts nondeterministic picks from a [`Step`].
///
/// This macro simplifies the implementation of [`Driver::step`] by providing a convenient
/// syntax for matching action names and extracting their parameters.
///
/// # Syntax
///
/// ```text
/// switch!(step {
///     action_name,                            // No parameters, calls self.action_name()
///     action_name(param),                     // Required parameter
///     action_name(param: Type),               // Required parameter with explicit type
///     action_name(param?),                    // Optional parameter (Option<_>)
///     action_name(param: Type?),              // Optional parameter with explicit type
///     action_name(p1, p2: Type, p3?) => expr, // Custom handler expression
///     _ => expr,                              // Catch-all for unmatched actions
/// })
/// ```
///
/// # Parameter Extraction
///
/// Parameters are extracted by name and deserialized using [`serde::Deserialize`]. The
/// parameter name in the pattern must match the variable name in the Quint specification.
///
/// - **Required parameters** (e.g., `param` or `param: Type`): Will fail if the parameter
///   is missing.
/// - **Optional parameters** (e.g., `param?` or `param: Type?`): Produces an `Option<Type>`,
///   with `None` if the parameter is missing.
///
/// # Handler Expressions
///
/// Each case can have a handler expression after `=>`:
///
/// - If no handler is provided, the macro generates a call to `self.action_name(params...)`.
/// - With a handler, you can provide custom logic, including blocks of code.
///
/// # Catch-All Pattern
///
/// The `_` pattern matches any action name that doesn't match previous cases. It requires
/// a handler expression. If no catch-all is provided, the macro generates an error for
/// unmatched actions.
///
/// # Examples
///
/// Basic usage with implicit handlers:
///
/// ```rust
/// use quint_connect::*;
/// # struct MyDriver;
/// # impl MyDriver {
/// #     fn init(&mut self) {}
/// #     fn increment(&mut self, amount: i64) {}
/// # }
///
/// impl Driver for MyDriver {
///     type State = ();
///
///     fn step(&mut self, step: &Step) -> Result {
///         switch!(step {
///             init,                    // Calls self.init()
///             increment(amount),       // Calls self.increment(amount)
///         })
///     }
/// }
/// ```
///
/// Using optional parameters:
///
/// ```rust
/// use quint_connect::*;
/// # struct MyDriver;
/// # impl MyDriver {
/// #     fn set_value(&mut self, x: i64, y: Option<i64>) {}
/// # }
///
/// impl Driver for MyDriver {
///     type State = ();
///
///     fn step(&mut self, step: &Step) -> Result {
///         switch!(step {
///             setValue(x, y?) => self.set_value(x, y),
///         })
///     }
/// }
/// ```
///
/// Using custom handlers:
///
/// ```rust
/// use quint_connect::*;
/// # struct MyDriver { value: i64 }
///
/// impl Driver for MyDriver {
///     type State = ();
///
///     fn step(&mut self, step: &Step) -> Result {
///         switch!(step {
///             init => {
///                 self.value = 0;
///             },
///             add(amount: i64) => {
///                 self.value += amount;
///             },
///             _ => {
///                 // Handle unknown actions
///             }
///         })
///     }
/// }
/// ```
pub use quint_connect_macros::switch;
