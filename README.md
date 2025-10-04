
# Stylus-panic

stylus-panic is a panic handler that's available for no_std environments that uses
stylus-interpreter's die function if it's available. It also provides a helpful console
logger macro.

## Using harness_dbg

```rust
harness_dbg!("Hello", "world");
```

Will use the `console` function, if the `stylus-interpreter` feature is enabled, to work
similarly to `dbg!`.

## The panic handler

The panic handler will be registered if the `std` feature is not enabled. It will use the
`die` `stylus-interpreter` function if the `stylus-interpreter` feature is enabled.
