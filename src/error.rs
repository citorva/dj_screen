use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    BufferSizeError { given: usize, expected: usize },
    InvalidScreen { screen_id: u8, screen_number: u8 },
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BufferSizeError { given, expected } => {
                write!(f, "The buffer size must be {expected}, given {given}")
            }
            Error::InvalidScreen {
                screen_id,
                screen_number,
            } => {
                write!(f, "Invalid screen. Given {screen_id}, must be between 0 and {screen_number} (not included)")
            }
        }
    }
}

impl Error {
    pub fn check_length(buf: &[u8], expected: usize) -> Result<()> {
        if buf.len() == expected {
            Ok(())
        } else {
            Err(Error::BufferSizeError {
                given: buf.len(),
                expected,
            })
        }
    }

    // pub(crate) fn check_screen(screen : u8, expected : u8) -> Result<()> {
    //     if screen < expected {
    //         Ok(())
    //     } else {
    //         Err(Error::InvalidScreen {
    //             screen_id: screen,
    //             screen_number: expected
    //         })
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    #[test]
    fn test_check_length() {
        assert_eq!(Error::check_length(&[0, 1, 2, 3, 4, 5], 6), Ok(()));
        assert_eq!(
            Error::check_length(&[0, 1, 2, 3, 4, 5], 5),
            Err(Error::BufferSizeError {
                expected: 5,
                given: 6
            })
        );
        assert_eq!(
            Error::check_length(&[0, 1, 2, 3, 4], 6),
            Err(Error::BufferSizeError {
                expected: 6,
                given: 5
            })
        );
    }

    // #[test]
    // fn test_check_screen() {
    //     assert_eq!(Error::check_screen(5, 8), Ok(()));
    //     assert_eq!(Error::check_screen(6, 6), Err(Error::InvalidScreen { screen_id: 6, screen_number: 6 }));
    //     assert_eq!(Error::check_screen(12, 5), Err(Error::InvalidScreen { screen_id: 12, screen_number: 5 }));
    // }
}
