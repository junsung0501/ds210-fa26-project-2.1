# Project 2: Build Your Own Vec

**Author:** _your name here_



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

### 2. Why push is fast on average

### 3. Your growth experiments

### 4. Why `Add` eventually loses

### 5. Two kinds of leak

### 6. The bug in `clear`

### 7. Something that failed at first, and how you adapted

### AI citation
