use pyo3::prelude::*;
use pyo3::PyIterProtocol;

use crate::PySyntaxNode;

/// Node iterator type. Can be acquired from SyntaxTree or SyntaxNode
///
/// Traverses the tree, maintaining a reverse sorted list of nodes to go through
/// (so new nodes get appended and old ones are popped off the top).
#[pyclass]
pub struct NodeIter {
    /// Contained node
    pub nodes: Vec<Py<PySyntaxNode>>,
}

#[pyproto]
impl PyIterProtocol for NodeIter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PySyntaxNode> {
        match slf.nodes.pop() {
            Some(node) => {
                let node = Python::with_gil(|py| {
                    let node = node.borrow(py);
                    let mut children = node.children.clone();
                    children.reverse();
                    slf.nodes.extend(children);
                    node.clone()
                });
                Some(node)
            }
            None => None,
        }
    }
}

/// Iterator for parsing tree events. This gives context for where you
/// are moving inside of the tree, vs. just listing all types.
#[pyclass]
pub struct NodeEventIter {
    /// Nodes to iterate through and return
    pub events: Vec<PyNodeEvent>,
}

#[pyproto]
impl PyIterProtocol for NodeEventIter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PyNodeEvent> {
        match slf.events.pop() {
            Some(mut event) => {
                let event = Python::with_gil(|py| {
                    // Insert the required "Leave" event if this is not one
                    if let None = event.event {
                        // TODO is unwrap bad here?
                        slf.events.push(PyNodeEvent {
                            event: Some(String::from("Leave")),
                            node: event.node.clone(),
                        });

                        // Set event type to enter
                        event.event = Some(String::from("Enter"));

                        // Add children to our list of events to iterate through
                        //
                        let mut children = event.node.borrow(py).children.clone();
                        children.reverse();
                        for child in children {
                            slf.events.push(PyNodeEvent {
                                event: None,
                                node: child.clone(),
                            });
                        }
                    }

                    event
                });
                Some(event)
            }
            None => None,
        }
    }
}

/// Node event. The library should only return events with the event specified.
#[pyclass(name=NodeEvent)]
#[derive(Clone)]
pub struct PyNodeEvent {
    /// "Enter" or "Leave"
    #[pyo3(get)]
    pub event: Option<String>,
    /// Reference to node in syntax tree.
    #[pyo3(get)]
    pub node: Py<PySyntaxNode>,
}
