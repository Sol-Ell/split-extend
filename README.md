# SplitExtend

`SplitExtend` is a Rust extension trait for `Vec<T>` that allows splitting a vector into two parts:
- **Left part**: Provides mutable access to elements before a specified index.
- **Right part**: Includes the indexed element and allows dynamic extension.

This is an **experimental personal project**, and its API may change over time. Use it at your own risk.

## Features
- Efficiently split a `Vec<T>` at an index.
- Mutably access elements on the left side.
- Mutably access and extend elements on the right side.

## Usage

Add `split_extend` to your `Cargo.toml`:

```toml
[dependencies]
split_extend = "0.1"
```

Example usage:

```rust
use split_extend::SplitExtend;

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    let (left, mut right) = vec.split_extend(2);

    left[0] = 10; // Modify left side

    right.push(6); // Extend right side

    println!("{:?}", vec); // Output: [10, 2, 3, 4, 5, 6]
}
```

## Experimental Status

This library is a personal experiment in Rust API design. While it works, it **has not been extensively tested** and **may undergo breaking changes**. 

## License

This project is licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).