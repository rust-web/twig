// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Escaper Extension

use api::Extension;
use api::error::Traced;
use engine;

#[derive(Default, Debug, PartialEq)]
pub struct Escaper {
    mode: Mode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Html,
    _Enabled,
    _Disabled,
    _Filename,
    _Callback,
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Html
    }
}

impl Extension for Escaper {
    fn name(&self) -> &'static str { "escaper" }

    fn init(&mut self, _registry: &mut engine::extension_registry::Builder, _options: &engine::Options)
        -> Result<(), Traced<engine::ExtensionRegistryError>> {
        // unimplemented!()

        Ok(())
    }
}

impl Escaper {
    pub fn new(mode: Mode) -> Box<Escaper> {
        Box::new(Escaper {
            mode: mode
        })
    }
}
