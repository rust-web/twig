// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! The Twig Engine.

mod template_cache;
use loader::Loader;
use std::rc::Rc;
use template;
use error::ErrorCode;
use setup::Setup;

pub mod error;
pub mod options;
pub mod extension_registry;
pub mod parser;
pub mod node;
pub use self::node::Node;
pub use self::parser::{Parser, lexer, Lexer};
pub use self::error::{TwigError, TwigErrorCode, ExtensionRegistryError, ExtensionRegistryErrorCode};
pub use self::options::Options;
pub use self::extension_registry::ExtensionRegistry;


#[derive(Debug)]
pub struct Engine {
    options: Options,
    ext: Rc<ExtensionRegistry>,
    loader: Option<Box<Loader>>,
    lexer: Option<Lexer>,
    parser: Option<Parser>,
}


impl Engine {
    /// Create a new Twig `Engine`.
    ///
    /// Note: You may want to
    ///
    /// # Examples
    ///
    /// ```
    /// use twig::engine::{Engine, Options, ExtensionRegistry};
    ///
    /// let twig = Engine::new(Options::default(), ExtensionRegistry::default());
    /// ```
    ///
    /// # Altnernative
    ///
    /// ```
    /// use twig::Setup;
    ///
    /// let twig = Setup::default().engine().unwrap();
    /// ```
    pub fn new(options: Options, ext: ExtensionRegistry) -> Self {
        Engine {
            options: options,
            ext: Rc::new(ext),
            loader: None,
            lexer: None,
            parser: None,
        }
    }

    /// Renders a template.
    ///
    /// # Failures
    /// * When the template cannot be found
    /// * When an error occurred during compilation
    /// * When an error occurred during rendering
    pub fn render(&mut self, _path: &str, _data: ()) -> Result<String, TwigError> {
        unimplemented!()
    }

    /// Displays a template.
    ///
    /// # Failures
    /// * When the template cannot be found
    /// * When an error occurred during compilation
    /// * When an error occurred during rendering
    pub fn display(&mut self, _path: &str, _data: ()) -> Result<(), TwigError> {
       unimplemented!()
    }

    /// Loads and compiles a template.
    ///
    /// # Failures
    /// * When the template cannot be found
    /// * When an error occurred during compilation
    pub fn load_template(&mut self, path: &str, _index: Option<u32>) -> Result<template::Compiled, TwigError> {
        // TODO: Cache compiled templates
        //  * cache lookup
        //  * check if cache is fresh
        //  * store in cache

        let template_raw = try!(self.load_template_raw(path));
        return self.compile_template(&template_raw);
    }

    /// Loads raw template.
    ///
    /// # Failures
    /// * When the template cannot be found
    fn load_template_raw(&mut self, path: &str) -> Result<template::Raw, TwigError> {
        let loader = try!(self.loader());
        let source = try_chain!(loader.source(path));
        Ok(template::Raw::new(source, path))
    }

    /// Compiles a template.
    ///
    /// # Failures
    /// * When an error occurred during lexing or parsing.
    fn compile_template(&mut self, template: &template::Raw) -> Result<template::Compiled, TwigError> {
        let tokenstream = {
            let lexer = try!(self.lexer());
            try_chain!(lexer.tokenize(template))
        };

        let compiled = {
            let parser = try!(self.parser());
            try_chain!(parser.parse(&tokenstream))
        };

        Ok(compiled)
    }

    /// Get the engine extensions.
    pub fn extensions(&self) -> &ExtensionRegistry {
        &self.ext
    }

    /// Sets the loader instance.
    pub fn set_loader(&mut self, loader: Box<Loader>) -> &mut Engine {
        self.loader = Some(loader); // TODO: switch to callback pattern to provide arguments

        self
    }

    /// Get the loader instance.
    pub fn loader(&mut self) -> Result<&mut Loader, TwigError> {
        match self.loader {
            Some(ref mut loader) => return Ok(&mut **loader),
            None => {
                return err!(TwigErrorCode::LoaderNotInitialized)
            }
        }
    }

    /// Get the lexer instance.
    pub fn lexer(&mut self) -> Result<&Lexer, TwigError> {
        match self.lexer {
            Some(ref lexer) => return Ok(lexer),
            None => {
                self.lexer = Some(try_chain!(Lexer::new(self, lexer::Options::default())));
                return self.lexer();
            }
        }
    }

    /// Get the parser instance.
    pub fn parser(&mut self) -> Result<&Parser, TwigError> {
        match self.parser {
            Some(ref parser) => return Ok(parser),
            None => {
                self.parser = match Parser::new(&self) {
                    Err(e) => return Err(TwigErrorCode::Parser
                        .at(loc!())
                        .caused_by(e)),
                    Ok(parser) => Some(parser)
                };
                return self.parser();
            }
        }
    }
}

// NOTE: `derive(Default)` would not initialize any extensions
impl Default for Engine {
    fn default() -> Engine {
        Setup::default().engine().unwrap()
    }
}
