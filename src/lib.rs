use pyo3::exceptions::PyLookupError;
use pyo3::Python;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass(name = "Router")]
struct Router {
    inner: matchit::Router<Py<PyAny>>,
}

#[pyclass(name = "MatchResult")]
struct MatchResult {
    #[pyo3(get)]
    value: Py<PyAny>,
    #[pyo3(get)]
    params: std::collections::HashMap<String, String>,
}

#[pymethods]
impl Router {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: matchit::Router::new(),
        }
    }

    pub fn insert(&mut self, route: String, value: Py<PyAny>) {
        self.inner
            .insert(route, value)
            .expect("Panic! Failed to insert route");
    }

    pub fn at(&self, _py: Python, path: &str) -> PyResult<MatchResult> {
        let matched = (&self.inner).at(path);
        if matched.is_ok() {
            let unwrapped = matched.unwrap();
            let mut d = HashMap::new();
            for (k, v) in unwrapped.params.iter() {
                d.insert(k.to_string(), v.to_string());
            }
            Ok(MatchResult {
                value: unwrapped.value.clone_ref(_py),
                params: d,
            })
        } else {
            Err(PyErr::new::<PyLookupError, _>("No route found"))
        }
    }
}

#[pymodule(name = "matchit", gil_used = false)]
mod python_matchit {
    #[pymodule_export]
    use super::MatchResult;
    #[pymodule_export]
    use super::Router;
}
