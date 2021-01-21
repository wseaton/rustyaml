from benchmark import YAML, DATA
from rustyaml import loads, dumps


def test_loads():
    res = loads(YAML)

    assert isinstance(res, list)
    assert res == [{'name': 'Mark McGwire',
  'accomplishment': 'Mark set a major league home run record in 1998.\n',
  'stats': '65 Home Runs\n0.278 Batting Average\n',
  'floats': [0.56]}]


def test_dumps():
    res = dumps(DATA)
    assert isinstance(res, str)
    assert res == '---\nHey: Man\n"1.0":\n  - a\n  - 1\n"3":\n  foo: bar'