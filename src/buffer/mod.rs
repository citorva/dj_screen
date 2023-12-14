mod color;
mod color_mode;
mod component;
mod integration;


use crate::{CoreError, CoreResult};
pub use color::Color;
pub use color_mode::*;
pub use component::Component;

const fn calculate_length(width: u16, height: u16) -> usize {
    width as usize * height as usize
}

#[inline]
fn generic_fill<const BYTES_PER_PIXEL: usize, C, I, T>(
    iter: T,
    target: &mut [u8],
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    convertor: fn(C, &mut [u8]) -> CoreResult<()>,
) -> CoreResult<()>
where
    C: Color,
    I: Iterator<Item = C>,
    T: IntoPixelIter<IntoIter = I, Item = C>,
{
    let length = calculate_length(width, height);

    CoreError::check_length(target, BYTES_PER_PIXEL * length)?;

    let mut begin = 0;
    let mut iterator = iter.into_pixel_iter(x, y, width, height);

    while let Some(pixel) = iterator.next() {
        let end = begin + BYTES_PER_PIXEL;

        convertor(pixel, &mut target[begin..end])?;

        begin = end;
    }

    Ok(())
}

#[inline]
fn generic_subpixel_fill<const PIXELS_PER_BYTE: usize, C, I, T>(
    _iter: T,
    _target: &mut [u8],
    _x: u16,
    _y: u16,
    _width: u16,
    _height: u16,
    _action: fn([C; PIXELS_PER_BYTE], &mut [u8]) -> CoreResult<()>,
) -> CoreResult<()>
where
    C: Color,
    I: Iterator<Item = C>,
    T: IntoPixelIter<IntoIter = I, Item = C>,
{
    todo!()
}

pub trait Buffer {
    fn fill_rgb(self, target: &mut [u8], x: u16, y: u16, width: u16, height: u16) -> CoreResult<()>;
    fn fill_bgr(self, target: &mut [u8], x: u16, y: u16, width: u16, height: u16) -> CoreResult<()>;

    fn fill_rgb565le(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()>;
    fn fill_rgb565be(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()>;

    fn fill_bgr565le(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()>;
    fn fill_bgr565be(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()>;

    fn fill_grayscale_8bit(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()>;
    fn fill_grayscale_16bit_le(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()>;
    fn fill_grayscale_16bit_be(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()>;

    fn fill_grayscale_1bit(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()>;
    fn fill_grayscale_2bit(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()>;
    fn fill_grayscale_4bit(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()>;
}

pub trait IntoPixelIter {
    type IntoIter: Iterator<Item = Self::Item>;
    type Item: Color;

    fn into_pixel_iter(self, x: u16, y: u16, width: u16, height: u16) -> Self::IntoIter;
}

impl<C, I, T> Buffer for T
where
    C: Color,
    I: Iterator<Item = C>,
    T: IntoPixelIter<IntoIter = I, Item = C>
{
    fn fill_rgb(self, target: &mut [u8], x: u16, y: u16, width: u16, height: u16) -> CoreResult<()> {
        generic_fill::<3, C, I, T>(self, target, x, y, width, height, C::fill_rgb)
    }

    fn fill_bgr(self, target: &mut [u8], x: u16, y: u16, width: u16, height: u16) -> CoreResult<()> {
        generic_fill::<3, C, I, T>(self, target, x, y, width, height, C::fill_bgr)
    }

    fn fill_rgb565le(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()> {
        generic_fill::<2, C, I, T>(self, target, x, y, width, height, C::fill_rgb565le)
    }

    fn fill_rgb565be(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()> {
        generic_fill::<2, C, I, T>(self, target, x, y, width, height, C::fill_rgb565be)
    }

    fn fill_bgr565le(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()> {
        generic_fill::<2, C, I, T>(self, target, x, y, width, height, C::fill_bgr565le)
    }

    fn fill_bgr565be(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()> {
        generic_fill::<2, C, I, T>(self, target, x, y, width, height, C::fill_bgr565be)
    }

    fn fill_grayscale_8bit(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()> {
        generic_fill::<1, C, I, T>(self, target, x, y, width, height, C::fill_grayscale_8bit)
    }

    fn fill_grayscale_16bit_le(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()> {
        generic_fill::<2, C, I, T>(
            self,
            target,
            x,
            y,
            width,
            height,
            C::fill_grayscale_16bit_le,
        )
    }

    fn fill_grayscale_16bit_be(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()> {
        generic_fill::<2, C, I, T>(
            self,
            target,
            x,
            y,
            width,
            height,
            C::fill_grayscale_16bit_be,
        )
    }

    fn fill_grayscale_1bit(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()> {
        generic_subpixel_fill::<8, C, I, T>(
            self,
            target,
            x,
            y,
            width,
            height,
            C::fill_grayscale_1bit,
        )
    }

    fn fill_grayscale_2bit(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()> {
        generic_subpixel_fill::<4, C, I, T>(
            self,
            target,
            x,
            y,
            width,
            height,
            C::fill_grayscale_2bit,
        )
    }

    fn fill_grayscale_4bit(
        self,
        target: &mut [u8],
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> CoreResult<()> {
        generic_subpixel_fill::<2, C, I, T>(
            self,
            target,
            x,
            y,
            width,
            height,
            C::fill_grayscale_4bit,
        )
    }
}
