use std::fmt::{Display, Formatter, Write};

#[derive(Default)]
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
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn up(self, up: u16) -> Self {
        Self { up, ..self }
    }

    #[inline]
    pub fn down(self, down: u16) -> Self {
        Self { down, ..self }
    }
    #[inline]

    pub fn right(self, right: u16) -> Self {
        Self { right, ..self }
    }
    #[inline]
    pub fn left(self, left: u16) -> Self {
        Self { left, ..self }
    }
}

/// Codes provided by https://handwiki.org/wiki/ANSI_escape_code#Colors
#[derive(Copy, Clone, Debug)]
pub enum AnsiSeq {
    Move {
        up: u16,
        down: u16,
        left: u16,
        right: u16,
    },
    MoveLines {
        up: u16,
        down: u16,
    },
    ResetStyle,
    Underline,
    ClearCursorToEndOfLine,
    ClearCursorToBeginningOfLine,
    ClearLine,
    ClearCursorToEndOfScreen,
    ClearCursorToBeginningOfScreen,
    ClearAllScreen,
    ShowAndHideCursor {
        show: bool,
    },
    AbsoluteMove {
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
