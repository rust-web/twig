// This file is part of Twig (ported to Rust).
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

#[macro_use]
pub mod error;

#[cfg(test)]
mod test {
    use tokens::{ Lexer, LexerOptions };

    #[test]
    fn lexer_usage() {
        // build the lexer once for project environment with extensions.
        let lexer = Lexer::new(LexerOptions::default(), vec![]);

        // use many times.
        {
            let source: String = "{{ var }}".into();
            for token in lexer.tokens(&source) {
                println!("{:?}", token);
            }
        }
    }
}

// I don't know where to put this, keeping it in root for now.
#[derive(Debug, Default, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

// Named this "tokens", in plural, to mean a place where you should expect
// to find your tokens.
// Similar convention would work for "nodes", that's where AST lives, and possibly
// "instructions", if we decide to go that route.
pub mod tokens {

    // It is possible to delay all string manipulation for later,
    // and we can simply store the slices into original full source
    // string.
    //
    // We can keep doing that even for Node<'a>, provided the strings remain untouched.
    // If something needs to be changed, we can create a special Node for that.
    #[derive(Debug)]
    pub enum TokenRef<'a> {
        Text(&'a str),
    }

    impl<'a> TokenRef<'a> {
        // Not used "into", because Gankro criticises using into for anything more than
        // moving data around. Not used into_owned, because we don't implement ToOwned trait.
        // So the only logical name remains `into_token`.
        pub fn into_token(self) -> Token {
            match self {
                TokenRef::Text(v) => Token::Text(v.into()),
            }
        }
    }

    // This will be used when we need to carry token lifetime longer than original
    // source string, for example, in error messages.
    pub enum Token {
        Text(String),
    }

    // Not pub, to make API more convenient.
    mod lexing {
        use Position;
        use tokens::TokenRef;



        /// TokenRef wrapper for `Lexer` that additionaly has position.
        #[derive(Debug)]
        pub struct ItemRef<'t> {
            pub token: TokenRef<'t>,
            pub position: Position,
        }



        // TBD simple lexer options (delimiters, whitespace, etc).
        #[derive(Copy, Clone)]
        pub struct Options;

        impl Options {
            pub fn default() -> Options { Options }
        }



        // I will be refering to 't as template lifetime, 'i as iteration lifetime.
        // This lexer should be reusable between the `tokenize` calls.
        // In addition to this I had `LexingEnvironment`, but it turned out to be redundant.
        pub struct Lexer;

        impl Lexer {
            // It's responsibility of someone else to take operators from extensions,
            // resolve any conflicts and compile final "operators" list.
            //
            // It looks like Lexer does not care if they are unary or binary, that will
            // become important in parser.
            //
            // Funny note: I found that "=" is considered neither unary nor binary ;)
            pub fn new(options: Options, operators: Vec<&'static str>) -> Lexer {
                // Here we will create patterns (I called them matchers), and
                // store them in Lexer
                Lexer
            }

            // twig-rust: https://github.com/colin-kiegel/twig-rust/blob/master/src/lexer/mod.rs#L64
            // twig-rs: https://github.com/Nercury/twig-rs/blob/master/src/tokens/lexer/mod.rs#L40
            //
            // I think it is possible to avoid the Template::Raw in lexer API.
            // We can probably deal with newlines in patterns?
            // Also maybe we won't need to fix line endings, but right now we both do that.
            //
            // twig-rs result was "Iter", twig-rust - "Job" :)
            //
            // I changed it to comcrete "Tokens" for now, which will implement Iterator.
            // No Result. Let's avoid lexing until Parser requests first token.
            pub fn tokens<'i, 't>(&'i self, code: &'t str) -> Tokens<'i, 't> {
                // Just take whole lexer by reference ;)
                Tokens::new(self, code)
            }
        }



        // 'i is iteration lifetime, or "one use of lexer".
        // 't is template lifetime. It will live longer than this iteration.
        pub struct Tokens<'i, 't> {
            env: &'i Lexer,
            code: &'t str,
        }

        impl<'i, 't> Tokens<'i, 't> {

            pub fn new<'ii, 'tt>(lexer: &'ii Lexer, code: &'tt str) -> Tokens<'ii, 'tt> {
                Tokens {
                    env: lexer,
                    code: code,
                }
            }
        }

        // I think we can avoid storing all tokens in Vec, instead just keep in memory the next
        // chunk of lexed tokens.
        impl<'i, 't> Iterator for Tokens<'i, 't> {
            // TODO: Use proper Result once we merge error handling.
            type Item = Result<ItemRef<'t>, ()>;

            fn next(&mut self) -> Option<Result<ItemRef<'t>, ()>> {

                return None;
            }
        }

    }

    pub use self::lexing::{
        Lexer,
        Tokens,
        ItemRef as LexerItemRef,
        Options as LexerOptions,
    };
}
