// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig Extension API
//!
//! Extensions can define new behavior during the compilation process, via **token parser** and
//! **node visitor**. They can also define new *node types* in the abstract syntax tree, like
//! **test**, **unary operator**, **binary operator**, **function**, **filter**, **global**
//!
//! See `twig::extension::core` for example implementations.
//! Note: The core extension is not yet fully implemented (see [CHANGELOG][changelog]).

use std::fmt;
use engine;
use api::parser::{Job, ParserError};
use api::token::stream::Item;
use api::Node;
use api::error::Traced;


/// Extends the Twig Engine with new behaviour.
pub trait Extension : fmt::Debug {
    /// Get the name of the extension.
    fn name(&self) -> &'static str;

    /// Initialize and register the extension.
    ///
    /// This method is supposed to push all filters, functions etc. of the extension to the
    /// extension registry builder.
    fn init(&mut self, registry: &mut engine::extension_registry::Builder, options: &engine::Options)
        -> Result<(), Traced<engine::ExtensionRegistryError>>; // TODO: add error handling ???
}

// Abstract extension traits + structs - TODO: check what needs to be trait / can be struct

/// Can modify the result of variable expressions.
///
/// E.g. the `default` filter returns the result of the variable expression if it is defined,
/// otherwise it returns the default value. The `escaper` filter escapes the result according
/// to the output channel (html, html attribute, css, js, url, ..)
pub trait Filter : fmt::Debug {}

/// Can be used to perform complex computations.
///
/// E.g. the `round` function rounds a floating number with a given precision.
pub trait Function : fmt::Debug {}

/// Can be used to define global constants.
///
/// Templates can test for these global constants to trigger conditional behavior, or use
/// them as argument for functions, etc
pub trait Global : fmt::Debug {}

/// Modifies the abstract syntax tree immediately after parsing.
///
/// E.g. `twig::extension::optimizer` defines the `optimizeRawFilter` node visitor which strips all "raw" filters from the syntax tree.
pub trait NodeVisitor : fmt::Debug {}

/// Can be used in conditional Twig statements.
///
/// E.g. the `defined` test checks if a variable is defined in the current context.
pub trait Test : fmt::Debug {}

/// Transforms a sub-sequence from the token stream (=lexed template) to nodes in the abstract syntax tree.
///
/// E.g. the `TokenParserIf` parses complex if-statements (if, elseif, else, endif) and creates the if-node with according child nodes for each test and conditional branch.
///
/// Note: Token parser are also called 'tag handler' by twig parser.
pub trait TokenParser : fmt::Debug {
    fn tag(&self) -> &'static str;

    fn parse(&self, job: &mut Job, item: &Item) -> Result<Box<Node>, Traced<ParserError>>;
}

pub mod token_parser {
    use api::token::stream::Item;

    pub type Test = Fn(&Item) -> TestResult;

    #[derive(Debug)]
    pub enum TestResult {
        Continue,  // orig: no_match
        KeepToken, // orig: is_match + dropNeedle == false
        DropToken, // orig: is_match + dropNeedle == true
    }
}

/// Can be used in variable expressions to process results.
///
/// E.g. the `-` (neg) operator inverts the sign of a numeric result.
#[derive(Debug, PartialEq)]
pub struct UnaryOperator {
    pub repr: String, // token representation like "-"
    pub ext: op::Extension,
    pub prec: op::Precedence,
    pub op: op::Operation,
}

/// Can be used in variable expressions to combine two results.
///
/// E.g. the `**` (power) operator takes one number to the power of another number.
#[derive(Debug, PartialEq)]
pub struct BinaryOperator {
    pub repr: String, // token representation like "!="
    pub ext: op::Extension,
    pub prec: op::Precedence,
    pub op: op::Operation,
    pub assoc: op::Assoc,
}

pub mod op {
    #[derive(Debug, PartialEq)]
    pub struct Extension(String); // might switch to ID for faster lookups

    #[derive(Debug, PartialEq, PartialOrd)]
    pub struct Precedence(pub usize);

    #[derive(Debug, PartialEq)]
    pub enum Operation {
        Class(Class),
        Callable(Function)
    }

    /// Associativity
    #[derive(Debug, PartialEq)]
    pub enum Assoc {
        Left,
        Right,
    }

    #[derive(Debug, PartialEq)]
    pub struct Function {
        name: String
    }

    #[derive(Debug, PartialEq)]
    pub struct Class {
        name: String
    }
}
