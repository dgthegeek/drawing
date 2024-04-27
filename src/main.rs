mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Drawable, Displayable};
use image::{ImageBuffer, Rgba};

fn main() {
    let mut image = ImageBuffer::new(1000, 1000);

    gs::Line::random(image.width() as u32, image.height() as u32).draw(&mut image);
    gs::Point::random(image.width() as u32, image.height() as u32).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width() as u32, image.height() as u32).draw(&mut image);
    }

    image.save("image.png").unwrap();
}

impl Displayable for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn display(&mut self, x: i32, y: i32, color: Rgba<u8>) {
        if x >= 0 && x < self.width() as i32 && y >= 0 && y < self.height() as i32 {
            self.put_pixel(x as u32, y as u32, color);
        }
    }
}