beginner_tools
====

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
beginner_tools = "1.1.1"
```

Next, add this to your crate root:

```rust
extern crate beginner_tools;
use beginner_tools::*;
```

And you are free to use any function the crate offers!

At the moment there are two functions:
- get_stdin
- input

The latter is basically a wrapper over the first that provides more functionality.

For further information refer to the [documentation](https://docs.rs/beginner_tools/1.1.1/beginner_tools/)