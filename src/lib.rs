use pyo3::prelude::*;

mod settings;
mod errors;
mod table;

#[pymodule]
fn tabler(py: Python, m: &PyModule) -> PyResult<()> {
    settings::regist_classes(py, m)?;
    table::regist_table(py, m)?;
    Ok(())
}