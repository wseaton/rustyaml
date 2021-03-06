# -*- coding: utf-8 -*-
from setuptools import setup

packages = \
['rustyaml']

package_data = \
{'': ['*']}

setup_kwargs = {
    'name': 'rustyaml',
    'version': '0.1.1',
    'description': 'Python bindings for rust-yaml',
    'long_description': open('readme.md').read(),
    'long_description_content_type': 'text/markdown',
    'author': 'Will Eaton',
    'author_email': 'me@wseaton.com',
    'maintainer': None,
    'maintainer_email': None,
    'url': None,
    'packages': packages,
    'package_data': package_data,
    'python_requires': '>=3.6.0,<4.0',
}
from build import *
build(setup_kwargs)

setup(**setup_kwargs)
