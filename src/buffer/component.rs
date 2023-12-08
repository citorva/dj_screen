use crate::error::*;

pub trait Component {
    const LENGTH: usize;

    fn convert_le(self, buf: &mut [u8]) -> Result<()>;
    fn convert_be(self, buf: &mut [u8]) -> Result<()>;

    fn max_component(self, other1: Self, other2: Self) -> Self;

    fn most_significant_byte(self) -> u8;
}

fn copy_buffer<const LENGTH: usize>(from: [u8; LENGTH], to: &mut [u8]) -> Result<()> {
    Error::check_length(to, LENGTH)?;

    for i in 0..LENGTH {
        to[i] = from[i];
    }

    Ok(())
}

macro_rules! impl_component {
    ($t:ty, $length:literal) => {
        impl Component for $t {
            const LENGTH: usize = $length;

            fn convert_le(self, buf: &mut [u8]) -> Result<()> {
                copy_buffer(self.to_le_bytes(), buf)
            }

            fn convert_be(self, buf: &mut [u8]) -> Result<()> {
                copy_buffer(self.to_be_bytes(), buf)
            }

            fn max_component(self, other1: Self, other2: Self) -> Self {
                self.max(other1.max(other2))
            }

            fn most_significant_byte(self) -> u8 {
                self.to_be() as u8
            }
        }
    };
}

impl_component!(u8, 1);
impl_component!(u16, 2);
impl_component!(u32, 4);
impl_component!(u64, 8);
impl_component!(u128, 16);

#[cfg(test)]
mod tests {
    use super::Component;

    #[test]
    fn test_convert_le() {
        let mut buf = [0u8; 16];

        0x00u8.convert_le(&mut buf[0..1]).unwrap();
        assert_eq!(buf[0..1], [0x00]);

        0x0011u16.convert_le(&mut buf[0..2]).unwrap();
        assert_eq!(buf[0..2], [0x11, 0x00]);

        0x00112233u32.convert_le(&mut buf[0..4]).unwrap();
        assert_eq!(buf[0..4], [0x33, 0x22, 0x11, 0x00]);

        0x0011223344556677u64.convert_le(&mut buf[0..8]).unwrap();
        assert_eq!(buf[0..8], [0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00]);

        0x00112233445566778899AABBCCDDEEFFu128
            .convert_le(&mut buf[0..16])
            .unwrap();
        assert_eq!(
            buf[0..16],
            [
                0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
                0x11, 0x00
            ]
        );
    }

    #[test]
    fn test_convert_be() {
        let mut buf = [0u8; 16];

        0x00u8.convert_be(&mut buf[0..1]).unwrap();
        assert_eq!(buf[0..1], [0x00]);

        0x0011u16.convert_be(&mut buf[0..2]).unwrap();
        assert_eq!(buf[0..2], [0x00, 0x11]);

        0x00112233u32.convert_be(&mut buf[0..4]).unwrap();
        assert_eq!(buf[0..4], [0x00, 0x11, 0x22, 0x33]);

        0x0011223344556677u64.convert_be(&mut buf[0..8]).unwrap();
        assert_eq!(buf[0..8], [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]);

        0x00112233445566778899AABBCCDDEEFFu128
            .convert_be(&mut buf[0..16])
            .unwrap();
        assert_eq!(
            buf[0..16],
            [
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD,
                0xEE, 0xFF
            ]
        );
    }

    #[test]
    fn test_most_significant_byte() {
        assert_eq!(0x00, 0x00u8.most_significant_byte());
        assert_eq!(0x00, 0x0011u16.most_significant_byte());
        assert_eq!(0x00, 0x00112233u32.most_significant_byte());
        assert_eq!(0x00, 0x0011223344556677u64.most_significant_byte());
        assert_eq!(
            0x00,
            0x00112233445566778899AABBCCDDEEFFu128.most_significant_byte()
        );
    }
}
