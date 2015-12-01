// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! # Twig Templating for Rust
//!
//! **Work in progress** - This library is still in development and not yet ready for use.
//! Take a look at the [CHANGELOG][changelog] for more details.
//!
//! [Twig Rust][github] is a template engine for everyone writing web applications with Rust.
//! It is derived from [Twig (for PHP)][twigphp] and intended to become a _fully compatible_ port - as far as it makes sense.
//!
//! By design Twig is
//!
//!   * flexible
//!   * fast
//!   * and secure
//!
//! ## Getting Started
//!
//! ```
//! use twig::{Engine, Setup};
//!
//! let twig = Engine::new(Setup::default()).unwrap();
//! // ..
//! ```
//!
//! ## Syntax and Semantics
//!
//! Twig uses a syntax similar to the Django and Jinja template languages which inspired the Twig runtime environment.
//!
//! ```html
//! <!DOCTYPE html>
//! <html>
//!     <head>
//!         <title>Display a thread of posts</title>
//!     </head>
//!     <body>
//!     <h1>{{ thread.title }}</h1>
//!     <ul>
//!       {% for post in thread.posts %}
//!         <li>{{ post }}</li>
//!       {% endfor %}
//!     </ul>
//!     {# note: this comment will be ignored #}
//!     </body>
//! </html>
//! ```
//!
//! Take a look at this introduction: [Twig for template designers](http://twig.sensiolabs.org/doc/templates.html).
//!
//! ## Flexible Architecture
//!
//! Twig is designed to be highly extensible:
//!
//!   * the Twig compiler only defines *general semantics* and a very flexible *extension mechanism*.
//!   * extensions define specific behavior and data transformations (like if-statement, for-loop, escape-filter, multiplication-operator, call-expression, etc.)
//!   * extensions are chosen at runtime.
//!   * if you don't like the default behavior (like if-statement, or call-expression) or if you are missing some functionality (like helpers for a new target like excel-files), all you need to do is replace, add or change extensions.
//!
//! ## License
//!
//! Twig-Rust is released under the [new BSD license][license] (code and documentation) - as is the original Twig for PHP.
//!
//! [github]: https://github.com/rust-web/twig
//! [license]: https://github.com/rust-web/twig/blob/master/LICENSE
//! [changelog]: https://github.com/rust-web/twig/blob/master/CHANGELOG.md
//! [twigphp]: http://twig.sensiolabs.org/documentation

extern crate regex;

#[macro_use] pub mod error; // should be first :-)
pub mod engine;
pub mod extension;
pub mod loader;
pub mod template;

pub use engine::Engine;
pub use engine::Setup;

#[cfg(test)]
mod test_error;

#[test]
fn it_works() {
}
