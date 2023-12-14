pub mod traktor;

use crate::error::*;
use crate::Buffer;
use crate::usb::UsbDevice;

pub trait Driver<'a, DEV: UsbDevice>: Sized {
    const NAME: &'static str;
    const SCREEN_NUMBER: usize;

    type Handle: ScreenHandle<DEV>;

    fn check_device_id(vendor_id: u16, device_id: u16) -> Result<(), DEV>;

    fn is_made_for(handle: &DEV) -> Result<(), DEV> {
        let ids = handle.product_information();

        Self::check_device_id(ids.vendor_id, ids.product_id)
    }

    fn try_init(handle: DEV) -> Result<Self, DEV>;

    fn acquire_screen(&'a self, screen_id: usize) -> Result<Self::Handle, DEV>;
}

pub trait ScreenHandle<DEV : UsbDevice> {
    fn send_buffer<B: Buffer>(
        &mut self,
        buffer: B,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> Result<(), DEV>;

    fn width(&self) -> u16;
    fn height(&self) -> u16;
}
