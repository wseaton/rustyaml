# rustyaml

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/wseaton/rustyaml/CI) ![PyPI - Python Version](https://img.shields.io/pypi/pyversions/rustyaml)


This is an experimental rust-powered python extension to parse YAML using rust's `serde_yaml` and `rust-yaml` crates.

Code is heavily inspired by https://github.com/mre/hyperjson/ and https://github.com/samuelcolvin/rtoml.

One `#TODO` is to expand the test suite to ensure the YAML is being parsed according to spec (or at least `pyyaml`'s implementation).

**Note:** Direct Python object support a-la `pyyaml` is a non-goal with this library.

## Known Issues

* Aliases are not recognized and supported via `dumps`. The object will be printed twice in its entirety. This could probably be fixed with some FFI trickery to check the references of the python objects from rust.



## Install

Cross platform wheels are distributed on pypi, to install:

```
pip install rustyaml
```

## Benchmarks

```sh
poetry run pytest tests/benchmark.py
```

```
------------------------------------------------------------------------------------ benchmark: 4 tests -----------------------------------------------------------------------------------
Name (time in us)         Min                 Max               Mean            StdDev             Median               IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_rust_dumps        5.1400 (1.0)       61.6540 (2.63)      5.6480 (1.0)      1.3588 (1.0)       5.4510 (1.0)      0.1480 (1.0)     1559;3169      177.0538 (1.0)       40621           1
test_rust_loads        6.5540 (1.28)      23.4780 (1.0)       7.6907 (1.36)     1.8920 (1.39)      6.8380 (1.25)     0.2360 (1.59)    2170;2331      130.0278 (0.73)      11757           1
test_python_load      42.3380 (8.24)      93.7420 (3.99)     49.7293 (8.80)     9.7517 (7.18)     47.5965 (8.73)     5.1035 (34.48)       12;14       20.1089 (0.11)        128           1
test_python_dump      43.7480 (8.51)     105.9500 (4.51)     46.1846 (8.18)     3.9249 (2.89)     45.2280 (8.30)     0.7410 (5.01)     427;1150       21.6522 (0.12)       7538           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```




## Develop

1) Install `rustup` and `poetry`. Ensure that `cargo` is somewhere on your path.

2) `poetry install` will automatically create the venv, compile the package and install it into the venv via the build script.
