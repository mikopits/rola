#[macro_use]
extern crate rola;

#[allow(non_snake_case)]
fn main() {
    // Fill a dense matrix with one number.
    let A1 = dense![0; 4, 20];
    // A sparse matrix of zeros.
    let Z = zeros!(4, 20);
    assert_eq!(A1, Z);
    println!("{:?} = {:?}", A1, Z);

    // By unwrapping here, you are implicitly signing a contract that your
    // matrix has valid dimensions!
    let A2 = dense![1, 0, 0;
                    0, 1, 0;
                    0, 0, 1].unwrap();
    // A sparse identity matrix.
    let I = eye!(3);
    assert_eq!(A2, I);
    println!("{:?} = {:?}", A2, I);

    let A3 = dense![0,-1, 0; 0, 0, 0; 3, 0, 0].unwrap();
    // A sparse matrix where a_01 = -1 and a_20 = 3.
    // Note that we index from 0 to m-1 and 0 to n-1.
    let B3 = sparse![(0, 1, -1), (2, 0, 3); 3, 3];
    assert_eq!(A3, B3);
    println!("{:?} = {:?}", A3, B3);
}
