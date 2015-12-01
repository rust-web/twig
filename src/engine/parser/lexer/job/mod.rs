// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! A lexer job - modeled as a FSM (Finite State Machine).

pub mod cursor;
pub use self::cursor::Cursor;

// Finite State Machine loosely inspired by
// * http://www.huffingtonpost.com/damien-radtke/rustic-state-machines-for_b_4466566.html

pub struct Job;
