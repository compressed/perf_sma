Rust:

```sh
~/D/perf_sma ❯❯❯ rustc sma.rs -O --test && ./sma --bench

running 1 test
test bench_sma ... bench:  75266923 ns/iter (+/- 49626127)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured
```

Julia:

```sh
~/D/perf_sma ❯❯❯ julia sma.jl
elapsed time: 0.004765301 seconds (2800264 bytes allocated)
elapsed time: 0.009514948 seconds (2800048 bytes allocated)
elapsed time: 0.022068369 seconds (2800048 bytes allocated)
elapsed time: 0.021218931 seconds (2800048 bytes allocated)
elapsed time: 0.084004372 seconds (2800048 bytes allocated)
```
