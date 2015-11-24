// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig Error API for error code conversion 

// will be used to transform error codes!
// can't use Into-trait - because we only have references
pub trait GeneralizeTo<T> {
    fn generalize(&self) -> T;
}
