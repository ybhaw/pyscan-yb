# pyscan-yb

[![PyPI version](https://img.shields.io/pypi/v/pyscan-yb)](https://pypi.org/project/pyscan-yb/)
[![CI](https://img.shields.io/github/actions/workflow/status/ybhaw/pyscan-yb/CI.yml?branch=main&label=CI)](https://github.com/ybhaw/pyscan-yb/actions)
[![Python versions](https://img.shields.io/pypi/pyversions/pyscan-yb)](https://pypi.org/project/pyscan-yb/)
[![License](https://img.shields.io/pypi/l/pyscan-yb)](https://github.com/ybhaw/pyscan-yb/blob/main/LICENSE)

A fast Python file scanner written in Rust.

Recursively finds all `.py` files in a directory, excluding system files and common non-source directories.

## Install

```bash
pip install pyscan-yb
```

For development:

```bash
pip install maturin
maturin develop
```

## Usage

### CLI

```bash
# Scan current directory
pyscan-yb

# Scan a specific path
pyscan-yb /path/to/project

# Quiet mode (paths only)
pyscan-yb -q /path/to/project
```

### Python API

```python
from pyscan_yb import scan

files = scan(".")
for f in files:
    print(f)
```

## Excluded Directories

`__pycache__`, `.git`, `.hg`, `.svn`, `.venv`, `venv`, `.tox`, `.mypy_cache`, `.pytest_cache`, `.egg-info`, `node_modules`

## Documentation

- [Wiki](https://github.com/ybhaw/pyscan-yb/wiki) — API reference, CLI usage, and more
- [Docs Site](https://ybhaw.github.io/pyscan-yb/) — Full documentation on GitHub Pages

## Contributing

See the [Contributing Guide](https://github.com/ybhaw/pyscan-yb/wiki/Contributing) for dev setup, testing, and PR guidelines.

## License

MIT
