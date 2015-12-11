// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Expression parser.

use engine::ExtensionRegistry;
use std::rc::Rc;

#[derive(Debug)]
pub struct Expression; // dummy

#[derive(Debug)]
#[allow(dead_code)]
pub struct ExpressionParser {
    ext: Rc<ExtensionRegistry>,
}
