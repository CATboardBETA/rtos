use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, Ordering};
use core::slice;
use limine::framebuffer::Framebuffer;

trait UnsafeSliceFramebuffer {
    unsafe fn to_mut_slice<'a>(&self) -> &'a mut [u32];
}

impl UnsafeSliceFramebuffer for Framebuffer {
    unsafe fn to_mut_slice<'a>(&self) -> &'a mut [u32] {
        unsafe {
            slice::from_raw_parts_mut(
                self.address().cast::<u32>(),
                (self.width * self.height) as usize,
            )
        }
    }
}

#[derive(Debug)]
pub enum DrawError {
    OutOfBounds,
    NoSuchGlyph(char),
}

pub type DrawResult = Result<(), DrawError>;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
}

impl From<Color> for [u8; 4] {
    fn from(value: Color) -> Self {
        [255, value.r, value.g, value.b]
    }
}

pub struct Gfx<'fb> {
    fb: &'fb Framebuffer,
    double_buffer: Vec<u32>,
    pitch: usize,
    width: usize,
    height: usize,
    pos: (usize, usize),
}

impl<'fb> From<&'fb Framebuffer> for Gfx<'fb> {
    fn from(value: &'fb Framebuffer) -> Self {
        let width = value.width as _;
        let height = value.height as _;
        let pitch = value.pitch as _;
        Self {
            fb: value,
            double_buffer: vec![0u32; height * width],
            pitch,
            width,
            height,
            pos: (0, 0),
        }
    }
}

impl Gfx<'_> {
    pub fn clear(&self, color: Color) {
        unsafe {
            slice::from_raw_parts_mut(
                self.fb.address().cast::<u32>(),
                self.width * self.height,
            ).fill(u32::from_be_bytes(color.into()));
        }
    }

    pub fn swap_buffers(&mut self) {
        self.double_buffer.swap_with_slice(unsafe { self.fb.to_mut_slice() });
    }

    pub unsafe fn write_px_unchecked(&self, x: usize, y: usize, color: [u8; 4]) {
        let offset = y * self.width + x;
        unsafe {
            let slice = slice::from_raw_parts_mut(
                self.fb.address().cast::<u32>(),
                self.width * self.height,
            );
            slice[offset] = u32::from_be_bytes(color);
        }
    }

    pub fn write_px(&self, x: usize, y: usize, color: Color) -> DrawResult {
        if x > self.width || y > self.height {
            Err(DrawError::OutOfBounds)
        } else {
            unsafe {
                self.write_px_unchecked(x, y, color.into());
                Ok(())
            }
        }
    }
    pub fn write_px_iter(
        &self,
        iter: impl IntoIterator<Item = (usize, usize)>,
        color: Color,
    ) -> DrawResult {
        for px in iter {
            self.write_px(px.0, px.1, color)?;
        }
        Ok(())
    }

    pub fn write_px_iter_split(
        &self,
        iter1: impl IntoIterator<Item = usize>,
        iter2: impl IntoIterator<Item = usize>,
        color: Color,
    ) -> DrawResult {
        for (x, y) in iter1.into_iter().zip(iter2) {
            self.write_px(x, y, color)?;
        }
        Ok(())
    }

    pub fn draw_line(
        &self,
        (x0, y0): (usize, usize),
        (x1, y1): (usize, usize),
        color: Color,
    ) -> DrawResult {
        if max(y0, y1) > self.height || max(x0, x1) > self.width {
            Err(DrawError::OutOfBounds)
        } else {
            unsafe { self.draw_line_unchecked((x0, y0), (x1, y1), color) }
            Ok(())
        }
    }

    pub unsafe fn draw_line_unchecked(
        &self,
        (x0, y0): (usize, usize),
        (x1, y1): (usize, usize),
        color: Color,
    ) {
        let dx = x1.abs_diff(x0);
        let dy = y1.abs_diff(y0);
        match usize::cmp(&dy, &dx) {
            Ordering::Less => {
                if x0 > x1 {
                    unsafe { self.draw_line_low_unchecked(x1, y1, x0, y0, color) }
                } else {
                    unsafe { self.draw_line_low_unchecked(x0, y0, x1, y1, color) }
                }
            }
            Ordering::Equal => todo!(),
            Ordering::Greater => {
                if y0 > y1 {
                    unsafe { self.draw_line_high_unchecked(x1, y1, x0, y0, color) }
                } else {
                    unsafe { self.draw_line_high_unchecked(x0, y0, x1, y1, color) }
                }
            }
        }
    }

    pub fn fill_rect(
        &self,
        start: (usize, usize),
        size: (usize, usize),
        color: Color,
    ) -> DrawResult {
        let (x0, y0) = start;
        let (x1, y1) = (x0 + size.0, y0 + size.1);
        if x1 > self.width || y1 > self.height {
            Err(DrawError::OutOfBounds)
        } else {
            unsafe {
                self.fill_rect_unchecked(x0, y0, x1, y1, color);
            }
            Ok(())
        }
    }

    /// Avoids unnecessary bounds checking in `fill_rect`
    pub unsafe fn fill_rect_unchecked(
        &self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        color: Color,
    ) {
        let slice = unsafe {
            slice::from_raw_parts_mut(self.fb.address().cast::<u32>(), self.width * self.height)
        };
        let mut offset = y0 * self.width + x0;
        for y in y0..y1 {
            for x in x0..x1 {
                slice[offset + x] = u32::from_be_bytes(color.into());
            }

            offset += self.width;
        }
    }

    unsafe fn draw_line_low_unchecked(
        &self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        color: Color,
    ) {
        let dx = x1.cast_signed() - x0.cast_signed();
        let mut dy = y1.cast_signed() - y0.cast_signed();
        let yi: isize = if dy < 0 {
            dy = -dy;
            -1
        } else {
            1
        };
        let mut d = 2 * dy - dx;
        let mut y = y0.cast_signed();
        for x in x0..x1 {
            unsafe { self.write_px_unchecked(x, y.cast_unsigned(), color.into()) };
            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }
    }

    unsafe fn draw_line_high_unchecked(
        &self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        color: Color,
    ) {
        let mut dx = x1.cast_signed() - x0.cast_signed();
        let dy = y1.cast_signed() - y0.cast_signed();
        let xi: isize = if dy < 0 {
            dx = -dx;
            -1
        } else {
            1
        };
        let mut d = 2 * dx - dy;
        let mut x = x0.cast_signed();
        for y in y0..y1 {
            unsafe { self.write_px_unchecked(x.cast_unsigned(), y, color.into()) };
            if d > 0 {
                x += xi;
                d += 2 * (dx - dy);
            } else {
                d += 2 * dx;
            }
        }
    }
}
