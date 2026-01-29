use matchit::InsertError;
use pyo3::prelude::*;


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

    pub fn at(&mut self, path: &str) -> &String {
        (&mut self.inner).at(path).unwrap().value
    }
}

/// A Python module implemented in Rust.
#[pymodule(name="matchit")]
mod python_matchit {
    #[pymodule_export]
    use super::Router;
}
