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
        let proposed = match self {
            Growth::Double => capacity.saturating_mul(2),
            Growth::Multiply(factor) => (capacity as f64 * *factor) as usize,
            Growth::Add(slots) => capacity.saturating_add(*slots),
        };
        // Must always be bigger than `capacity` (covers capacity 0, factor <= 1, Add(0)).
        proposed.max(capacity.saturating_add(1))
    }
}
