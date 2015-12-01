// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Parser

use engine::Engine;
use template;

pub mod error;
pub mod expression_parser;
pub mod token;
pub mod lexer;
pub mod job;
pub use self::job::Job;
pub use self::lexer::Lexer;
pub use self::lexer::LexerError;
pub use self::lexer::LexerErrorCode;
pub use self::token::Token;
pub use self::error::{ParserError, ParserErrorCode};

#[derive(Debug, Default)]
pub struct Parser;

impl Parser {
    pub fn new(_twig: &Engine) -> Result<Parser, ParserError> {
        unimplemented!()
    }

    #[allow(dead_code)] // TODO: testcase
    pub fn parse<'a, 't> (&'a self, _stream: &'t token::Stream<'t>) -> Result<template::Compiled, ParserError>
        where 't: 'a // the token stream must outlive the Parser
    {
        unimplemented!()
    }
}
