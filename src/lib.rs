extern crate pyo3;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDateTime, PyDict, PyFloat, PyList, PyTuple};
use pyo3::{create_exception, wrap_pyfunction};
use serde::ser::{self, Serialize, SerializeMap, SerializeSeq, Serializer};

use std::str::FromStr;

use yaml_rust::Yaml;
use yaml_rust::Yaml::{
    Alias, Array, BadValue, Boolean, Hash, Integer, Null, Real, String as YamlString,
};
use yaml_rust::{YamlEmitter, YamlLoader};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn convert_value(t: &Yaml, py: Python) -> PyResult<PyObject> {
    match t {
        Hash(table) => {
            let d = PyDict::new(py);
            for (key, value) in table.iter() {
                d.set_item(convert_value(key, py)?, convert_value(value, py)?)?;
            }
            Ok(d.to_object(py))
        }

        Array(array) => {
            let mut list: Vec<PyObject> = Vec::with_capacity(array.len());
            for value in array {
                list.push(convert_value(value, py)?)
            }
            Ok(list.to_object(py))
        }
        YamlString(v) => Ok(v.to_object(py)),
        Integer(v) => Ok(v.to_object(py)),
        Real(v) => Ok(parse_f64(v).unwrap().to_object(py)),
        Boolean(v) => Ok(v.to_object(py)),
        Null => Ok(py.None()),
        BadValue => Ok(py.None()),
        _ => Err(PyValueError::new_err("Serialization error")),
    }
}


// parse f64 as Core schema
// See: https://github.com/chyh1990/yaml-rust/issues/51
fn parse_f64(v: &str) -> Option<f64> {
    match v {
        ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => Some(f64::INFINITY),
        "-.inf" | "-.Inf" | "-.INF" => Some(f64::NEG_INFINITY),
        ".nan" | "NaN" | ".NAN" => Some(f64::NAN),
        _ => v.parse::<f64>().ok(),
    }
}


#[pyfunction]
fn parse_yaml(py: Python, yaml: String) -> PyResult<PyObject> {
    match YamlLoader::load_from_str(&yaml) {
        Ok(v) => {
            let l = PyList::empty(py);
            for i in v.iter() {
                l.append(convert_value(i, py)?)?;
            }
            Ok(l.to_object(py))
        }
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}

#[pymodule]
fn _rspyaml(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("VERSION", VERSION)?;
    m.add_function(wrap_pyfunction!(parse_yaml, m)?).unwrap();
    Ok(())
}