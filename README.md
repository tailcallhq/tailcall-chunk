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

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
chunk = "0.1.0"
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

## Implementation Details

The `Chunk<A>` type is implemented as an enum with four variants:

- `Empty`: Represents an empty chunk
- `Append`: Represents a single element appended to another chunk
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

## References and Further Reading

- [Persistent Data Structures](https://en.wikipedia.org/wiki/Persistent_data_structure)
- [Understanding Persistent Vector - Part 1](https://hypirion.com/musings/understanding-persistent-vector-pt-1)
- [Structural Sharing in Functional Programming](https://hypirion.com/musings/understanding-persistent-vector-pt-1)

## Changelog

### 0.1.0 (Initial Release)

- Basic chunk implementation with O(1) append and concat operations
- Full documentation and examples
- Complete test coverage
