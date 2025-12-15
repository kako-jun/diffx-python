//! Python bindings for diffx - semantic diffing of structured data files.
//!
//! This module provides Python bindings for the diffx-core library using PyO3.

#![allow(clippy::useless_conversion)]
#![allow(clippy::uninlined_format_args)]

use diffx_core::{
    diff as core_diff, format_output as core_format_output, parse_csv as core_parse_csv,
    parse_ini as core_parse_ini, parse_json as core_parse_json, parse_toml as core_parse_toml,
    parse_xml as core_parse_xml, parse_yaml as core_parse_yaml, DiffOptions, DiffResult,
    DiffxSpecificOptions, OutputFormat,
};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use regex::Regex;
use serde_json::Value;

// ============================================================================
// Main diff function
// ============================================================================

/// Unified diff function for Python
///
/// Compare two Python objects (dicts, lists, or primitives) and return differences.
///
/// Args:
///     old: The old value (dict, list, or primitive)
///     new: The new value (dict, list, or primitive)
///     **kwargs: Optional parameters:
///         epsilon (float): Numerical comparison tolerance
///         array_id_key (str): Key to use for array element identification
///         ignore_keys_regex (str): Regex pattern for keys to ignore
///         path_filter (str): Only show differences in paths containing this string
///         output_format (str): Output format ("diffx", "json", "yaml")
///         ignore_whitespace (bool): Ignore whitespace differences
///         ignore_case (bool): Ignore case differences
///         brief_mode (bool): Report only whether files differ
///         quiet_mode (bool): Suppress normal output
///
/// Returns:
///     List[Dict]: List of differences found
#[pyfunction]
#[pyo3(signature = (old, new, **kwargs))]
fn diff(
    py: Python,
    old: &Bound<'_, PyAny>,
    new: &Bound<'_, PyAny>,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<PyObject> {
    let old_json = python_to_json_value(old)?;
    let new_json = python_to_json_value(new)?;
    let options = build_options_from_kwargs(kwargs)?;

    let results = core_diff(&old_json, &new_json, Some(&options)).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Diff error: {e}"))
    })?;

    let py_results = PyList::empty_bound(py);
    for result in results {
        let py_result = diff_result_to_python(py, &result)?;
        py_results.append(py_result)?;
    }

    Ok(py_results.into())
}

// ============================================================================
// Parser functions
// ============================================================================

/// Parse JSON string to Python object
///
/// Args:
///     content: JSON string to parse
///
/// Returns:
///     Parsed Python object (dict, list, or primitive)
#[pyfunction]
fn parse_json(py: Python, content: &str) -> PyResult<PyObject> {
    let value = core_parse_json(content).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON parse error: {e}"))
    })?;
    json_value_to_python(py, &value)
}

/// Parse YAML string to Python object
///
/// Args:
///     content: YAML string to parse
///
/// Returns:
///     Parsed Python object (dict, list, or primitive)
#[pyfunction]
fn parse_yaml(py: Python, content: &str) -> PyResult<PyObject> {
    let value = core_parse_yaml(content).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("YAML parse error: {e}"))
    })?;
    json_value_to_python(py, &value)
}

/// Parse TOML string to Python object
///
/// Args:
///     content: TOML string to parse
///
/// Returns:
///     Parsed Python object (dict)
#[pyfunction]
fn parse_toml(py: Python, content: &str) -> PyResult<PyObject> {
    let value = core_parse_toml(content).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("TOML parse error: {e}"))
    })?;
    json_value_to_python(py, &value)
}

/// Parse CSV string to Python list of dicts
///
/// Args:
///     content: CSV string to parse
///
/// Returns:
///     List of dictionaries representing CSV rows
#[pyfunction]
fn parse_csv(py: Python, content: &str) -> PyResult<PyObject> {
    let value = core_parse_csv(content).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("CSV parse error: {e}"))
    })?;
    json_value_to_python(py, &value)
}

/// Parse INI string to Python dict
///
/// Args:
///     content: INI string to parse
///
/// Returns:
///     Parsed Python dictionary
#[pyfunction]
fn parse_ini(py: Python, content: &str) -> PyResult<PyObject> {
    let value = core_parse_ini(content).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("INI parse error: {e}"))
    })?;
    json_value_to_python(py, &value)
}

/// Parse XML string to Python dict
///
/// Args:
///     content: XML string to parse
///
/// Returns:
///     Parsed Python dictionary
#[pyfunction]
fn parse_xml(py: Python, content: &str) -> PyResult<PyObject> {
    let value = core_parse_xml(content).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("XML parse error: {e}"))
    })?;
    json_value_to_python(py, &value)
}

// ============================================================================
// Format output function
// ============================================================================

/// Format diff results as string
///
/// Args:
///     results: List of diff results from diff() function
///     format: Output format ("diffx", "json", "yaml")
///
/// Returns:
///     Formatted string output
#[pyfunction]
fn format_output(results: &Bound<'_, PyList>, format: &str) -> PyResult<String> {
    let rust_results = python_results_to_rust(results)?;

    let output_format = OutputFormat::parse_format(format).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid format: {e}"))
    })?;

    core_format_output(&rust_results, output_format).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Format error: {e}"))
    })
}

// ============================================================================
// Helper functions
// ============================================================================

fn python_to_json_value(py_obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    if py_obj.is_none() {
        Ok(Value::Null)
    } else if let Ok(b) = py_obj.extract::<bool>() {
        Ok(Value::Bool(b))
    } else if let Ok(i) = py_obj.extract::<i64>() {
        Ok(Value::Number(i.into()))
    } else if let Ok(f) = py_obj.extract::<f64>() {
        Ok(Value::Number(
            serde_json::Number::from_f64(f).unwrap_or(0.into()),
        ))
    } else if let Ok(s) = py_obj.extract::<String>() {
        Ok(Value::String(s))
    } else if let Ok(list) = py_obj.downcast::<PyList>() {
        let mut vec = Vec::new();
        for item in list.iter() {
            vec.push(python_to_json_value(&item)?);
        }
        Ok(Value::Array(vec))
    } else if let Ok(dict) = py_obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str = key.extract::<String>()?;
            let json_value = python_to_json_value(&value)?;
            map.insert(key_str, json_value);
        }
        Ok(Value::Object(map))
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported Python type",
        ))
    }
}

fn json_value_to_python(py: Python, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.to_object(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.to_object(py))
            } else {
                Ok(py.None())
            }
        }
        Value::String(s) => Ok(s.to_object(py)),
        Value::Array(arr) => {
            let py_list = PyList::empty_bound(py);
            for item in arr {
                let py_item = json_value_to_python(py, item)?;
                py_list.append(py_item)?;
            }
            Ok(py_list.into())
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new_bound(py);
            for (key, val) in obj {
                let py_value = json_value_to_python(py, val)?;
                py_dict.set_item(key, py_value)?;
            }
            Ok(py_dict.into())
        }
    }
}

fn diff_result_to_python(py: Python, result: &DiffResult) -> PyResult<PyObject> {
    let py_dict = PyDict::new_bound(py);

    match result {
        DiffResult::Added(path, value) => {
            py_dict.set_item("type", "Added")?;
            py_dict.set_item("path", path)?;
            py_dict.set_item("value", json_value_to_python(py, value)?)?;
        }
        DiffResult::Removed(path, value) => {
            py_dict.set_item("type", "Removed")?;
            py_dict.set_item("path", path)?;
            py_dict.set_item("value", json_value_to_python(py, value)?)?;
        }
        DiffResult::Modified(path, old_val, new_val) => {
            py_dict.set_item("type", "Modified")?;
            py_dict.set_item("path", path)?;
            py_dict.set_item("old_value", json_value_to_python(py, old_val)?)?;
            py_dict.set_item("new_value", json_value_to_python(py, new_val)?)?;
        }
        DiffResult::TypeChanged(path, old_val, new_val) => {
            py_dict.set_item("type", "TypeChanged")?;
            py_dict.set_item("path", path)?;
            py_dict.set_item("old_value", json_value_to_python(py, old_val)?)?;
            py_dict.set_item("new_value", json_value_to_python(py, new_val)?)?;
        }
    }

    Ok(py_dict.into())
}

fn python_results_to_rust(results: &Bound<'_, PyList>) -> PyResult<Vec<DiffResult>> {
    let mut rust_results = Vec::new();

    for item in results.iter() {
        let dict = item.downcast::<PyDict>()?;

        let diff_type: String = dict
            .get_item("type")?
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'type' field"))?
            .extract()?;

        let path: String = dict
            .get_item("path")?
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'path' field"))?
            .extract()?;

        let result = match diff_type.as_str() {
            "Added" => {
                let value = dict.get_item("value")?.ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'value' field")
                })?;
                DiffResult::Added(path, python_to_json_value(&value)?)
            }
            "Removed" => {
                let value = dict.get_item("value")?.ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'value' field")
                })?;
                DiffResult::Removed(path, python_to_json_value(&value)?)
            }
            "Modified" => {
                let old_value = dict.get_item("old_value")?.ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'old_value' field")
                })?;
                let new_value = dict.get_item("new_value")?.ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'new_value' field")
                })?;
                DiffResult::Modified(
                    path,
                    python_to_json_value(&old_value)?,
                    python_to_json_value(&new_value)?,
                )
            }
            "TypeChanged" => {
                let old_value = dict.get_item("old_value")?.ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'old_value' field")
                })?;
                let new_value = dict.get_item("new_value")?.ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'new_value' field")
                })?;
                DiffResult::TypeChanged(
                    path,
                    python_to_json_value(&old_value)?,
                    python_to_json_value(&new_value)?,
                )
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid diff type: {}",
                    diff_type
                )))
            }
        };

        rust_results.push(result);
    }

    Ok(rust_results)
}

fn build_options_from_kwargs(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<DiffOptions> {
    let mut options = DiffOptions::default();

    if let Some(kwargs) = kwargs {
        if let Some(epsilon) = kwargs.get_item("epsilon")? {
            options.epsilon = Some(epsilon.extract::<f64>()?);
        }

        if let Some(array_id_key) = kwargs.get_item("array_id_key")? {
            options.array_id_key = Some(array_id_key.extract::<String>()?);
        }

        if let Some(ignore_keys_regex) = kwargs.get_item("ignore_keys_regex")? {
            let pattern: String = ignore_keys_regex.extract()?;
            let regex = Regex::new(&pattern).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid regex: {e}"))
            })?;
            options.ignore_keys_regex = Some(regex);
        }

        if let Some(path_filter) = kwargs.get_item("path_filter")? {
            options.path_filter = Some(path_filter.extract::<String>()?);
        }

        if let Some(output_format) = kwargs.get_item("output_format")? {
            let format_str: String = output_format.extract()?;
            let format = OutputFormat::parse_format(&format_str).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid output format: {e}"
                ))
            })?;
            options.output_format = Some(format);
        }

        // diffx-specific options
        let mut diffx_options = DiffxSpecificOptions::default();
        let mut has_diffx_options = false;

        if let Some(ignore_whitespace) = kwargs.get_item("ignore_whitespace")? {
            diffx_options.ignore_whitespace = Some(ignore_whitespace.extract::<bool>()?);
            has_diffx_options = true;
        }

        if let Some(ignore_case) = kwargs.get_item("ignore_case")? {
            diffx_options.ignore_case = Some(ignore_case.extract::<bool>()?);
            has_diffx_options = true;
        }

        if let Some(brief_mode) = kwargs.get_item("brief_mode")? {
            diffx_options.brief_mode = Some(brief_mode.extract::<bool>()?);
            has_diffx_options = true;
        }

        if let Some(quiet_mode) = kwargs.get_item("quiet_mode")? {
            diffx_options.quiet_mode = Some(quiet_mode.extract::<bool>()?);
            has_diffx_options = true;
        }

        if has_diffx_options {
            options.diffx_options = Some(diffx_options);
        }
    }

    Ok(options)
}

// ============================================================================
// Python module
// ============================================================================

/// diffx-python: Semantic diffing for structured data files
///
/// Provides high-performance comparison of JSON, YAML, TOML, CSV, INI, and XML files.
/// Powered by Rust for blazing fast performance.
#[pymodule]
fn diffx_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Main diff function
    m.add_function(wrap_pyfunction!(diff, m)?)?;

    // Parser functions
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    m.add_function(wrap_pyfunction!(parse_yaml, m)?)?;
    m.add_function(wrap_pyfunction!(parse_toml, m)?)?;
    m.add_function(wrap_pyfunction!(parse_csv, m)?)?;
    m.add_function(wrap_pyfunction!(parse_ini, m)?)?;
    m.add_function(wrap_pyfunction!(parse_xml, m)?)?;

    // Format output function
    m.add_function(wrap_pyfunction!(format_output, m)?)?;

    // Version
    m.add("__version__", "0.6.1")?;

    Ok(())
}
