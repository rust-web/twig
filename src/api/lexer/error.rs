// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of lexer and syntax errors.

use std::fmt::{self, Display};
use error::Error;
use error::ErrorCode;

use api::token;
use api::lexer::job::cursor;

pub type SyntaxError = Error<SyntaxErrorCode>;
pub type LexerError = Error<LexerErrorCode>;

// impl GeneralizeTo<LexerErrorCode> for SyntaxErrorCode {
//     fn generalize(&self) -> LexerErrorCode { LexerErrorCode::SyntaxError }
// }

#[derive(Debug)]
pub enum SyntaxErrorCode {
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

impl ErrorCode for SyntaxErrorCode {
    fn description(&self) -> &str {
        match *self {
            SyntaxErrorCode::UnexpectedCharacter{..} => "Unexpected character.",
            SyntaxErrorCode::UnexpectedBracket{..} => "Unexpected bracket.",
            SyntaxErrorCode::UnexpectedEof{..} => "Unexpected end of template.",
            SyntaxErrorCode::UnclosedBracket{..} => "Unclosed bracket.",
            SyntaxErrorCode::UnclosedComment{..} => "Unclosed comment.",
            SyntaxErrorCode::UnclosedBlock{..} => "Unclosed block.",
            SyntaxErrorCode::UnclosedVariable{..} => "Unclosed variable.",
        }
    }
}

impl Display for SyntaxErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            SyntaxErrorCode::UnexpectedCharacter {
                character, ref cursor
            } => {
                write!(f, " found '{c}' at {cursor}.",
                    c = character, cursor = cursor)
            },
            SyntaxErrorCode::UnexpectedBracket {
                ref cursor, ref bracket
            } => {
                write!(f, " Unexpected {bracket:?} at {cursor}.",
                    cursor = cursor, bracket = bracket)
            },
            SyntaxErrorCode::UnexpectedEof {
                reason: ref r,
                cursor: ref c
            } => {
                write!(f, " {reason} at {cursor}.",
                    reason = r,
                    cursor = c)
            },
            SyntaxErrorCode::UnclosedBracket {
                ref cursor, ref bracket, ref bracket_before, line_before
            } => {
                write!(f, " Unclosed {b_before:?} from line\
                                {line_before} but found {b:?} at {cursor}.",
                    cursor = cursor,
                    b = bracket,
                    b_before = bracket_before,
                    line_before = line_before)
            },
            SyntaxErrorCode::UnclosedComment {
                ref cursor
            } => {
                write!(f, " At {cursor}.",
                    cursor = cursor)
            },
            SyntaxErrorCode::UnclosedBlock {
                ref cursor
            } => {
                write!(f, " At {cursor}.",
                    cursor = cursor)
            },
            SyntaxErrorCode::UnclosedVariable {
                ref cursor
            } => {
                write!(f, " At {cursor}.",
                    cursor = cursor)
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexerErrorCode {
    PatternRegexError,
    _InvalidPatternMatch,
    InvalidValue {
        value: String
    },
}

impl ErrorCode for LexerErrorCode {
    fn description(&self) -> &str {
        match *self {
            LexerErrorCode::PatternRegexError => "Could not initialize lexer due to invalid regular expression.",
            LexerErrorCode::_InvalidPatternMatch => "Invalid pattern match.",
            LexerErrorCode::InvalidValue{..} => "Invalid value.",
        }
    }
}

impl Display for LexerErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            LexerErrorCode::PatternRegexError => Ok(()),
            LexerErrorCode::_InvalidPatternMatch => Ok(()),
            LexerErrorCode::InvalidValue {
                ref value
            } => {
                write!(f, " Found value {}", value)
            },
        }
    }
}
