//! Checkpoint 1: SlowVec's push and remove.
//!
//!     cargo test --test cp1
//!
//! You do not write any of these, but read them when one fails. The final
//! autograder runs these again, plus cases that are not here.

use slow_vec::SlowVec;

mod basic {
    use super::*;
    use rand::RngExt;

    #[test]
    fn new_vec_is_empty() {
        let v: SlowVec<i32> = SlowVec::new();
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn one_push_gives_length_one() {
        let mut v: SlowVec<i32> = SlowVec::new();
        v.push(10);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn push_then_get_numbers() {
        let mut v = SlowVec::new();
        v.push(1);
        v.push(33);
        v.push(-5);
        assert_eq!(v.get(0), &1);
        assert_eq!(v.get(2), &-5);
        assert_eq!(v.get(1), &33);
    }

    #[test]
    fn push_then_get_strings() {
        let mut v = SlowVec::new();
        v.push(String::from("hello"));
        v.push(String::from("bye"));
        v.push(String::from("morning"));
        assert_eq!(v.get(1), "bye");
        assert_eq!(v.get(0), "hello");
        assert_eq!(v.get(2), "morning");
    }

    #[test]
    fn hundred_random_numbers_survive_pushing() {
        let mut rng = rand::rng();
        let mut input = Vec::with_capacity(100);
        for _ in 0..100 {
            input.push(rng.random_range(0..50));
        }

        let mut v = SlowVec::new();
        for number in &input {
            v.push(*number);
        }

        assert_eq!(input, v.into_vec());
    }

    #[test]
    fn hundred_random_strings_survive_pushing() {
        let mut rng = rand::rng();
        let mut input = Vec::with_capacity(100);
        for _ in 0..100 {
            let random_string = format!("str{}", rng.random_range(0..50));
            input.push(random_string);
        }

        let mut v = SlowVec::new();
        for string in &input {
            v.push(string.to_owned());
        }

        assert_eq!(input, v.into_vec());
    }

    #[test]
    fn remove_matches_vec_at_every_step() {
        // Set up the vectors.
        let mut input = vec![-1, 3, -200, 25, 33];
        let mut v = SlowVec::new();
        for num in &input {
            v.push(*num);
        }

        // Indices to remove in order.
        let removes = vec![1, 3, 0, 1, 0];
        for remove in removes {
            v.remove(remove);
            input.remove(remove);

            assert_eq!(v.len(), input.len());
            for i in 0..input.len() {
                assert_eq!(v.get(i), &input[i]);
            }
        }

        assert_eq!(v.len(), 0);
    }

    #[test]
    fn clear_then_push_again() {
        let mut v = SlowVec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.clear();
        assert_eq!(v.len(), 0);
        v.push(4);
        assert_eq!(v.len(), 1);
        assert_eq!(v.get(0), &4);
    }
}

// These watch every allocation and free, so they check that push and remove
// resize the way the handout describes: a new array one bigger or one smaller,
// and the old one given back.
mod memory {
    use super::*;
    use malloc::MALLOC;

    #[test]
    fn new_vec_allocates_nothing() {
        MALLOC.clear();

        let v: SlowVec<i32> = SlowVec::new();
        assert_eq!(v.len(), 0);
        assert_eq!(MALLOC.state().transcript().len(), 0);

        MALLOC.clear();
    }

    #[test]
    fn push_allocates_one_bigger_and_frees_the_old() {
        MALLOC.clear();

        let mut v = SlowVec::new();
        assert_eq!(MALLOC.state().allocations().len(), 0);
        assert_eq!(MALLOC.state().transcript().len(), 0);

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
        assert!(MALLOC.state().transcript()[3].is_allocation(size_of::<String>() * 3));
        assert!(MALLOC.state().transcript()[4].is_free());

        v.clear();
        assert_eq!(MALLOC.state().allocations().len(),  0);
        assert_eq!(MALLOC.state().transcript().len(), 6);
        assert!(MALLOC.state().transcript()[5].is_free());

        MALLOC.clear();
    }

    #[test]
    fn remove_allocates_one_smaller_and_frees_the_old() {
        MALLOC.clear();

        // Set up the vectors.
        let input = vec![-1, 3, -200, 25, 33];
        let mut v = SlowVec::new();
        for num in input {
            v.push(num);
        }

        assert_eq!(MALLOC.state().allocations().len(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 9);

        // Indices to remove in order.
        let removes = vec![1, 3, 0, 1, 0];
        for i in 0..removes.len() - 1 {
            let remove = removes[i];
            v.remove(remove);

            assert_eq!(v.len(), 5 - i - 1);
            assert_eq!(MALLOC.state().allocations().len(), 1);
            assert_eq!(MALLOC.state().transcript().len(), 9 + 2*i + 2);
            assert!(MALLOC.state().transcript()[9 + 2*i].is_allocation(size_of::<i32>() * v.len()));
            assert!(MALLOC.state().transcript()[9 + 2*i + 1].is_free());
        }

        v.remove(*removes.last().unwrap());
        assert_eq!(MALLOC.state().allocations().len(), 0);
        assert_eq!(MALLOC.state().transcript().len(), 9 + 2 * removes.len() - 1);
        assert!(MALLOC.state().transcript()[9 + 2 * removes.len() - 2].is_free());

        MALLOC.clear();
    }

    #[test]
    fn clear_frees_the_array() {
        MALLOC.clear();

        let mut v = SlowVec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(MALLOC.state().allocations().len(), 1);
        let before = MALLOC.state().transcript().len();

        v.clear();
        assert_eq!(MALLOC.state().allocations().len(), 0);
        assert_eq!(MALLOC.state().transcript().len(), before + 1);
        assert!(MALLOC.state().transcript()[before].is_free());

        MALLOC.clear();
    }
}
