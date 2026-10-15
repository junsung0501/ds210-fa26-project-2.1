//! Checkpoint 3: FastVec's remove, the bug in clear, Drop, and the timing
//! functions for the benchmark.
//!
//!     cargo test --test cp3
//!
//! You do not write any of these, but read them when one fails. The final
//! autograder runs these again, plus cases that are not here.

use fast_vec::FastVec;
use malloc::MALLOC;
use tracker::Tracker;

mod remove {
    use super::*;

    #[test]
    fn remove_matches_vec_and_keeps_capacity() {
        // Set up the vectors.
        let mut input = vec![-1, 3, -200, 25, 33];
        let mut v = FastVec::from_vec(input.clone());
        assert_eq!(v.capacity(), 5);

        // Indices to remove in order.
        let removes = vec![1, 3, 0, 1, 0];
        for remove in removes {
            v.remove(remove);
            input.remove(remove);

            assert_eq!(v.len(), input.len());
            assert_eq!(v.capacity(), 5);
            for i in 0..input.len() {
                assert_eq!(v.get(i), &input[i]);
            }
        }

        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 5);
    }

    #[test]
    #[should_panic(expected = "FastVec: remove out of bounds")]
    fn remove_out_of_bounds_panics() {
        let mut v = FastVec::new();
        v.push(1);
        v.push(33);
        v.push(-5);
        v.remove(3);
    }

    // Remove never allocates or frees. It only moves elements around inside
    // the memory the vector already has.
    #[test]
    fn remove_never_allocates_or_frees() {
        MALLOC.clear();

        let input = vec![-1, 3, -200, 25, 33];
        let mut v = FastVec::new();
        for num in input {
            v.push(num);
        }

        assert_eq!(MALLOC.state().allocations().len(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 7);

        let removes = vec![1, 3, 0, 1, 0];
        for remove in removes {
            v.remove(remove);
            assert_eq!(MALLOC.state().allocations().len(), 1);
            assert_eq!(MALLOC.state().transcript().len(), 7);
        }

        MALLOC.clear();
    }

    // The removed element is dropped, and nothing else is.
    #[test]
    fn remove_drops_exactly_one_element() {
        MALLOC.clear();

        let mut tracker = Tracker::new();
        let mut v = FastVec::from_vec(
            vec![
                tracker.track(String::from("hello")),
                tracker.track(String::from("bye")),
                tracker.track(String::from("morning")),
                tracker.track(String::from("hello again")),
            ]
        );

        assert_eq!(v.len(), 4);
        assert_eq!(v.capacity(), 4);
        assert_eq!(tracker.tracked_count(), 4);

        v.remove(1);
        assert_eq!(tracker.tracked_count(), 3);
        v.remove(2);
        assert_eq!(tracker.tracked_count(), 2);
        v.remove(0);
        assert_eq!(tracker.tracked_count(), 1);
        v.remove(0);
        assert_eq!(tracker.tracked_count(), 0);

        drop(v);

        MALLOC.clear();
    }
}

mod clear {
    use super::*;

    #[test]
    fn clear_frees_the_memory() {
        MALLOC.clear();

        let mut v = FastVec::from_vec(vec![1, 2, 3]);
        v.clear();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 0);
        assert_eq!(MALLOC.state().allocations().len(), 0);

        MALLOC.clear();
    }

    // Clear drops every element, not just the memory they sat in.
    #[test]
    fn clear_drops_every_element() {
        MALLOC.clear();

        let mut tracker = Tracker::new();
        let mut v = FastVec::from_vec(
            vec![
                tracker.track(String::from("hello")),
                tracker.track(String::from("bye")),
                tracker.track(String::from("morning")),
                tracker.track(String::from("hello again")),
            ]
        );

        assert_eq!(v.len(), 4);
        assert_eq!(v.capacity(), 4);
        assert_eq!(tracker.tracked_count(), 4);

        v.clear();
        assert!(tracker.is_empty());

        drop(v);

        MALLOC.clear();
    }
}

// What happens when a FastVec goes out of scope. Each test makes one inside
// a block, so it is dropped at the closing brace, then checks what is left.
mod dropping {
    use super::*;

    #[test]
    fn drop_frees_memory() {
        MALLOC.clear();

        {
            let mut v = FastVec::new();
            v.push(1);
            v.push(2);
            v.push(3);
            assert_eq!(MALLOC.state().allocations().len(), 1);
        }
        assert_eq!(MALLOC.state().allocations().len(), 0);

        MALLOC.clear();
    }

    #[test]
    fn drop_drops_every_element() {
        MALLOC.clear();

        let mut tracker = Tracker::new();
        {
            let mut v = FastVec::new();
            v.push(tracker.track(String::from("hello")));
            v.push(tracker.track(String::from("bye")));
            v.push(tracker.track(String::from("morning")));
            assert_eq!(tracker.tracked_count(), 3);
        }
        assert!(tracker.is_empty());

        MALLOC.clear();
    }

    // into_vec moves every element out, so the FastVec has nothing left to
    // drop and nothing left to free.
    #[test]
    fn drop_after_into_vec_has_nothing_left() {
        MALLOC.clear();

        let mut tracker = Tracker::new();
        let mut v = FastVec::new();
        v.push(tracker.track(1));
        v.push(tracker.track(2));
        let regular = v.into_vec();
        assert_eq!(tracker.tracked_count(), 2);
        assert_eq!(MALLOC.state().allocations().len(), 0);
        drop(regular);
        assert!(tracker.is_empty());

        MALLOC.clear();
    }
}

mod timing {
    use super::*;
    use fast_vec::timing::{time_fast_pushes, time_slow_pushes};

    fn check_samples(samples: &Vec<(usize, f64)>, n: usize, every: usize) {
        assert_eq!(samples.len(), n / every);
        for i in 0..samples.len() {
            assert_eq!(samples[i].0, (i + 1) * every);
            assert!(samples[i].1 >= 0.0);
            if i > 0 {
                assert!(samples[i].1 >= samples[i - 1].1, "times should never go down");
            }
        }
    }

    #[test]
    fn fast_pushes_sample_every_hundred() {
        let samples = time_fast_pushes(FastVec::new(), 1000, 100);
        check_samples(&samples, 1000, 100);
    }

    #[test]
    fn slow_pushes_sample_every_twenty() {
        let samples = time_slow_pushes(200, 20);
        check_samples(&samples, 200, 20);
    }

    #[test]
    fn fast_samples_leave_nothing_behind() {
        MALLOC.clear();
        time_fast_pushes(FastVec::new(), 1000, 100);
        assert_eq!(MALLOC.state().allocations().len(), 0);
        MALLOC.clear();
    }
}
