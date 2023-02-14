# scs-rust-bindings
Rust bindings for https://github.com/cvxgrp/scs

## Check if it works
This assumes the library has been installed in `/usr/local/lib`

Tests are defined in `tests/mod.rs`
```
cargo build
cargo test
```

The output should be something like this:

```
running 6 tests
test bindgen_test_layout_ScsCone ... ok
test bindgen_test_layout_ScsData ... ok
test bindgen_test_layout_ScsInfo ... ok
test bindgen_test_layout_ScsMatrix ... ok
test bindgen_test_layout_ScsSettings ... ok
test bindgen_test_layout_ScsSolution ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/mod.rs (target/debug/deps/mod-1182fb08a073599c)

running 4 tests
------------------------------------------------------------------
               SCS v3.2.2 - Splitting Conic Solver
        (c) Brendan O'Donoghue, Stanford University, 2012
------------------------------------------------------------------
problem:  variables n: 2, constraints m: 3
cones:    z: primal zero / dual free vars: 1
          l: linear vars: 2
settings: eps_abs: 1.0e-09, eps_rel: 1.0e-09, eps_infeas: 1.0e-07
          alpha: 1.50, scale: 1.00e-01, adaptive_scale: 1
          max_iters: 100000, normalize: 1, rho_x: 1.00e-06
          acceleration_lookback: 10, acceleration_interval: 10
lin-sys:  sparse-direct-amd-qdldl
          nnz(A): 4, nnz(P): 3
------------------------------------------------------------------
 iter | pri res | dua res |   gap   |   obj   |  scale  | time (s)
------------------------------------------------------------------
     0| 1.51e+00  1.00e+00  4.91e+00  1.14e+00  1.00e-01  1.39e-04 
    75| 4.09e-10  7.93e-10  1.07e-09  1.24e+00  1.00e-01  6.03e-04 
------------------------------------------------------------------
status:  solved
timings: total: 6.10e-04s = setup: 1.13e-04s + solve: 4.97e-04s
         lin-sys: 1.13e-05s, cones: 5.52e-06s, accel: 4.17e-04s
------------------------------------------------------------------
objective = 1.235000
------------------------------------------------------------------
test c_example ... ok
test scs_flags ... ok
test scs_matrix_struct ... ok
test basic_alloc ... ok
```