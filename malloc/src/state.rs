use std::collections::HashMap;
use multi_containers::HashMultiSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Allocation(usize, usize),
    Free(usize),
}
impl Operation {
    pub fn is_allocation(&self, size: usize) -> bool {
        match self {
            Operation::Allocation(_, sz) => *sz == size,
            Operation::Free(_) => false,
        }
    }
    pub fn is_free(&self) -> bool {
        match self {
            Operation::Allocation(_, _) => false,
            Operation::Free(_) => true,
        }
    }
}

pub struct State {
    allocations: HashMap<usize, usize>,
    transcript: Vec<Operation>,
}
impl State {
    pub fn new() -> Self {
        Self {
            allocations: Default::default(),
            transcript: Default::default(),
        }
    }
    pub fn transcript(&self) -> &[Operation] {
        &self.transcript
    }
    pub fn allocations(&self) -> &HashMap<usize, usize> {
        &self.allocations
    }
    pub fn record_allocation(&mut self, address: usize, size: usize) {
        self.allocations.insert(address, size);
        self.transcript.push(Operation::Allocation(address, size));
    }
    pub fn record_free(&mut self, address: usize) {
        self.transcript.push(Operation::Free(address));
        self.allocations.remove(&address);
    }
    pub fn clear(&mut self) {
        self.allocations.clear();
        self.transcript.clear();
    }

    pub fn has_allocations(&self, sizes: Vec<usize>) -> bool {
        let mut multiset =
            HashMultiSet::from_iter(self.allocations.iter().map(|(_k, v)| *v));
        for size in sizes {
            if multiset.remove(&size) == 0 {
                return false;
            }
        }
        multiset.is_empty()
    }
}