use std::cell::{Ref, RefCell};
use crate::state::State;

pub struct Allocator {
    state: RefCell<State>,
}
impl Allocator {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(State::new()),
        }
    }

    pub unsafe fn malloc(&self, bytes: usize) -> *mut u8 {
        let ptr = unsafe {
            libc::malloc(bytes) as *mut u8
        };
        if ptr.is_null() {
            panic!("malloc failed");
        }

        self.state.borrow_mut().record_allocation(ptr as usize, bytes);
        ptr
    }

    pub unsafe fn free(&self, ptr: *mut u8) {
        self.state.borrow_mut().record_free(ptr as usize);
        unsafe { libc::free(ptr as *mut libc::c_void) };
    }

    pub fn state(&self) -> Ref<'_, State> {
        self.state.borrow()
    }

    pub fn clear(&self) {
        self.state.borrow_mut().clear();
    }
}

unsafe impl Sync for Allocator {}
unsafe impl Send for Allocator {}