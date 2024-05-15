use std::fmt::{Display, Formatter, Write};

/// A shorthand builder for `AnsiSeq::Move`
/// ```no_run
/// use tty_overwriter::prelude::Movement;
/// let movement = Movement::new().down(3).left(2);
/// ```
#[derive(Default, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash, Debug)]
pub struct Movement {
    up: u16,
    down: u16,
    left: u16,
    right: u16,
}

impl From<&Movement> for AnsiSeq {
    fn from(value: &Movement) -> Self {
        Self::Move {
            up: value.up,
            down: value.down,
            left: value.left,
            right: value.right,
        }
    }
}

impl Display for Movement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <&Movement as Into<AnsiSeq>>::into(self).fmt(f)
    }
}

impl Movement {
    /// empty constructor
    pub fn new() -> Self {
        Default::default()
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn up(self, up: u16) -> Self {
        Self { up, ..self }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn down(self, down: u16) -> Self {
        Self { down, ..self }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn right(self, right: u16) -> Self {
        Self { right, ..self }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn left(self, left: u16) -> Self {
        Self { left, ..self }
    }
}

/// A list of Ansi sequences.
/// Thought to be written to `std::io::Write`.
/// ```
/// use tty_overwriter::prelude::AnsiSeq;
/// print!("Hello, word!");
/// // we did a spelling mistake
/// let left = AnsiSeq::AbsoluteMove {horizontal: 0};
/// println!("{left}Hello, world!")
/// ```
/// Codes provided by [Hand Wiki](https://handwiki.org/wiki/ANSI_escape_code#Colors)
/// For ease of debugging, if cfg(test) then `AnsiSeq::fmt` will actually
/// display the code.
#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash, Debug)]
pub enum AnsiSeq {
    /// Move in straight line. Applied in order:  Up, Left, Down, Right.
    Move {
        /// Move n spaces right
        up: u16,
        /// Move n spaces down
        down: u16,
        /// Move n spaces left
        left: u16,
        /// Move n spaces right
        right: u16,
    },
    /// Move vertically to the beginning of lines. Applied in order: Up, Down.
    MoveLines {
        /// Move n lines up
        up: u16,
        /// Move n lines down
        down: u16,
    },
    /// toggle OFF all styles
    ResetStyle,
    /// toggle ON the Underline style
    Underline,
    /// clear the part of the line on the right of the cursor
    ClearCursorToEndOfLine,
    /// clear the part of the line on the left of the cursor
    ClearCursorToBeginningOfLine,
    /// Clear the line of the terminal on which is the cursor
    ClearLine,
    /// Clear the terminal under the cursor
    ClearCursorToEndOfScreen,
    /// Clear the terminal over the cursor
    ClearCursorToBeginningOfScreen,
    /// Clear the entirety of the terminal
    ClearAllScreen,
    /// Show and hide mouse cursor
    ShowAndHideCursor {
        /// should or shouldn't show the cursor
        show: bool,
    },
    /// Set the cursor horizontally
    AbsoluteMove {
        /// horizontal position of the cursor
        horizontal: u16,
    },
}

fn encode(f: &mut Formatter<'_>, code: &str) -> std::fmt::Result {
    #[cfg(not(test))]
    f.write_char(27 as char)?;
    #[cfg(test)]
    f.write_str("\\e")?;
    f.write_char('[')?;
    f.write_str(code)?;
    Ok(())
}

impl Display for AnsiSeq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Move {
                up,
                down,
                left,
                right,
            } => {
                if 0 < *up {
                    encode(f, &format!("{up}A"))?;
                }
                if 0 < *left {
                    encode(f, &format!("{left}D"))?;
                }
                if 0 < *down {
                    encode(f, &format!("{down}B"))?;
                }
                if 0 < *right {
                    encode(f, &format!("{right}C"))?;
                }
            }
            Self::MoveLines { up, down } => {
                if 0 < *up {
                    encode(f, &format!("{up}F"))?;
                }
                if 0 < *down {
                    encode(f, &format!("{down}E"))?;
                }
            }
            Self::ResetStyle => encode(f, "0m")?,
            Self::Underline => encode(f, "4m")?,
            Self::ClearCursorToEndOfLine => encode(f, "0K")?,
            Self::ClearCursorToBeginningOfLine => encode(f, "1K")?,
            Self::ClearLine => encode(f, "2K")?,
            Self::ClearAllScreen => encode(f, "2J")?,
            Self::ClearCursorToBeginningOfScreen => encode(f, "1J")?,
            Self::ClearCursorToEndOfScreen => encode(f, "0J")?,
            Self::ShowAndHideCursor { show } => encode(f, if *show { "?25h" } else { "?25l" })?,
            Self::AbsoluteMove { horizontal } => encode(f, &format!("{horizontal}G"))?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_up() {
        let seq = AnsiSeq::Move {
            up: 3,
            down: 0,
            left: 0,
            right: 0,
        };

        assert_eq!(&format!("{seq}"), "\\e[3A")
    }
}
