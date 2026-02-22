# pyscan-yb

A fast Python file scanner written in Rust.

Recursively finds all `.py` files in a directory, excluding system files and common non-source directories.

## Install

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

## License

MIT
