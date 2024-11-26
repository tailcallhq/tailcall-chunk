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
//! use tailcall_chunk::Chunk;
//!
//! let chunk1 = Chunk::default().append(1).append(2);
//! let chunk2 = Chunk::default().append(3).append(4);
//! let combined = chunk1.concat(chunk2);
//!
//! assert_eq!(combined.as_vec(), vec![1, 2, 3, 4]);
//! ```

use std::{cell::RefCell, rc::Rc};

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
/// use tailcall_chunk::Chunk;
///
/// let mut chunk = Chunk::default();
/// chunk = chunk.append(1);
/// chunk = chunk.append(2);
///
/// let other_chunk = Chunk::default().append(3).append(4);
/// let combined = chunk.concat(other_chunk);
///
/// assert_eq!(combined.as_vec(), vec![1, 2, 3, 4]);
/// ```
///
/// # References
/// - [Persistent Data Structures](https://en.wikipedia.org/wiki/Persistent_data_structure)
/// - [Structural Sharing](https://hypirion.com/musings/understanding-persistent-vector-pt-1)
#[derive(Clone)]
pub enum Chunk<A> {
    /// Represents an empty chunk with no elements
    Empty,
    /// Represents a chunk containing exactly one element
    Single(A),
    /// Represents the concatenation of two chunks, enabling O(1) concatenation
    Concat(Rc<Chunk<A>>, Rc<Chunk<A>>),
    /// Represents a collection of elements
    Collect(Rc<RefCell<Vec<A>>>),
    /// Represents a lazy transformation that flattens elements
    TransformFlatten(Rc<Chunk<A>>, Rc<dyn Fn(A) -> Chunk<A>>),
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
    /// Creates a new chunk containing a single element.
    ///
    /// # Arguments
    /// * `a` - The element to store in the chunk
    ///
    /// # Examples
    /// ```
    /// use tailcall_chunk::Chunk;
    ///
    /// let chunk: Chunk<i32> = Chunk::new(100);
    /// assert!(!chunk.is_null());
    /// ```
    pub fn new(a: A) -> Self {
        Chunk::Single(a)
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
    /// let non_empty = chunk.append(42);
    /// assert!(!non_empty.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        match self {
            Chunk::Empty => true,
            Chunk::Collect(vec) => vec.borrow().is_empty(),
            _ => false,
        }
    }

    /// Append a new element to the chunk.
    ///
    /// This operation has O(1) complexity as it creates a new `Append` variant
    /// that references the existing chunk through an [`Rc`].
    ///
    /// # Examples
    /// ```
    /// use tailcall_chunk::Chunk;
    ///
    /// let chunk = Chunk::default().append(1).append(2);
    /// assert_eq!(chunk.as_vec(), vec![1, 2]);
    /// ```
    pub fn append(self, a: A) -> Self {
        self.concat(Chunk::new(a))
    }

    /// Prepend a new element to the beginning of the chunk.
    ///
    /// This operation has O(1) complexity as it creates a new `Concat` variant
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
        if self.is_null() {
            Chunk::new(a)
        } else {
            Chunk::new(a).concat(self)
        }
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
    /// let chunk1 = Chunk::default().append(1).append(2);
    /// let chunk2 = Chunk::default().append(3).append(4);
    /// let combined = chunk1.concat(chunk2);
    /// assert_eq!(combined.as_vec(), vec![1, 2, 3, 4]);
    /// ```
    pub fn concat(self, other: Chunk<A>) -> Chunk<A> {
        match (self, other) {
            // Handle null cases
            (Chunk::Empty, other) => other,
            (this, Chunk::Empty) => this,
            (Chunk::Single(a), Chunk::Single(b)) => {
                Chunk::Collect(Rc::new(RefCell::new(vec![a, b])))
            }
            (Chunk::Collect(vec), Chunk::Single(a)) => {
                if Rc::strong_count(&vec) == 1 {
                    // Only clone if there are no other references
                    vec.borrow_mut().push(a);
                    Chunk::Collect(vec)
                } else {
                    Chunk::Concat(Rc::new(Chunk::Collect(vec)), Rc::new(Chunk::Single(a)))
                }
            }
            // Handle all other cases with Concat
            (this, that) => Chunk::Concat(Rc::new(this), Rc::new(that)),
        }
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
    /// let chunk = Chunk::default().append(1).append(2).append(3);
    /// // This operation is O(1) and doesn't actually transform the elements
    /// let doubled = chunk.transform(|x| x * 2);
    /// // The transformation happens here, when we call as_vec()
    /// assert_eq!(doubled.as_vec(), vec![2, 4, 6]);
    /// ```
    pub fn transform(self, f: impl Fn(A) -> A + 'static) -> Self {
        self.transform_flatten(move |a| Chunk::new(f(a)))
    }

    /// Materializes a chunk by converting it into a collected form.
    ///
    /// This method evaluates any lazy transformations and creates a new chunk containing
    /// all elements in a `Collect` variant. This can be useful for performance when you
    /// plan to reuse the chunk multiple times, as it prevents re-evaluation of transformations.
    ///
    /// # Performance
    /// - Time complexity: O(n) where n is the number of elements
    /// - Space complexity: O(n) as it creates a new vector containing all elements
    ///
    /// # Examples
    /// ```
    /// use tailcall_chunk::Chunk;
    ///
    /// let chunk = Chunk::default()
    ///     .append(1)
    ///     .append(2)
    ///     .transform(|x| x * 2);  // Lazy transformation
    ///
    /// // Materialize the chunk to evaluate the transformation once
    /// let materialized = chunk.materialize();
    ///
    /// assert_eq!(materialized.as_vec(), vec![2, 4]);
    /// ```
    pub fn materialize(self) -> Chunk<A>
    where
        A: Clone,
    {
        Chunk::Collect(Rc::new(RefCell::new(self.as_vec())))
    }

    /// Transforms each element in the chunk into a new chunk and flattens the result.
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
    /// * `f` - A function that takes an element of type `A` and returns
    ///         a new `Chunk<A>`
    ///
    /// # Examples
    /// ```
    /// use tailcall_chunk::Chunk;
    ///
    /// let chunk = Chunk::default().append(1).append(2);
    /// // Transform each number x into a chunk containing [x, x+1]
    /// let expanded = chunk.transform_flatten(|x| {
    ///     Chunk::default().append(x).append(x + 1)
    /// });
    /// assert_eq!(expanded.as_vec(), vec![1, 2, 2, 3]);
    /// ```
    pub fn transform_flatten(self, f: impl Fn(A) -> Chunk<A> + 'static) -> Self {
        Chunk::TransformFlatten(Rc::new(self), Rc::new(f))
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
    /// let chunk = Chunk::default().append(1).append(2).append(3);
    /// assert_eq!(chunk.as_vec(), vec![1, 2, 3]);
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
            Chunk::Single(a) => {
                buf.push(a.clone());
            }
            Chunk::Concat(a, b) => {
                a.as_vec_mut(buf);
                b.as_vec_mut(buf);
            }
            Chunk::TransformFlatten(a, f) => {
                let mut tmp = Vec::new();
                a.as_vec_mut(&mut tmp);
                for elem in tmp.into_iter() {
                    f(elem).as_vec_mut(buf);
                }
            }
            Chunk::Collect(vec) => {
                buf.extend(vec.borrow().iter().cloned());
            }
        }
    }
}

impl<A> FromIterator<A> for Chunk<A> {
    /// Creates a chunk from an iterator.
    ///
    /// # Examples
    /// ```
    /// use tailcall_chunk::Chunk;
    ///
    /// let vec = vec![1, 2, 3];
    /// let chunk: Chunk<_> = vec.into_iter().collect();
    /// assert_eq!(chunk.as_vec(), vec![1, 2, 3]);
    /// ```
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let vec: Vec<_> = iter.into_iter().collect();

        Chunk::Collect(Rc::new(RefCell::new(vec)))
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

        let non_empty = empty.append(1);
        assert!(!non_empty.is_null());
    }

    #[test]
    fn test_append() {
        let chunk = Chunk::default().append(1).append(2).append(3);
        assert_eq!(chunk.as_vec(), vec![1, 2, 3]);

        // Test that original chunk remains unchanged (persistence)
        let chunk1 = Chunk::default().append(1);
        let chunk2 = chunk1.clone().append(2);
        assert_eq!(chunk1.as_vec(), vec![1]);
        assert_eq!(chunk2.as_vec(), vec![1, 2]);
    }

    #[test]
    fn test_concat() {
        let chunk1 = Chunk::default().append(1).append(2);
        let chunk2 = Chunk::default().append(3).append(4);
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
        let single = Chunk::default().append(42);
        assert_eq!(single.as_vec(), vec![42]);

        // Test multiple elements
        let multiple = Chunk::default().append(1).append(2).append(3);
        assert_eq!(multiple.as_vec(), vec![1, 2, 3]);

        // Test complex structure with concatenation
        let chunk1 = Chunk::default().append(1).append(2);
        let chunk2 = Chunk::default().append(3).append(4);
        let complex = chunk1.concat(chunk2);
        assert_eq!(complex.as_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_structural_sharing() {
        let chunk1 = Chunk::default().append(1).append(2);
        let chunk2 = chunk1.clone().append(3);
        let chunk3 = chunk1.clone().append(4);

        // Verify that modifications create new structures while preserving the original
        assert_eq!(chunk1.as_vec(), vec![1, 2]);
        assert_eq!(chunk2.as_vec(), vec![1, 2, 3]);
        assert_eq!(chunk3.as_vec(), vec![1, 2, 4]);
    }

    #[test]
    fn test_with_different_types() {
        // Test with strings
        let string_chunk = Chunk::default()
            .append(String::from("hello"))
            .append(String::from("world"));
        assert_eq!(string_chunk.as_vec().len(), 2);

        // Test with floating point numbers - using standard constants
        let float_chunk = Chunk::default()
            .append(std::f64::consts::PI)
            .append(std::f64::consts::E);
        assert_eq!(
            float_chunk.as_vec(),
            vec![std::f64::consts::PI, std::f64::consts::E]
        );

        // Test with boolean values
        let bool_chunk = Chunk::default().append(true).append(false).append(true);
        assert_eq!(bool_chunk.as_vec(), vec![true, false, true]);
    }

    #[test]
    fn test_transform() {
        // Test transform on empty chunk
        let empty: Chunk<i32> = Chunk::default();
        let transformed_empty = empty.transform(|x| x * 2);
        assert_eq!(transformed_empty.as_vec(), Vec::<i32>::new());

        // Test transform on single element
        let single = Chunk::default().append(5);
        let doubled = single.transform(|x| x * 2);
        assert_eq!(doubled.as_vec(), vec![10]);

        // Test transform on multiple elements
        let multiple = Chunk::default().append(1).append(2).append(3);
        let doubled = multiple.transform(|x| x * 2);
        assert_eq!(doubled.as_vec(), vec![2, 4, 6]);

        // Test transform with string manipulation
        let string_chunk = Chunk::default()
            .append(String::from("hello"))
            .append(String::from("world"));
        let uppercase = string_chunk.transform(|s| s.to_uppercase());
        assert_eq!(uppercase.as_vec(), vec!["HELLO", "WORLD"]);

        // Test chaining multiple transforms
        let numbers = Chunk::default().append(1).append(2).append(3);
        let result = numbers
            .transform(|x| x * 2)
            .transform(|x| x + 1)
            .transform(|x| x * 3);
        assert_eq!(result.as_vec(), vec![9, 15, 21]);
    }

    #[test]
    fn test_transform_flatten() {
        // Test transform_flatten on empty chunk
        let empty: Chunk<i32> = Chunk::default();
        let transformed_empty = empty.transform_flatten(|x| Chunk::new(x * 2));
        assert_eq!(transformed_empty.as_vec(), Vec::<i32>::new());

        // Test transform_flatten on single element
        let single = Chunk::default().append(5);
        let doubled = single.transform_flatten(|x| Chunk::new(x * 2));
        assert_eq!(doubled.as_vec(), vec![10]);

        // Test expanding each element into multiple elements
        let numbers = Chunk::default().append(1).append(2);
        let expanded = numbers.transform_flatten(|x| Chunk::default().append(x + 1).append(x));
        assert_eq!(expanded.as_vec(), vec![2, 1, 3, 2]);

        // Test with nested chunks
        let chunk = Chunk::default().append(1).append(2).append(3);
        let nested = chunk.transform_flatten(|x| {
            if x % 2 == 0 {
                // Even numbers expand to [x, x+1]
                Chunk::default().append(x).append(x + 1)
            } else {
                // Odd numbers expand to [x]
                Chunk::new(x)
            }
        });
        assert_eq!(nested.as_vec(), vec![1, 2, 3, 3]);

        // Test chaining transform_flatten operations
        let numbers = Chunk::default().append(1).append(2);
        let result = numbers
            .transform_flatten(|x| Chunk::default().append(x).append(x))
            .transform_flatten(|x| Chunk::default().append(x).append(x + 1));
        assert_eq!(result.as_vec(), vec![1, 2, 1, 2, 2, 3, 2, 3]);

        // Test with empty chunk results
        let chunk = Chunk::default().append(1).append(2);
        let filtered = chunk.transform_flatten(|x| {
            if x % 2 == 0 {
                Chunk::new(x)
            } else {
                Chunk::default() // Empty chunk for odd numbers
            }
        });
        assert_eq!(filtered.as_vec(), vec![2]);
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

        // Test mixing prepend and append
        let mixed = Chunk::default()
            .prepend(1) // [1]
            .append(2) // [1, 2]
            .prepend(3); // [3, 1, 2]
        assert_eq!(mixed.as_vec(), vec![3, 1, 2]);
    }

    #[test]
    fn test_from_iterator() {
        // Test collecting from an empty iterator
        let empty_vec: Vec<i32> = vec![];
        let empty_chunk: Chunk<i32> = empty_vec.into_iter().collect();
        assert!(empty_chunk.is_null());

        // Test collecting from a vector
        let vec = vec![1, 2, 3];
        let chunk: Chunk<_> = vec.into_iter().collect();
        assert_eq!(chunk.as_vec(), vec![1, 2, 3]);

        // Test collecting from a range
        let range_chunk: Chunk<_> = (1..=5).collect();
        assert_eq!(range_chunk.as_vec(), vec![1, 2, 3, 4, 5]);

        // Test collecting from map iterator
        let doubled: Chunk<_> = vec![1, 2, 3].into_iter().map(|x| x * 2).collect();
        assert_eq!(doubled.as_vec(), vec![2, 4, 6]);
    }

    #[test]
    fn test_concat_optimization() {
        // Create a collected chunk
        let collected: Chunk<i32> = vec![1, 2, 3].into_iter().collect();

        // Concat a single element
        let result = collected.concat(Chunk::Single(4));

        // Verify the result
        assert_eq!(result.as_vec(), vec![1, 2, 3, 4]);

        // Verify it's still a Collect variant (not a Concat)
        match result {
            Chunk::Collect(_) => (), // This is what we want
            _ => panic!("Expected Collect variant after optimization"),
        }
    }
}
