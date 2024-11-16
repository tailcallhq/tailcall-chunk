//! A Rust implementation of a persistent data structure for efficient append and concatenation operations.
//!
//! This crate provides the [`Chunk`] type, which implements a persistent data structure
//! that allows O(1) append and concatenation operations through structural sharing.
//!
//! # Features
//! - O(1) append operations
//! - O(1) concatenation operations
//! - Immutable/persistent data structure
//! - Memory efficient through structural sharing
//!
//! # Example
//! ```
//! use your_crate_name::Chunk;
//!
//! let chunk1 = Chunk::new().append(1).append(2);
//! let chunk2 = Chunk::new().append(3).append(4);
//! let combined = chunk1.concat(chunk2);
//!
//! assert_eq!(combined.as_vec(), vec![&1, &2, &3, &4]);
//! ```

use std::rc::Rc;

/// A persistent data structure that provides efficient append and concatenation operations.
///
/// # Overview
/// `Chunk<A>` is an immutable data structure that allows O(1) complexity for append and
/// concatenation operations through structural sharing. It uses [`Rc`] (Reference Counting)
/// for efficient memory management.
///
/// # Performance
/// - Append operation: O(1)
/// - Concatenation operation: O(1)
/// - Converting to Vec: O(n)
///
/// # Implementation Details
/// The data structure is implemented as an enum with three variants:
/// - `Empty`: Represents an empty chunk
/// - `Append`: Represents a single element appended to another chunk
/// - `Concat`: Represents the concatenation of two chunks
///
/// # Examples
/// ```
/// use your_crate_name::Chunk;
///
/// let mut chunk = Chunk::new();
/// chunk = chunk.append(1);
/// chunk = chunk.append(2);
///
/// let other_chunk = Chunk::new().append(3).append(4);
/// let combined = chunk.concat(other_chunk);
///
/// assert_eq!(combined.as_vec(), vec![&1, &2, &3, &4]);
/// ```
///
/// # References
/// - [Persistent Data Structures](https://en.wikipedia.org/wiki/Persistent_data_structure)
/// - [Structural Sharing](https://hypirion.com/musings/understanding-persistent-vector-pt-1)
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Chunk<A> {
    /// Represents an empty chunk
    Empty,
    /// Represents a single element `A` appended to another chunk
    Append(A, Rc<Chunk<A>>),
    /// Represents the concatenation of two chunks
    Concat(Rc<Chunk<A>>, Rc<Chunk<A>>),
}

impl<A> Default for Chunk<A> {
    /// Creates a new empty chunk.
    ///
    /// This is equivalent to calling [`Chunk::new()`].
    fn default() -> Self {
        Self::new()
    }
}

impl<A> Chunk<A> {
    /// Creates a new empty chunk.
    ///
    /// # Examples
    /// ```
    /// use your_crate_name::Chunk;
    ///
    /// let chunk: Chunk<i32> = Chunk::new();
    /// assert!(chunk.is_null());
    /// ```
    pub fn new() -> Self {
        Self::Empty
    }

    /// Returns `true` if the chunk is empty.
    ///
    /// # Examples
    /// ```
    /// use your_crate_name::Chunk;
    ///
    /// let chunk: Chunk<i32> = Chunk::new();
    /// assert!(chunk.is_null());
    ///
    /// let non_empty = chunk.append(42);
    /// assert!(!non_empty.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self, Chunk::Empty)
    }

    /// Appends a new element to the chunk.
    ///
    /// This operation has O(1) complexity as it creates a new `Append` variant
    /// that references the existing chunk through an [`Rc`].
    ///
    /// # Examples
    /// ```
    /// use your_crate_name::Chunk;
    ///
    /// let chunk = Chunk::new().append(1).append(2);
    /// assert_eq!(chunk.as_vec(), vec![&1, &2]);
    /// ```
    pub fn append(self, a: A) -> Self {
        Chunk::Append(a, Rc::new(self))
    }

    /// Concatenates this chunk with another chunk.
    ///
    /// This operation has O(1) complexity as it creates a new `Concat` variant
    /// that references both chunks through [`Rc`]s.
    ///
    /// # Performance Optimization
    /// If either chunk is empty, returns the other chunk instead of creating
    /// a new `Concat` variant.
    ///
    /// # Examples
    /// ```
    /// use your_crate_name::Chunk;
    ///
    /// let chunk1 = Chunk::new().append(1).append(2);
    /// let chunk2 = Chunk::new().append(3).append(4);
    /// let combined = chunk1.concat(chunk2);
    /// assert_eq!(combined.as_vec(), vec![&1, &2, &3, &4]);
    /// ```
    pub fn concat(self, other: Chunk<A>) -> Self {
        if self.is_null() {
            return other;
        }
        if other.is_null() {
            return self;
        }
        Self::Concat(Rc::new(self), Rc::new(other))
    }

    /// Converts the chunk into a vector of references to its elements.
    ///
    /// This operation has O(n) complexity where n is the number of elements
    /// in the chunk.
    ///
    /// # Examples
    /// ```
    /// use your_crate_name::Chunk;
    ///
    /// let chunk = Chunk::new().append(1).append(2).append(3);
    /// assert_eq!(chunk.as_vec(), vec![&1, &2, &3]);
    /// ```
    pub fn as_vec(&self) -> Vec<&A> {
        let mut vec = Vec::new();
        self.as_vec_mut(&mut vec);
        vec
    }

    /// Helper method that populates a vector with references to the chunk's elements.
    ///
    /// This method is used internally by [`as_vec`](Chunk::as_vec) to avoid
    /// allocating multiple vectors during the traversal.
    ///
    /// # Arguments
    /// * `buf` - A mutable reference to a vector that will be populated with
    ///           references to the chunk's elements
    pub fn as_vec_mut<'a>(&'a self, buf: &mut Vec<&'a A>) {
        match self {
            Chunk::Empty => {}
            Chunk::Append(a, rest) => {
                rest.as_vec_mut(buf);
                buf.push(a);
            }
            Chunk::Concat(a, b) => {
                b.as_vec_mut(buf);
                a.as_vec_mut(buf);
            }
        }
    }
}
