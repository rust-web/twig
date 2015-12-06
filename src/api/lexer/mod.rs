// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/*!
Twig lexer.

Produces a token stream from source template.

# Summary

This module is capable of taking a Twig input template, for example, this one:

```twig
Hello
{% if world %}
    world
{% else %}
    {{ other }}
{% endif %}
```

And chopping it into tokens like these:

TODO: Example code for this.
*/

mod lexing;

pub use self::lexing::{ Tokens, ItemRef };

#[derive(Copy, Clone)]
pub struct Options;

impl Options {
    pub fn default() -> Options { Options }
}

pub struct Lexer;

impl Lexer {
    /// Create a new lexer from options and operator list.
    pub fn new(options: Options, operators: Vec<&'static str>) -> Lexer {
        Lexer
    }

    /// Get a lexed stream of tokens.
    pub fn tokens<'i, 't>(&'i self, code: &'t str) -> Tokens<'i, 't> {
        Tokens::new(self, code)
    }
}
