use pyo3::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

const SKIP_NAMES: &[&str] = &[
    "__pycache__",
    ".DS_Store",
    ".egg-info",
    ".tox",
    ".venv",
    "venv",
    ".mypy_cache",
    ".pytest_cache",
    ".git",
    ".hg",
    ".svn",
    "node_modules",
];

const SKIP_EXTENSIONS: &[&str] = &["pyc", "pyo"];

fn should_skip(name: &str) -> bool {
    SKIP_NAMES.iter().any(|s| name == *s || name.ends_with(".egg-info"))
}

pub fn scan_directory(path: &str) -> Vec<String> {
    let base = Path::new(path);
    let mut results = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !should_skip(&name)
        })
    {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let file_path = entry.path();
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        if SKIP_EXTENSIONS.contains(&ext) {
            continue;
        }

        if ext != "py" {
            continue;
        }

        if let Ok(rel) = file_path.strip_prefix(base) {
            results.push(rel.to_string_lossy().to_string());
        }
    }

    results.sort();
    results
}

#[pyfunction]
fn scan(path: &str) -> Vec<String> {
    scan_directory(path)
}

#[pymodule]
fn _pyscan(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(scan, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_filters_py_extension() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("hello.py"), "").unwrap();
        fs::write(tmp.path().join("readme.txt"), "").unwrap();
        fs::write(tmp.path().join("data.json"), "").unwrap();

        let results = scan_directory(tmp.path().to_str().unwrap());
        assert_eq!(results, vec!["hello.py"]);
    }

    #[test]
    fn test_skips_system_dirs() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("top.py"), "").unwrap();

        let pycache = tmp.path().join("__pycache__");
        fs::create_dir(&pycache).unwrap();
        fs::write(pycache.join("cached.py"), "").unwrap();

        let git = tmp.path().join(".git");
        fs::create_dir(&git).unwrap();
        fs::write(git.join("hook.py"), "").unwrap();

        let results = scan_directory(tmp.path().to_str().unwrap());
        assert_eq!(results, vec!["top.py"]);
    }

    #[test]
    fn test_empty_dir() {
        let tmp = TempDir::new().unwrap();
        let results = scan_directory(tmp.path().to_str().unwrap());
        assert!(results.is_empty());
    }

    #[test]
    fn test_nested_dirs() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("pkg").join("sub");
        fs::create_dir_all(&sub).unwrap();
        fs::write(tmp.path().join("a.py"), "").unwrap();
        fs::write(sub.join("b.py"), "").unwrap();

        let results = scan_directory(tmp.path().to_str().unwrap());
        assert_eq!(results, vec!["a.py", "pkg/sub/b.py"]);
    }
}
