use super::Component;
use crate::error::*;

pub trait Color: Sized {
    type Component: Component;

    fn components(self) -> [Self::Component; 3];

    fn into_rgb565(self) -> u16 {
        let [red, green, blue] = self.components();

        let r_rgb565 = (red.most_significant_byte() as u16 & 0b11111000) << 8;
        let g_rgb565 = (green.most_significant_byte() as u16 & 0b11111100) << 3;
        let b_rgb565 = (blue.most_significant_byte() as u16 & 0b11111000) >> 3;

        r_rgb565 | g_rgb565 | b_rgb565
    }

    fn into_bgr565(self) -> u16 {
        let [red, green, blue] = self.components();

        let r_rgb565 = (red.most_significant_byte() as u16 & 0b11111000) >> 3;
        let g_rgb565 = (green.most_significant_byte() as u16 & 0b11111100) << 3;
        let b_rgb565 = (blue.most_significant_byte() as u16 & 0b11111000) << 8;

        r_rgb565 | g_rgb565 | b_rgb565
    }

    fn luminance(self) -> Self::Component {
        let [red, green, blue] = self.components();

        Self::Component::max_component(red, green, blue)
    }

    fn fill_rgb(self, buf: &mut [u8]) -> Result<()> {
        Error::check_length(buf, 3)?;

        let [red, green, blue] = self.components();

        buf[0] = red.most_significant_byte();
        buf[1] = green.most_significant_byte();
        buf[2] = blue.most_significant_byte();

        Ok(())
    }
    fn fill_bgr(self, buf: &mut [u8]) -> Result<()> {
        Error::check_length(buf, 3)?;

        let [red, green, blue] = self.components();

        buf[0] = blue.most_significant_byte();
        buf[1] = green.most_significant_byte();
        buf[2] = red.most_significant_byte();

        Ok(())
    }
    fn fill_rgb565le(self, buf: &mut [u8]) -> Result<()> {
        self.into_rgb565().convert_le(buf)
    }
    fn fill_rgb565be(self, buf: &mut [u8]) -> Result<()> {
        self.into_rgb565().convert_be(buf)
    }
    fn fill_bgr565le(self, buf: &mut [u8]) -> Result<()> {
        self.into_bgr565().convert_le(buf)
    }
    fn fill_bgr565be(self, buf: &mut [u8]) -> Result<()> {
        self.into_bgr565().convert_be(buf)
    }
    fn fill_grayscale_1bit(from: [Self; 8], buf: &mut [u8]) -> Result<()> {
        Error::check_length(buf, 1)?;

        buf[0] = 0;

        for pixel in from {
            let data = pixel.luminance().most_significant_byte();

            buf[0] = buf[0] << 1 | data >> 7
        }

        Ok(())
    }
    fn fill_grayscale_2bit(from: [Self; 4], buf: &mut [u8]) -> Result<()> {
        Error::check_length(buf, 1)?;

        buf[0] = 0;

        for pixel in from {
            let data = pixel.luminance().most_significant_byte();

            buf[0] = buf[0] << 2 | data >> 6;
        }

        Ok(())
    }
    fn fill_grayscale_4bit(from: [Self; 2], buf: &mut [u8]) -> Result<()> {
        Error::check_length(buf, 1)?;

        buf[0] = 0;

        for pixel in from {
            let data = pixel.luminance().most_significant_byte();

            buf[0] = buf[0] << 4 | data >> 4;
        }

        Ok(())
    }
    fn fill_grayscale_8bit(self, buf: &mut [u8]) -> Result<()> {
        Error::check_length(buf, 1)?;

        buf[0] = self.luminance().most_significant_byte();

        Ok(())
    }

    fn fill_grayscale_16bit_le(self, _buf: &mut [u8]) -> Result<()> {
        todo!()
    }

    fn fill_grayscale_16bit_be(self, _buf: &mut [u8]) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Color;

    const RED: u8 = 0b10011010;
    const GREEN: u8 = 0b11100111;
    const BLUE: u8 = 0b00101010;

    const GRAYSCALE1: u8 = 0b11100111; // 0b1 0b11 0b1110 0b11100111

    const GRAYSCALE2: u8 = 0b01000011; // 0b0 0b01 0b0100 0b01000011

    const RGB565: u16 = 0b10011_111001_00101;
    const BGR565: u16 = 0b00101_111001_10011;

    const COLOR1: RGB888Color = RGB888Color::new(RED, GREEN, BLUE);
    const COLOR2: RGB888Color = RGB888Color::new(GRAYSCALE2, GRAYSCALE2, GRAYSCALE2);

    struct RGB888Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl RGB888Color {
        pub const fn new(red: u8, green: u8, blue: u8) -> RGB888Color {
            RGB888Color { red, green, blue }
        }
    }

    impl Color for RGB888Color {
        type Component = u8;

        fn components(self) -> [u8; 3] {
            [self.red, self.green, self.blue]
        }
    }

    #[test]
    fn test_rgb565() {
        assert_eq!(
            RGB888Color::new(0, 0, 0).into_rgb565(),
            0b00000_000000_00000
        );
        assert_eq!(
            RGB888Color::new(255, 255, 255).into_rgb565(),
            0b11111_111111_11111
        );

        assert_eq!(
            RGB888Color::new(255, 0, 0).into_rgb565(),
            0b11111_000000_00000
        );
        assert_eq!(
            RGB888Color::new(0, 255, 0).into_rgb565(),
            0b00000_111111_00000
        );
        assert_eq!(
            RGB888Color::new(0, 0, 255).into_rgb565(),
            0b00000_000000_11111
        );
    }

    #[test]
    fn test_bgr565() {
        assert_eq!(
            RGB888Color::new(0, 0, 0).into_bgr565(),
            0b00000_000000_00000
        );
        assert_eq!(
            RGB888Color::new(255, 255, 255).into_bgr565(),
            0b11111_111111_11111
        );

        assert_eq!(
            RGB888Color::new(255, 0, 0).into_bgr565(),
            0b00000_000000_11111
        );
        assert_eq!(
            RGB888Color::new(0, 255, 0).into_bgr565(),
            0b00000_111111_00000
        );
        assert_eq!(
            RGB888Color::new(0, 0, 255).into_bgr565(),
            0b11111_000000_00000
        );
    }

    #[test]
    fn test_luminance() {
        assert_eq!(COLOR1.luminance(), GRAYSCALE1);
        assert_eq!(COLOR2.luminance(), GRAYSCALE2);
    }

    #[test]
    fn test_fill_rgb() {
        let mut buffer = [0u8; 3];

        COLOR1.fill_rgb(&mut buffer).unwrap();

        assert_eq!(buffer, [RED, GREEN, BLUE]);
    }

    #[test]
    fn test_fill_bgr() {
        let mut buffer = [0u8; 3];

        COLOR1.fill_bgr(&mut buffer).unwrap();

        assert_eq!(buffer, [BLUE, GREEN, RED]);
    }

    #[test]
    fn test_fill_rgb565le() {
        let mut buffer = [0u8; 2];

        COLOR1.fill_rgb565le(&mut buffer).unwrap();

        assert_eq!(buffer, RGB565.to_le_bytes());
    }

    #[test]
    fn test_fill_rgb565be() {
        let mut buffer = [0u8; 2];

        COLOR1.fill_rgb565be(&mut buffer).unwrap();

        assert_eq!(buffer, RGB565.to_be_bytes());
    }

    #[test]
    fn test_fill_bgr565le() {
        let mut buffer = [0u8; 2];

        COLOR1.fill_bgr565le(&mut buffer).unwrap();

        assert_eq!(buffer, BGR565.to_le_bytes());
    }

    #[test]
    fn test_fill_bgr565be() {
        let mut buffer = [0u8; 2];

        COLOR1.fill_bgr565be(&mut buffer).unwrap();

        assert_eq!(buffer, BGR565.to_be_bytes());
    }

    #[test]
    fn test_fill_grayscale1bit() {
        let mut buffer = [0u8; 1];
        let grayscale = [
            COLOR1, COLOR2, COLOR1, COLOR2, COLOR1, COLOR2, COLOR1, COLOR2,
        ];

        Color::fill_grayscale_1bit(grayscale, &mut buffer).unwrap();

        assert_eq!(buffer, [0b10101010]);
    }

    #[test]
    fn test_fill_grayscale2bit() {
        let mut buffer = [0u8; 1];
        let grayscale = [COLOR1, COLOR2, COLOR1, COLOR2];

        Color::fill_grayscale_2bit(grayscale, &mut buffer).unwrap();

        assert_eq!(buffer, [0b11_01_11_01]);
    }

    #[test]
    fn test_fill_grayscale4bit() {
        let mut buffer = [0u8; 1];
        let grayscale = [COLOR1, COLOR2];

        Color::fill_grayscale_4bit(grayscale, &mut buffer).unwrap();

        assert_eq!(buffer, [0b1110_0100]);
    }

    #[test]
    fn test_fill_grayscale8bit() {
        let mut buffer = [0u8];

        COLOR1.fill_grayscale_8bit(&mut buffer).unwrap();

        assert_eq!(buffer, [GRAYSCALE1]);

        COLOR2.fill_grayscale_8bit(&mut buffer).unwrap();

        assert_eq!(buffer, [GRAYSCALE2]);
    }
}
