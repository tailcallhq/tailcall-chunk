//! A Rust implementation of a persistent data structure for efficient prepend and concatenation operations.
//!
//! This crate provides the [`Chunk`] type, which implements a persistent data structure
//! that allows O(1) prepend and concatenation operations through structural sharing.
//!
//! # Features
//! - O(1) prepend operations
//! - O(1) concatenation operations
//! - Immutable/persistent data structure
//! - Memory efficient through structural sharing
//!
//! # Example
//! ```
//! use tailcall_chunk::Chunk;
//!
//! let chunk1 = Chunk::default().prepend(1).prepend(2);
//! let chunk2 = Chunk::default().prepend(3).prepend(4);
//! let combined = chunk1.concat(chunk2);
//!
//! assert_eq!(combined.as_vec(), vec![2, 1, 4, 3]);
//! ```

use std::rc::Rc;

/// A persistent data structure that provides efficient prepend and concatenation operations.
///
/// # Overview
/// `Chunk<A>` is an immutable data structure that allows O(1) complexity for prepend and
/// concatenation operations through structural sharing. It uses [`Rc`] (Reference Counting)
/// for efficient memory management.
///
/// # Performance
/// - Prepend operation: O(1)
/// - Concatenation operation: O(1)
/// - Converting to Vec: O(n)
///
/// # Implementation Details
/// The data structure is implemented as an enum with three variants:
/// - `Empty`: Represents an empty chunk
/// - `Prepend`: Represents a single element prepended to another chunk
/// - `Concat`: Represents the concatenation of two chunks
///
/// # Examples
/// ```
/// use tailcall_chunk::Chunk;
///
/// let mut chunk = Chunk::default();
/// chunk = chunk.prepend(1);
/// chunk = chunk.prepend(2);
///
/// let other_chunk = Chunk::default().prepend(3).prepend(4);
/// let combined = chunk.concat(other_chunk);
///
/// assert_eq!(combined.as_vec(), vec![2,1, 4, 3]);
/// ```
///
/// # References
/// - [Persistent Data Structures](https://en.wikipedia.org/wiki/Persistent_data_structure)
/// - [Structural Sharing](https://hypirion.com/musings/understanding-persistent-vector-pt-1)
#[derive(Clone)]
pub enum Chunk<A> {
    /// Represents an empty chunk
    Empty,
    /// Represents a single element `A` prepended to another chunk
    Prepend(A, Rc<Chunk<A>>),

    /// Represents the concatenation of two chunks
    Concat(Rc<Chunk<A>>, Rc<Chunk<A>>),

    /// Represents a lazy mapping of elements
    Map(Rc<Chunk<A>>, Rc<dyn Fn(A) -> A>),
}

impl<A> Default for Chunk<A> {
    /// Creates a new empty chunk.
    ///
    /// This is equivalent to using [`Chunk::Empty`].
    fn default() -> Self {
        Chunk::Empty
    }
}

impl<A> Chunk<A> {
    /// Creates a new empty chunk.
    ///
    /// # Examples
    /// ```
    /// use tailcall_chunk::Chunk;
    ///
    /// let chunk: Chunk<i32> = Chunk::new(100);
    /// assert!(!chunk.is_null());
    /// ```
    pub fn new(a: A) -> Self {
        Chunk::default().prepend(a)
    }

    /// Returns `true` if the chunk is empty.
    ///
    /// # Examples
    /// ```
    /// use tailcall_chunk::Chunk;
    ///
    /// let chunk: Chunk<i32> = Chunk::default();
    /// assert!(chunk.is_null());
    ///
    /// let non_empty = chunk.prepend(42);
    /// assert!(!non_empty.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self, Chunk::Empty)
    }

    /// Prepend a new element to the chunk.
    ///
    /// This operation has O(1) complexity as it creates a new `Prepend` variant
    /// that references the existing chunk through an [`Rc`].
    ///
    /// # Examples
    /// ```
    /// use tailcall_chunk::Chunk;
    ///
    /// let chunk = Chunk::default().prepend(1).prepend(2);
    /// assert_eq!(chunk.as_vec(), vec![2, 1]);
    /// ```
    pub fn prepend(self, a: A) -> Self {
        Chunk::Prepend(a, Rc::new(self))
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
    /// use tailcall_chunk::Chunk;
    ///
    /// let chunk1 = Chunk::default().prepend(1).prepend(2);
    /// let chunk2 = Chunk::default().prepend(3).prepend(4);
    /// let combined = chunk1.concat(chunk2);
    /// assert_eq!(combined.as_vec(), vec![2,1, 4,3]);
    /// ```
    pub fn concat(self, other: Chunk<A>) -> Chunk<A> {
        if self.is_null() {
            return other;
        }
        if other.is_null() {
            return self;
        }
        Chunk::Concat(Rc::new(self), Rc::new(other))
    }

    /// Transforms each element in the chunk using the provided function.
    ///
    /// This method creates a lazy representation of the transformation without actually
    /// performing it. The transformation is only executed when [`as_vec`](Chunk::as_vec)
    /// or [`as_vec_mut`](Chunk::as_vec_mut) is called.
    ///
    /// # Performance
    /// - Creating the transformation: O(1)
    /// - Executing the transformation (during [`as_vec`](Chunk::as_vec)): O(n)
    ///
    /// # Arguments
    /// * `f` - A function that takes a reference to an element of type `A` and returns
    ///         a new element of type `A`
    ///
    /// # Examples
    /// ```
    /// use tailcall_chunk::Chunk;
    ///
    /// let chunk = Chunk::default().prepend(1).prepend(2).prepend(3);
    /// // This operation is O(1) and doesn't actually transform the elements
    /// let doubled = chunk.transform(|x| x * 2);
    /// // The transformation happens here, when we call as_vec()
    /// assert_eq!(doubled.as_vec(), vec![6, 4, 2]);
    /// ```
    pub fn transform(self, f: impl Fn(A) -> A + 'static) -> Self {
        Chunk::Map(Rc::new(self), Rc::new(f))
    }

    /// Converts the chunk into a vector of references to its elements.
    ///
    /// This operation has O(n) complexity where n is the number of elements
    /// in the chunk.
    ///
    /// # Examples
    /// ```
    /// use tailcall_chunk::Chunk;
    ///
    /// let chunk = Chunk::default().prepend(1).prepend(2).prepend(3);
    /// assert_eq!(chunk.as_vec(), vec![3, 2, 1]);
    /// ```
    pub fn as_vec(&self) -> Vec<A>
    where
        A: Clone,
    {
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
    pub fn as_vec_mut(&self, buf: &mut Vec<A>)
    where
        A: Clone,
    {
        match self {
            Chunk::Empty => {}
            Chunk::Prepend(a, rest) => {
                buf.push(a.clone());
                rest.as_vec_mut(buf);
            }
            Chunk::Concat(a, b) => {
                a.as_vec_mut(buf);
                b.as_vec_mut(buf);
            }
            Chunk::Map(a, f) => {
                let mut tmp = Vec::new();
                a.as_vec_mut(&mut tmp);
                for elem in tmp {
                    buf.push(f(elem));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let chunk: Chunk<i32> = Chunk::default();
        assert!(chunk.is_null());
    }

    #[test]
    fn test_default() {
        let chunk: Chunk<i32> = Chunk::default();
        assert!(chunk.is_null());
    }

    #[test]
    fn test_is_null() {
        let empty: Chunk<i32> = Chunk::default();
        assert!(empty.is_null());

        let non_empty = empty.prepend(1);
        assert!(!non_empty.is_null());
    }

    #[test]
    fn test_prepend() {
        let chunk = Chunk::default().prepend(1).prepend(2).prepend(3);
        assert_eq!(chunk.as_vec(), vec![3, 2, 1]);

        // Test that original chunk remains unchanged (persistence)
        let chunk1 = Chunk::default().prepend(1);
        let chunk2 = chunk1.clone().prepend(2);
        assert_eq!(chunk1.as_vec(), vec![1]);
        assert_eq!(chunk2.as_vec(), vec![2, 1]);
    }

    #[test]
    fn test_concat() {
        let chunk1 = Chunk::default().prepend(2).prepend(1);
        let chunk2 = Chunk::default().prepend(4).prepend(3);
        let combined = chunk1.clone().concat(chunk2.clone());

        assert_eq!(combined.as_vec(), vec![1, 2, 3, 4]);

        // Test concatenation with empty chunks
        let empty = Chunk::default();
        assert_eq!(
            empty.clone().concat(chunk1.clone()).as_vec(),
            chunk1.as_vec()
        );
        assert_eq!(
            chunk1.clone().concat(empty.clone()).as_vec(),
            chunk1.as_vec()
        );
        assert_eq!(empty.clone().concat(empty).as_vec(), Vec::<i32>::new());
    }

    #[test]
    fn test_as_vec() {
        // Test empty chunk
        let empty: Chunk<i32> = Chunk::default();
        assert_eq!(empty.as_vec(), Vec::<i32>::new());

        // Test single element
        let single = Chunk::default().prepend(42);
        assert_eq!(single.as_vec(), vec![42]);

        // Test multiple elements
        let multiple = Chunk::default().prepend(3).prepend(2).prepend(1);
        assert_eq!(multiple.as_vec(), vec![1, 2, 3]);

        // Test complex structure with concatenation
        let chunk1 = Chunk::default().prepend(2).prepend(1);
        let chunk2 = Chunk::default().prepend(4).prepend(3);
        let complex = chunk1.concat(chunk2);
        assert_eq!(complex.as_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_structural_sharing() {
        let chunk1 = Chunk::default().prepend(1).prepend(2);
        let chunk2 = chunk1.clone().prepend(3);
        let chunk3 = chunk1.clone().prepend(4);

        // Verify that modifications create new structures while preserving the original
        assert_eq!(chunk1.as_vec(), vec![2, 1]);
        assert_eq!(chunk2.as_vec(), vec![3, 2, 1]);
        assert_eq!(chunk3.as_vec(), vec![4, 2, 1]);
    }

    #[test]
    fn test_with_different_types() {
        // Test with strings
        let string_chunk = Chunk::default()
            .prepend(String::from("hello"))
            .prepend(String::from("world"));
        assert_eq!(string_chunk.as_vec().len(), 2);

        // Test with floating point numbers
        let float_chunk = Chunk::default().prepend(3.14).prepend(2.718);
        assert_eq!(float_chunk.as_vec(), vec![2.718, 3.14]);

        // Test with boolean values
        let bool_chunk = Chunk::default().prepend(true).prepend(false).prepend(true);
        assert_eq!(bool_chunk.as_vec(), vec![true, false, true]);
    }
}
