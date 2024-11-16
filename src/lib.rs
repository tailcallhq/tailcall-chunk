//! A Rust implementation of a persistent data structure that provides O(1) append and concatenation operations
//! through structural sharing.
//!
//! # Overview
//! `Chunk` is a persistent data structure that offers:
//! - **O(1) Append Operations**: Add elements to your chunk in constant time
//! - **O(1) Concatenation**: Combine two chunks efficiently
//! - **Immutable/Persistent**: All operations create new versions while preserving the original
//! - **Memory Efficient**: Uses structural sharing via reference counting
//! - **Safe Rust**: Implemented using 100% safe Rust
//!
//! # Theoretical Background
//!
//! This implementation is inspired by the concepts presented in Hinze and Paterson's work on
//! [Finger Trees](https://en.wikipedia.org/wiki/Finger_tree), though simplified for our specific use case.
//! While our implementation differs in structure, it shares similar performance goals and theoretical foundations.
//!
//! ## Relationship to Finger Trees
//!
//! Finger Trees are a functional data structure that supports:
//! - Access to both ends in amortized constant time
//! - Concatenation in logarithmic time
//! - Persistence through structural sharing
//!
//! Our `Chunk` implementation achieves similar goals through a simplified approach:
//! - We use `Append` nodes for constant-time additions
//! - The `Concat` variant enables efficient concatenation
//! - `Rc` (Reference Counting) provides persistence and structural sharing
//!
//! # Example Usage
//! ```rust
//! use tailcall_chunk::Chunk;
//!
//! // Create a new chunk and append some elements
//! let chunk1 = Chunk::default()
//!     .append(1)
//!     .append(2);
//!
//! // Create another chunk
//! let chunk2 = Chunk::default()
//!     .append(3)
//!     .append(4);
//!
//! // Concatenate chunks in O(1) time
//! let combined = chunk1.concat(chunk2);
//!
//! // Convert to vector when needed
//! assert_eq!(combined.as_vec(), vec![1, 2, 3, 4]);
//! ```
//!
//! # Performance Characteristics
//!
//! ## Time Complexity Analysis
//!
//! | Operation             | Worst Case | Amortized    | Space        |
//! | --------------------- | ---------- | ------------ | ------------ |
//! | `new()`               | O(1)       | O(1)         | O(1)         |
//! | `append()`            | O(1)       | O(1)         | O(1)         |
//! | `concat()`            | O(1)       | O(1)         | O(1)         |
//! | `transform()`         | O(1)       | O(1)         | O(1)         |
//! | `transform_flatten()` | O(1)       | O(1)         | O(1)         |
//! | `as_vec()`            | O(n)       | O(n)         | O(n)         |
//! | `clone()`             | O(1)       | O(1)         | O(1)         |
//!
//! ## Amortized Analysis Details
//!
//! ### Append Operation
//! The `append` operation is O(1) amortized because:
//! - The actual append is always O(1) as it only creates a new `Append` node
//! - No rebalancing is required
//! - Memory allocation is constant time
//!
//! ### Concat Operation
//! The `concat` operation achieves O(1) amortized time through:
//! - Lazy evaluation: immediate concatenation is O(1)
//! - The actual work is deferred until `as_vec()` is called
//! - No immediate copying or restructuring of data
//!
//! ### Transform Operations
//! Both `transform` and `transform_flatten` are O(1) amortized because:
//! - They create a new node with a transformation function
//! - Actual transformation is deferred until materialization
//! - No immediate computation is performed on elements
//!
//! ### as_vec Operation
//! The `as_vec` operation is O(n) because:
//! - It must process all elements to create the final vector
//! - For a chunk with n elements:
//!   - Basic traversal: O(n)
//!   - Applying deferred transformations: O(n)
//!   - Memory allocation and copying: O(n)
//!
//! ### Memory Usage Patterns
//!
//! The space complexity has interesting properties:
//! - Immediate space usage for operations is O(1)
//! - Deferred space cost accumulates with operations
//! - Final materialization requires O(n) space
//! - Structural sharing reduces memory overhead for clones and versions
//!
//! ```rust
//! use tailcall_chunk::Chunk;
//!
//! // Each operation has O(1) immediate cost
//! let chunk = Chunk::default()
//!     .append(1)    // O(1) time and space
//!     .append(2)    // O(1) time and space
//!     .transform(|x| x + 1);  // O(1) time and space
//!
//! // O(n) cost is paid here
//! let vec = chunk.as_vec();
//! ```
//!
//! # Implementation Details
//!
//! The `Chunk<A>` type is implemented as an enum with four variants:
//! - `Empty`: Represents an empty chunk
//! - `Append`: Represents a single element appended to another chunk
//! - `Concat`: Represents the concatenation of two chunks
//! - `TransformFlatten`: Represents a lazy transformation and flattening of elements
//!
//! The data structure achieves its performance characteristics through:
//! - Structural sharing using `Rc`
//! - Lazy evaluation of concatenation and transformations
//! - Immutable operations that preserve previous versions
//!
//! # Memory Efficiency
//!
//! The `Chunk` type uses structural sharing through reference counting (`Rc`), which means:
//! - Appending or concatenating chunks doesn't copy the existing elements
//! - Memory is automatically freed when no references remain
//! - Multiple versions of the data structure can coexist efficiently
//!
//! ```rust
//! use tailcall_chunk::Chunk;
//!
//! let original = Chunk::default().append(1).append(2);
//! let version1 = original.clone().append(3);  // Efficient cloning
//! let version2 = original.clone().append(4);  // Both versions share data
//! ```
//!
//! # References
//!
//! 1. Ralf Hinze and Ross Paterson. "Finger Trees: A Simple General-purpose Data Structure",
//!    Journal of Functional Programming 16(2):197-217, 2006.
//! 2. Chris Okasaki. "Purely Functional Data Structures", Cambridge University Press, 1998.

mod chunk;
pub use chunk::*;
