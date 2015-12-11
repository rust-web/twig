// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig configuration options.

use std::path::{Path, PathBuf};
use extension::escaper;
use extension::optimizer;

pub type AutoEscape = escaper::Mode;
pub type Optimizations = optimizer::Mode;


#[derive(Debug)]
pub struct Options {
    debug: bool,
    strict_variables: bool,
    auto_escape: AutoEscape,
    cache: Option<PathBuf>,
    auto_reload: Option<bool>, // defaults to `self.debug` if unset
    optimizations: Optimizations,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            debug: false,
            strict_variables: false,
            auto_escape: escaper::Mode::default(),
            cache: None,
            auto_reload: None,
            optimizations: optimizer::Mode::default(),
        }
    }
}

impl Options {
    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn strict_variables(&self) -> bool {
        self.strict_variables
    }

    pub fn set_strict_variables(&mut self, strict_variables: bool) {
        self.strict_variables = strict_variables;
    }

    pub fn auto_escape(&self) -> AutoEscape {
        self.auto_escape
    }

    pub fn set_auto_escape(&mut  self, auto_escape: AutoEscape) {
        self.auto_escape = auto_escape;
    }

    pub fn cache(&self) -> Option<&Path> {
        // TODO: why doesn't this work? -> self.cache.map(|ref buf| buf.as_ref())
        match self.cache {
            Some(ref buf) => Some(buf.as_ref()),
            None => None
        }
    }

    pub fn set_cache(&mut self, cache: Option<PathBuf>) {
        self.cache = cache;
    }

    /// if unset it defaults to `self.debug()`
    pub fn auto_reload(&self) -> bool {
        self.auto_reload.unwrap_or(self.debug)
    }

    pub fn set_auto_reload(&mut self, auto_reload: bool) {
        self.auto_reload = Some(auto_reload);
    }

    pub fn optimizations(&self) -> Optimizations {
        self.optimizations
    }

    pub fn set_optimizations(&mut self, optimizations: Optimizations) {
        self.optimizations = optimizations;
    }
}
