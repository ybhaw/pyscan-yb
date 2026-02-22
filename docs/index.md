# pyscan-yb

A fast Python file scanner written in Rust.

Recursively finds all `.py` files in a directory, automatically excluding system files and common non-source directories.

## Quick Install

```bash
pip install pyscan-yb
```

## Quick Start

### CLI

```bash
pyscan-yb /path/to/project
```

### Python API

```python
from pyscan_yb import scan

files = scan(".")
for f in files:
    print(f)
```

## Learn More

- [API Reference](api.md) — `scan()` function details, parameters, return type, examples
- [CLI Usage](cli.md) — Command-line flags, exit codes, examples
- [Contributing](contributing.md) — Dev setup, testing, code style, PR guidelines

## Links

- [PyPI Package](https://pypi.org/project/pyscan-yb/)
- [GitHub Repository](https://github.com/ybhaw/pyscan-yb)
- [Issue Tracker](https://github.com/ybhaw/pyscan-yb/issues)
