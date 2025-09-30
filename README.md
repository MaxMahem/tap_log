# tap_log

A lightweight Rust utility trait for inline logging of values in pipelines, using [`tracing`](https://crates.io/crates/tracing).

## Overview

The `TapLog` trait adds a single method, `.tap_log()`, which logs any value that implements [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) at a given [`tracing::Level`](https://docs.rs/tracing/latest/tracing/struct.Level.html), with an optional context message, and then returns the value unchanged.

This makes it easy to inspect values inside method chains or pipelines without breaking flow, ala [Tap](https://docs.rs/tap/latest/tap/tap/trait.Tap.html).

---

## Example

```rust
use tracing::Level;
use tap_log::TapLog;

tracing_subscriber::fmt::init();

let number = 42
    .tap_log(Level::INFO, "my_val") // logs "INFO my_val: 42"
    .tap_log(Level::DEBUG, "debugging") // logs "DEBUG debugging: 42"
    .tap_log(Level::INFO, ""); // logs "INFO 42"

assert_eq!(number, 42);
```
