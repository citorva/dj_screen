use crate::vendor::Driver;
use rusb::UsbContext;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    BufferSizeError {
        given: usize,
        expected: usize,
    },
    InvalidScreen {
        screen_id: usize,
        screen_number: usize,
    },
    UnsupportedDevice {
        vendor_id: u16,
        product_id: u16,
        driver: &'static str,
    },
    BusyScreen {
        screen_id: usize,
    },
    UsbError(rusb::Error),
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
            Error::UsbError(error) => Display::fmt(error, f),
            Error::UnsupportedDevice {
                vendor_id,
                product_id,
                driver,
            } => {
                write!(f, "Trying to open a device with id {vendor_id}:{product_id} from the driver `{driver}` which is not made for")
            }
            Error::BusyScreen { screen_id } => {
                write!(
                    f,
                    "The screen with id {screen_id} is already used bay an other object"
                )
            }
        }
    }
}

impl Error {
    pub(crate) fn check_length(buf: &[u8], expected: usize) -> Result<()> {
        if buf.len() == expected {
            Ok(())
        } else {
            Err(Error::BufferSizeError {
                given: buf.len(),
                expected,
            })
        }
    }

    pub(crate) fn check_screen(screen: usize, expected: usize) -> Result<()> {
        if screen < expected {
            Ok(())
        } else {
            Err(Error::InvalidScreen {
                screen_id: screen,
                screen_number: expected,
            })
        }
    }

    #[inline]
    pub(crate) fn throw_unsupported_device_error<'a, D: Driver<'a, CTX>, CTX: UsbContext>(
        vendor_id: u16,
        product_id: u16,
    ) -> Result<()> {
        Err(Error::UnsupportedDevice {
            vendor_id,
            product_id,
            driver: D::NAME,
        })
    }

    #[inline]
    pub(crate) fn throw_busy_screen_error(screen_id: usize) -> Result<()> {
        Err(Error::BusyScreen { screen_id })
    }
}

impl From<rusb::Error> for Error {
    fn from(value: rusb::Error) -> Self {
        Error::UsbError(value)
    }
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

    #[test]
    fn test_check_screen() {
        assert_eq!(Error::check_screen(5, 8), Ok(()));
        assert_eq!(
            Error::check_screen(6, 6),
            Err(Error::InvalidScreen {
                screen_id: 6,
                screen_number: 6
            })
        );
        assert_eq!(
            Error::check_screen(12, 5),
            Err(Error::InvalidScreen {
                screen_id: 12,
                screen_number: 5
            })
        );
    }
}
