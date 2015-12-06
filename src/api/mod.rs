// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/*!
Twig extension writer's API.
*/

pub mod tokens;
pub mod lexer;
pub mod error;

/// Line-column position in a file.
#[derive(Debug, Default, Copy, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}