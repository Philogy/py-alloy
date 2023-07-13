use alloy_primitives::U256;
use alloy_sol_types::{sol, SolInterface};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

sol! {
    #[derive(Debug, PartialEq)]
    #[pyclass]
    interface IERC20 {
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);

        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 amount) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function transferFrom(address from, address to, uint256 amount) external returns (bool);
    }
}

#[pyclass]
struct TransferCall {
    #[pyo3(get)]
    to: String,
    #[pyo3(get)]
    amount: U256,
}

#[pymethods]
impl TransferCall {
    #[new]
    fn new(to: String, amount: U256) -> Self {
        Self { to, amount }
    }
}

#[pyclass]
struct TransferFromCall {
    #[pyo3(get)]
    from: String,
    #[pyo3(get)]
    to: String,
    #[pyo3(get)]
    amount: U256,
}

#[pymethods]
impl TransferFromCall {
    #[new]
    fn new(from: String, to: String, amount: U256) -> Self {
        Self { from, to, amount }
    }
}

#[pyclass]
struct ApproveCall {
    #[pyo3(get)]
    spender: String,
    #[pyo3(get)]
    amount: U256,
}

#[pymethods]
impl ApproveCall {
    #[new]
    fn new(spender: String, amount: U256) -> Self {
        Self { spender, amount }
    }
}

#[pyclass]
struct AllowanceCall {
    #[pyo3(get)]
    owner: String,
    #[pyo3(get)]
    spender: String,
}

#[pymethods]
impl AllowanceCall {
    #[new]
    fn new(owner: String, spender: String) -> Self {
        Self { owner, spender }
    }
}

#[pyclass]
struct BalanceOfCall {
    #[pyo3(get)]
    account: String,
}

#[pymethods]
impl BalanceOfCall {
    #[new]
    fn new(account: String) -> Self {
        Self { account }
    }
}

#[pyclass]
struct TotalSupplyCall;

#[pymethods]
impl TotalSupplyCall {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

#[pyfunction]
fn decode(py: Python<'_>, encoded: &[u8]) -> PyResult<PyObject> {
    let decoded = IERC20::IERC20Calls::decode(&encoded, true)
        .map_err(|err| PyValueError::new_err(format!("{}", err)))?;
    match decoded {
        IERC20::IERC20Calls::transfer(IERC20::transferCall { to, amount }) => Ok(TransferCall {
            to: format!("{}", to),
            amount,
        }
        .into_py(py)),
        IERC20::IERC20Calls::transferFrom(IERC20::transferFromCall { from, to, amount }) => {
            Ok(TransferFromCall {
                from: format!("{}", from),
                to: format!("{}", to),
                amount,
            }
            .into_py(py))
        }
        IERC20::IERC20Calls::approve(IERC20::approveCall { spender, amount }) => Ok(ApproveCall {
            spender: format!("{}", spender),
            amount,
        }
        .into_py(py)),
        IERC20::IERC20Calls::allowance(IERC20::allowanceCall { owner, spender }) => {
            Ok(AllowanceCall {
                owner: format!("{}", owner),
                spender: format!("{}", spender),
            }
            .into_py(py))
        }
        IERC20::IERC20Calls::balanceOf(IERC20::balanceOfCall { account }) => Ok(BalanceOfCall {
            account: format!("{}", account),
        }
        .into_py(py)),
        IERC20::IERC20Calls::totalSupply(IERC20::totalSupplyCall {}) => {
            Ok(TotalSupplyCall.into_py(py))
        }
    }
}

#[pymodule]
pub fn register_erc20(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "ERC20")?;
    m.add_class::<TransferCall>()?;
    m.add_class::<TransferFromCall>()?;
    m.add_class::<ApproveCall>()?;
    m.add_class::<AllowanceCall>()?;
    m.add_class::<BalanceOfCall>()?;
    m.add_class::<TotalSupplyCall>()?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    parent_module.add_submodule(m)?;
    Ok(())
}
