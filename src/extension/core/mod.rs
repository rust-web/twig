// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Core Extension.

use extension::api::Extension;

#[allow(dead_code)] // dummy
#[derive(Default, Debug, PartialEq)]
pub struct Core;

impl Extension for Core {
    fn name(&self) -> &'static str { "core" }
}

impl Core {
    pub fn new() -> Box<Core> {
        Box::new(Core::default())
    }
}
