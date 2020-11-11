use pyo3::prelude::*;

use sv_parser::{Define, DefineText};

#[pyclass(name=Define)]
#[derive(Clone)]
pub struct PyDefine {
    /// ID, "name" of the define
    pub identifier: String,
    /// List of arguments, with optional default value
    pub arguments: Vec<(String, Option<String>)>,
    /// Text to substitute in.
    // TODO implement the origin stuff
    pub text: Option<String>,
}

#[pymethods]
impl PyDefine {
    #[new]
    fn new(
        identifier: String,
        arguments: Vec<(String, Option<String>)>,
        text: Option<String>,
    ) -> Self {
        PyDefine {
            identifier,
            arguments,
            text,
        }
    }
}

impl PyDefine {
    /// Helper function to translate into sv_parser types
    ///
    /// Necessary because I can't just apply the pyo3 macros directly to types
    /// from another crate.
    pub fn into_define(self) -> Define {
        let text = match self.text {
            Some(text) => Some(DefineText::new(text, None)),
            None => None,
        };
        Define::new(self.identifier, self.arguments, text)
    }
}
