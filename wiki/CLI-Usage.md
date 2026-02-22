# CLI Usage

## Command

```bash
pyscan-yb [OPTIONS] [PATH]
```

## Arguments

| Argument | Required | Default | Description |
|----------|----------|---------|-------------|
| `PATH` | No | `.` (current directory) | Directory to scan for Python files |

## Options

| Flag | Long | Description |
|------|------|-------------|
| `-q` | `--quiet` | Quiet mode: only print file paths, suppress informational messages |
| `-h` | `--help` | Show help message and exit |

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Python files were found and printed |
| `1` | No Python files found in the specified directory |

## Examples

### Scan the current directory

```bash
pyscan-yb
```

### Scan a specific path

```bash
pyscan-yb /path/to/project
```

### Quiet mode

Suppresses the "No Python files found" message when no files are found. Useful in scripts.

```bash
pyscan-yb -q /path/to/project
```

### Use in a shell pipeline

```bash
# Count Python files
pyscan-yb /path/to/project | wc -l

# Feed to another tool
pyscan-yb . | xargs wc -l

# Check if a project has Python files
if pyscan-yb -q /path/to/project > /dev/null 2>&1; then
    echo "Python files found"
fi
```
