use crate::error::*;
use crate::Buffer;

pub trait Driver<'a, CTX: rusb::UsbContext>: Sized {
    const NAME: &'static str;
    const SCREEN_NUMBER: usize;

    type Handle: ScreenHandle;

    fn check_device_id(vendor_id: u16, device_id: u16) -> Result<()>;

    fn is_made_for(handle: &rusb::DeviceHandle<CTX>) -> Result<()> {
        // Since libusb 1.0.16 (2013), the function device_descriptor() does not return error
        // Assuming that the used version is older than libusb 1.0.16
        let descriptor = handle.device().device_descriptor()?;

        Self::check_device_id(descriptor.vendor_id(), descriptor.product_id())
    }

    fn try_init(handle: rusb::DeviceHandle<CTX>) -> Result<Self>;

    fn acquire_screen(&'a self, screen_id: usize) -> Result<Self::Handle>;
}

pub trait ScreenHandle {
    fn send_buffer<B: Buffer>(
        &mut self,
        buffer: B,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> Result<()>;

    fn width(&self) -> u16;
    fn height(&self) -> u16;
}
