pub mod ansi_seq;
pub mod body;

pub mod prelude {
    pub use crate::ansi_seq::*;
    pub use crate::body::*;
}

#[cfg(test)]
mod tests {}
