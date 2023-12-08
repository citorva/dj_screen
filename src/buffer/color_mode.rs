pub enum ColorMode {
    RGB,
    BGR,
    RGB565LE,
    RGB565BE,
    BGR565LE,
    BGR565BE,
    Grayscale1Bit,
    Grayscale2Bit,
    Grayscale4Bit,
    Grayscale8Bit,
    Grayscale16BitLE,
    Grayscale16BitBE,
}

pub enum StorageMode {
    BytesPerPixel(usize),
    PixelsPerByte(usize),
}

impl ColorMode {
    /// Gives the buffer required length to host one pixel.
    ///
    /// Some color modes can use a fraction of a byte and must be took in account in the pixel
    /// adaptation.
    ///
    /// # Returns
    ///
    /// If the color needs less than 8 bits, the function returns the bit size of the color.
    /// Otherwise, the function return the byte size.
    pub fn byte_size(&self) -> StorageMode {
        match self {
            ColorMode::RGB => StorageMode::BytesPerPixel(3),
            ColorMode::BGR => StorageMode::BytesPerPixel(3),
            ColorMode::RGB565LE => StorageMode::BytesPerPixel(2),
            ColorMode::RGB565BE => StorageMode::BytesPerPixel(2),
            ColorMode::BGR565LE => StorageMode::BytesPerPixel(2),
            ColorMode::BGR565BE => StorageMode::BytesPerPixel(2),
            ColorMode::Grayscale8Bit => StorageMode::BytesPerPixel(1),
            ColorMode::Grayscale16BitLE => StorageMode::BytesPerPixel(2),
            ColorMode::Grayscale16BitBE => StorageMode::BytesPerPixel(2),

            ColorMode::Grayscale1Bit => StorageMode::PixelsPerByte(8),
            ColorMode::Grayscale2Bit => StorageMode::PixelsPerByte(4),
            ColorMode::Grayscale4Bit => StorageMode::PixelsPerByte(2),
        }
    }
}
