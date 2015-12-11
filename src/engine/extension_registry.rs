// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Extension registry.
//!
//! Stores 

use std::collections::{HashSet, HashMap};
use api::ext::{self, Extension};
use engine::Options;
use engine::error::{ExtensionRegistryError, ExtensionRegistryErrorCode};

pub type Iter<'a> = ::std::collections::hash_map::Values<'a, String, Box<Extension>>;

#[derive(Debug, Default)]
pub struct ExtensionRegistry {
    ext_names: HashSet<String>,
    filters: HashMap<String, Box<ext::Filter>>,
    functions: HashMap<String, Box<ext::Function>>,
    tests: HashMap<String, Box<ext::Test>>,
    token_parsers: HashMap<String, Box<ext::TokenParser>>,
    node_visitors: Vec<Box<ext::NodeVisitor>>,
    operators_unary: HashMap<String, ext::UnaryOperator>,
    operators_binary: HashMap<String, ext::BinaryOperator>,
    _globals: Vec<Box<ext::Global>>,
}

impl ExtensionRegistry {
    /// Initialize new extension registry instance.
    pub fn new<I>(iterable: I, options: &Options) -> Result<Self, ExtensionRegistryError> where
        I: IntoIterator<Item=Box<Extension>>
    {
        let mut builder = Builder::default();

        for mut ext in iterable {
            if !builder.staged.ext_names.insert(ext.name().to_string())  {
                return err!(ExtensionRegistryErrorCode::DuplicateExtension {
                    name: ext.name().to_string()
                })
            };

            try!(ext.init(&mut builder, options));
        }

        Ok(builder.into())
    }

    /// Returns true if the given extension is registered
    pub fn has(&self, name: &str) -> bool {
        self.ext_names.contains(name)
    }

    /// Get all registered extension names.
    pub fn ext_names(&self) -> &HashSet<String> {
        &self.ext_names
    }

    /// Get the token parser instances defined by engine extensions.
    pub fn token_parsers(&self) -> &HashMap<String, Box<ext::TokenParser>> {
        &self.token_parsers
    }

    // /// Get token parsers by registered tag
    // pub fn _token_parser_by_tags(&self) -> &HashMap<String, Box<api::TokenParser>> {
    //     &self._token_parser_by_tags
    // }

    /// Get the node visitor instances defined by engine extensions.
    pub fn node_visitors(&self) -> &Vec<Box<ext::NodeVisitor>> {
        &self.node_visitors
    }

    /// Get the filters defined by engine extensions.
    pub fn filters(&self) -> &HashMap<String, Box<ext::Filter>> {
        &self.filters
    }

    /// Get the tests defined by engine extensions.
    pub fn tests(&self) -> &HashMap<String, Box<ext::Test>> {
        &self.tests
    }

    /// Get the functions defined by engine extensions.
    pub fn functions(&self) -> &HashMap<String, Box<ext::Function>> {
        &self.functions
    }

    /// Get the unary operators defined by engine extensions.
    pub fn operators_unary(&self) -> &HashMap<String, ext::UnaryOperator> {
        &self.operators_unary
    }

    /// Get the binary operators defined by engine extensions.
    pub fn operators_binary(&self) -> &HashMap<String, ext::BinaryOperator> {
        &self.operators_binary
    }

    /// Get the global variables defined by engine extensions.
    pub fn _globals(&self) -> &Vec<Box<ext::Global>> {
        &self._globals
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    staged: ExtensionRegistry,
}

impl Builder {
    #[allow(dead_code)]
    /// Register token parser instances with the engine.
    fn push_token_parsers<I>(&mut self, iterable: I) -> Result<(), ExtensionRegistryError> where
        I: IntoIterator<Item=(String, Box<ext::TokenParser>)>
    {
        for (k, v) in iterable {
            // #NOTE:60 can't have a reference to something owned within the same struct
            // and don't want to clone!
            //
            // if let Some(prev) = self._token_parser_by_tags.insert(v.tag().to_string(), &v) {
            //     return err!(ExtensionRegistryErrorCode::DuplicateTagHandler {
            //         prev: prev,
            //         ext_name: ext.name()
            //     })
            // }

            if let Some(prev) = self.staged.token_parsers.insert(k, v) {
                return err!(ExtensionRegistryErrorCode::DuplicateTokenParser {
                    prev: prev
                })
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Register node visitor instances with the engine.
    fn push_node_visitors<I>(&mut self, iterable: I) -> Result<(), ExtensionRegistryError> where
        I: IntoIterator<Item=Box<ext::NodeVisitor>>
    {
        for v in iterable {
            self.staged.node_visitors.push(v)
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Register filters with the engine.
    fn push_filters<I>(&mut self, iterable: I) -> Result<(), ExtensionRegistryError> where
        I: IntoIterator<Item=(String, Box<ext::Filter>)>
    {
        for (k, v) in iterable {
            if let Some(prev) = self.staged.filters.insert(k, v) {
                return err!(ExtensionRegistryErrorCode::DuplicateFilter {
                    prev: prev
                })
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Register tests with the engine.
    fn push_tests<I>(&mut self, iterable: I) -> Result<(), ExtensionRegistryError> where
        I: IntoIterator<Item=(String, Box<ext::Test>)>
    {
        for (k, v) in iterable {
            if let Some(prev) = self.staged.tests.insert(k, v) {
                return err!(ExtensionRegistryErrorCode::DuplicateTest {
                    prev: prev
                })
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Register functions with the engine.
    fn push_functions<I>(&mut self, iterable: I) -> Result<(), ExtensionRegistryError> where
        I: IntoIterator<Item=(String, Box<ext::Function>)>
    {
        for (k, v) in iterable {
            if let Some(prev) = self.staged.functions.insert(k, v) {
                return err!(ExtensionRegistryErrorCode::DuplicateFunction {
                    prev: prev
                })
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Register unary operators with the engine.
    fn push_operators_unary<I>(&mut self, iterable: I) -> Result<(), ExtensionRegistryError> where
        I: IntoIterator<Item=ext::UnaryOperator>
    {
        for v in iterable {
            if let Some(prev) = self.staged.operators_unary.insert(v.repr.clone(), v) {
                return err!(ExtensionRegistryErrorCode::DuplicateOperatorUnary {
                    prev: prev
                })
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Register binary operators with the engine.
    fn push_operators_binary<I>(&mut self, iterable: I) -> Result<(), ExtensionRegistryError> where
        I: IntoIterator<Item=ext::BinaryOperator>
    {
        for v in iterable {
            if let Some(prev) = self.staged.operators_binary.insert(v.repr.clone(), v) {
                return err!(ExtensionRegistryErrorCode::DuplicateOperatorBinary {
                    prev: prev
                })
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Register global variables with the engine.
    fn push_globals<I>(&mut self, _iterable: I) -> Result<(), ExtensionRegistryError> where
        I: IntoIterator<Item=Box<ext::Global>>
    {
        unimplemented!()
    }
}

impl Into<ExtensionRegistry> for Builder {
    fn into(self) -> ExtensionRegistry {
        self.staged
    }
}
