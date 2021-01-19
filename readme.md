# rustyaml

This is an experimental rust-powered python extension to parse YAML using rust's `serde_yaml` and `rust-yaml` crates.

Code is heavily inspired by https://github.com/mre/hyperjson/ and https://github.com/samuelcolvin/rtoml.

## Benchmarks

```sh
poetry run pytest tests/benchmark.py 
```

```
------------------------------------------------------------------------------------- benchmark: 4 tests ------------------------------------------------------------------------------------
Name (time in us)         Min                 Max               Mean             StdDev             Median                IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_rust_dumps        6.4650 (1.0)       60.2810 (1.15)      7.8765 (1.0)       3.0938 (1.0)       7.1150 (1.0)       0.6750 (3.41)    2399;2677      126.9592 (1.0)       35121           1
test_rust_loads        7.7380 (1.20)      52.4300 (1.0)       9.0518 (1.15)      3.7701 (1.22)      8.1540 (1.15)      0.1980 (1.0)      721;1158      110.4749 (0.87)      13852           1
test_python_load      55.8370 (8.64)     226.2910 (4.32)     66.1816 (8.40)     15.8740 (5.13)     60.7450 (8.54)      8.5375 (43.12)     466;483       15.1099 (0.12)       4811           1
test_python_dump      61.0590 (9.44)     226.9110 (4.33)     75.1551 (9.54)     20.4618 (6.61)     64.7145 (9.10)     17.3575 (87.67)     730;455       13.3058 (0.10)       5056           1
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```




## Develop

1) Install `rustup` and `poetry`.

2) `poetry install` will automatically create the venv, compile the package and install it into the venv via the build script.
