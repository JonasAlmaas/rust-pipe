# Rust Pipe

This macro provides a more readable way to chain function calls in Rust. Nested calls
become hard to follow as a chain grows, while a pipeline keeps each step in execution
order.

```rust
fn add(a: usize, b: usize) -> usize { a + b }
fn double(value: usize) -> usize { value * 2 }

let result = double(add(2, 4));

let result = pipe! {
    4
        |> add(2, _)
        |> double(_)
};

assert_eq!(result, 12);
```

The `_` placeholder marks where the value from the previous step goes.
