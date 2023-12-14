use crate::vendor::Driver;
use std::fmt::{Debug, Display, Formatter};
use crate::usb::UsbDevice;

pub type Result<T, BACKEND> = std::result::Result<T, Error<BACKEND>>;
pub type CoreResult<T> = std::result::Result<T, CoreError>;

pub enum Error<BACKEND : UsbDevice> {
    Core(CoreError),
    Usb(BACKEND::Error),
}

#[derive(Debug, PartialEq)]
pub enum CoreError {
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
}

impl<BACKEND : UsbDevice> std::error::Error for Error<BACKEND> {}

impl std::error::Error for CoreError {}

impl<BACKEND : UsbDevice> Debug for Error<BACKEND> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Core(e) => write!(f, "Core({e:?})"),
            Error::Usb(e) => write!(f, "Usb({e:?})"),
        }
    }
}

impl<BACKEND: UsbDevice> Display for Error<BACKEND> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let e : &dyn std::error::Error = match self {
            Error::Core(e) => e,
            Error::Usb(e) => e,
        };

        Display::fmt(e, f)
    }
}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::BufferSizeError { given, expected } => {
                write!(f, "The buffer size must be {expected}, given {given}")
            }
            CoreError::InvalidScreen {
                screen_id,
                screen_number,
            } => {
                write!(f, "Invalid screen. Given {screen_id}, must be between 0 and {screen_number} (not included)")
            }
            CoreError::UnsupportedDevice {
                vendor_id,
                product_id,
                driver,
            } => {
                write!(f, "Trying to open a device with id {vendor_id}:{product_id} from the driver `{driver}` which is not made for")
            }
            CoreError::BusyScreen { screen_id } => {
                write!(
                    f,
                    "The screen with id {screen_id} is already used bay an other object"
                )
            }
        }
    }
}

impl CoreError {
    pub(crate) fn check_length(buf: &[u8], expected: usize) -> CoreResult<()> {
        if buf.len() == expected {
            Ok(())
        } else {
            Err(CoreError::BufferSizeError {
                given: buf.len(),
                expected,
            })
        }
    }

    pub(crate) fn check_screen(screen: usize, expected: usize) -> CoreResult<()> {
        if screen < expected {
            Ok(())
        } else {
            Err(CoreError::InvalidScreen {
                screen_id: screen,
                screen_number: expected,
            })
        }
    }

    #[inline]
    pub(crate) fn throw_unsupported_device_error<'a, D: Driver<'a, DEV>, DEV : UsbDevice>(
        vendor_id: u16,
        product_id: u16,
    ) -> CoreResult<()> {
        Err(CoreError::UnsupportedDevice {
            vendor_id,
            product_id,
            driver: D::NAME,
        })
    }

    #[inline]
    pub(crate) fn throw_busy_screen_error(screen_id: usize) -> CoreResult<()> {
        Err(CoreError::BusyScreen { screen_id })
    }
}

// impl<BACKEND : UsbDevice> From<BACKEND::Error> for Error<BACKEND> {
//     fn from(error: rusb::Error) -> Self {
//         Error::Usb(error)
//     }
// }

impl<BACKEND : UsbDevice> From<CoreError> for Error<BACKEND> {
    fn from(error: CoreError) -> Self {
        Error::Core(error)
    }
}

#[cfg(test)]
mod tests {
    use crate::CoreError;

    #[test]
    fn test_check_length() {
        assert_eq!(CoreError::check_length(&[0, 1, 2, 3, 4, 5], 6), Ok(()));
        assert_eq!(
            CoreError::check_length(&[0, 1, 2, 3, 4, 5], 5),
            Err(CoreError::BufferSizeError {
                expected: 5,
                given: 6
            })
        );
        assert_eq!(
            CoreError::check_length(&[0, 1, 2, 3, 4], 6),
            Err(CoreError::BufferSizeError {
                expected: 6,
                given: 5
            })
        );
    }

    #[test]
    fn test_check_screen() {
        assert_eq!(CoreError::check_screen(5, 8), Ok(()));
        assert_eq!(
            CoreError::check_screen(6, 6),
            Err(CoreError::InvalidScreen {
                screen_id: 6,
                screen_number: 6
            })
        );
        assert_eq!(
            CoreError::check_screen(12, 5),
            Err(CoreError::InvalidScreen {
                screen_id: 12,
                screen_number: 5
            })
        );
    }
}
