//! Python binding for sv-parser crate.
//!
//! Generates a second tree out of python-friendly objects from
//! the parsed tree generated by sv-parser. This could likely be made
//! more efficient by making iterators operate directly on the tree,
//! but that's very much above my skill level at the moment.

// Turns out large syntax trees can recurse a lot.
#![recursion_limit = "256"]

use std::collections::{HashMap, hash_map::RandomState};

use pyo3::exceptions::PyFileNotFoundError;
use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::{wrap_pyfunction, PyIterProtocol};
use sv_parser::Define;
use sv_parser::{
    parse_lib as lib_parse_lib, parse_lib_str as lib_parse_lib_str, parse_sv as lib_parse_sv,
    parse_sv_str as lib_parse_sv_str, SyntaxTree, Defines, Error
};

mod defines;
mod iterators;
mod tree;

use defines::*;
use iterators::*;
use tree::*;

// XXX I think I could potentially remove the generics from these types,
//     but then I run into lifetime issues, not too sure how to deal with
//     that but it's not too terrible to leave it as is so I'm doing that.

/// Generic type for an sv-parser function that parses from the provided file.
type ParseFile<T, U, V> =
    fn(
        path: T,
        pre_defines: &HashMap<String, Option<Define>, V>,
        include_paths: &[U],
        ignore_include: bool,
        allow_incomplete: bool,
    ) -> Result<(SyntaxTree, Defines), Error>;

/// Generic type for an sv-parser function that parses from the given text.
type ParseText<T, U, V> =
    fn(
        s: &str,
        path: T,
        pre_defines: &HashMap<String, Option<Define>, V>,
        include_paths: &[U],
        ignore_include: bool,
        allow_incomplete: bool
    ) -> Result<(SyntaxTree, Defines), Error>;

/// Generically parses a provided file using the provided sv-parser library function.
fn parse_file<'a>(
    parse_fn: ParseFile<&'a str, String, RandomState>,
    path: &'a str,
    pre_defines: &PyDict,
    include_paths: Vec<String>,
    ignore_include: bool,
    allow_incomplete: bool,
) -> PyResult<PySyntaxTree> {
    let defines = process_pre_defines(pre_defines);
    let (tree, _defines) = parse_fn(
        path,
        &defines,
        &include_paths,
        ignore_include,
        allow_incomplete,
    )
    .map_err(|e| PyFileNotFoundError::new_err(format!("{}", e)))?;

    // Grab first node
    let node = (&tree).into_iter().next().unwrap();

    // Read in original file
    let text = std::fs::read_to_string(path)
        .map_err(|e| pyo3::exceptions::PyFileNotFoundError::new_err(format!("{}", e)))?;

    let tree = PySyntaxNode::build_tree(node, &tree);
    Ok(PySyntaxTree {
        tree: tree,
        text: text,
    })
}

/// Base function to remove redundant code
fn parse_text<'a>(
    parse_fn: ParseText<&'a str, String, RandomState>,
    text: &'a str,
    path: &'a str,
    pre_defines: &PyDict,
    include_paths: Vec<String>,
    ignore_include: bool,
    allow_incomplete: bool,
) -> PyResult<PySyntaxTree> {
    let defines = process_pre_defines(pre_defines);
    let (tree, _defines) = parse_fn(
        text,
        path,
        &defines,
        &include_paths,
        ignore_include,
        allow_incomplete,
    )
    .map_err(|e| PyFileNotFoundError::new_err(format!("{}", e)))?;

    // Grab first node
    let node = (&tree).into_iter().next().unwrap();

    // Read in original file
    let tree = PySyntaxNode::build_tree(node, &tree);
    Ok(PySyntaxTree {
        tree: tree,
        text: String::from(text),
    })
}

/// Parse file at given path for SV syntax tree.
#[pyfunction]
#[text_signature = "(path, pre_defines, include_paths, ignore_include, allow_incomplete)"]
fn parse_sv(
    path: &str,
    pre_defines: &PyDict,
    include_paths: Vec<String>,
    ignore_include: bool,
    allow_incomplete: bool,
) -> PyResult<PySyntaxTree> {
    parse_file(lib_parse_sv, path, pre_defines, include_paths, ignore_include, allow_incomplete)
}

/// Parse provided text for SV syntax tree.
#[pyfunction]
#[text_signature = "(text, path, pre_defines, include_paths, ignore_include, allow_incomplete)"]
fn parse_sv_str(
    text: &str,
    path: &str,
    pre_defines: &PyDict,
    include_paths: Vec<String>,
    ignore_include: bool,
    allow_incomplete: bool,
) -> PyResult<PySyntaxTree> {
    parse_text(lib_parse_sv_str, text, path, pre_defines, include_paths, ignore_include, allow_incomplete)
}

#[pyfunction]
#[text_signature = "(path, pre_defines, include_paths, ignore_include, allow_incomplete)"]
fn parse_lib(
    path: &str,
    pre_defines: &PyDict,
    include_paths: Vec<String>,
    ignore_include: bool,
    allow_incomplete: bool,
) -> PyResult<PySyntaxTree> {
    parse_file(lib_parse_lib, path, pre_defines, include_paths, ignore_include, allow_incomplete)
}

#[pyfunction]
#[text_signature = "(text, path, pre_defines, include_paths, ignore_include, allow_incomplete)"]
fn parse_lib_str(
    text: &str,
    path: &str,
    pre_defines: &PyDict,
    include_paths: Vec<String>,
    ignore_include: bool,
    allow_incomplete: bool,
) -> PyResult<PySyntaxTree> {
    parse_text(lib_parse_lib_str, text, path, pre_defines, include_paths, ignore_include, allow_incomplete)
}

/// Transform a Python dictionary into the HashMap required by parse_sv
fn process_pre_defines(pre_defines: &PyDict) -> HashMap<String, Option<Define>> {
    // Convert dictionary to correct define types
    pre_defines
        .iter()
        .map(|(key, define)| {
            let define = define
                .extract::<PyDefine>()
                .map(|def| def.into_inner())
                .ok();
            let key = key.downcast::<PyString>().unwrap();
            (key.to_string(), define)
        })
        .collect()
}

/// Finds the first node of one of the given types in the provided node.
#[pyfunction]
#[text_signature = "(node, node_types)"]
fn unwrap_node(
    node: PyRefMut<PySyntaxNode>,
    node_types: Vec<String>,
) -> PyResult<Option<Py<PySyntaxNode>>> {
    return Python::with_gil(|py| {
        let iter = PySyntaxNode::__iter__(node).unwrap();
        let iter = PyCell::new(py, iter).unwrap();
        loop {
            let node = NodeIter::__next__(iter.borrow_mut());

            if let Some(node) = node {
                if node_types.contains(&node.type_name) {
                    return Ok(Some(Py::new(py, node).unwrap()));
                }
            } else {
                break;
            }
        }

        Ok(None)
    });
}

/// Finds the first locate node in the provided node.
#[pyfunction]
#[text_signature = "(node)"]
fn unwrap_locate(node: PyRefMut<PySyntaxNode>) -> PyResult<Option<Py<PySyntaxNode>>> {
    unwrap_node(node, vec![String::from("Locate")])
}

/// Simple Python wrapper for sv-parser.
///
/// Does not export all features, but allows you to build a simple tree from an SV file.
#[pymodule]
fn py_sv_parser(_py: Python, module: &PyModule) -> PyResult<()> {
    // Main parsing functions
    module.add_function(wrap_pyfunction!(parse_sv, module)?)?;
    module.add_function(wrap_pyfunction!(parse_sv_str, module)?)?;
    module.add_function(wrap_pyfunction!(parse_lib, module)?)?;
    module.add_function(wrap_pyfunction!(parse_lib_str, module)?)?;

    // Convenience functions
    module.add_function(wrap_pyfunction!(unwrap_node, module)?)?;
    module.add_function(wrap_pyfunction!(unwrap_locate, module)?)?;

    // I'm only adding these classes for typing information, these should
    // not be directly instantiated.
    module.add_class::<PySyntaxTree>()?;
    module.add_class::<PyDefine>()?;
    module.add_class::<PyDefineText>()?;
    module.add_class::<PySyntaxNode>()?;
    module.add_class::<PySyntaxLocation>()?;

    Ok(())
}
