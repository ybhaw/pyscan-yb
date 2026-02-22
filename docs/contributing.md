# Contributing

Thank you for your interest in contributing to pyscan-yb!

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- Python 3.9+
- [maturin](https://www.maturin.rs/) (`pip install maturin`)

## Dev Setup

1. Clone the repository:

```bash
git clone https://github.com/ybhaw/pyscan-yb.git
cd pyscan-yb
```

2. Create a virtual environment:

```bash
python -m venv .venv
source .venv/bin/activate  # Linux/macOS
# .venv\Scripts\activate   # Windows
```

3. Install in development mode:

```bash
pip install maturin
maturin develop
```

## Running Tests

### Rust tests

```bash
cargo test
```

### Python tests

```bash
pip install pytest
pytest
```

## Code Style

This project uses the following tools for code quality:

| Tool | Purpose | Config |
|------|---------|--------|
| `cargo fmt` | Rust formatting | Default settings |
| `cargo clippy` | Rust linting | Default settings |
| `black` | Python formatting | `target-version = ["py39"]` |
| `isort` | Import sorting | `profile = "black"` |
| `ruff` | Python linting | `line-length = 88`, `target-version = "py39"` |
| `mypy` | Type checking | `python_version = "3.9"` |

Run all checks before submitting a PR:

```bash
cargo fmt --check
cargo clippy
black --check python/
isort --check python/
ruff check python/
```

## Pull Request Guidelines

1. Fork the repository and create a feature branch from `main`.
2. Make your changes with clear, focused commits.
3. Add tests for new functionality.
4. Ensure all tests pass (`cargo test` and `pytest`).
5. Run code style checks (see above).
6. Open a pull request against `main` with a clear description of what and why.

## Project Structure

```
pyscan-yb/
├── src/lib.rs              # Rust core: scan logic, skip lists
├── python/pyscan_yb/
│   ├── __init__.py         # Python package, re-exports scan()
│   └── cli.py              # CLI entry point
├── pyproject.toml           # Build config (maturin)
├── Cargo.toml               # Rust dependencies
└── tests/                   # Python tests
```
