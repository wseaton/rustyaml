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
test_rust_dumps        5.7510 (1.0)       41.9760 (1.08)      6.0786 (1.0)      1.2679 (1.08)      5.9240 (1.0)      0.0710 (1.0)      810;2916      164.5115 (1.0)       32602           1
test_rust_loads        6.3290 (1.10)      38.8540 (1.0)       6.9765 (1.15)     1.1720 (1.0)       6.6220 (1.12)     0.7585 (10.68)     578;468      143.3390 (0.87)      13820           1
test_python_load      40.3580 (7.02)     105.9330 (2.73)     42.4503 (6.98)     3.9638 (3.38)     41.7440 (7.05)     0.5960 (8.39)      178;699       23.5570 (0.14)       6394           1
test_python_dump      44.1710 (7.68)     108.8190 (2.80)     45.9539 (7.56)     2.8028 (2.39)     45.5050 (7.68)     0.5975 (8.42)      294;780       21.7609 (0.13)       8349           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```




## Develop

1) Install `rustup` and `poetry`. Ensure that `cargo` is somewhere on your path.

2) `poetry install` will automatically create the venv, compile the package and install it into the venv via the build script.
