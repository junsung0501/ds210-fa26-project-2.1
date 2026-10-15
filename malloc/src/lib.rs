use lazy_static::lazy_static;

pub mod allocator;
pub mod state;

lazy_static! {
    pub static ref MALLOC: allocator::Allocator = {
        allocator::Allocator::new()
    };
}