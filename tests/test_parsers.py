"""Tests for parser functions.

These tests verify that the parser functions work correctly
based on the diffx-core specification and diffx-js parity.
"""

import pytest
import diffx_python


class TestParseJson:
    """JSON parser tests - based on diffx-core spec."""

    def test_parses_simple_object(self):
        result = diffx_python.parse_json('{"name": "Alice", "age": 30}')
        assert result == {"name": "Alice", "age": 30}

    def test_parses_array(self):
        result = diffx_python.parse_json("[1, 2, 3]")
        assert result == [1, 2, 3]

    def test_parses_nested_structure(self):
        result = diffx_python.parse_json('{"user": {"profile": {"age": 30}}}')
        assert result["user"]["profile"]["age"] == 30

    def test_parses_primitives(self):
        assert diffx_python.parse_json("null") is None
        assert diffx_python.parse_json("true") is True
        assert diffx_python.parse_json("false") is False
        assert diffx_python.parse_json("42") == 42
        assert diffx_python.parse_json('"hello"') == "hello"

    def test_raises_on_invalid_json(self):
        with pytest.raises(Exception):
            diffx_python.parse_json("invalid json")


class TestParseYaml:
    """YAML parser tests - based on diffx-core spec."""

    def test_parses_simple_yaml(self):
        result = diffx_python.parse_yaml("name: Alice\nage: 30")
        assert result["name"] == "Alice"
        assert result["age"] == 30

    def test_parses_nested_yaml(self):
        yaml = """
user:
  profile:
    age: 30
"""
        result = diffx_python.parse_yaml(yaml)
        assert result["user"]["profile"]["age"] == 30

    def test_parses_yaml_arrays(self):
        yaml = """
items:
  - 1
  - 2
  - 3
"""
        result = diffx_python.parse_yaml(yaml)
        assert result["items"] == [1, 2, 3]


class TestParseToml:
    """TOML parser tests - based on diffx-core spec."""

    def test_parses_simple_toml(self):
        result = diffx_python.parse_toml('name = "Alice"\nage = 30')
        assert result["name"] == "Alice"
        assert result["age"] == 30

    def test_parses_toml_sections(self):
        toml = """
[user]
name = "Alice"

[user.profile]
age = 30
"""
        result = diffx_python.parse_toml(toml)
        assert result["user"]["name"] == "Alice"
        assert result["user"]["profile"]["age"] == 30

    def test_parses_toml_arrays(self):
        result = diffx_python.parse_toml("items = [1, 2, 3]")
        assert result["items"] == [1, 2, 3]


class TestParseCsv:
    """CSV parser tests - based on diffx-core spec."""

    def test_parses_csv_with_headers(self):
        csv = "name,age\nAlice,30\nBob,25"
        result = diffx_python.parse_csv(csv)

        assert isinstance(result, list)
        assert len(result) == 2
        assert result[0]["name"] == "Alice"
        assert result[0]["age"] == "30"  # CSV values are strings

    def test_parses_quoted_csv(self):
        csv = 'name,description\nAlice,"Hello, World"'
        result = diffx_python.parse_csv(csv)
        assert result[0]["description"] == "Hello, World"


class TestParseIni:
    """INI parser tests - based on diffx-core spec."""

    def test_parses_simple_ini(self):
        ini = """
[user]
name = Alice
age = 30
"""
        result = diffx_python.parse_ini(ini)
        assert result["user"]["name"] == "Alice"
        assert result["user"]["age"] == "30"  # INI values are strings

    def test_parses_multiple_sections(self):
        ini = """
[database]
host = localhost

[cache]
enabled = true
"""
        result = diffx_python.parse_ini(ini)
        assert result["database"]["host"] == "localhost"
        assert result["cache"]["enabled"] == "true"


class TestParseXml:
    """XML parser tests - based on diffx-core spec."""

    def test_parses_simple_xml(self):
        result = diffx_python.parse_xml("<user><name>Alice</name></user>")
        assert result is not None

    def test_raises_on_invalid_xml(self):
        with pytest.raises(Exception):
            diffx_python.parse_xml("<invalid")


class TestParserDiffIntegration:
    """Integration: parser + diff."""

    def test_diff_parsed_json(self):
        obj1 = diffx_python.parse_json('{"name": "Alice", "age": 30}')
        obj2 = diffx_python.parse_json('{"name": "Alice", "age": 31}')
        results = diffx_python.diff(obj1, obj2)

        assert len(results) == 1
        assert results[0]["path"] == "age"

    def test_diff_parsed_yaml(self):
        obj1 = diffx_python.parse_yaml("name: Alice\nage: 30")
        obj2 = diffx_python.parse_yaml("name: Alice\nage: 31")
        results = diffx_python.diff(obj1, obj2)

        assert len(results) == 1
        assert results[0]["path"] == "age"

    def test_diff_parsed_toml(self):
        obj1 = diffx_python.parse_toml('name = "Alice"\nage = 30')
        obj2 = diffx_python.parse_toml('name = "Alice"\nage = 31')
        results = diffx_python.diff(obj1, obj2)

        assert len(results) == 1
        assert results[0]["path"] == "age"
