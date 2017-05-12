#[macro_export]
macro_rules! dense {
    ($elem:expr; $m:expr, $n:expr) => {
        $crate::DenseMatrix::from_vec(vec![$elem; $m*$n], $m, $n, None)
    };
    ($($($a:expr),+);+) => {
        $crate::DenseMatrix::new(&[$(vec![$(($a),)+],)+])
    };
}

#[macro_export]
macro_rules! eye {
    ($n:expr) => {
        $crate::IdentityMatrix::new($n)
    };
}

#[macro_export]
macro_rules! sparse {
    ($($tup:expr),+; $m:expr, $n:expr) => {
        $crate::SparseMatrix::from_tuple(vec![$($tup),+], $m, $n)
    };
}

#[macro_export]
macro_rules! zeros {
    ($m:expr, $n:expr) => {
        $crate::ZeroMatrix::new($m, $n)
    };
}
