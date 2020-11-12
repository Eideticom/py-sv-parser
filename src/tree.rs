use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::PyIterProtocol;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use sv_parser::{parse_sv, NodeEvent, RefNode, SyntaxTree};

use crate::defines::PyDefine;
use crate::iterators::*;

/// Base object for creating a syntax tree.
///
/// Use this class to parse a file, and then it can be used as an iterator
/// to traverse the tree.
#[pyclass(name=SyntaxTree)]
#[text_signature = "(path, pre_defines, include_paths, ignore_include)"]
pub struct PySyntaxTree {
    /// Top node of syntax tree
    #[pyo3(get)]
    pub tree: PySyntaxNode,

    /// Private, used for get_str
    pub text: String,
}

// TODO implement pre_defines... somehow
#[pymethods]
impl PySyntaxTree {
    /// Parses the given file for it's CST.
    #[new]
    fn new(
        path: &str,
        pre_defines: &pyo3::types::PyDict,
        include_paths: Vec<String>,
        ignore_include: bool,
    ) -> PyResult<Self> {
        // Convert dictionary to correct define types
        // TODO should I just directly map from a dict?
        // do I really need my custom types?
        let mut defines = HashMap::new();
        for key in pre_defines.keys() {
            let define = pre_defines.get_item(key).unwrap();
            let define = match define.extract::<PyDefine>() {
                Ok(def) => Some(def.into_define()),
                Err(_) => None,
            };
            let key = key.downcast::<PyString>().unwrap();
            let key = key.to_string();
            defines.insert(key.clone(), define);
        }

        let (tree, _defines) = match parse_sv(path, &defines, &include_paths, ignore_include, true)
        {
            Ok(results) => results,
            Err(e) => {
                return Err(pyo3::exceptions::PyArithmeticError::new_err(format!(
                    "{}",
                    e
                )))
            }
        };

        // Grab first node
        let node = (&tree).into_iter().next().unwrap();

        // Read in original file
        let mut text = String::new();
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                return Err(pyo3::exceptions::PyFileNotFoundError::new_err(format!(
                    "{}",
                    e
                )));
            }
        };
        match file.read_to_string(&mut text) {
            Err(e) => {
                return Err(pyo3::exceptions::PyFileNotFoundError::new_err(format!(
                    "{}",
                    e
                )));
            }
            _ => (),
        }

        let tree = PySyntaxNode::build_tree(node, &tree);
        Ok(PySyntaxTree {
            tree: tree,
            text: text,
        })
    }

    /// Gets the original string from a node.
    #[text_signature = "($self, node)"]
    fn get_str(&self, node: &PySyntaxNode) -> Option<String> {
        match node.origin.clone() {
            Some(origin) => Some(String::from(
                &self.text[origin.offset..origin.offset + origin.len],
            )),
            None => None,
        }
    }

    /// Returns an iterator of events for traversing the tree.
    #[text_signature = "($self)"]
    fn events(&mut self) -> PyResult<NodeEventIter> {
        let event = Python::with_gil(|py| PyNodeEvent {
            event: None,
            node: Py::new(py, self.tree.clone()).unwrap(),
        });
        let events = vec![event];
        Ok(NodeEventIter { events: events })
    }
}

#[pyproto]
impl PyIterProtocol for PySyntaxTree {
    /// Returns a simple iterator for viewing the tree.
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<NodeIter> {
        let node = Python::with_gil(|py| Py::new(py, slf.tree.clone()))?;
        let nodes = vec![node];
        Ok(NodeIter { nodes: nodes })
    }
}

/// Node of the concrete syntax tree created by SyntaxTree
#[pyclass(name=SyntaxNode)]
#[derive(Clone)]
pub struct PySyntaxNode {
    /// Placement and length in original file.
    /// Optional because of preprocessor shenanigans
    #[pyo3(get)]
    pub origin: Option<PySyntaxLocation>,
    /// Name of the type in the syntax tree.
    /// Notably provided in PascalCase, as opposed to the snake_case used in the standard.
    #[pyo3(get)]
    pub type_name: String,
    /// References to all of the children of this node
    #[pyo3(get)]
    pub children: Vec<Py<PySyntaxNode>>,
}

impl PySyntaxNode {
    /// Helper function used to create the tree.
    fn build_tree(node: RefNode, tree: &SyntaxTree) -> Self {
        let name = format!("{}", node);
        // Basically pulled wholesale from get_str() impl on SyntaxTree
        let mut offset: Option<usize> = None;
        let mut file: Option<String> = None;
        let mut beg: Option<usize> = None;
        let mut end: usize = 0;
        // Every Locate node is a piece of text, so when we iterate
        // over all of them, we get all of the text contained in the node.
        for n in node.clone() {
            if let RefNode::Locate(x) = n {
                if beg.is_none() {
                    beg = Some(x.offset);
                }
                end = x.offset + x.len;

                if file.is_none() {
                    if let Some((path, off)) = tree.get_origin(x) {
                        file = Some(String::from(path.to_str().unwrap()));
                        offset = Some(off)
                    }
                }
            }
        }

        // TODO
        // SOUNDNESS problem! These lengths are applied to the expanded text...
        // Maybe we need to disable the preprocessor?
        // Talk about it with Jonathan
        let origin: Option<PySyntaxLocation>;
        if let Some(beg) = beg {
            origin = Some(PySyntaxLocation {
                file: file.unwrap(),
                offset: offset.unwrap(),
                len: end - beg,
            })
        } else {
            origin = None;
        }

        let mut children = Vec::new();
        let mut level: usize = 0;
        // EXTREMELY naive tree building
        for node in node.into_iter().event() {
            match node {
                NodeEvent::Enter(node) => {
                    // First node is always this node...
                    if level == 1 {
                        Python::with_gil(|py| {
                            let node = Py::new(py, PySyntaxNode::build_tree(node, tree));
                            if let Ok(node) = node {
                                children.push(node);
                            } else {
                                // TODO error handling
                            }
                        })
                    }
                    level += 1;
                }
                NodeEvent::Leave(_) => {
                    level -= 1;
                }
            }
        }

        Self {
            origin: origin,
            type_name: name,
            children: children,
        }
    }
}

#[pymethods]
impl PySyntaxNode {
    /// Creates an event iterator object.
    /// This object will iterate through NodeEvent objects
    #[text_signature = "($self)"]
    fn events(&self) -> PyResult<NodeEventIter> {
        let event = Python::with_gil(|py| PyNodeEvent {
            event: None,
            node: Py::new(py, self.clone()).unwrap(),
        });
        let events = vec![event];
        Ok(NodeEventIter { events: events })
    }
}

#[pyproto]
impl PyIterProtocol for PySyntaxNode {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<NodeIter> {
        let node = Python::with_gil(|py| Py::new(py, slf.clone()))?;
        let nodes = vec![node];
        Ok(NodeIter { nodes: nodes })
    }
}

/// Location information of a syntax node.
#[pyclass(name=SyntaxLocation)]
#[derive(Clone)]
pub struct PySyntaxLocation {
    /// Original file this came from.
    /// Only relevant if file uses `include's, because then syntax can come from anywhere.
    #[pyo3(get)]
    pub file: String,
    /// Offset of syntax piece in original file.
    #[pyo3(get)]
    pub offset: usize,
    /// Length of syntax node in original file.
    #[pyo3(get)]
    pub len: usize,
}
