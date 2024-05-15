#![deny(missing_docs)]

//! TTY Overwriter
//! the crate is a small library consisting of two modules :
//! `ansi_seq` and `̀body`
//! ansi_seq are ansi_sequences to be Written.
//! body is a small tool which uses ansi sequences to rewrite text to a terminal in a loop without flickering.

/// Easy Ansi Sequences code
pub mod ansi_seq;
/// A tool to overwrite to terminal without flickering
pub mod body;

/// The prelude of the lib, to easily include all you might want to include from the lib
pub mod prelude {
    pub use crate::ansi_seq::*;
    pub use crate::body::*;
}

