// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Position within a token stream.

use engine::parser::token::stream::{self, Stream};
use std::fmt;
use error::Dump;

pub type Position = usize;

#[derive(Debug)]
pub struct Cursor<'stream> {
    next: Position,   // 0,..
    end: Position,   // 0,..
    stream: &'stream Stream<'stream>, // inner lifetime: 'template
}

impl<'stream> fmt::Display for Cursor<'stream> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "cursor (next: {next}/{end}) for {tokens:?}",
            next = self.next,
            end = self.end,
            tokens = self.stream)
    }
}

#[derive(Debug)]
pub struct CursorDump {
    next: Position,
    end: Position,
    stream_dump: stream::StreamDump,
}

impl<'stream> Dump for Cursor<'stream> {
    type Data = CursorDump;

    fn dump(&self) -> Self::Data {
        CursorDump {
            next: self.next,
            end: self.end,
            stream_dump: self.stream.dump(),
        }
    }
}

impl fmt::Display for CursorDump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cursor (next: {next}/{end}) for {stream_dump}",
            next = self.next,
            end = self.end,
            stream_dump = self.stream_dump)
    }
}
