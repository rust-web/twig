// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Stores the Twig configuration.

use std::path::Path;
use extension;
use api::Extension;
use engine::{Engine, options, Options, ExtensionRegistry};
use engine::error::{TwigError};

#[allow(dead_code)]
pub const VERSION : &'static str = "1.18.1";

#[derive(Debug)]
pub struct Setup {
    opt: Options,
    ext: Vec<Box<Extension>>,
}

impl Default for Setup {
    fn default() -> Setup {
        // prepend default extensions
        let mut ext: Vec<Box<Extension>> = vec![];
        ext.push(extension::Core::new());

        Setup {
            opt: Options::default(),
            ext: ext,
        }
    }
}

/// Builds an instance of the Twig Engine, according to supplied options and engine extensions.
///
/// The following extensions will be registered by default:
/// * core
/// * escaper
/// * optimizer
///
/// # Examples
///
/// ```
/// use twig::Setup;
/// use twig::extension::Debug;
///
/// let twig = Setup::default()
///     .set_strict_variables(true)
///     .add_extension(Debug::new())
///     .init_engine()
///     .unwrap();
/// ```
#[allow(dead_code)]
impl Setup {
    /// Create engine from setup.
    ///
    /// # Examples
    ///
    /// ```
    /// use twig::Setup;
    ///
    /// let twig = Setup::default().init_engine().unwrap();
    /// ```
    pub fn init_engine(self) -> Result<Engine, TwigError> {
        let Setup { opt, mut ext } = self;

        // append default extensions
        ext.push(extension::Escaper::new(opt.auto_escape()));
        ext.push(extension::Optimizer::new(opt.optimizations()));

        // init extensions
        let extension_registry = try_chain!(ExtensionRegistry::new(ext, &opt));

        Ok(Engine::new(extension_registry, opt))
    }

    /// Registers an extension
    pub fn add_extension(mut self, extension: Box<Extension>) -> Self {
        self.ext.push(extension);

        self
    }

    /// When set to true, it automatically set "auto_reload" to true as well
    ///     (default to false)
    pub fn set_debug(mut self, debug: bool) -> Self {
        self.opt.set_debug(debug);

        self
    }

    /// Whether to ignore invalid variables in templates
    ///     (default to false).
    pub fn set_strict_variables(mut self, strict_variables: bool) -> Self {
        self.opt.set_strict_variables(strict_variables);

        self
    }

    /// Whether to enable auto-escaping (default to html):
    ///     * false: disable auto-escaping
    ///     * true: equivalent to html
    ///     * html, js: set the autoescaping to one of the supported strategies
    ///     * filename: set the autoescaping strategy based on the template filename extension
    ///     * PHP callback: a PHP callback that returns an escaping strategy based on the template "filename"
    pub fn set_auto_escape(mut self, auto_escape: options::AutoEscape) -> Self {
        self.opt.set_auto_escape(auto_escape);

        self
    }

    /// An absolute path where to store the compiled templates (optional)
    pub fn set_cache(mut self, cache: Option<&Path>) -> Self {
        self.opt.set_cache(cache.map(|reference| reference.to_owned()));

        self
    }

    /// Whether to reload the template if the original source changed (optional).
    ///     If you don't provide the auto_reload option, it will be
    ///     determined automatically based on the debug value.
    pub fn set_auto_reload(mut self, auto_reload: bool) -> Self {
        self.opt.set_auto_reload(auto_reload);

        self
    }

    /// A flag that indicates whether optimizations are applied
    pub fn set_optimizations(mut self, optimizations: options::Optimizations) -> Self {
        self.opt.set_optimizations(optimizations);

        self
    }

    /// Get all options
    pub fn options(&self) -> &Options {
        &self.opt
    }

    /// Get all extensions
    pub fn extensions(&self) -> &Vec<Box<Extension>> {
        &self.ext
    }
}
