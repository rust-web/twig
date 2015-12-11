// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Core Extension.

use api::Extension;
use engine;

#[allow(dead_code)] // dummy
#[derive(Default, Debug, PartialEq)]
pub struct Core;

impl Extension for Core {
    fn name(&self) -> &'static str { "core" }

    fn init(&mut self, _registry: &mut engine::extension_registry::Builder, _options: &engine::Options)
        -> Result<(), engine::ExtensionRegistryError> {
        // unimplemented!()
        Ok(())
    }
}

impl Core {
    pub fn new() -> Box<Core> {
        Box::new(Core::default())
    }
}
