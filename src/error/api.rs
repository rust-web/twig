// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Rust macro
///
/// @author Colin Kiegel <kiegel@gmx.de>

/////////////
// imports //
/////////////



/////////////
// exports //
/////////////


// will be used to transform error codes!
// can't use Into-trait - because we only have references
pub trait GeneralizeTo<T> {
    fn generalize(&self) -> T;
}
