# Project 2: Build Your Own Vec

**Author:** John Lee



### Getting started

You will build Rust's `Vec` yourself, twice. First a slow version that is safe and simple, then a fast one that manages its own memory with raw pointers. Then you will time the versions against each other.

Each vector has a playground you can run and edit freely. Run these from this top folder:

```bash
cargo run --bin slow_playground   # try out SlowVec and FixedSizeArray
cargo run --bin slow_memory       # watch what gets dropped, and when
cargo run --bin fast_playground   # try out FastVec, MALLOC, and raw pointers
cargo run --bin fast_memory       # watch what gets dropped and freed, and when
```

All four crash with `not yet implemented` until you write the functions they call.

### The folders

```
slow_vec/    Part 1. Your SlowVec goes in src/lib.rs
fast_vec/    Part 2. Your FastVec goes in src/lib.rs, plus src/growth.rs and src/timing.rs
             src/benchmark.rs runs the benchmark and draws the plots
fixed/       FixedSizeArray, which SlowVec is built on. Read it if you are curious
malloc/      the memory allocator FastVec uses. It records every allocation and free
tracker/     wraps values so you can see which ones have been dropped
```

**Change only what the handout asks for**: the functions marked `todo!`, the `Drop` for `FastVec`, your choices in `benchmark.rs`, and the writeup at the bottom of this file. Leave everything else alone.

### About the warnings

A fresh clone builds with a lot of warnings, all of them some version of "you declared this and never used it." That is expected, since you haven't written the code yet.

### Running your tests

There is one test file per checkpoint. Run them from this top folder:

```bash
cargo test --test cp1     # SlowVec
cargo test --test cp2     # FastVec's get and push, and growth
cargo test --test cp3     # remove, clear, Drop, and timing
cargo test                # all of them
```

Add part of a test's name to run just that one: `cargo test --test cp2 push_memory`.

**If a run stops partway with `SIGSEGV`, `SIGBUS`, `SIGABRT`, or some other signal**, one of your `unsafe` blocks has corrupted memory, and it took the whole test run down with it. The last test named before the crash is the place to start. Try running it on its own to diagnose the problem.

### The benchmark

```bash
cargo run --release --bin benchmark
```

Remember, the `--release` matters: without it the timings are slow and misleading. This writes `benchmark_slow.png` and `benchmark_growth.png` into the top folder, which you'll need for the writeup. Commit both: they have to be on `main` when you submit.

## Your writeup

Answer the questions from the project handout here, under the headings below. Leave the headings where they are, and leave everything above this line alone. Answers should be about 2-3 sentences per question unless the handout says otherwise.

### 1. SlowVec against FastVec

SlowVec's curve bends upward and reaches about 800 ms at 5,000 pushes, while FastVec's curve stays almost flat near zero. SlowVec bends because every push makes a new array that is one slot bigger and moves all the old elements into it, so the k-th push moves k - 1 elements and writes one more. That adds up to 1 + 2 + ... + n = n(n + 1)/2 steps for n pushes, which is about n^2 / 2.

### 2. Why push is fast on average

For 16 pushes with Double, the resizes happen at pushes 2, 3, 5 and 9 and cost 2, 3, 5 and 9 steps, and the other 12 pushes cost 1 step each, so the total is 19 + 12 = 31 steps, compared with 136 for SlowVec. That is only about 2 steps per push, because my `push` only resizes when `len` reaches `capacity` and Double makes the new block twice as big. The resizes copy 1, 2, 4, 8, ... elements, which adds up to less than n, so the average cost per push stays about the same however big n gets.

### 3. Your growth experiments

On `growth-multiply` I tried Multiply 1.5, Multiply 3 and Multiply 1.1 against Double, and on `growth-add` I tried Add 100, Add 1000 and Add 10000 against Double, both with 1,000,000 pushes. The Multiply lines all stay roughly straight, with Multiply 3 lowest and Multiply 1.1 highest because it resizes the most, but the Add lines curve upward and Add 100 shoots far above everything else. I merged `growth-add` because it is the branch I submitted with a perfect score, and the strategies also differ in memory: right after a resize, a Multiply(f) vector has 1 - 1/f of its slots empty, so Double wastes about half and Multiply 3 about two thirds, while Add(k) never has more than k empty.

### 4. Why `Add` eventually loses

Add(k) resizes every k pushes and copies everything each time, so n pushes cost about n^2 / (2k) steps, while Double costs only about 2n. A bigger k makes Add's curve lower because k is on the bottom of the fraction, so a big k can win at one particular n, but the curve still bends upward like SlowVec's, and a curve that bends upward always ends up above Double's straight line. On my plot the Add 10000 line is already above Double at 1,000,000 pushes.

### 5. Two kinds of leak

`Tracker` catches elements that were never dropped, such as a String whose letters never get freed, and `MALLOC`'s log catches a block that was allocated but never freed, such as the vector's own memory, so each one misses the other's problem. The old `clear` passed the `MALLOC` check but failed the `Tracker` check because it freed the block without dropping the elements, and a `Drop` that drops every element but forgets to free the block would do the opposite. My `Drop` just calls `clear`, which drops every element and then frees the block, so it passes both.

### 6. The bug in `clear`

The starter `clear` freed the block of memory but never dropped the elements inside it, so anything they owned, like a String's letters, leaked. I saw it when `clear_drops_every_element` failed with `tracker.is_empty()` even though the memory was freed, and I fixed it by moving each element out with `ptr::read` and dropping it before freeing the block. SlowVec can't have this bug because its `clear` just replaces its `FixedSizeArray`, and the old array's `Drop` already drops every element and frees the block, but FastVec only holds a raw pointer, so Rust doesn't clean up for me.

### 7. Something that failed at first, and how you adapted

I submitted Checkpoint 3 early just to check my code, and Gradescope failed 10 tests, covering `remove`, `clear`, `Drop` and the timing functions, even though all 41 tests passed on my laptop. The errors said `not yet implemented`, which is what `todo!()` prints, and `clear_drops_every_element` failed with `tracker.is_empty()`, so the grader was running the starter code and not mine. I learned that Gradescope only grades the branch I point it at, and my new code was only on `checkpoint-3` and my growth branches, so I resubmitted from `growth-add` and got a perfect score.

### AI citation

I used Claude as a coding and study assistant. It explained the checkpoints and the unsafe pointer code, helped me write the code for FastVec's `get`, `push`, `next_capacity`, `remove`, `clear`, `Drop` and the timing functions plus the growth settings in `benchmark.rs`.
