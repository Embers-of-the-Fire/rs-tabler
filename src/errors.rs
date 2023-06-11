use pyo3::{prelude::*, exceptions};

#[derive(Debug)]
pub struct FormatterError {
    message: String,
}

impl FormatterError {
    pub const fn new(string: String) -> FormatterError {
        FormatterError { message: string }
    }
}

impl std::error::Error for FormatterError {}

impl std::fmt::Display for FormatterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::convert::From<FormatterError> for PyErr {
    fn from(err: FormatterError) -> PyErr {
        exceptions::PyValueError::new_err(err.to_string())
    }
}
