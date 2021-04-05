# rustyaml

 🚧🚧🚧

This is an experimental rust-powered python extension to parse YAML using rust's `serde_yaml` and `rust-yaml` crates.

Code is heavily inspired by https://github.com/mre/hyperjson/ and https://github.com/samuelcolvin/rtoml.

One `#TODO` is to expand the test suite to ensure the YAML is being parsed according to spec (or at least `pyyaml`'s implementation).

**Note:** Direct Python object support a-la `pyyaml` is a non-goal with this library.

## Known Issues

* Aliases are not recognized and supported via `dumps`. The object will be printed twice in its entirety. This could probably be fixed with some FFI trickery to check the references of the python objects from rust.
* Objects are not gauranteed to roundtrip on the first iteration due to some formatting getting automatically applied by `serde-yaml` (although they will roundtrip in subsequent iterations)

## Install

Cross platform wheels are distributed on pypi, to install:

```sh
pip install rustyaml
```

## Benchmarks

```sh
poetry run pytest tests/benchmark.py
```

```
----------------------------------------------------------------------------------- benchmark: 4 tests -----------------------------------------------------------------------------------
Name (time in us)         Min                Max               Mean            StdDev             Median               IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_rust_dumps        3.9600 (1.0)      19.5400 (1.54)      4.4272 (1.0)      0.4513 (1.45)      4.3500 (1.0)      0.0900 (1.0)     1799;3994      225.8787 (1.0)       50378           1
test_rust_loads        4.1500 (1.05)     12.7000 (1.0)       4.5778 (1.03)     0.3117 (1.0)       4.5600 (1.05)     0.0900 (1.0)       412;862      218.4474 (0.97)      22630           1
test_python_load      28.3200 (7.15)     58.1000 (4.57)     31.5107 (7.12)     2.8282 (9.07)     30.8200 (7.09)     0.8700 (9.67)     669;1078       31.7353 (0.14)       8668           1
test_python_dump      29.8700 (7.54)     61.8600 (4.87)     32.8807 (7.43)     2.5988 (8.34)     32.1400 (7.39)     1.1200 (12.44)    820;1114       30.4129 (0.13)      11467           1
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
```

## Develop

1) Install `rustup` and `poetry`. Ensure that `cargo` is somewhere on your path.

2) `poetry install` will automatically create the venv, compile the package and install it into the venv via the build script.
