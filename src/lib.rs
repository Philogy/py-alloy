use alloy_dyn_abi::{DynSolType, DynSolValue};
use pyo3::exceptions::{PyException, PyValueError};
use pyo3::ffi;
use pyo3::prelude::*;
use ruint2::Uint;
use std::ffi::c_uchar;

fn dyn_sol_to_py(sol_val: &DynSolValue, py: Python<'_>) -> PyResult<PyObject> {
    // TODO: Int, Custom Struct, Custom Value
    match sol_val {
        DynSolValue::Address(a) => Ok(format!("{}", a).into_py(py)),
        DynSolValue::Bool(b) => Ok(b.into_py(py)),
        DynSolValue::Bytes(b) => Ok(b[..].into_py(py)),
        // Slice ensures that value is converted into Python `bytes` not list of ints.
        DynSolValue::FixedBytes(_, _) => Ok(sol_val.encode_packed()[..].into_py(py)),
        DynSolValue::String(s) => Ok(s.into_py(py)),
        DynSolValue::Uint(x, _) => Ok(x.into_py(py)),
        DynSolValue::Array(arr) | DynSolValue::Tuple(arr) | DynSolValue::FixedArray(arr) => Ok(arr
            .iter()
            .map(|v| dyn_sol_to_py(&v, py))
            .collect::<PyResult<Vec<PyObject>>>()?
            .into_py(py)),
        t => Err(PyException::new_err(format!(
            "Unsupported DynSolValue {t:?}"
        ))),
    }
}

fn general_decode(sol_type: &DynSolType, encoded: &[u8]) -> Result<DynSolValue, PyErr> {
    if let Ok(value) = sol_type.decode_single(&encoded) {
        return Ok(value);
    }
    sol_type
        .decode_sequence(&encoded)
        .map_err(|err| PyValueError::new_err(format!("{}", err)))
}

#[pyfunction]
fn decode(py: Python, type_str: &str, encoded: &[u8]) -> PyResult<PyObject> {
    let sol_type: DynSolType = type_str
        .parse()
        .map_err(|err| PyValueError::new_err(format!("{}", err)))?;

    let value = general_decode(&sol_type, &encoded)?;

    dyn_sol_to_py(&value, py)
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_alloy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    Ok(())
}
