// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Node.

use std::fmt::Debug;
use std::collections::HashMap;
use engine::parser::error::{NodeError, NodeErrorCode};
use engine::parser::token::stream::Position;

pub trait Node : Debug {
    fn tag(&self) -> &str;
    fn position(&self) -> &Position;
    fn children(&self) -> &Vec<Box<Node>>;
    fn children_mut(&mut self) -> &mut Vec<Box<Node>>;
    fn has_attribute(&self, key: &str) -> bool;
    fn attribute(&self, key: &str) -> Result<&str, NodeError>;
    fn set_attribute(&mut self, key: &str, value: &str) -> Option<String>;
    fn rm_attribute(&mut self, key: &str) -> Option<String>;
}

type NodeDataAttibutes = HashMap<String, String>;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct GenericNode<T> {
    tag: String,
    position: Position,
    nodes: Vec<Box<Node>>, // ??
    attributes: NodeDataAttibutes,
    data: T,
}

#[allow(dead_code)]
impl<T> Node for GenericNode<T> where
    T: Debug
{
    fn position(&self) -> &Position {
        &self.position
    }

    fn tag(&self) -> &str {
        &self.tag
    }

    fn has_attribute(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }

    fn attribute(&self, key: &str) -> Result<&str, NodeError> {
        match self.attributes.get(key) {
            None => {
                err!(NodeErrorCode::AttributeNotFound {
                    key: key.to_string(),
                    node_tag: self.tag.to_string()
                })
            },
            Some(value) => Ok(value)
        }
    }

    fn set_attribute(&mut self, key: &str, value: &str) -> Option<String> {
        self.attributes.insert(key.to_string(), value.to_string())
    }

    fn rm_attribute(&mut self, key: &str) -> Option<String> {
        self.attributes.remove(key)
    }

    fn children(&self) -> &Vec<Box<Node>> {
        &self.nodes
    }

    fn children_mut(&mut self) -> &mut Vec<Box<Node>> {
        &mut self.nodes
    }
}
