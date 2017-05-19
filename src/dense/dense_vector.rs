use std::cell::Cell;
use std::fmt;

use ::{Num, Vector, Zero};

#[derive(Clone, Debug)]
pub struct DenseRow<T> where T: Copy {
    elems: Vec<Cell<T>>,
}

impl<T: Clone + Copy + Num> DenseRow<T> {
    pub fn zeros(size: usize) -> DenseRow<T> where T: Zero {
        DenseRow { elems: vec![Cell::new(T::zero()); size] }
    }

    pub fn from_vec(vec: Vec<T>) -> DenseRow<T> {
        DenseRow { elems: vec.iter().map(|&e| Cell::new(e)).collect() }
    }
}

impl<T: Clone + Copy + Num> Vector<T> for DenseRow<T> {
    type Transpose = DenseColumn<T>;

    fn transpose(self) -> DenseColumn<T> {
        DenseColumn { elems: self.elems }
    }

    fn get(&self, index: usize) -> Option<T> {
        match self.elems.get(index) {
            Some(e) => Some(e.get()),
            None => None,
        }
    }

    fn set(&self, index: usize, val: T) -> Option<T> {
        match self.elems.get(index) {
            Some(e) => { e.set(val); Some(val) },
            None => None
        }
    }

    fn len(&self) -> usize {
        self.elems.len()
    }
}

impl<T: Copy + fmt::Debug> fmt::Display for DenseRow<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DenseRow(elems: {:?})", self.elems)
    }
}

#[derive(Clone, Debug)]
pub struct DenseColumn<T> where T: Copy {
    elems: Vec<Cell<T>>,
}

impl<T: Clone + Copy + Num> DenseColumn<T> {
    pub fn zeros(size: usize) -> DenseColumn<T> where T: Zero {
        DenseColumn { elems: vec![Cell::new(T::zero()); size] }
    }

    pub fn from_vec(vec: Vec<T>) -> DenseColumn<T> {
        DenseColumn { elems: vec.iter().map(|&e| Cell::new(e)).collect() }
    }
}

impl<T: Clone + Copy + Num> Vector<T> for DenseColumn<T> {
    type Transpose = DenseRow<T>;

    fn transpose(self) -> DenseRow<T> {
        DenseRow { elems: self.elems }
    }

    fn set(&self, index: usize, val: T) -> Option<T> {
        match self.elems.get(index) {
            Some(e) => { e.set(val); Some(val) },
            None => None
        }
    }

    fn get(&self, index: usize) -> Option<T> {
        match self.elems.get(index) {
            Some(e) => Some(e.get()),
            None => None,
        }
    }

    fn len(&self) -> usize {
        self.elems.len()
    }
}

impl<T: Copy + fmt::Debug> fmt::Display for DenseColumn<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DenseRow(elems: {:?})", self.elems)
    }
}
