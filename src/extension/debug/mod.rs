// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Debug Extension.

use api::Extension;
use engine;

#[allow(dead_code)] // dummy
#[derive(Default, Debug, PartialEq)]
pub struct Debug;

impl Extension for Debug {
    fn name(&self) -> &'static str { "debug" }

    fn init(&mut self, _registry: &mut engine::extension_registry::Builder, _options: &engine::Options)
        -> Result<(), engine::ExtensionRegistryError> {
        // unimplemented!()

        Ok(())
    }
}

impl Debug {
    pub fn new() -> Box<Debug> {
        Box::new(Debug)
    }
}
