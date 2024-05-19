use crate::frame_buffer_writer::{FrameBufferWriter};
use crate::frame_buffer_writer::pixel_color::PixelColor;
use crate::frame_buffer_writer::vector2d::Vector2D;


impl FrameBufferWriter {
    pub fn fill_rectangle(&mut self, pos: Vector2D<usize>, size: Vector2D<usize>, color: PixelColor) {
        for x in 0..size.x {
            for y in 0..size.y {
                self.write_pixel(pos + Vector2D::new(x, y), &color);
            }
        }
    }

    pub fn draw_rectangle(&mut self, pos: Vector2D<usize>, size: Vector2D<usize>, color: PixelColor) {
        for x in 0..size.x {
            self.write_pixel(pos + Vector2D::new(x, 0), &color);
            self.write_pixel(pos + Vector2D::new(x, size.y), &color);
        }

        for y in 0..size.y {
            self.write_pixel(pos + Vector2D::new(0, y), &color);
            self.write_pixel(pos + Vector2D::new(size.x, y), &color);
        }
    }
}