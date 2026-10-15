// Timing pushes, for the benchmark. Both functions are yours (checkpoint 3).
//
// Each one pushes `n` elements, one at a time, into a vector it is given or makes,
// and every `every` pushes records a sample:
//
//     (how many elements have been pushed so far, milliseconds since the first push)
//
// So with n = 100 and every = 10 you get 10 samples, the first at 10 elements and
// the last at 100. If n is not a multiple of every, the leftover pushes at the end
// still happen but produce no sample: n = 105 gives the same 10 samples. The times
// only ever go up, because each one is measured from the same starting point.
//
// std::time::Instant is the stopwatch. `Instant::now()` starts it, and
// `start.elapsed().as_secs_f64() * 1000.0` reads it in milliseconds.

use std::time::Instant;

use slow_vec::SlowVec;

use crate::FastVec;

pub fn time_fast_pushes(mut fast_vec: FastVec<u64>, n: usize, every: usize) -> Vec<(usize, f64)> {
    todo!("time_fast_pushes")
}

pub fn time_slow_pushes(n: usize, every: usize) -> Vec<(usize, f64)> {
    todo!("time_slow_pushes")
}
