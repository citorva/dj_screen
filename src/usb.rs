use std::time::Duration;

pub struct ProductInformation {
    pub vendor_id : u16,
    pub product_id : u16,
}

pub trait UsbDevice {
    type Error : std::error::Error;

    fn product_information(&self) -> ProductInformation;

    fn write_bulk(&self, bulk_endpoint : u8, data : &[u8], timeout : Duration) -> Result<usize, Self::Error>;
}

#[cfg(feature = "rusb")]
impl<CTX : rusb::UsbContext> UsbDevice for rusb::DeviceHandle<CTX> {
    type Error = rusb::Error;

    fn product_information(&self) -> ProductInformation {
        // Since libusb 1.0.16 (2013), the function device_descriptor() does not return error
        // Assuming that the used version is older than libusb 1.0.16
        let desc = self.device().device_descriptor().unwrap();

        ProductInformation {
            vendor_id: desc.vendor_id(),
            product_id: desc.product_id(),
        }
    }

    fn write_bulk(&self, bulk_endpoint: u8, data: &[u8], timeout: Duration) -> Result<usize, Self::Error> {
        self.write_bulk(bulk_endpoint, data, timeout)
    }
}