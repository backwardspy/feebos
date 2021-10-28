use bootloader::boot_info::FrameBuffer;
use font8x8::{UnicodeFonts, BASIC_FONTS};

use crate::text_buffer::TextBuffer;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub struct GraphicsContext<'a> {
    fb: Option<&'a mut FrameBuffer>,
}

impl Color {
    // https://lospec.com/palette-list/sweetie-16
    pub const BLACK: Color = Color::from_hex(0x1A1C2C);
    pub const PURPLE: Color = Color::from_hex(0x5D275D);
    pub const RED: Color = Color::from_hex(0xB13E53);
    pub const ORANGE: Color = Color::from_hex(0xEF7D57);
    pub const YELLOW: Color = Color::from_hex(0xFFCD75);
    pub const LIME: Color = Color::from_hex(0xA7F070);
    pub const GREEN: Color = Color::from_hex(0x38B764);
    pub const TEAL: Color = Color::from_hex(0x257179);
    pub const DARKBLUE: Color = Color::from_hex(0x29366F);
    pub const BLUE: Color = Color::from_hex(0x3B5DC9);
    pub const LIGHTBLUE: Color = Color::from_hex(0x41A6F6);
    pub const CYAN: Color = Color::from_hex(0x73EFF7);
    pub const WHITE: Color = Color::from_hex(0xF4F4F4);
    pub const LIGHTGREY: Color = Color::from_hex(0x94B0C2);
    pub const GREY: Color = Color::from_hex(0x566C86);
    pub const DARKGREY: Color = Color::from_hex(0x333C57);

    pub const fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub const fn from_hex(hex: u32) -> Color {
        let red = (hex & 0xFF0000) >> 16;
        let green = (hex & 0x00FF00) >> 8;
        let blue = hex & 0x0000FF;

        Color::new(red as u8, green as u8, blue as u8)
    }
}

impl<'a> GraphicsContext<'a> {
    pub fn new() -> Self {
        Self { fb: None }
    }

    pub fn set_framebuffer(&mut self, fb: &'a mut FrameBuffer) {
        self.fb = Some(fb);
    }

    pub fn width(&self) -> u32 {
        self.fb.as_ref().unwrap().info().horizontal_resolution as u32
    }

    pub fn height(&self) -> u32 {
        self.fb.as_ref().unwrap().info().vertical_resolution as u32
    }

    pub fn clear(&mut self, colour: Color) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.set_pixel(x, y, colour);
            }
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let fbinfo = self.fb.as_ref().unwrap().info();
        let x = x as usize;
        let y = y as usize;
        if x >= fbinfo.horizontal_resolution || y >= fbinfo.vertical_resolution {
            return;
        }

        let stride = fbinfo.stride;
        let bpp = fbinfo.bytes_per_pixel;

        let pixel_index = (y * stride + x) * bpp;
        let colour = match fbinfo.pixel_format {
            bootloader::boot_info::PixelFormat::RGB => [color.red, color.green, color.blue, 0],
            bootloader::boot_info::PixelFormat::U8 => [color.red, 0, 0, 0],
            _ => [color.blue, color.green, color.red, 0], // we assume BGR as it seems to be quite common
        };
        self.fb.as_mut().unwrap().buffer_mut()[pixel_index..pixel_index + bpp]
            .copy_from_slice(&colour[..bpp]);
    }

    pub fn char(&mut self, c: char, x: u32, y: u32, fg: Color, bg: Color) {
        if let Some(glyph) = BASIC_FONTS.get(c) {
            for (y_offset, row) in glyph.iter().enumerate() {
                for bit in 0..8 {
                    let colour = match *row & 1 << bit {
                        0 => bg,
                        _ => fg,
                    };
                    self.set_pixel(x + bit, y + y_offset as u32, colour)
                }
            }
        }
    }

    pub fn text(&mut self, string: &str, x: u32, y: u32, fg: Color, bg: Color) {
        for (char_x_offset, char) in string.chars().enumerate() {
            self.char(char, x + char_x_offset as u32 * 8, y, fg, bg);
        }
    }

    pub fn text_buffer(
        &mut self,
        text_buffer: &mut TextBuffer,
        padding: u32,
        line_spacing: u32,
        fg: Color,
        bg: Color,
    ) {
        let width = text_buffer.width;
        for cursor in text_buffer.dirty() {
            self.char(
                text_buffer.at(cursor),
                padding + 8 * (cursor % width) as u32,
                padding + (8 + line_spacing) * (cursor / width) as u32,
                fg,
                bg,
            );
        }
        text_buffer.mark_all_clean();
    }
}

impl<'a> Default for GraphicsContext<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn calculate_text_buffer_size(
    width: u32,
    height: u32,
    padding: u32,
    spacing: u32,
) -> (usize, usize) {
    let xspace = (width - padding * 2) / 8;
    let yspace = (height - padding * 2) / (8 + spacing);
    (xspace as usize, yspace as usize)
}
