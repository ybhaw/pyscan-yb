# API Reference

## `scan(path)`

Scans a directory recursively for Python (`.py`) files, skipping common non-source directories and file types.

### Signature

```python
from pyscan_yb import scan

scan(path: str) -> list[str]
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `str` | Absolute or relative path to the directory to scan |

### Returns

A sorted `list[str]` of relative file paths (using forward slashes) for all `.py` files found under `path`.

Returns an empty list if no Python files are found.

### Excluded Directories

The following directories are automatically skipped during scanning:

- `__pycache__`
- `.DS_Store`
- `.egg-info` (any directory ending with `.egg-info`)
- `.tox`
- `.venv`
- `venv`
- `.mypy_cache`
- `.pytest_cache`
- `.git`
- `.hg`
- `.svn`
- `node_modules`

### Excluded File Extensions

- `.pyc`
- `.pyo`

### Examples

#### Scan the current directory

```python
from pyscan_yb import scan

files = scan(".")
for f in files:
    print(f)
```

#### Scan a specific project

```python
from pyscan_yb import scan

files = scan("/home/user/myproject")
print(f"Found {len(files)} Python files")
```

#### Filter results

```python
from pyscan_yb import scan

# Only test files
test_files = [f for f in scan(".") if f.startswith("tests/")]

# Only files in a specific package
pkg_files = [f for f in scan(".") if f.startswith("src/")]
```

### Notes

- Paths in the returned list always use forward slashes (`/`), even on Windows.
- The function is implemented in Rust for performance and exposed to Python via PyO3.
- Only files with the `.py` extension are included in results.
