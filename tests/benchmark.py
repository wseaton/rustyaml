from rustyaml import loads, dumps

import yaml
from yaml import CLoader as Loader, CDumper as Dumper

YAML = """
name: Mark McGwire
accomplishment: >
  Mark set a major league
  home run record in 1998.
stats: |
  65 Home Runs
  0.278 Batting Average
floats:
  - 0.56
"""

DATA = {"Hey": "Man", 1.0: ["a", 1], 3: {"foo": "bar"}}


def bench_rust_loads():
    return loads(YAML)


def bench_rust_dumps():
    return dumps(DATA)


def test_rust_loads(benchmark):
    benchmark(bench_rust_loads)


def test_rust_dumps(benchmark):
    benchmark(bench_rust_dumps)


def bench_python_load():
    return yaml.load(YAML, Loader=Loader)


def bench_python_dump():
    return yaml.dump(DATA, Dumper=Dumper)


def test_python_load(benchmark):
    benchmark(bench_python_load)


def test_python_dump(benchmark):
    benchmark(bench_python_dump)
