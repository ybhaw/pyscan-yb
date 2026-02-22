# Project Plan: pyscan

> A fast Python file scanner written in Rust, importable as a Python library and usable as a CLI. Configurable via `pyproject.toml`.

## What it does

Given a folder:
- Finds all Python files (`.py`) recursively
- Respects `.gitignore` by default
- Reads config from `pyproject.toml` `[tool.pyscan]` section
- Supports whitelist/blacklist for directories
- Supports named scan profiles (e.g., "source only", "tests only", "ci")
- Ignores system files by default (`__pycache__`, `.DS_Store`, `.pyc`, `.egg-info`, `.tox`, `.venv`, `.mypy_cache`)
- Output formats: flat list (default), tree, colored, JSON

## Value Proposition

*"fd-like speed, but as a Python library you can `import` and configure via `pyproject.toml`."*

Key differentiators vs `find`, `fd`, `git ls-files`:
- **Python-native API** -- `from pyscan import scan; files = scan("src")`
- **pyproject.toml-aware** -- respects project-level include/exclude config where Python tools live
- **Profile system** -- named configurations for different scan contexts (linting, testing, CI)
- **Zero Python dependencies** -- Rust handles everything natively

## Target Audience

1. Python tool/framework authors who need fast file discovery as a library dependency
2. Developers in large monorepos needing fast, filtered Python file listing
3. CI/CD pipelines discovering test files, lint targets, etc.

## Tech Stack

- **Language:** Rust core with Python bindings
- **Build system:** maturin + PyO3
- **Core traversal:** `ignore` crate (from ripgrep) -- handles parallel walks, gitignore, symlink detection, cross-platform paths
- **TOML parsing:** `toml` crate + serde
- **CLI:** Python-side `cli.py` (argparse) calling into Rust functions
- **Output:** Rust-native color (`colored` crate) and tree formatting
- **Dependencies:** Zero Python runtime deps -- everything compiled into the Rust extension

### Architecture Decision: No pip extras

Since Rust handles gitignore (`ignore` crate), color (`colored` crate), TOML parsing (`toml` crate), and tree formatting natively, ALL features are compiled into a single wheel. No need for `[git]`, `[color]`, `[all]` extras.

`pip install pyscan` gives you everything.

## Project Layout

```
pyscan/
  Cargo.toml
  pyproject.toml
  src/
    lib.rs          # PyO3 module entry point
    scanner.rs      # Core traversal logic
    config.rs       # pyproject.toml parsing
    output.rs       # Output formatting (flat, tree, color, JSON)
  python/
    pyscan/
      __init__.py   # Thin wrapper re-exporting Rust module
      cli.py        # CLI entry point (argparse)
  tests/
    fixtures/       # Test directory structures
    test_scan.py    # Python integration tests
```

## Configuration (pyproject.toml)

```toml
[tool.pyscan]
default_profile = "standard"

[tool.pyscan.profiles.standard]
include = ["src", "lib"]
exclude = ["tests", "docs"]
respect_gitignore = true
follow_symlinks = false

[tool.pyscan.profiles.full]
include = []
exclude = []
respect_gitignore = false
follow_symlinks = true

[tool.pyscan.profiles.ci]
include = ["src"]
exclude = ["tests/fixtures"]
respect_gitignore = true
```

Config loading order (highest priority last):
1. Built-in defaults
2. `pyproject.toml` `[tool.pyscan]`
3. CLI flags

## CLI Design

```
pyscan [OPTIONS] [PATH]

Arguments:
  PATH                    Directory to scan [default: .]

Options:
  -p, --profile NAME      Profile to use [default: from pyproject.toml or "default"]
  --include GLOB          Additional include patterns
  --exclude GLOB          Additional exclude patterns
  --no-gitignore          Disable .gitignore respect
  --follow-symlinks       Follow symbolic links
  --color / --no-color    Enable/disable colored output (auto-detects TTY)
  --tree                  Display results as tree
  --json                  Output as JSON
  -q, --quiet             Only print file paths, one per line
  -v, --verbose           Show which rules are applied, which dirs skipped
```

Defaults:
- Respects `.gitignore` (opt out with `--no-gitignore`)
- Auto-detects TTY for color (no color when piped)
- Relative paths, sorted alphabetically, one per line
- Exit code 0 if files found, 1 if none found

## Python API

```python
from pyscan import scan

# Simple
files = scan(".")

# With profile
files = scan(".", profile="ci")

# With overrides
files = scan(".", include=["src"], exclude=["tests"], respect_gitignore=True)

# Full result object
result = scan(".", full_result=True)
result.files        # List[FileEntry]
result.root         # Path
result.duration_ms  # int
```

## Phased Delivery Plan

### Phase 1 -- Foundation (v0.1.0) ~2-3 weekends
- Rust core: recursive walk with `ignore` crate, `.py` filtering, system file exclusion
- PyO3 bindings: `scan(path) -> list[str]`
- CLI entry point: `pyscan <path>`
- maturin build + wheel publishing
- Unit tests (Rust) + integration tests (Python)
- CI pipeline (GitHub Actions): lint, test, build wheels (Linux/macOS/Windows)
- README, LICENSE, pyproject.toml
- Publish to PyPI

**Milestone:** `pip install pyscan && pyscan .` works.

### Phase 2 -- Configuration (v0.2.0) ~1 weekend
- `pyproject.toml` `[tool.pyscan]` config reading
- Whitelist/blacklist folder support
- `--include` / `--exclude` CLI flags

**Milestone:** Users customize scan behavior via config or flags.

### Phase 3 -- Git Integration (v0.3.0) ~1 weekend
- `.gitignore` parsing via `ignore` crate (already compiled in)
- Respect gitignore by default, `--no-gitignore` to opt out
- Nested `.gitignore`, global gitignore, `.git/info/exclude`

**Milestone:** Scanner seamlessly skips gitignored paths.

### Phase 4 -- Output Formatting (v0.4.0) ~1 weekend
- `--tree` flag for tree-view output
- `--color` with TTY auto-detection
- `--json` for programmatic consumption
- Color-differentiate test files, `__init__.py`, regular modules

**Milestone:** Polished developer-facing output.

### Phase 5 -- Profiles (v0.5.0) ~1-2 weekends
- Named profiles in `pyproject.toml`
- `--profile <name>` CLI flag
- `pyscan --list-profiles` to discover available profiles

**Milestone:** Teams share standardized scan configurations.

### Phase 6 -- Polish and 1.0 (v1.0.0) ~2-3 weekends
- Comprehensive docs (mkdocs-material)
- Performance benchmarks vs alternatives (pathlib, find, fd)
- Shell completions (bash/zsh/fish)
- `pyscan info` diagnostic command
- Stable public API commitment

**Total: ~8-12 weekends to v1.0**

## Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Wheel building complexity | HIGH | Use maturin-action + cibuildwheel from day one |
| "Why not use fd?" | HIGH | Differentiate on Python API + pyproject.toml awareness |
| Scope creep before v0.1 | HIGH | Ship Phase 1 first, everything else is v0.2+ |
| Rust+Python maintenance burden | MEDIUM | Keep Rust core minimal and focused |
| Windows path handling | MEDIUM | Use `std::path::Path` + test on Windows in CI |
| Solo maintainer burnout | MEDIUM | Keep scope tight, say no to feature creep |

## Testing Strategy

- **Rust unit tests:** `cargo test` -- file traversal, filtering, config parsing (~30 test cases)
- **Python integration tests:** `pytest` -- bridge tests, API contract, CLI output (~30 test cases)
- **Benchmarks:** `criterion` (Rust) + `pytest-benchmark` (Python) -- prove the Rust advantage
- **Gitignore validation:** Compare results against `git ls-files` on identical fixtures
- **CI matrix:** 3 OS x 5 Python versions = 15 jobs + benchmarks on Linux
- **Edge cases:** empty dirs, permission denied, circular symlinks, unicode filenames, 200-level nesting, race conditions
- **Test fixtures:** `simple/`, `nested/`, `with_gitignore/`, `with_pyproject/`, `symlinks/`, `empty/`, `unicode_names/`, `system_files/`, `huge/` (generated at test time)

## Naming

Package name: **pyscan** (check PyPI availability; fallbacks: `pyfind`, `pyfiles`, `pyff`)
CLI command: **pyscan**

## Success Metrics

| Timeframe | Metric | Target |
|---|---|---|
| 1 month | PyPI downloads | 500+ |
| 3 months | GitHub stars | 50+ |
| 6 months | Monthly downloads | 2,000+ |
| 6 months | Dependent packages on PyPI | 1+ |
