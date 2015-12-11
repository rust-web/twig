// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Options of the parser.

#[derive(Debug, PartialEq)]
pub struct Options;

//... should these Options structures have public members, or not?
// PRO: Public members would allow composition / decomposition outside of the respective module without getter/setter-bloat.
// CON: I think public members should not be mixed with logic - i.e. we shouldn't have any impl Options {...} in this case.
//
// My tendency is to use opaque structs with setters+getters
