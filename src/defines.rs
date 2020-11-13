use std::path::PathBuf;

use pyo3::prelude::*;
use sv_parser::{Define, DefineText};
use sv_parser_pp::range::Range;

/// Wrapper python type for sv-parser Define type
///
/// Used to specify preprocessor defines.
#[pyclass(name=Define)]
#[derive(Clone)]
pub struct PyDefine {
    inner: Define,
}

#[pymethods]
impl PyDefine {
    #[new]
    fn new(
        identifier: String,
        arguments: Vec<(String, Option<String>)>,
        text: Option<PyDefineText>,
    ) -> Self {
        Self {
            inner: Define::new(
                identifier,
                arguments,
                text.map_or(None, |t| Some(t.into_inner())),
            ),
        }
    }
}

impl PyDefine {
    /// Helper function to translate into sv_parser types
    ///
    /// Necessary because I can't just apply the pyo3 macros directly to types
    /// from another crate.
    pub fn into_inner(self) -> Define {
        self.inner
    }
}

/// Wrapper python type for sv-parser DefineText type
///
/// Used for filling in what a preprocessor define expands to.
#[pyclass(name=DefineText)]
#[derive(Clone)]
pub struct PyDefineText {
    inner: DefineText,
}

#[pymethods]
impl PyDefineText {
    #[new]
    fn new(text: String, origin: Option<(String, usize, usize)>) -> Self {
        let origin = origin.map(|(path, beg, end)| {
            (
                PathBuf::from(path),
                Range {
                    begin: beg,
                    end: end,
                },
            )
        });

        Self {
            inner: DefineText::new(text, origin),
        }
    }
}

impl PyDefineText {
    pub fn into_inner(self) -> DefineText {
        self.inner
    }
}
