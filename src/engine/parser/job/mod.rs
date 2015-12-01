// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! A parser job.

pub mod cursor;
pub use self::cursor::Cursor;

pub struct Job;
pub type JobDump = String;
