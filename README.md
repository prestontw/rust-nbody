Rust version of upc program in Parallel Systems class.

Criterion results (from running against no par on compute forces ec):
```
run simulation          time:   [179.28 ms 179.85 ms 180.49 ms]
                        change: [+19.879% +20.743% +21.513%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  9 (9.00%) high severe
```
