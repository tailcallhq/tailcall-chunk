# Chunk

[![Crates.io Version](https://img.shields.io/crates/v/tailcall-chunk?style=flat-square)](https://crates.io/crates/tailcall-chunk)
[![Documentation](https://img.shields.io/docsrs/tailcall-chunk?style=flat-square)](https://docs.rs/tailcall-chunk)
[![Build Status](https://img.shields.io/github/actions/workflow/status/tailcallhq/tailcall-chunk/ci.yml?style=flat-square)](https://github.com/tailcallhq/tailcall-chunk/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](LICENSE)

A Rust implementation of a persistent data structure that provides O(1) append and concatenation operations through structural sharing.

## Features

- **O(1) Append Operations**: Add elements to your chunk in constant time
- **O(1) Concatenation**: Combine two chunks efficiently
- **Immutable/Persistent**: All operations create new versions while preserving the original
- **Memory Efficient**: Uses structural sharing via reference counting
- **Safe Rust**: Implemented using 100% safe Rust

## Theoretical Background

This implementation is inspired by the concepts presented in Hinze and Paterson's work on Finger Trees[^1], though simplified for our specific use case. While our implementation differs in structure, it shares similar performance goals and theoretical foundations.

### Relationship to Finger Trees

Finger Trees are a functional data structure that supports:

- Access to both ends in amortized constant time
- Concatenation in logarithmic time
- Persistence through structural sharing

Our `Chunk` implementation achieves similar goals through a simplified approach:

- We use `Append` nodes for constant-time additions
- The `Concat` variant enables efficient concatenation
- `Rc` (Reference Counting) provides persistence and structural sharing

Like Finger Trees, our structure can be viewed as an extension of Okasaki's implicit deques[^2], but optimized for our specific use cases. While Finger Trees offer a more general-purpose solution with additional capabilities, our implementation focuses on providing:

- Simpler implementation
- More straightforward mental model
- Specialized performance characteristics for append/concat operations

### Performance Trade-offs

While Finger Trees achieve logarithmic time for concatenation, our implementation optimizes for constant-time operations through lazy evaluation. This means:

- Append and concatenation are always O(1)
- The cost is deferred to when we need to materialize the sequence (via `as_vec()`)
- Memory usage grows with the number of operations until materialization

This trade-off is particularly beneficial in scenarios where:

- Multiple transformations are chained
- Not all elements need to be materialized
- Structural sharing can be leveraged across operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tailcall-chunk = "0.1.0"
```

## Quick Start

```rust
use chunk::Chunk;

// Create a new chunk and append some elements
let chunk1 = Chunk::default()
    .append(1)
    .append(2);

// Create another chunk
let chunk2 = Chunk::default()
    .append(3)
    .append(4);

// Concatenate chunks in O(1) time
let combined = chunk1.concat(chunk2);

// Convert to vector when needed
assert_eq!(combined.as_vec(), vec![1, 2, 3, 4]);
```

## Detailed Usage

### Working with Custom Types

```rust
use chunk::Chunk;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = Chunk::default()
    .append(Person {
        name: "Alice".to_string(),
        age: 30
    })
    .append(Person {
        name: "Bob".to_string(),
        age: 25
    });

// Access elements
let people_vec = people.as_vec();
assert_eq!(people_vec[0].name, "Alice");
assert_eq!(people_vec[1].name, "Bob");
```

### Memory Efficiency

The `Chunk` type uses structural sharing through reference counting (`Rc`), which means:

- Appending or concatenating chunks doesn't copy the existing elements
- Memory is automatically freed when no references remain
- Multiple versions of the data structure can coexist efficiently

```rust
use chunk::Chunk;

let original = Chunk::default().append(1).append(2);
let version1 = original.clone().append(3);  // Efficient cloning
let version2 = original.clone().append(4);  // Both versions share data
```

## Performance Characteristics

| Operation             | Time Complexity | Space Complexity |
| --------------------- | --------------- | ---------------- |
| `new()`               | O(1)            | O(1)             |
| `append()`            | O(1)            | O(1)             |
| `concat()`            | O(1)            | O(1)             |
| `transform()`         | O(1)            | O(1)             |
| `transform_flatten()` | O(1)            | O(1)             |
| `as_vec()`            | O(n)            | O(n)             |
| `clone()`             | O(1)            | O(1)             |

### Benchmark Comparison

The following table compares the actual performance of Chunk vs Vector operations based on [our benchmarks](benches/operations.rs) (lower is better):

| Operation | Chunk Performance | Vector Performance | Faster                       |
| --------- | ----------------- | ------------------ | ---------------------------- |
| Append    | 1.93 ms           | 553.03 µs          | Vec is `~3.50` times faster    |
| Prepend   | 1.62 ms           | 21.71 ms           | Chunk is `~13.00` times faster |
| Concat    | 77.84 ns          | 513.04 µs          | Chunk is `~6,600` times faster |
| Clone     | 5.82 ns           | 1.18 µs            | Vec is `~200` times faster     |

Note: These benchmarks represent specific test scenarios and actual performance may vary based on usage patterns. Chunk operations are optimized for bulk operations and scenarios where structural sharing provides benefits. View the complete benchmark code and results in our [operations.rs](benches/operations.rs) benchmark file.

## Implementation Details

The `Chunk<A>` type is implemented as an enum with four variants:

- `Empty`: Represents an empty chunk
- `Single`: Represents a chunk with a single element
- `Concat`: Represents the concatenation of two chunks
- `TransformFlatten`: Represents a lazy transformation and flattening of elements

The data structure achieves its performance characteristics through:

- Structural sharing using `Rc`
- Lazy evaluation of concatenation and transformations
- Immutable operations that preserve previous versions

## Contributing

We welcome contributions! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -am 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please make sure to:

- Update documentation
- Add tests for new features
- Follow the existing code style
- Update the README.md if needed

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## References

[^1]: Ralf Hinze and Ross Paterson. "Finger Trees: A Simple General-purpose Data Structure", Journal of Functional Programming 16(2):197-217, 2006.
[^2]: Chris Okasaki. "Purely Functional Data Structures", Cambridge University Press, 1998.
