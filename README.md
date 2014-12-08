Note: all gains seem to from turning off bounds checking in julia

# Rust

```sh
~/D/perf_sma git:master ❯❯❯ rustc -v                                ✱ ◼
rustc 0.13.0-nightly (a31ad6bfc 2014-12-06 23:12:17 +0000)
~/D/perf_sma ❯❯❯ rustc sma.rs -O --test && ./sma --bench

running 1 test
test bench_sma_100 ... bench:  82935096 ns/iter (+/- 11693865)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured
```

# Julia

Compare with 100 day above.

```sh
~/D/perf_sma ❯❯❯ julia -v
julia version 0.4.0-dev+1756
~/D/perf_sma ❯❯❯ julia sma.jl
elapsed time: 0.004547032 seconds (2800264 bytes allocated)
sma_original = 3
elapsed time: 0.010534306 seconds (2800048 bytes allocated)
sma_original = 14
elapsed time: 0.027884941 seconds (2800048 bytes allocated)
sma_original = 100
elapsed time: 0.09842968 seconds (2800048 bytes allocated)
sma_original = 600
elapsed time: 0.545872561 seconds (2800048 bytes allocated)
sma_improved = 100
elapsed time: 0.020131132 seconds (2800048 bytes allocated)
```
