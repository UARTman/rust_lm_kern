use core::{fmt::Write, ptr::{write_volatile, read_volatile}};

use limine::NonNullPtr;

use crate::psf::Font;

pub struct Framebuffer {
    fb: *mut u8,
    fb_size: usize,
    pub width: usize,
    pub height: usize,
    pitch: usize,
    red_mask_shift: usize,
    green_mask_shift: usize,
    blue_mask_shift: usize,
    bpp: usize,
}

impl Framebuffer {
    pub unsafe fn new(limine_fb: &'static NonNullPtr<limine::Framebuffer>) -> Self {
        Self {
            fb: limine_fb.address.as_ptr().unwrap(),
            fb_size: limine_fb.height as usize * limine_fb.pitch as usize,
            width: limine_fb.width as usize,
            height: limine_fb.height as usize,
            pitch: limine_fb.pitch as usize,
            red_mask_shift: limine_fb.red_mask_shift as usize,
            green_mask_shift: limine_fb.green_mask_shift as usize,
            blue_mask_shift: limine_fb.blue_mask_shift as usize,
            bpp: limine_fb.bpp as usize,
        }
    }

    #[inline(always)]
    pub fn write_pixel(&mut self, row: usize, col: usize, color: Color) {
        let px_begin = row * self.pitch + col * (self.bpp / 8);
        if px_begin + 3 > self.fb_size {
            return;
        }
        let r_offset = self.red_mask_shift / 8;
        let g_offset = self.green_mask_shift / 8;
        let b_offset = self.blue_mask_shift / 8;
        unsafe {
            write_volatile(self.fb.offset((px_begin + r_offset) as isize), color.r);
            write_volatile(self.fb.offset((px_begin + g_offset) as isize), color.g);
            write_volatile(self.fb.offset((px_begin + b_offset) as isize), color.b);
        }
    }

    #[inline(always)]
    pub fn read_pixel(&self, row: usize, col: usize) -> Color {
        let px_begin = row * self.pitch + col * (self.bpp / 8);
        if px_begin + 3 >= self.fb_size {
            return Color::BLACK;
        }
        let r_offset = self.red_mask_shift / 8;
        let g_offset = self.green_mask_shift / 8;
        let b_offset = self.blue_mask_shift / 8;
        unsafe {
            let r = read_volatile(self.fb.offset((px_begin + r_offset) as isize));
            let g = read_volatile(self.fb.offset((px_begin + g_offset) as isize));
            let b = read_volatile(self.fb.offset((px_begin + b_offset) as isize));
            Color {
                r,
                g,
                b,
            }
        }
    }

    pub fn copy_pixel(&mut self, row: usize, col: usize, to_row: usize, to_col: usize) {
        let px_begin = row * self.pitch + col * (self.bpp / 8);
        let px_to_begin = row * self.pitch + col * (self.bpp / 8);
        let by_pp = self.bpp / 8;
        if px_begin + by_pp - 1 > self.fb_size || px_to_begin + by_pp - 1 > self.fb_size {
            return;
        }
        for i in 0..by_pp {
            unsafe {
                let b = read_volatile(self.fb.offset((px_begin + i) as isize));
                write_volatile(self.fb.offset((px_to_begin + i) as isize), b);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
    };

    pub const BLACK: Color = Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
    };
}

static mut buf: [u8; 4000 * 4000 * 4] = [0; 4000 * 4000 * 4];

pub struct DoubleBufferedFramebuffer {
    pub fb: Framebuffer,
    buffer: &'static mut [u8; 4000 * 4000 * 4],
}

impl DoubleBufferedFramebuffer {
    pub fn new(fb: Framebuffer) -> Self {
        if fb.fb_size > 4000 * 4000 * 4 {
            panic!();
        }
        Self {
            fb,
            buffer: unsafe {&mut buf},
        }
    }

    pub fn write_pixel(&mut self, row: usize, col: usize, color: Color) {
        self.write_px_lazy(row, col, color);
        self.blit_px(row, col);
    }

    #[inline]
    pub fn write_px_lazy(&mut self, row: usize, col: usize, color: Color) {
        let px_begin = row * self.fb.pitch + col * (self.fb.bpp / 8);
        let by_pp = self.fb.bpp / 8;
        if px_begin + by_pp - 1 > self.fb.fb_size {
            panic!()
        }
        let r_offset = self.fb.red_mask_shift / 8;
        let g_offset = self.fb.green_mask_shift / 8;
        let b_offset = self.fb.blue_mask_shift / 8;
        self.buffer[px_begin + r_offset] = color.r;
        self.buffer[px_begin + g_offset] = color.g;
        self.buffer[px_begin + b_offset] = color.b;
    }

    #[inline]
    pub fn blit_px(&mut self, row: usize, col: usize) {
        let px_begin = row * self.fb.pitch + col * (self.fb.bpp / 8);
        let by_pp = self.fb.bpp / 8;
        if px_begin + by_pp - 1 >= self.fb.fb_size {
            panic!()
        }
        for i in 0..by_pp {
            unsafe {
                write_volatile(self.fb.fb.offset((px_begin + i) as isize), self.buffer[px_begin+i]);
            }
        }
    }

    #[inline]
    pub fn read_pixel(&self, row: usize, col: usize) -> Color {
        let px_begin = row * self.fb.pitch + col * (self.fb.bpp / 8);
        if px_begin + 3 >= self.fb.fb_size {
            panic!()
        }
        let r_offset = self.fb.red_mask_shift / 8;
        let g_offset = self.fb.green_mask_shift / 8;
        let b_offset = self.fb.blue_mask_shift / 8;
        Color {
            r: self.buffer[px_begin + r_offset],
            g: self.buffer[px_begin + g_offset],
            b: self.buffer[px_begin + b_offset],
        }
    }

    pub fn copy_pixel(&mut self, row: usize, col: usize, to_row: usize, to_col: usize) {
        let px_begin = row * self.fb.pitch + col * (self.fb.bpp / 8);
        let px_to_begin = to_row * self.fb.pitch + to_col * (self.fb.bpp / 8);
        let by_pp = self.fb.bpp / 8;
        for i in 0..by_pp {
            self.buffer[px_to_begin + i] = self.buffer[px_begin + i];
        }
    }
}

const PSF_FONT: &[u8] = include_bytes!("ter-powerline-v16b.psf");

pub struct FramebufferWriter {
    pub fb: DoubleBufferedFramebuffer,
    font: Font<&'static [u8]>,
    font_width: usize,
    font_height: usize,
    px_row: usize,
    px_col: usize,
    pub fg: Color,
    pub bg: Color,
    fb_width_glyphs: usize,
    fb_height_glyphs: usize,
    current_glyph_row: usize,
    current_glyph_col: usize,
}

impl FramebufferWriter {
    pub fn new(
        fb: DoubleBufferedFramebuffer,
        font_data: &'static [u8],
        px_row: usize,
        px_col: usize,
        fg: Color,
        bg: Color,
    ) -> Self {
        let font = Font::new(font_data).unwrap();
        let font_width = font.width() as usize;
        let font_height = font.height() as usize;
        let fb_width_glyphs = (fb.fb.width - px_col) / font_width;
        let fb_height_glyphs = (fb.fb.height - px_row) / font_height;
        Self {
            fb,
            font,
            font_width,
            font_height,
            px_row,
            px_col,
            fg,
            bg,
            fb_width_glyphs,
            fb_height_glyphs,
            current_glyph_row: 0,
            current_glyph_col: 0,
        }
    }

    fn write_byte_at_pixel(&mut self, px_row: usize, px_col: usize, c: u8) {
        for (r, row) in self
            .font
            .get_ascii(c)
            .unwrap_or_else(|| self.font.get_ascii(b' ').unwrap())
            .enumerate()
        {
            for (c, px) in row.enumerate() {
                self.fb
                    .write_pixel(px_row + r, px_col + c, if px { self.fg } else { self.bg });
            }
        }
    }

    pub fn write_byte_at(&mut self, row: usize, col: usize, c: u8) {
        self.write_byte_at_pixel(self.px_row + row * self.font_height, self.px_col + col * self.font_width, c);
    }

    pub fn scroll_down_px(&mut self, by: usize) {
        for row in self.px_row+by..self.fb.fb.height {
            for col in self.px_col..self.fb.fb.width {
                self.fb.copy_pixel(row, col, row - by, col);
            }
        }
        for row in self.px_row..self.fb.fb.height-by {
            for col in self.px_col..self.fb.fb.width {
                self.fb.blit_px(row, col);
            }
        }
    }

    pub fn newline(&mut self) {
        if self.current_glyph_row == self.fb_height_glyphs - 1 {
            self.scroll_down_px(self.font_height)
        } else {
            self.current_glyph_row += 1;
        }
        self.current_glyph_col = 0;
    }

    pub fn advance(&mut self) {
        self.current_glyph_col += 1;
        if self.current_glyph_col == self.fb_width_glyphs {
            self.newline();
        }
    }

    pub fn write_byte(&mut self, c: u8) {
        match c {
            b'\n' => self.newline(),
            c => {
                self.write_byte_at(self.current_glyph_row, self.current_glyph_col, c);
                self.advance();
            }
        }
    }

    pub fn write_str_lossy(&mut self, s: &str) {
        for b in s.bytes() {
            self.write_byte(b);
        }
    }
}

impl Write for FramebufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str_lossy(s);
        Ok(())
    }
}