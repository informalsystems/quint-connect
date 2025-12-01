# Quint Connect

A model-based testing framework for Rust that connects
[Quint](https://quint-lang.org/) specifications with Rust applications.

## Overview

Quint Connect enables rigorous testing of Rust code against formal
specifications written in Quint. By automatically generating traces from your
Quint specifications and replaying them against your Rust implementation, you
can verify that your code behaves exactly as specified.

## Key Features

- **Automatic Trace Generation**: Generates test cases from Quint specifications
- **State Validation**: Automatically verifies that implementation state matches
  specification state
- **Model-Based Testing**: Leverage Quint's powerful specification language for
  comprehensive testing
- **Declarative Test Definition**: Simple macros for defining model-based tests
- **Detailed Error Reporting**: Clear diffs when implementation diverges from
  specification

## Requirements

- Rust 1.70 or later
- [Quint](https://github.com/informalsystems/quint) installed and available in
  PATH

## Quick Start

### 1. Define Your Specification

Write your system specification in Quint (e.g., `spec.qnt`).

### 2. Implement the Test Driver

Create a test driver that connects your Rust implementation to the test
framework:

```rust
use quint_connect::*;
use serde::Deserialize;

#[derive(Eq, PartialEq, Deserialize, Debug)]
struct MyState {
    // Fields matching your Quint variables
}

impl State<MyDriver> for MyState {
    fn from_driver(driver: &MyDriver) -> Result<Self> {
        // Convert driver's state to specification state
        todo!()
    }
}

#[derive(Default)]
struct MyDriver {
    // Your implementation state
}

impl Driver for MyDriver {
    type State = MyState;

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            init => self.init(),
            MyAction(param1, param2?) => {
                self.my_action(param1, param2);
            }
        })
    }
}

impl MyDriver {
    fn init(&mut self) {
        // Initialize the implementation state
    }
    
    fn my_action(&mut self, param1: String, param: Option<usize>) {
        // Call equivalent code in your implementation
    }
}
```

### 3. Create Tests

Use the provided macros to create model-based tests:

```rust
use quint_connect::*;

// Run a single test as specified by a Quint run 
#[quint_test(spec = "spec.qnt", test = "myTest")]
fn my_test() -> impl Driver {
    MyDriver::default()
}

// Run multiple traces in simulation mode
#[quint_run(spec = "spec.qnt")]
fn simulation() -> impl Driver {
    MyDriver::default()
}
```

### 4. Run Test

Run tests with:

```bash
cargo test -- --nocapture
```

Increase verbosity to see step's actions and nondeterministic choices available:

```bash
QUINT_VERBOSE=1 cargo test -- --nocapture
```

## Tips and Tricks

### Enums

Quint sum types are serialized by Quint as records with the `tag` and `value`
fields representing the type variant and its arguments, respectively. For
example:

```quint
type SumType = 
  | Variant1
  | Variant2
  
// Serializes as:
// { tag: "Variant1", value: [] }
```

The previous Quint sum type can be deserialized in Rust with:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "tag")]
enum SumType {
  Variant1,
  Variant2
}
```

Sum types with associated values **require** specifying serde's `content`
attribute. For example:

```quint
type SumType = 
  | Variant1(int)
  | Variant2(str)
```

The sum type above **must** be deserialized as:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "tag", content = "value")] // <- note the `content` attribute
enum SumType {
  Variant1(usize),
  Variant2(String)
}
```


### Optional Fields

Optional types in Quint are defined as a sum type: 

```quint
type Option[a] = Some(a) | None
```

The [`itf`](itf) crate has utilities to help deserializing optional fields:

```rust
use serde::Deserialize;
use itf::de::{self, As};

#[derive(Deserialize)]
struct MyState {
    #[serde(with = "As::<de::Option::<_>>")]
    pub optional_field: Option<String>
}
```

## Configuration

### Driver Configuration

In some cases the state that you want to check is nested within some global
variable. To narrow down the verifiable state, override the Driver `config`
method with the path to the variable that holds the state to check:

```rust
use quint_connect::*;

fn config() -> Config {
    Config {
        state: &["global_var", "nested_record", "my_state"],
        ..Config::default()
    }
}
```

Similarly, in some scenarios the user may choose to track nondeterminism
manually other than using Quint's builtin variables. In those cases, track
nondeterminism as a sum type where each variant holds a record containing
nondeterministic choices, then specify its path in the driver's configuration:

```rust
use quint_connect::*;

fn config() -> Config {
    Config {
        nondet: &["global_var", "nondet_choices"],
        ..Config::default()
    }
}
```

These driver configurations are most commonly used when checking traces from
specifications using the [Choreo](https://github.com/informalsystems/choreo/)
library. See the `two_phase_commit` example in the [examples](#examples) folder
for details on how its implementation is checked with Quint connect.

### Verbosity Control

Set the `QUINT_VERBOSE` environment variable to control output verbosity:

- `QUINT_VERBOSE=0`: Minimal output
- `QUINT_VERBOSE=1`: Show trace and step information
- `QUINT_VERBOSE=2`: Show detailed state and step derivation

### Reproducible Tests

Failed tests display the random seed used:

```text
Reproduce this error with `QUINT_SEED=42`
```

Set the seed to reproduce exact test traces:

```bash
QUINT_SEED=42 cargo test
```

## Examples

See the `connect/examples/` directory for complete examples:

- **tictactoe**: A simple tic-tac-toe game implementation
- **two_phase_commit**: A simplified version of the two-phase commit protocol
  using [Choreo](https://github.com/informalsystems/choreo/)

## License

TBD

## Contributing

TBD
