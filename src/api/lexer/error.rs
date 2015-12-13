// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of lexer and syntax errors.

use std::fmt::{self, Display};
use std::error::Error;

use api::token;
use api::lexer::job::cursor;

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedCharacter {
        character: char,
        cursor: cursor::CursorDump,
    },
    UnexpectedBracket {
        bracket: token::BracketType,
        cursor: cursor::CursorDump,
    },
    UnexpectedEof {
        reason: &'static str,
        cursor: cursor::CursorDump,
    },
    UnclosedBracket {
        bracket: token::BracketType,
        bracket_before: token::BracketType,
        line_before: usize,
        cursor: cursor::CursorDump,
    },
    UnclosedComment {
        cursor: cursor::CursorDump,
    },
    UnclosedBlock {
        cursor: cursor::CursorDump,
    },
    UnclosedVariable {
        cursor: cursor::CursorDump
    },
}

impl Error for SyntaxError {
    fn description(&self) -> &str {
        match *self {
            SyntaxError::UnexpectedCharacter{..} => "Unexpected character.",
            SyntaxError::UnexpectedBracket{..} => "Unexpected bracket.",
            SyntaxError::UnexpectedEof{..} => "Unexpected end of template.",
            SyntaxError::UnclosedBracket{..} => "Unclosed bracket.",
            SyntaxError::UnclosedComment{..} => "Unclosed comment.",
            SyntaxError::UnclosedBlock{..} => "Unclosed block.",
            SyntaxError::UnclosedVariable{..} => "Unclosed variable.",
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            SyntaxError::UnexpectedCharacter {
                character, ref cursor
            } => {
                write!(f, " found '{c}' at {cursor}.",
                    c = character, cursor = cursor)
            },
            SyntaxError::UnexpectedBracket {
                ref cursor, ref bracket
            } => {
                write!(f, " Unexpected {bracket:?} at {cursor}.",
                    cursor = cursor, bracket = bracket)
            },
            SyntaxError::UnexpectedEof {
                reason: ref r,
                cursor: ref c
            } => {
                write!(f, " {reason} at {cursor}.",
                    reason = r,
                    cursor = c)
            },
            SyntaxError::UnclosedBracket {
                ref cursor, ref bracket, ref bracket_before, line_before
            } => {
                write!(f, " Unclosed {b_before:?} from line\
                                {line_before} but found {b:?} at {cursor}.",
                    cursor = cursor,
                    b = bracket,
                    b_before = bracket_before,
                    line_before = line_before)
            },
            SyntaxError::UnclosedComment {
                ref cursor
            } => {
                write!(f, " At {cursor}.",
                    cursor = cursor)
            },
            SyntaxError::UnclosedBlock {
                ref cursor
            } => {
                write!(f, " At {cursor}.",
                    cursor = cursor)
            },
            SyntaxError::UnclosedVariable {
                ref cursor
            } => {
                write!(f, " At {cursor}.",
                    cursor = cursor)
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    PatternRegexError,
    _InvalidPatternMatch,
    InvalidValue {
        value: String
    },
}

impl Error for LexerError {
    fn description(&self) -> &str {
        match *self {
            LexerError::PatternRegexError => "Could not initialize lexer due to invalid regular expression.",
            LexerError::_InvalidPatternMatch => "Invalid pattern match.",
            LexerError::InvalidValue{..} => "Invalid value.",
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            LexerError::PatternRegexError => Ok(()),
            LexerError::_InvalidPatternMatch => Ok(()),
            LexerError::InvalidValue {
                ref value
            } => {
                write!(f, " Found value {}", value)
            },
        }
    }
}
