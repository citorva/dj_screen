use crate::IntoPixelIter;
use crate::test_helper::Color;

pub struct DummyBuffer {
    color: Color,
}

pub struct BufferIter<'a> {
    color: &'a Color,
    pos: usize,
    length: usize,
}

impl DummyBuffer {
    pub const fn new(color: Color) -> Self {
        DummyBuffer { color }
    }
}

impl<'a> IntoPixelIter for &'a DummyBuffer {
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
    use crate::test_helper::COLOR_RED;
    use crate::IntoPixelIter;

    #[test]
    fn test_iterator() {
        const WIDTH: u16 = 35;
        const HEIGHT: u16 = 45;

        const LENGTH: usize = WIDTH as usize * HEIGHT as usize;

        let buffer = crate::test_helper::DummyBuffer::new(COLOR_RED);
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
