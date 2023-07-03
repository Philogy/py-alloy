use alloy_dyn_abi::{DynSolType, DynSolValue};
use pyo3::exceptions::{PyException, PyValueError};
use pyo3::prelude::*;

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

#[pyfunction]
fn decode(py: Python, type_str: &str, encoded: &[u8]) -> PyResult<PyObject> {
    let sol_type: DynSolType = type_str
        .parse()
        .map_err(|err| PyValueError::new_err(format!("{}", err)))?;

    let value = sol_type
        .decode_params(&encoded)
        .map_err(|err| PyValueError::new_err(format!("{}", err)))?;

    dyn_sol_to_py(&value, py)
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_alloy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    Ok(())
}
