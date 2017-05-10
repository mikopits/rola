pub use self::identity_matrix::IdentityMatrix;
pub use self::zero_matrix::ZeroMatrix;

mod identity_matrix;
mod zero_matrix;

#[cfg(test)]
mod tests {
    use sparse::{IdentityMatrix, ZeroMatrix};

    #[allow(non_snake_case)]
    #[test]
    fn test_identity_iter() {
        let I2 = IdentityMatrix::new(2);
        let mut iter = I2.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_zero_iter() {
        let Z2 = ZeroMatrix::new(2, 2);
        let mut iter = Z2.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
    }
}
