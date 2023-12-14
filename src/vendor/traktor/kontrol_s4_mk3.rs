//! Driver fot the Native Instruments Traktor Kontrol S4 MK3 Screen
//!
//! The DJ set uses a non-documented protocol for its screens. A sniffing of the USB traffic between+
//! the Traktor Pro software and the table where needed to correctly uses the table.
//!
//! # The USB endpoint
//!
//! The embedded device is accessible through a bulk endpoint of the USB device. The device
//! descriptor gives the following information about the screen endpoint.
//!
//! ```ansi
//! Interface Descriptor:
//!   bLength                 9
//!   bDescriptorType         4
//!   bInterfaceNumber        4
//!   bAlternateSetting       0
//!   bNumEndpoints           1
//!   bInterfaceClass       255 Vendor Specific Class
//!   bInterfaceSubClass    189 [unknown]
//!   bInterfaceProtocol      0
//!   iInterface             12 Traktor Kontrol S4 MK3 BD
//!   Endpoint Descriptor:
//!     bLength                 7
//!     bDescriptorType         5
//!     bEndpointAddress     0x03  EP 3 OUT
//!     bmAttributes            2
//!       Transfer Type            Bulk
//!       Synch Type               None
//!       Usage Type               Data
//!     wMaxPacketSize     0x0200  1x 512 bytes
//!     bInterval               1
//! ```
//!
//! The display module uses the USB Bulk mode on the endpoint 3.
//!
//! # The protocol
//!
//! A packet is split into three parts:
//!  - The header contains the format signature, the screen identifier and the image's dimensions.
//!  - The body contains a set of pixels coded into the BGR565 format.
//!  - The footer contains the footer signature and the screen identifier again.
//!
//! All numbers uses the big endian as order of byte sequence.
//!
//! ## The header
//!
//! The header takes 16 bytes and is encoded as described below:
//!
//! ```txt
//!  0                   1                   2                   3
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |             0x0084            | Screen Select |      0x21     |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                           0x00000000                          |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |               X               |               Y               |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |             Width             |            Height             |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!
//! HEADER frame contains the following fields;
//!
//! Screen: The screen selector. This device only allow 0 and 1 because of the presence of two
//!     screens on the device
//!
//! X: The X position of the frame buffer.
//!
//! Y: The Y position of the frame buffer.
//!
//! Width: The width of the frame buffer. This value can be between 0 and 320
//!
//! Height: THe height of the frame buffer. This value can be between 0 and 240
//! ```
//!
//! ## The body
//!
//! The body is a set of pixel color. It is composed of Width * Height pixels encoded in line and
//! then cells beginning at the top left of the image.
//!
//! A pixel takes 2 bytes and is encoded as described below:
//!
//! ```txt
//!  0                   1
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |   Blue  |   Green   |   Red   |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! ```
//!
//! ## The footer
//!
//! The footer takes 4 bytes and is encoded as described below:
//!
//! ```txt
//!  0                   1                   2                   3
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |      0x40     |      0x00     | Screen Select |      0x00     |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! ```
//!
//!
use crate::error::*;
use crate::vendor::{Driver, ScreenHandle};
use crate::Buffer;
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;
use crate::usb::UsbDevice;

const WIDTH: u16 = 320;
const HEIGHT: u16 = 240;

const SCREEN_NUMBER: usize = 2;

const ENDPOINT: u8 = 3;

const HEADER_LENGTH: usize = 16;
const FOOTER_LENGTH: usize = 4;

pub const MAX_LENGTH: usize = PacketBuilder::new(WIDTH, HEIGHT, 0).packet_length();

pub const TIMEOUT: Duration = Duration::new(1, 0);

const fn calculate_pixel_count(width: u16, height: u16) -> usize {
    width as usize * height as usize
}

#[derive(Clone)]
pub struct PacketBuilder {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    screen: u8,
}

pub struct KontrolS4MK3Driver<'a, DEV: UsbDevice> {
    handle: DEV,
    acquired_screens: [Mutex<bool>; SCREEN_NUMBER],
    _lifetime: PhantomData<&'a ()>,
}

pub struct Handle<'a, DEV : UsbDevice> {
    driver: &'a KontrolS4MK3Driver<'a, DEV>,
    screen: u8,
    buffer: [u8; MAX_LENGTH],
}

impl PacketBuilder {
    pub const fn new(width: u16, height: u16, screen: u8) -> PacketBuilder {
        Self {
            x: 0,
            y: 0,
            width,
            height,
            screen,
        }
    }

    pub const fn with_position(mut self, x: u16, y: u16) -> PacketBuilder {
        self.x = x;
        self.y = y;

        self
    }

    pub const fn packet_length(&self) -> usize {
        HEADER_LENGTH + FOOTER_LENGTH + 2 * calculate_pixel_count(self.width, self.height)
    }

    fn trim_dimensions(&self) -> Option<[u16; 4]> {
        let x1 = 320.min(self.x);
        let x2 = 320.min(self.x + self.width);

        let y1 = 240.min(self.y);
        let y2 = 240.min(self.y + self.height);

        if x1 == x2 || y1 == y2 {
            None
        } else {
            Some([x1, y1, (x2 - x1), (y2 - y1)])
        }
    }

    pub fn fill_from_buffer<B: Buffer>(&self, target: &mut [u8], source: B) -> CoreResult<()> {
        if let Some([x, y, width, height]) = self.trim_dimensions() {
            let length = self.packet_length();

            CoreError::check_length(target, length)?;

            let pixbuf_offset = HEADER_LENGTH;
            let footer_offset = length - FOOTER_LENGTH;

            self.fill_header_unchecked(&mut target[..pixbuf_offset]);

            source.fill_bgr565be(
                &mut target[pixbuf_offset..footer_offset],
                x,
                y,
                width,
                height,
            )?;

            self.fill_footer_unchecked(&mut target[footer_offset..]);
        }

        Ok(())
    }

    fn fill_header_unchecked(&self, buffer: &mut [u8]) {
        buffer[0] = 0x84;
        buffer[1] = 0x00;
        buffer[2] = self.screen;
        buffer[3] = 0x21;

        buffer[4..8].fill(0x00);

        buffer[8..10].clone_from_slice(self.x.to_be_bytes().as_slice());
        buffer[10..12].clone_from_slice(self.y.to_be_bytes().as_slice());
        buffer[12..14].clone_from_slice(self.width.to_be_bytes().as_slice());
        buffer[14..16].clone_from_slice(self.height.to_be_bytes().as_slice());
    }

    fn fill_footer_unchecked(&self, buffer: &mut [u8]) {
        buffer[0] = 0x40;
        buffer[1] = 0x00;
        buffer[2] = self.screen;
        buffer[3] = 0x00;
    }
}

impl<'a, DEV: UsbDevice> Handle<'a, DEV> {
    fn fill_from_buffer<B: Buffer>(
        &mut self,
        buffer: B,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> Result<usize, DEV> {
        let builder = PacketBuilder::new(width, height, self.screen).with_position(x, y);
        let to = builder.packet_length();
        let bytes = &mut self.buffer[..to];

        builder.fill_from_buffer(bytes, buffer)?;

        Ok(to)
    }
}

impl<'a, DEV: UsbDevice> KontrolS4MK3Driver<'a, DEV> {
    fn get_guard(&self, screen_id: usize) -> MutexGuard<bool> {
        match self.acquired_screens[screen_id].lock() {
            Ok(v) => v,
            Err(e) => e.into_inner(),
        }
    }

    fn try_acquire(&self, screen_id: usize) -> Result<(), DEV> {
        let mut guard = self.get_guard(screen_id);
        let acquired = guard.deref_mut();

        if *acquired {
            CoreError::throw_busy_screen_error(screen_id)?;
        }

        *acquired = true;

        Ok(())
    }

    fn release_screen(&self, screen_id: usize) {
        *self.get_guard(screen_id).deref_mut() = false;
    }
}

impl<'a, DEV: UsbDevice + 'a> Driver<'a, DEV> for KontrolS4MK3Driver<'a, DEV> {
    const NAME: &'static str = "National Instrument Traktor Kontrol S4 MK3";

    const SCREEN_NUMBER: usize = 2;

    type Handle = Handle<'a, DEV>;

    fn check_device_id(vendor_id: u16, device_id: u16) -> Result<(), DEV> {
        if vendor_id != 0x17cc || device_id != 0x1720 {
            CoreError::throw_unsupported_device_error::<Self, DEV>(vendor_id, device_id)?;
        }

        Ok(())
    }

    fn try_init(handle: DEV) -> Result<Self, DEV> {
        Self::is_made_for(&handle)?;

        Ok(Self {
            handle,
            acquired_screens: [Mutex::default(), Mutex::default()],
            _lifetime: PhantomData,
        })
    }

    fn acquire_screen(&'a self, screen_id: usize) -> Result<Self::Handle, DEV> {
        CoreError::check_screen(screen_id, SCREEN_NUMBER)?;

        self.try_acquire(screen_id)?;

        Ok(Handle {
            driver: self,
            screen: screen_id as u8,
            buffer: [0u8; MAX_LENGTH],
        })
    }
}

impl<'a, DEV: UsbDevice> ScreenHandle<DEV> for Handle<'a, DEV> {
    fn send_buffer<B: Buffer>(
        &mut self,
        buffer: B,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> Result<(), DEV> {
        let buffer_to = self.fill_from_buffer(buffer, x, y, width, height)?;

        if let Err(e) = self.driver.handle.write_bulk(ENDPOINT, &self.buffer[..buffer_to], TIMEOUT) {
            Err(Error::Usb(e))
        } else {
            Ok(())
        }
    }

    fn width(&self) -> u16 {
        WIDTH
    }

    fn height(&self) -> u16 {
        HEIGHT
    }
}

impl<'a, DEV: UsbDevice> Drop for Handle<'a, DEV> {
    fn drop(&mut self) {
        self.driver.release_screen(self.screen as usize);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{DummyBuffer, COLOR_RED};
    use crate::vendor::traktor::kontrol_s4_mk3::PacketBuilder;

    const BUILDER: PacketBuilder = PacketBuilder::new(16, 16, 5)
        .with_position(16, 16);

    #[test]
    fn test_buffer_size() {
        assert_eq!(BUILDER.packet_length(), 2 * 256 + 16 + 4);
    }

    #[test]
    fn test_reference_packet() {
        let mut buffer = [0u8; BUILDER.packet_length()];
        let dummy = DummyBuffer::new(COLOR_RED);

        let reference = include_bytes!("reference_1.data");

        BUILDER.fill_from_buffer(&mut buffer, &dummy).unwrap();

        assert_eq!(&buffer, reference)
    }
}
