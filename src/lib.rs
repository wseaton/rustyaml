extern crate pyo3;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDateTime, PyDict, PyFloat, PyList, PyTuple};
use pyo3::{create_exception, wrap_pyfunction};
use serde::ser::{self, Serialize, SerializeMap, SerializeSeq, Serializer};
use serde_yaml::to_string;

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
        // Floats are stored as strings, we have to unwrap them on load
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
fn loads(py: Python, yaml: String) -> PyResult<PyObject> {
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


// adapted from https://github.com/mre/hyperjson/blob/10d31608584ef4499d6b6b10b6dc9455b358fe3d/src/lib.rs#L287-L402
// and https://github.com/samuelcolvin/rtoml/blob/master/src/lib.rs
struct SerializePyObject<'p, 'a> {
    py: Python<'p>,
    obj: &'a PyAny,
}

impl<'p, 'a> Serialize for SerializePyObject<'p, 'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        macro_rules! cast {
            ($f:expr) => {
                if let Ok(val) = PyTryFrom::try_from(self.obj) {
                    return $f(val);
                }
            };
        }

        macro_rules! extract {
            ($t:ty) => {
                if let Ok(val) = <$t as FromPyObject>::extract(self.obj) {
                    return val.serialize(serializer);
                }
            };
        }

        macro_rules! isa {
            ($v:ident, $t:ty) => {
                self.py.is_instance::<$t, _>($v).map_err(debug_py_err)?
            };
        }

        macro_rules! add_to_map {
            ($map:ident, $key:ident, $value:ident) => {
                if $key.is_none() {
                    $map.serialize_key("null")?;
                } else if let Ok(key) = $key.extract::<bool>() {
                    $map.serialize_key(if key { "true" } else { "false" })?;
                } else if let Ok(key) = $key.str() {
                    let key = key.to_string();
                    $map.serialize_key(&key)?;
                } else {
                    return Err(ser::Error::custom(format_args!(
                        "Dictionary key is not a string: {:?}",
                        $key
                    )));
                }
                $map.serialize_value(&SerializePyObject {
                    py: self.py,
                    obj: $value,
                })?;
            };
        }

        fn debug_py_err<E: ser::Error>(err: PyErr) -> E {
            E::custom(format_args!("{:?}", err))
        }

        cast!(|x: &PyDict| {
            let mut map = serializer.serialize_map(Some(x.len()))?;

            // https://github.com/alexcrichton/toml-rs/issues/142#issuecomment-278970591
            // taken from alexcrichton/toml-rs/blob/ec4e821f3bb081391801e4c00aa90bf66a53562c/src/value.rs#L364-L387
            for (k, v) in x {
                if !isa!(v, PyList) && !isa!(v, PyTuple) && !isa!(v, PyDict) {
                    add_to_map!(map, k, v);
                }
            }
            for (k, v) in x {
                if isa!(v, PyList) || isa!(v, PyTuple) {
                    add_to_map!(map, k, v);
                }
            }
            for (k, v) in x {
                if isa!(v, PyDict) {
                    add_to_map!(map, k, v);
                }
            }
            map.end()
        });

        macro_rules! to_seq {
            ($type:ty) => {
                cast!(|x: $type| {
                    let mut seq = serializer.serialize_seq(Some(x.len()))?;
                    for element in x {
                        seq.serialize_element(&SerializePyObject {
                            py: self.py,
                            obj: element,
                        })?
                    }
                    return seq.end();
                });
            };
        }

        to_seq!(&PyList);
        to_seq!(&PyTuple);

        extract!(String);
        extract!(bool);

        cast!(|x: &PyFloat| x.value().serialize(serializer));
        extract!(u64);
        extract!(i64);

        if self.obj.is_none() {
            return serializer.serialize_str("null");
        }

        let name = self.obj.get_type().name();
        match self.obj.repr() {
            Ok(repr) => Err(ser::Error::custom(format_args!(
                "{:?} is not serializable to YAML: {:?}",
                name, repr,
            ))),
            Err(_) => Err(ser::Error::custom(format_args!("{:?} is not serializable to YAML", name))),
        }
    }
}


#[pyfunction]
fn dumps(py: Python, obj: PyObject) -> PyResult<String> {
    let s = SerializePyObject {
        py,
        obj: obj.extract(py)?,
    };
    match to_string(&s) {
        Ok(s) => Ok(s),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}


#[pymodule]
fn _rustyaml(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("VERSION", VERSION)?;
    m.add_function(wrap_pyfunction!(loads, m)?).unwrap();
    m.add_function(wrap_pyfunction!(dumps, m)?).unwrap();
    Ok(())
}
