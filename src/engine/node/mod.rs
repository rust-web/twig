// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Node.

use std::fmt::Debug;
use engine::parser::token::stream::Position;

pub trait Node : Debug {
    fn tag(&self) -> &str;
    fn position(&self) -> &Position;
    fn children(&self) -> &Vec<Box<Node>>;
    fn children_mut(&mut self) -> &mut Vec<Box<Node>>;
}
