#[macro_use]
extern crate rola;

use rola::Vector;

#[allow(non_snake_case)]
fn main() {
    // Fill a dense matrix with one number.
    let A1 = dense![0; 4, 20];
    // A sparse matrix of zeros.
    let Z = zeros!(4, 20);
    assert_eq!(A1, Z);

    // By unwrapping here, you are implicitly signing a contract that your
    // matrix has valid dimensions!
    let A2 = dense![1, 0, 0;
                    0, 1, 0;
                    0, 0, 1].unwrap();
    // A sparse identity matrix.
    let I = eye!(3);
    assert_eq!(A2, I);

    let A3 = dense![0,-1, 0; 0, 0, 0; 3, 0, 0].unwrap();
    // A sparse matrix where a_01 = -1 and a_20 = 3.
    // Note that we index from 0 to m-1 and 0 to n-1.
    let B3 = sparse![(0, 1, -1), (2, 0, 3); 3, 3];
    assert_eq!(A3, B3);

    // Calculate an inner product.
    let u1 = vector![1, 2, 3, 4, 5];
    let v1 = vector![1, 2, 3, 4, 5];
    assert_eq!(u1.transpose()*v1, 1+4+9+16+25);

    // Calculate an outer Product.
    let u2 = vector![1, 2, 3, 4];
    let v2 = vector![1, 2, 3];
    let A4 = dense![1, 2, 3;
                    2, 4, 6;
                    3, 6, 9;
                    4, 8, 12].unwrap();
    assert_eq!(u2*v2.transpose(), A4);
}
