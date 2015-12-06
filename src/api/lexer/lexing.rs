// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use api::Position;
use api::tokens::TokenRef;

use api::lexer::Lexer;
use api::error::{ SyntaxErrorCode, UnexpectedToken, LexingResult };

/// TokenRef wrapper for `Lexer` that additionaly has position.
#[derive(Debug)]
pub struct ItemRef<'t> {
    pub token: TokenRef<'t>,
    pub position: Position,
}

/// Lexer token iterator.
///
/// 'i is iteration lifetime, or "one use of lexer".
/// 't is template lifetime. It will live longer than this iteration.
pub struct Tokens<'i, 't> {
    /// Position of the next token to get.
    next_pos: Position, // temporary field until I get cursor in.
    env: &'i Lexer,
    code: &'t str,
}

impl<'i, 't> Iterator for Tokens<'i, 't> {
    type Item = LexingResult<ItemRef<'t>>;

    fn next(&mut self) -> Option<LexingResult<ItemRef<'t>>> {
        // Hello, my name is Lexer. Twig Lexer.
        // I am not very complicated.
        match self.next_pos {
            Position { line: 1, .. } => {
                self.next_pos.line = 2;
                Some(Ok(ItemRef { token: TokenRef::BlockStart, position: self.next_pos }))
            },
            Position { line: 2, .. } => {
                self.next_pos.line = 3;
                Some(Ok(ItemRef { token: TokenRef::Name("§"), position: self.next_pos }))
            },
            _ => None
        }
    }
}

impl<'i, 't> Tokens<'i, 't> {
    pub fn new<'ii, 'tt>(lexer: &'ii Lexer, code: &'tt str) -> Tokens<'ii, 'tt> {
        Tokens {
            next_pos: Position { line: 1, column: 1 },
            env: lexer,
            code: code,
        }
    }

    pub fn expect(&mut self, expected: TokenRef<'t>) -> LexingResult<TokenRef<'t>> {
        let maybe_item = self.next();
        match maybe_item {
            None => Err(
                SyntaxErrorCode::ExpectedTokenButReceived {
                    expected: expected.into(),
                    received: UnexpectedToken::EndOfStream
                }.starts_at(self.next_pos).into()
            ),
            Some(Ok(item)) => {
                if item.token == expected {
                    Ok(item.token)
                } else {
                    Err(
                        SyntaxErrorCode::ExpectedTokenButReceived {
                            expected: expected.into(),
                            received: UnexpectedToken::Token(item.token.into())
                        }.starts_at(item.position).into()
                    )
                }
            },
            Some(Err(error)) => Err(error),
        }
    }
}
