"""
diffx - Semantic diff for structured data (JSON, YAML, TOML, XML, INI, CSV)

Example:
    from diffx import diff

    old = {"name": "Alice", "age": 30}
    new = {"name": "Alice", "age": 31, "city": "Tokyo"}

    results = diff(old, new)
    for change in results:
        print(f"{change['type']}: {change['path']}")
"""

from ._diffx import (
    diff,
    parse_json,
    parse_yaml,
    parse_toml,
    parse_csv,
    parse_ini,
    parse_xml,
    format_output,
)

__all__ = [
    "diff",
    "parse_json",
    "parse_yaml",
    "parse_toml",
    "parse_csv",
    "parse_ini",
    "parse_xml",
    "format_output",
]
__version__ = "0.7.0"
