from typing import Any, Dict

from setuptools_rust import RustExtension, Binding


def build(setup_kwargs: Dict[str, Any]) -> None:
    setup_kwargs.update(
        {
            "rust_extensions": [RustExtension("rustyaml._rustyaml", "Cargo.toml", debug=False, binding=Binding.PyO3)],
            "zip_safe": False
        }
    )