from rspyaml import parse_yaml


def test_parse():

    yaml = """
name: Mark McGwire
accomplishment: >
  Mark set a major league
  home run record in 1998.
stats: |
  65 Home Runs
  0.278 Batting Average"""

    parse_yaml(yaml)