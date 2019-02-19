Implementing the N-body program in Rust (with a little bit of EC(S) thrown in).

This is almost a direct translation from an assignment in a Parallel Systems
course I took algorithm-wise.
The interesting part of that assignment is that we had to use UPC—the
interesting parts of this implementation are:
* it's in Rust,
* it's a little optimized, and
* it has the data layout of an EC(S) implementation.

Documentation: clone and run `cargo doc`

Benchmark: clone and run `cargo bench`. On my desktop computer with an AMD Ryzen 5 2400g processor, the result is:
```
run simulation          time:   [147.24 ms 147.54 ms 147.87 ms]
```
which is slightly better (2%) than the `update` branch.
