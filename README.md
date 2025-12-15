# diffx-python

Python bindings for [diffx](https://github.com/kako-jun/diffx) - semantic diff for structured data (JSON, YAML, TOML, XML, INI, CSV). Powered by Rust via PyO3 for blazing fast performance.

## Installation

```bash
pip install diffx-python
```

### Supported Platforms

| Platform | Architecture |
|----------|--------------|
| Linux | x64 (glibc) |
| Linux | ARM64 |
| macOS | x64 (Intel) |
| macOS | ARM64 (Apple Silicon) |
| Windows | x64 |

## Usage

### Basic Diff

```python
import diffx_python as diffx

old = {"name": "Alice", "age": 30}
new = {"name": "Alice", "age": 31, "city": "Tokyo"}

results = diffx.diff(old, new)

for change in results:
    print(f"{change['type']}: {change['path']}")
    # Modified: age
    # Added: city
```

### With Options

```python
results = diffx.diff(data1, data2,
    epsilon=0.001,                      # Tolerance for float comparison
    array_id_key='id',                  # Match array elements by ID
    ignore_keys_regex='timestamp|updatedAt',  # Ignore keys matching regex
    path_filter='user',                 # Only show diffs in paths containing "user"
    ignore_case=True,                   # Ignore case differences
    ignore_whitespace=True,             # Ignore whitespace differences
)
```

### Parsers

Parse various formats to Python objects:

```python
import diffx_python as diffx

json_obj = diffx.parse_json('{"name": "Alice"}')
yaml_obj = diffx.parse_yaml('name: Alice\nage: 30')
toml_obj = diffx.parse_toml('name = "Alice"')
csv_list = diffx.parse_csv('name,age\nAlice,30')
ini_obj = diffx.parse_ini('[user]\nname = Alice')
xml_obj = diffx.parse_xml('<user><name>Alice</name></user>')
```

### Format Output

```python
import diffx_python as diffx

results = diffx.diff(old, new)
print(diffx.format_output(results, 'json'))   # JSON format
print(diffx.format_output(results, 'yaml'))   # YAML format
print(diffx.format_output(results, 'diffx'))  # diffx format
```

### File Comparison

```python
import diffx_python as diffx

# Compare files (auto-detects format from extension)
results = diffx.diff_files('old.json', 'new.json')
results = diffx.diff_files('config1.yaml', 'config2.yaml', epsilon=0.1)

# Compare strings
json1 = '{"name": "Alice", "age": 30}'
json2 = '{"name": "Alice", "age": 31}'
results = diffx.diff_strings(json1, json2, 'json')
```

## API Reference

### `diff(old, new, **kwargs)`

Compare two values and return differences.

**Options (kwargs):**
| Option | Type | Description |
|--------|------|-------------|
| `epsilon` | float | Tolerance for floating-point comparisons |
| `array_id_key` | str | Key to identify array elements |
| `ignore_keys_regex` | str | Regex pattern for keys to ignore |
| `path_filter` | str | Only show diffs in matching paths |
| `output_format` | str | Output format ("diffx", "json", "yaml") |
| `ignore_whitespace` | bool | Ignore whitespace differences |
| `ignore_case` | bool | Ignore case differences |
| `brief_mode` | bool | Report only whether objects differ |
| `quiet_mode` | bool | Suppress normal output |

**Returns:** List of diff results:
```python
# For Added/Removed:
{"type": "Added", "path": "key", "value": ...}
{"type": "Removed", "path": "key", "value": ...}

# For Modified/TypeChanged:
{"type": "Modified", "path": "key", "old_value": ..., "new_value": ...}
{"type": "TypeChanged", "path": "key", "old_value": ..., "new_value": ...}
```

### Parsers

- `parse_json(content: str) -> Any`
- `parse_yaml(content: str) -> Any`
- `parse_toml(content: str) -> dict`
- `parse_csv(content: str) -> list[dict]`
- `parse_ini(content: str) -> dict`
- `parse_xml(content: str) -> dict`

### Utility Functions

- `format_output(results: list, format: str) -> str` - Format diff results as string
- `diff_files(file1: str, file2: str, **kwargs) -> list` - Compare two files
- `diff_strings(str1: str, str2: str, format: str, **kwargs) -> list` - Compare two strings

### Exception

- `DiffError` - Raised when diff operations fail

## Development

```bash
# Setup
uv sync --all-extras

# Build
uv run maturin develop

# Test
uv run pytest -v

# Format & Lint
cargo fmt --check
cargo clippy
```

## License

MIT
