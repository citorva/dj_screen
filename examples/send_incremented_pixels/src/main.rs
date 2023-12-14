extern crate rusb;

use dj_screen::vendor::traktor::kontrol_s4_mk3::KontrolS4MK3Driver;
use dj_screen::vendor::{Driver, ScreenHandle};
use dj_screen::{Color, IntoPixelIter};
use std::time::Duration;

struct CounterColorBGR(u16);

struct CounterBufferBGR {
    init: u16,
}

struct CounterBGRIterator {
    value: u16,
    max: usize,
    pos: usize,
}

impl CounterBufferBGR {
    pub const fn new() -> CounterBufferBGR {
        CounterBufferBGR { init: 0 }
    }
}

impl Color for CounterColorBGR {
    type Component = u8;

    fn components(self) -> [Self::Component; 3] {
        let r = (self.0 << 3) as u8 & 0xF8;
        let g = (self.0 >> 3) as u8 & 0xFC;
        let b = (self.0 >> 9) as u8 & 0xF8;

        [r, g, b]
    }

    fn into_bgr565(self) -> u16 {
        self.0
    }
}

impl IntoPixelIter for &CounterBufferBGR {
    type IntoIter = CounterBGRIterator;
    type Item = CounterColorBGR;

    fn into_pixel_iter(self, _x: u16, _y: u16, width: u16, height: u16) -> Self::IntoIter {
        let size = width as usize * height as usize;

        CounterBGRIterator {
            value: self.init,
            max: size,
            pos: 0,
        }
    }
}

impl Iterator for CounterBGRIterator {
    type Item = CounterColorBGR;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.max {
            self.pos += 1;

            let value = self.value;

            self.value = self.value.overflowing_add(1).0;

            Some(CounterColorBGR(value))
        } else {
            None
        }
    }
}

const COUNTER_BUFFER: &'static CounterBufferBGR = &CounterBufferBGR::new();

fn main() {
    let handle = rusb::open_device_with_vid_pid(0x17cc, 0x1720).unwrap();

    let driver = KontrolS4MK3Driver::try_init(handle).unwrap();

    let mut screen1 = driver.acquire_screen(0).unwrap();
    let mut screen2 = driver.acquire_screen(1).unwrap();

    screen1
        .send_buffer(COUNTER_BUFFER, 0, 0, screen1.width(), screen1.height())
        .unwrap();

    std::thread::sleep(Duration::from_millis(50));

    screen2
        .send_buffer(COUNTER_BUFFER, 0, 0, screen2.width(), screen2.height())
        .unwrap();

    std::thread::park();
}
