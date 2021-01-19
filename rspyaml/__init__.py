from . import _rspyaml
from io import TextIOBase
from pathlib import Path
from typing import Any, TextIO, Union

__all__ = ["loads", "load", "dump", "dumps", "VERSION"]

VERSION = _rspyaml.VERSION
loads = _rspyaml.loads
dumps = _rspyaml.dumps


def load(yaml: Union[str, Path, TextIO]) -> Any:
    """
    Parse YAML via a string or file and return a python object. The `yaml` argument may be a `str`,
    `Path` or file object from `open()`.
    """
    if isinstance(yaml, Path):
        yaml = yaml.read_text()
    elif isinstance(yaml, (TextIOBase, TextIO)):
        yaml = yaml.read()

    return loads(yaml)


def dump(obj: Any, file: Union[Path, TextIO]) -> int:
    """
    Serialize a python object to YAML and write it to a file. `file` may be a `Path` or file object from `open()`.
    """
    s = dumps(obj)
    if isinstance(file, Path):
        return file.write_text(s)
    else:
        return file.write(s)
