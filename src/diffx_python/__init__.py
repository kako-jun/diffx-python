"""
diffx-python: Semantic diffing for structured data files

High-performance comparison of JSON, YAML, TOML, CSV, INI, and XML files.
Powered by Rust for blazing fast performance.

Example:
    >>> import diffx_python as diffx
    >>> old = {"name": "Alice", "age": 30}
    >>> new = {"name": "Alice", "age": 31}
    >>> result = diffx.diff(old, new)
    >>> print(result)
    [{'type': 'Modified', 'path': 'age', 'old_value': 30, 'new_value': 31}]
"""

from __future__ import annotations

from typing import Any

# Import from native Rust module
try:
    from diffx_python.diffx_python import (
        __version__,
        diff,
        format_output,
        parse_csv,
        parse_ini,
        parse_json,
        parse_toml,
        parse_xml,
        parse_yaml,
    )
except ImportError:
    # Fallback for development mode
    from diffx_python import (  # type: ignore[attr-defined]
        __version__,
        diff,
        format_output,
        parse_csv,
        parse_ini,
        parse_json,
        parse_toml,
        parse_xml,
        parse_yaml,
    )


class DiffError(Exception):
    """Exception raised when a diff operation fails."""

    pass


def diff_files(file1_path: str, file2_path: str, **kwargs: Any) -> list[dict[str, Any]]:
    """
    Compare two files directly.

    Auto-detects file format from extension and parses accordingly.

    Args:
        file1_path: Path to first file
        file2_path: Path to second file
        **kwargs: Options passed to diff() function

    Returns:
        List of differences found

    Example:
        >>> result = diff_files("old.json", "new.json")
        >>> result = diff_files("config1.yaml", "config2.yaml", epsilon=0.1)
    """
    from pathlib import Path

    path1 = Path(file1_path)
    path2 = Path(file2_path)

    content1 = path1.read_text(encoding="utf-8")
    content2 = path2.read_text(encoding="utf-8")

    ext1 = path1.suffix.lower()
    ext2 = path2.suffix.lower()

    data1 = _parse_by_extension(content1, ext1)
    data2 = _parse_by_extension(content2, ext2)

    return diff(data1, data2, **kwargs)


def diff_strings(
    content1: str, content2: str, format: str, **kwargs: Any
) -> list[dict[str, Any]]:
    """
    Compare two string contents directly.

    Args:
        content1: First content string
        content2: Second content string
        format: Content format ("json", "yaml", "toml", "ini", "xml", "csv")
        **kwargs: Options passed to diff() function

    Returns:
        List of differences found

    Example:
        >>> json1 = '{"name": "Alice", "age": 30}'
        >>> json2 = '{"name": "Alice", "age": 31}'
        >>> result = diff_strings(json1, json2, "json")
    """
    data1 = _parse_by_format(content1, format)
    data2 = _parse_by_format(content2, format)
    return diff(data1, data2, **kwargs)


def _parse_by_extension(content: str, ext: str) -> Any:
    """Parse content based on file extension."""
    parsers = {
        ".json": parse_json,
        ".yaml": parse_yaml,
        ".yml": parse_yaml,
        ".toml": parse_toml,
        ".ini": parse_ini,
        ".cfg": parse_ini,
        ".xml": parse_xml,
        ".csv": parse_csv,
    }

    parser = parsers.get(ext)
    if parser:
        return parser(content)

    # Try JSON as fallback
    try:
        return parse_json(content)
    except Exception as e:
        raise DiffError(f"Unsupported file format: {ext}") from e


def _parse_by_format(content: str, format: str) -> Any:
    """Parse content based on format string."""
    parsers = {
        "json": parse_json,
        "yaml": parse_yaml,
        "yml": parse_yaml,
        "toml": parse_toml,
        "ini": parse_ini,
        "cfg": parse_ini,
        "xml": parse_xml,
        "csv": parse_csv,
    }

    parser = parsers.get(format.lower())
    if parser:
        return parser(content)

    raise DiffError(f"Unsupported format: {format}")


__all__ = [
    # Version
    "__version__",
    # Main function
    "diff",
    # Parser functions
    "parse_json",
    "parse_yaml",
    "parse_toml",
    "parse_csv",
    "parse_ini",
    "parse_xml",
    # Utility functions
    "format_output",
    "diff_files",
    "diff_strings",
    # Exception
    "DiffError",
]
