"""Tests for format_output function.

Based on diffx-js format.test.js for parity.
"""

import json

import pytest

import diffx_python


class TestFormatOutput:
    """format_output tests - based on diffx-core spec."""

    @staticmethod
    def create_diff_results():
        """Helper to create diff results for testing."""
        old = {"name": "Alice", "age": 30}
        new = {"name": "Bob", "age": 30, "city": "Tokyo"}
        return diffx_python.diff(old, new)

    def test_formats_as_json(self):
        results = self.create_diff_results()
        output = diffx_python.format_output(results, "json")

        assert isinstance(output, str)
        # Should be valid JSON
        parsed = json.loads(output)
        assert isinstance(parsed, list)

    def test_json_output_contains_all_diff_info(self):
        results = self.create_diff_results()
        output = diffx_python.format_output(results, "json")
        parsed = json.loads(output)

        # Should have entries for Modified (name) and Added (city)
        assert len(parsed) >= 2

    def test_formats_as_yaml(self):
        results = self.create_diff_results()
        output = diffx_python.format_output(results, "yaml")

        assert isinstance(output, str)
        assert len(output) > 0

    def test_formats_as_diffx(self):
        results = self.create_diff_results()
        output = diffx_python.format_output(results, "diffx")

        assert isinstance(output, str)
        assert len(output) > 0

    def test_raises_on_invalid_format(self):
        results = self.create_diff_results()

        with pytest.raises(Exception):
            diffx_python.format_output(results, "invalid")

    def test_handles_empty_results(self):
        output = diffx_python.format_output([], "json")

        assert output == "[]"

    def test_formats_manually_constructed_added_result(self):
        results = [{"type": "Added", "path": "newField", "value": "value"}]

        output = diffx_python.format_output(results, "json")
        assert isinstance(output, str)

    def test_formats_manually_constructed_modified_result(self):
        results = [
            {"type": "Modified", "path": "field", "old_value": "old", "new_value": "new"}
        ]

        output = diffx_python.format_output(results, "json")
        assert isinstance(output, str)

    def test_formats_manually_constructed_removed_result(self):
        results = [{"type": "Removed", "path": "removedField", "value": "removedValue"}]

        output = diffx_python.format_output(results, "json")
        assert isinstance(output, str)
