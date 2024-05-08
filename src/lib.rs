pub mod ansi_seq;

pub mod prelude {
    pub use crate::ansi_seq::*;
}

#[cfg(test)]
mod tests {}
