use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ptr::null_mut;
use malloc::MALLOC;

pub struct FixedSizeArray<T> {
    ptr: *mut T,
    len: usize,
    initialized: HashSet<usize>,
}
impl<T> FixedSizeArray<T> {
    pub fn allocate(len: usize) -> Self {
        let mut ptr = null_mut();
        if len > 0 {
            let bytes = len * size_of::<T>();
            ptr = unsafe { MALLOC.malloc(bytes) as *mut T };
        }
        return FixedSizeArray {
            ptr: ptr,
            initialized: HashSet::new(),
            len: len
        };
    }

    pub fn get(&self, i: usize) -> &T {
        unsafe {
            if i >= self.len() {
                panic!("Index out of bounds");
            }
            if !self.initialized.contains(&i) {
                panic!("attempting to get element at {} before it is initialized", i);
            }
            let tmp = self.ptr.add(i);
            return &*tmp;
        }
    }

    pub fn move_out(&mut self, i: usize) -> T {
        unsafe {
            if i >= self.len() {
                panic!("Index out of bounds");
            }
            if !self.initialized.contains(&i) {
                panic!("attempting to move_out element at {} before it is initialized", i);
            }
            self.initialized.remove(&i);
            let tmp = self.ptr.add(i);
            return std::ptr::read(tmp);
        }
    }

    pub fn put(&mut self, element: T, i: usize) {
        unsafe {
            if i >= self.len() {
                panic!("Index out of bounds");
            }
            if self.initialized.contains(&i) {
                panic!("attempting to put element at {} but a different element already exists there", i);
            }
            self.initialized.insert(i);
            let tmp = self.ptr.add(i);
            std::ptr::write(tmp, element);
        }
    }
    pub fn len(&self) -> usize {
        return self.len;
    }
}

// Freeing memory
impl<T> Drop for FixedSizeArray<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            for i in 0..self.len() {
                if self.initialized.contains(&i) {
                    let _element = self.move_out(i);
                }
            }
            unsafe { MALLOC.free(self.ptr as *mut u8); }
        }
    }
}

// Allows printing variables of type FixedSizeArray using println!
impl<T: Display> Display for FixedSizeArray<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fixed[")?;
        if self.len > 0 {
            for i in 0..self.len-1 {
                write!(f, "{}, ", self.get(i))?;
            }
            write!(f, "{}", self.get(self.len - 1))?;
        }
        return write!(f, "]");
    }
}