// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of parser errors.

use std::fmt::{self, Display};
use error::Error;
use error::ErrorCode;

use api::parser::job::{self, cursor};
use api::token;

pub type ParserError = Error<ParserErrorCode>;
pub type NodeError = Error<NodeErrorCode>; // todo move somewhere else??

// impl GeneralizeTo<ParserErrorCode> for TokenErrorCode {
//     fn generalize(&self) -> ParserErrorCode { ParserErrorCode::TokenError }
// }

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParserErrorCode {
    TokenParserError {
        tag: &'static str, // known at compile-time
        error: String,
        job: job::JobDump,
    },
    NoTagHandler {
        tag: String, // only known at runtime
        position: token::stream::Position,
        job: job::JobDump,
    },
    UnexpectedBinaryOperator {
        name: String,
        job: job::JobDump,
    },
    UnexpectedToken {
        reason: Option<&'static str>,
        expected: token::PatternDump,
        found: token::stream::Item,
    },
    UnexpectedEof {
        reason: Option<&'static str>,
        expected: Option<token::PatternDump>,
        cursor: cursor::CursorDump,
    },
}

impl ErrorCode for ParserErrorCode {
    fn description(&self) -> &str {
        match *self {
            ParserErrorCode::TokenParserError{..} => "Token parser error.",
            ParserErrorCode::NoTagHandler{..} => "There is no registered tag handler for named block.",
            ParserErrorCode::UnexpectedBinaryOperator{..} => "Unexpected Binary Operator.",
            ParserErrorCode::UnexpectedToken{..} => "Unexpected Token.",
            ParserErrorCode::UnexpectedEof{..} => "Unexpected end of token stream.",
        }
    }
}

impl Display for ParserErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            ParserErrorCode::TokenParserError {
                tag, ref error, ref job
            } => {
                write!(f, " {tag:?}-block: {error} for job {job}.",
                    tag = tag, error = error, job = job)
            },
            ParserErrorCode::NoTagHandler {
                tag: ref t, position: ref p, job: ref j
            } => {
                write!(f, " Found block {tag} at {pos} for job {job}.",
                    tag = t, pos = p, job = j)
            },
            ParserErrorCode::UnexpectedBinaryOperator {
                name: ref n, job: ref j
            } => {
                write!(f, " The binary operator {name:?} is unknown to the engine for job {job}",
                    name = n,
                    job = j)
            },
            ParserErrorCode::UnexpectedToken {
                reason: r, expected: ref x, found: ref i
            } => {
                try!(write!(f, " Expected token {x:?} but found {t:?} at {p:?}.",
                    x = x, t = i.token(), p = i.position()));

                if let Some(reason) = r {
                    try!(write!(f, " {}", reason));
                }

                Ok(())
            },
            ParserErrorCode::UnexpectedEof {
                reason, ref expected, ref cursor
            } => {
                if let Some(ref expected) = *expected {
                    try!(write!(f, " Expected token to match {:?}.", expected));
                }

                if let Some(reason) = reason {
                    try!(write!(f, " {}", reason))
                }

                write!(f, " For {}.", cursor)
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum NodeErrorCode {
    AttributeNotFound {
        key: String,
        node_tag: String
    }
}

impl ErrorCode for NodeErrorCode {
    fn description(&self) -> &str {
        match *self {
            NodeErrorCode::AttributeNotFound{..} => "Attribute not found.",
        }
    }
}

impl Display for NodeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            NodeErrorCode::AttributeNotFound{
                ref key, ref node_tag
            } => {
                write!(f, " Attribute {key:?} does not exist for Node {node:?}.",
                    key = key, node = node_tag)
            },
        }
    }
}
