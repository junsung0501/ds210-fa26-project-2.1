use std::fmt::{Display, Formatter};
use fixed::FixedSizeArray;

// A SlowVec contains a fixed number of elements.
// The elements are of type "T"
// This is Rust's way of saying that SlowVec can accept any type for the elements.
// E.g., SlowVec<i32> represents a SlowVec with integer elements,
//       SlowVec<bool> represents a SlowVec with bool elements,
//       etc.
// look at playground.rs for an example.
pub struct SlowVec<T> {
    fixed: FixedSizeArray<T>,
}

// Functions inside SlowVec.
impl<T> SlowVec<T> {
    pub fn new() -> Self {
        return SlowVec {
            fixed: FixedSizeArray::allocate(0)
        };
    }
    
    // returns the length of the SlowVec.
    pub fn len(&self) -> usize {
        return self.fixed.len();
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.fixed.len());
        for i in 0..self.fixed.len() {
            v.push(self.fixed.move_out(i));
        }
        v
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> SlowVec<T> {
        let mut tmp = FixedSizeArray::allocate(vec.len());
        let mut index = 0;
        for element in vec {
            tmp.put(element, index);
            index = index + 1;
        }
        return SlowVec { fixed: tmp };
    }

    // Clear the content of this vector.
    pub fn clear(&mut self) {
        self.fixed = FixedSizeArray::allocate(0);
    }

    // Get a reference to the element at position i.
    // Think of a reference as a read-only "copy" of the element.
    // We will talk about what references are in class.
    // Note: the element remains stored in the SlowVec after get(). It is not removed.
    pub fn get(&self, i: usize) -> &T {
        self.fixed.get(i)
    }

    pub fn push(&mut self, t: T) {
        todo!()
    }

    pub fn remove(&mut self, i: usize) {
        todo!()
    }
}


// This allows us to print the SlowVec using println!().
impl<T: Display> Display for SlowVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlowVec({})", self.fixed)
    }
}
