use stretch::node;

use crate::{Number,Size,Style};

#[derive(Debug)]
pub struct Node(pub(crate) node::Node);

impl Node {
    pub fn new(style: Style) -> Node {
        Self::with_children(style,Vec::new())
    }
    pub(crate) fn with_children(style: Style,children: Vec<Node>) -> Node {
        Node(node::Node::new(
            style.0,
            children.iter().map(|c| &c.0).collect(),
        ))
    }
    pub fn with_measure<F>(style: Style,measure: F) -> Node
    where
        F: 'static + Fn(Size<Number>) -> Size(f32),
    {
        Node(node::Node::new_leaf(
            style.0,
            Box::new(move |size| Ok(measure(size))),
        ))
    }
}