// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig Extensions
//!
//! Define new behavior during the compilation process.
//!
//! # API
//!
//! See `twig::api::ext` for details on implementing new extensions.

pub mod core;
pub mod debug;
pub mod escaper;
pub mod optimizer;
pub use self::core::Core;
pub use self::debug::Debug;
pub use self::escaper::Escaper;
pub use self::optimizer::Optimizer;
