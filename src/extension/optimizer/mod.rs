// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Optimizer Extension

use api::Extension;
use api::error::Traced;
use engine;

#[derive(Default, Debug, PartialEq)]
pub struct Optimizer {
    mode: Mode
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Enabled,
    _Disabled,
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Enabled
    }
}

impl Extension for Optimizer {
    fn name(&self) -> &'static str { "optimizer" }

    fn init(&mut self, _registry: &mut engine::extension_registry::Builder, _options: &engine::Options)
        -> Result<(), Traced<engine::ExtensionRegistryError>> {
        // unimplemented!()

        Ok(())
    }
}

impl Optimizer {
    pub fn new(mode: Mode) -> Box<Optimizer> {
        Box::new(Optimizer {
            mode: mode
        })
    }
}
