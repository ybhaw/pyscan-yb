import os

from pyscan_yb import scan


def test_finds_py_files_in_flat_dir(tmp_path):
    (tmp_path / "a.py").write_text("print('a')")
    (tmp_path / "b.py").write_text("print('b')")

    result = scan(str(tmp_path))
    assert result == ["a.py", "b.py"]


def test_finds_py_files_recursively(tmp_path):
    sub = tmp_path / "pkg" / "sub"
    sub.mkdir(parents=True)
    (tmp_path / "top.py").write_text("")
    (sub / "deep.py").write_text("")

    result = scan(str(tmp_path))
    assert "top.py" in result
    assert os.path.join("pkg", "sub", "deep.py") in result


def test_empty_directory_returns_empty(tmp_path):
    result = scan(str(tmp_path))
    assert result == []


def test_ignores_pycache(tmp_path):
    (tmp_path / "good.py").write_text("")
    cache = tmp_path / "__pycache__"
    cache.mkdir()
    (cache / "bad.py").write_text("")

    result = scan(str(tmp_path))
    assert result == ["good.py"]


def test_ignores_system_files(tmp_path):
    (tmp_path / "good.py").write_text("")

    for dirname in [
        ".git",
        ".venv",
        "venv",
        ".mypy_cache",
        ".pytest_cache",
        "node_modules",
    ]:
        d = tmp_path / dirname
        d.mkdir()
        (d / "hidden.py").write_text("")

    result = scan(str(tmp_path))
    assert result == ["good.py"]


def test_returns_relative_paths(tmp_path):
    sub = tmp_path / "pkg"
    sub.mkdir()
    (sub / "mod.py").write_text("")

    result = scan(str(tmp_path))
    assert result == [os.path.join("pkg", "mod.py")]
    # Should not contain absolute paths
    for p in result:
        assert not os.path.isabs(p)


def test_results_are_sorted(tmp_path):
    for name in ["z.py", "a.py", "m.py"]:
        (tmp_path / name).write_text("")

    result = scan(str(tmp_path))
    assert result == sorted(result)
    assert result == ["a.py", "m.py", "z.py"]
