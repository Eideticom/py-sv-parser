use pyo3::prelude::*;
use pyo3::PyIterProtocol;

use sv_parser::{NodeEvent, RefNode, SyntaxTree, Locate};

use crate::iterators::*;

/// Representation of a top-level syntax tree.
///
/// Iterate on this class to iterate through the entire tree.
#[pyclass(name=SyntaxTree)]
#[text_signature = "(path, pre_defines, include_paths, ignore_include)"]
pub struct PySyntaxTree {
    /// Top node of syntax tree
    #[pyo3(get)]
    pub tree: PySyntaxNode,

    /// Internal tree, used for pulling origin.
    pub sv_tree: SyntaxTree,

    /// Private, used for get_str
    pub text: String,
}

#[pymethods]
impl PySyntaxTree {
    /// Gets the original string from a node.
    #[text_signature = "($self, node)"]
    fn get_str(&self, node: &PySyntaxNode) -> Option<String> {
        let origin = node.location.clone()?;
        Some(String::from(
            &self.text[origin.offset..origin.offset + origin.len],
        ))
    }

    #[text_signature = "($self, node)"]
    fn get_origin(&self, node: PySyntaxNode) -> Option<(String, usize)> {
        let locate = node.location?;
        let locate = Locate{
            offset: locate.offset,
            len: locate.len,
            line: 0,
        };
        let (path, offset) = self.sv_tree.get_origin(&locate)?;
        let path = String::from(path.to_str()?);
        Some((path, offset))
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
    /// Placement and length in processed tree.
    #[pyo3(get)]
    pub location: Option<PySyntaxLocation>,
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
    pub fn build_tree(node: RefNode, tree: &SyntaxTree) -> Self {
        let name = format!("{}", node);
        // Basically pulled wholesale from get_str() impl on SyntaxTree
        //let mut offset: Option<usize> = None;
        let mut beg: Option<usize> = None;
        let mut line: Option<u32> = None;
        let mut end: usize = 0;
        // Every Locate node is a piece of text, so when we iterate
        // over all of them, we get all of the text contained in the node.
        for n in node.clone() {
            if let RefNode::Locate(x) = n {
                if beg.is_none() {
                    beg = Some(x.offset);
                    line = Some(x.line);
                }
                end = x.offset + x.len;
            }
        }

        let location: Option<PySyntaxLocation>;
        if let Some(beg) = beg {
            location = Some(PySyntaxLocation {
                offset: beg,
                len: end - beg,
                line: line.unwrap(),
            })
        } else {
            location = None;
        }

        let mut children = Vec::new();
        let mut level: usize = 0;
        // EXTREMELY naive tree building
        // GIL closure placed here to avoid acquiring it every loop
        Python::with_gil(|py| {
            for node in node.into_iter().event() {
                match node {
                    NodeEvent::Enter(node) => {
                        // First node is always this node...
                        if level == 1 {
                            // unwrap() here because this should not fail
                            let node = Py::new(py, PySyntaxNode::build_tree(node, tree)).unwrap();
                            children.push(node);
                        }
                        level += 1;
                    }
                    NodeEvent::Leave(_) => {
                        level -= 1;
                    }
                }
            }
        });

        Self {
            location: location,
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
    /// Offset of syntax piece in processed text.
    #[pyo3(get)]
    pub offset: usize,
    /// Length of syntax node in processed text.
    #[pyo3(get)]
    pub len: usize,

    /// Line of syntax node in processed text.
    #[pyo3(get)]
    pub line: u32,
}
