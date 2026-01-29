use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::Python as py;
use pyo3::exceptions::PyLookupError;


#[pyclass(name = "Router")]
struct Router {
    inner: matchit::Router<String>,
}

#[pymethods]
impl Router {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: matchit::Router::new(),
        }
    }

    pub fn insert(&mut self, route: String, value: String) {
        self.inner.insert(route, value).expect("TODO: panic message");
    }

    pub fn at<'py>(&mut self, py: pyo3::Python<'py>, path: &str) -> PyResult<Bound<'py, PyDict>> {
        let matched = (&mut self.inner).at(path);
        if matched.is_ok() {
            let d = PyDict::new(py);
            for (k, v) in matched.unwrap().params.iter()  {
                d.set_item(k, v)?;
            }
            Ok(d)
            // Ok(PyDict::newmatched.unwrap().params)
        } else {
            Err(PyErr::new::<PyLookupError, _>("No route found"))
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule(name="matchit")]
mod python_matchit {
    #[pymodule_export]
    use super::Router;
}
