//! Checkpoint 2: FastVec's get and push, and Growth::next_capacity.
//!
//!     cargo test --test cp2
//!
//! You do not write any of these, but read them when one fails. The final
//! autograder runs these again, plus cases that are not here.

use fast_vec::{FastVec, Growth};

mod growth {
    use super::*;

    #[test]
    fn double_doubles() {
        assert_eq!(Growth::Double.next_capacity(1), 2);
        assert_eq!(Growth::Double.next_capacity(4), 8);
        assert_eq!(Growth::Double.next_capacity(64), 128);
    }

    #[test]
    fn multiply_scales_by_the_factor() {
        assert_eq!(Growth::Multiply(1.5).next_capacity(4), 6);
        assert_eq!(Growth::Multiply(1.5).next_capacity(10), 15);
        assert_eq!(Growth::Multiply(3.0).next_capacity(2), 6);
    }

    #[test]
    fn multiply_rounds_down() {
        assert_eq!(Growth::Multiply(1.5).next_capacity(5), 7);
    }

    #[test]
    fn add_adds_the_slots() {
        assert_eq!(Growth::Add(3).next_capacity(1), 4);
        assert_eq!(Growth::Add(3).next_capacity(10), 13);
        assert_eq!(Growth::Add(1000).next_capacity(5), 1005);
    }
}

mod basic {
    use super::*;
    use rand::RngExt;

    #[test]
    fn new_vec_has_capacity_one() {
        let v: FastVec<i32> = FastVec::new();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 1);
    }

    #[test]
    fn first_push_fits_without_growing() {
        let mut v: FastVec<i32> = FastVec::new();
        v.push(10);
        assert_eq!(v.len(), 1);
        assert_eq!(v.capacity(), 1);
    }

    #[test]
    fn get_strings_by_index() {
        let inputs = vec![
            String::from("hello"),
            String::from("bye"),
            String::from("morning"),
        ];
        let fast_vec = FastVec::from_vec(inputs);
        assert_eq!(fast_vec.get(1), "bye");
        assert_eq!(fast_vec.get(0), "hello");
        assert_eq!(fast_vec.get(2), "morning");
    }

    #[test]
    fn get_twice_leaves_the_element_in_place() {
        let fast_vec = FastVec::from_vec(vec![String::from("still here")]);
        assert_eq!(fast_vec.get(0), "still here");
        assert_eq!(fast_vec.get(0), "still here");
    }

    #[test]
    #[should_panic(expected = "FastVec: get out of bounds")]
    fn get_out_of_bounds_panics() {
        let mut v = FastVec::new();
        v.push(1);
        v.push(33);
        v.push(-5);
        v.get(3);
    }

    #[test]
    fn push_grows_one_two_four() {
        let mut v = FastVec::new();
        v.push(1);
        assert_eq!(v.len(), 1);
        assert_eq!(v.capacity(), 1);
        v.push(33);
        assert_eq!(v.len(), 2);
        assert_eq!(v.capacity(), 2);
        v.push(-5);
        assert_eq!(v.len(), 3);
        assert_eq!(v.capacity(), 4);
        assert_eq!(v.get(0), &1);
        assert_eq!(v.get(1), &33);
        assert_eq!(v.get(2), &-5);
        assert_eq!(v.into_vec(), vec![1, 33, -5]);
    }

    #[test]
    fn hundred_random_numbers_survive_growing() {
        let mut rng = rand::rng();
        let mut input = Vec::with_capacity(100);
        for _ in 0..100 {
            input.push(rng.random_range(0..50));
        }

        let mut v = FastVec::new();
        for number in &input {
            v.push(*number);
        }

        assert_eq!(input, v.into_vec());
    }

    #[test]
    fn hundred_random_strings_survive_growing() {
        let mut rng = rand::rng();
        let mut input = Vec::with_capacity(100);
        for _ in 0..100 {
            let random_string = format!("str{}", rng.random_range(0..50));
            input.push(random_string);
        }

        let mut v = FastVec::new();
        for string in &input {
            v.push(string.to_owned());
        }

        assert_eq!(input, v.into_vec());
    }

    #[test]
    fn push_with_multiply_three() {
        let mut v = FastVec::with_growth(Growth::Multiply(3.0));
        let mut capacities = Vec::new();
        for i in 0..10 {
            v.push(i);
            capacities.push(v.capacity());
        }
        assert_eq!(capacities, vec![1, 3, 3, 9, 9, 9, 9, 9, 9, 27]);
        assert_eq!(v.into_vec(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn push_with_add_four() {
        let mut v = FastVec::with_growth(Growth::Add(4));
        let mut capacities = Vec::new();
        for i in 0..10 {
            v.push(i);
            capacities.push(v.capacity());
        }
        assert_eq!(capacities, vec![1, 5, 5, 5, 5, 9, 9, 9, 9, 13]);
        assert_eq!(v.into_vec(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

// These watch every allocation and free, so they check that push grows the
// way the handout describes, and gives back the memory it no longer needs.
mod memory {
    use super::*;
    use malloc::MALLOC;
    use tracker::Tracker;

    // A new FastVec already owns one slot, so the first push needs no new memory.
    #[test]
    fn first_push_allocates_nothing_new() {
        MALLOC.clear();

        let mut v: FastVec<i32> = FastVec::new();
        assert_eq!(MALLOC.state().allocations().len(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 1);
        assert!(MALLOC.state().transcript()[0].is_allocation(size_of::<i32>()));

        v.push(7);
        assert_eq!(v.len(), 1);
        assert_eq!(v.capacity(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 1);

        MALLOC.clear();
    }

    #[test]
    fn push_allocates_then_frees_the_old() {
        MALLOC.clear();

        let mut v = FastVec::new();
        assert_eq!(MALLOC.state().allocations().len(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 1);
        assert!(MALLOC.state().transcript()[0].is_allocation(size_of::<String>()));

        v.push(String::from("hello"));
        assert_eq!(MALLOC.state().allocations().len(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 1);
        assert!(MALLOC.state().transcript()[0].is_allocation(size_of::<String>()));

        v.push(String::from("bye"));
        assert_eq!(MALLOC.state().allocations().len(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 3);
        assert!(MALLOC.state().transcript()[1].is_allocation(size_of::<String>() * 2));
        assert!(MALLOC.state().transcript()[2].is_free());

        v.push(String::from("morning"));
        assert_eq!(MALLOC.state().allocations().len(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 5);
        assert!(MALLOC.state().transcript()[3].is_allocation(size_of::<String>() * 4));
        assert!(MALLOC.state().transcript()[4].is_free());

        v.push(String::from("again"));
        assert_eq!(MALLOC.state().allocations().len(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 5);

        MALLOC.clear();
    }

    #[test]
    fn growing_drops_nothing() {
        MALLOC.clear();

        let mut tracker = Tracker::new();
        let mut v = FastVec::new();

        v.push(tracker.track(String::from("hello")));
        v.push(tracker.track(String::from("bye")));
        v.push(tracker.track(String::from("morning")));

        // Growing moves the elements to the new memory. Nothing should be
        // dropped along the way.
        assert_eq!(v.len(), 3);
        assert_eq!(v.capacity(), 4);
        assert_eq!(tracker.tracked_count(), 3);

        drop(v);

        MALLOC.clear();
    }
}
