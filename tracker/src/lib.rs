use std::any::Any;
use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;

trait MyTrait: Display + Any + 'static {}
impl<T: Display + Any> MyTrait for T {}

// Something that can be printed.
struct Displayable {
    value: Box<dyn MyTrait>,
}
impl Displayable {
    fn new<T: MyTrait>(value: T) -> Displayable {
        return Displayable { value: Box::new(value) };
    }
    fn cast_ref<T: MyTrait>(&self) -> &T {
        let r = self.value.as_ref() as &dyn Any;
        return r.downcast_ref().unwrap();
    }
}
impl Display for Displayable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.value.as_ref() as &dyn Display;
        return write!(f, "Tracked({})", x);
    }
}

// Use this to track values in memory.
pub struct Tracker {
    counter: usize,
    map: HashMap<usize, Displayable>,
}
impl Tracker {
    pub fn new() -> Tracker {
        Tracker {
            counter: 0,
            map: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.map.is_empty();
    }

    pub fn track<T: Display + Any + 'static>(&mut self, t: T) -> Tracked<T> {
        let index = self.counter;
        self.counter += 1;
        self.map.insert(index, Displayable::new(t));
        return Tracked::new(self as *mut Tracker, index);
    }

    pub fn print_status(&self) {
        if self.map.is_empty() {
            println!("All values have been freed");
        } else {
            print!("These values have not been freed:");
            for (_, displayable) in &self.map {
                print!("  {}", displayable);
            }
            println!("");
        }
    }

    pub fn tracked_count(&self) -> usize {
        return self.map.len();
    }
}


// Represents a tracked value of type T.
pub struct Tracked<T: Display + Any + 'static> {
    tracker: *mut Tracker,
    index: usize,
    _t: PhantomData<T>,
}
impl<T: Display + Any + 'static> Tracked<T> {
    fn new(tracker: *mut Tracker, index: usize) -> Tracked<T> {
        Tracked {
            tracker: tracker,
            index: index,
            _t: PhantomData,
        }
    }
    fn get(&self) -> &T {
        unsafe {
            let tracker = &*self.tracker;
            let displayable = tracker.map.get(&self.index).unwrap();
            return displayable.cast_ref();
        }
    }
}
impl<T: Display + Any + 'static> Display for Tracked<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.get());
    }
}
impl<T: Display + Any + 'static> Drop for Tracked<T> {
    fn drop(&mut self) {
        unsafe {
            let tracker = &mut *self.tracker;
            let _ = tracker.map.remove(&self.index).unwrap();
        }
    }
}