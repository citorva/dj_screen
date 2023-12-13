#[allow(dead_code)]
pub const COLOR_BLACK: Color = Color::new(0x00, 0x00, 0x00);
#[allow(dead_code)]
pub const COLOR_WHITE: Color = Color::new(0xFF, 0xFF, 0xFF);

#[allow(dead_code)]
pub const COLOR_RED: Color = Color::new(0xFF, 0x00, 0x00);
#[allow(dead_code)]
pub const COLOR_GREEN: Color = Color::new(0x00, 0xFF, 0x00);
#[allow(dead_code)]
pub const COLOR_BLUE: Color = Color::new(0x00, 0x00, 0xFF);

#[allow(dead_code)]
pub const COLOR_YELLOW: Color = Color::new(0xFF, 0xFF, 0x00);
#[allow(dead_code)]
pub const COLOR_MAGENTA: Color = Color::new(0xFF, 0x00, 0xFF);
#[allow(dead_code)]
pub const COLOR_CYAN: Color = Color::new(0x00, 0xFF, 0xFF);

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub struct DummyBuffer {
    color: Color,
}

pub struct BufferIter<'a> {
    color: &'a Color,
    pos: usize,
    length: usize,
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub const fn red(&self) -> u8 {
        self.red
    }
    pub const fn green(&self) -> u8 {
        self.green
    }
    pub const fn blue(&self) -> u8 {
        self.blue
    }
}

impl DummyBuffer {
    pub const fn new(color: Color) -> Self {
        DummyBuffer { color }
    }
}

impl super::buffer::Color for &Color {
    type Component = u8;

    fn components(self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}

impl<'a> super::buffer::IntoPixelIter for &'a DummyBuffer {
    type IntoIter = BufferIter<'a>;
    type Item = &'a Color;

    fn into_pixel_iter(self, _x: u16, _y: u16, width: u16, height: u16) -> Self::IntoIter {
        BufferIter {
            color: &self.color,
            pos: 0,
            length: width as usize * height as usize,
        }
    }
}

impl<'a> Iterator for BufferIter<'a> {
    type Item = &'a Color;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.length {
            self.pos += 1;

            Some(self.color)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test_helper::{DummyBuffer, COLOR_RED};
    use crate::IntoPixelIter;

    #[test]
    fn test_iterator() {
        const WIDTH: u16 = 35;
        const HEIGHT: u16 = 45;

        const LENGTH: usize = WIDTH as usize * HEIGHT as usize;

        let buffer = DummyBuffer::new(COLOR_RED);
        let mut iter = buffer.into_pixel_iter(10, 10, WIDTH, HEIGHT);

        let mut i = 0;

        while let Some(_) = iter.next() {
            i += 1;

            if i > LENGTH {
                panic!("Number of access exceeded")
            }
        }

        assert_eq!(i, LENGTH);
    }
}
