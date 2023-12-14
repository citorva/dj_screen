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

impl crate::buffer::Color for &Color {
    type Component = u8;

    fn components(self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}