use std::{
    fmt::{Display, Formatter},
    ptr::{self, null_mut},
};

use malloc::MALLOC;

mod growth;
pub use growth::Growth;
pub mod timing;

pub struct FastVec<T> {
    ptr_to_data: *mut T,
    len: usize,
    capacity: usize,
    growth: Growth,
}
impl<T> FastVec<T> {
    // Creating a new FastVec that is either empty or has capacity for some future elements.
    // Both of these grow by doubling.
    pub fn new() -> FastVec<T> {
        return FastVec::with_capacity(1);
    }
    pub fn with_capacity(capacity: usize) -> FastVec<T> {
        let ptr = unsafe { MALLOC.malloc(size_of::<T>() * capacity) as *mut T };
        return FastVec {
            ptr_to_data: ptr,
            len: 0,
            capacity: capacity,
            growth: Growth::Double,
        };
    }

    // An empty FastVec with capacity 1 that grows by the given strategy.
    pub fn with_growth(growth: Growth) -> FastVec<T> {
        let mut fast_vec = FastVec::with_capacity(1);
        fast_vec.growth = growth;
        return fast_vec;
    }

    // Retrieve the FastVec's length and capacity
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    // Transforms an instance of FastVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len);
        for i in 0..self.len {
            unsafe {
                let ptr = self.ptr_to_data.add(i);
                let element = ptr::read(ptr);
                v.push(element);
            }
        }
        unsafe {
            MALLOC.free(self.ptr_to_data as *mut u8);
        }
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
        return v;
    }

    // Transforms a vector to a FastVec.
    pub fn from_vec(vec: Vec<T>) -> FastVec<T> {
        let mut fast_vec: FastVec<T> = FastVec::with_capacity(vec.len());
        for element in vec {
            unsafe {
                let ptr = fast_vec.ptr_to_data.add(fast_vec.len);
                ptr::write(ptr, element);
            }
            fast_vec.len = fast_vec.len + 1;
        }
        return fast_vec;
    }

    // Use the project handout as a guide for this part!
    pub fn get(&self, i: usize) -> &T {
        if i >= self.len {
            panic!("FastVec: get out of bounds");
        }
        unsafe { &*self.ptr_to_data.add(i) }
    }

    pub fn push(&mut self, t: T) {
        if self.len == self.capacity {
            // Full: ask the growth strategy for a bigger size, allocate that block,
            // move every element over, then free the old block.
            let new_capacity = self.growth.next_capacity(self.capacity);
            let new_ptr = unsafe { MALLOC.malloc(size_of::<T>() * new_capacity) as *mut T };
            for i in 0..self.len {
                unsafe {
                    let element = ptr::read(self.ptr_to_data.add(i));
                    ptr::write(new_ptr.add(i), element);
                }
            }
            if !self.ptr_to_data.is_null() {
                unsafe {
                    MALLOC.free(self.ptr_to_data as *mut u8);
                }
            }
            self.ptr_to_data = new_ptr;
            self.capacity = new_capacity;
        }
        // There is room now (it either had room already, or we just grew): write t.
        unsafe {
            ptr::write(self.ptr_to_data.add(self.len), t);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, i: usize) {
        if i >= self.len {
            panic!("FastVec: remove out of bounds");
        }
        unsafe {
            // SAFETY: i < self.len, so slot i holds a valid element. ptr::read moves it
            // out, so this slot must not be dropped again by anyone else.
            let removed = ptr::read(self.ptr_to_data.add(i));
            // SAFETY: j + 1 < self.len, so slot j + 1 is valid. We read it out and write it
            // into slot j, which was already moved out (or just shifted), so nothing is
            // overwritten while it is still alive.
            for j in i..self.len - 1 {
                let element = ptr::read(self.ptr_to_data.add(j + 1));
                ptr::write(self.ptr_to_data.add(j), element);
            }
            self.len -= 1;
            // The removed element is dropped here, at the end of the block's scope.
            drop(removed);
        }
    }

    // This appears correct but with further testing, you will notice it has a bug!
    // Hint: check out case 2 in memory.rs, which you can run using
    //       cargo run --bin fast_memory
    pub fn clear(&mut self) {
        // The bug: the old clear freed the block but never dropped the elements in it,
        // so anything they owned (like a String's heap buffer) leaked.
        // Fix: move every element out and drop it first, then free the block.
        for i in 0..self.len {
            // SAFETY: i < self.len, so slot i holds a valid element. We read it out
            // exactly once, and len is reset below, so it is never dropped twice.
            unsafe {
                let element = ptr::read(self.ptr_to_data.add(i));
                drop(element);
            }
        }
        if !self.ptr_to_data.is_null() {
            // SAFETY: ptr_to_data came from MALLOC.malloc and is freed only once here,
            // then set to null below.
            unsafe {
                MALLOC.free(self.ptr_to_data as *mut u8);
            }
        }
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
    }
}

// When a FastVec goes out of scope, drop its elements and free its memory.
// clear() already does both, and it is safe after into_vec() too (len is 0, ptr is null).
impl<T> Drop for FastVec<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

// This allows printing FastVecs with println!.
impl<T: Display> Display for FastVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FastVec[")?;
        if self.len > 0 {
            for i in 0..self.len() - 1 {
                write!(f, "{}, ", self.get(i))?;
            }
            write!(f, "{}", self.get(self.len - 1))?;
        }
        return write!(f, "]");
    }
}
