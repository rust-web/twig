// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Position within a raw template.

use std::fmt;
use template;
use error::Dump;

pub type Position = usize;
pub type Line = usize;
pub type CursorDump = String;

#[derive(Debug)]
pub struct Cursor<'a> {
    pos: Position,   // 0,..
    end: Position,   // 0,..
    line: Line,    // 1,..
    template: &'a template::Raw,
}

impl<'a> fmt::Display for Cursor<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unimplemented!()
    }
}

impl<'a> Dump for Cursor<'a> {
    type Data = CursorDump;

    fn dump(&self) -> Self::Data {
        self.to_string()
    }
}
