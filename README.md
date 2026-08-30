# Rust pipe

```rs
fn add(a: usize, b: usize) -> usize { a + b }
fn double(v: usize) -> usize { v * 2 }

let num = 4;
let result = pipe! {
    num
        |> add(2, _)
        |> double(_)
};
assert_eq!(result, 12);
```
