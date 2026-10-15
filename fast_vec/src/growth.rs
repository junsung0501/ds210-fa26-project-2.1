// How a FastVec picks its new capacity when it is full.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Growth {
    // Twice the old capacity. What Rust's own Vec does.
    Double,
    // The old capacity times this factor, rounded down. Multiply(1.5) grows by half.
    Multiply(f64),
    // The old capacity plus this many slots.
    Add(usize),
}

impl Growth {
    // The capacity a full FastVec should grow to, given its current capacity.
    //
    // Whatever the strategy, the answer must always be bigger than `capacity`,
    // or push has nowhere to put the new element.
    pub fn next_capacity(&self, capacity: usize) -> usize {
        todo!("implement next_capacity!");
    }
}
