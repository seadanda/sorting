# Sorting algorithm comparisons

An exercise to build various sorting algos from scratch and benchmark and compare under a range of conditions.

## Scope

I'll start off with a short list from [Toptal](https://www.toptal.com/developers/sorting-algorithms):

- Insertion
- Selection
- Bubble
- Shell
- Merge
- Heap
- Quick

I'd like to implement them all naively first from first principles and then work on optimising to improve performance and memory usage and explore any tradeoffs. Then I'll look at their implementation in crates etc and see what I've missed out on.

I'll benchmark with [Criterion](https://crates.io/crates/criterion). Benchmarks can be run using `cargo bench`.

## TODO

Any tasks I'm pushing to the end. Currently these are queued after the naive first-pass write of all algos:

- [ ] Increase random dataset size.
- [ ] Add data generator module for shared code between tests and benches.
