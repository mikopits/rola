pub trait Vector<T> where T: ::Num + Clone + Copy {
    type Transpose;

    /// Transpose the `Vector`.
    #[inline]
    fn transpose(self) -> Self::Transpose;

    /// Set an element at position `index`. Returns None if index is out of
    /// bounds.
    #[inline]
    fn set(&self, index: usize, val: T) -> Option<T>;

    /// Get an element at position `index`.
    #[inline]
    fn get(&self, index: usize) -> Option<T>;

    /// Get the length of the `Vector`.
    #[inline]
    fn len(&self) -> usize;
}
