use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn random(width: u32, height: u32) -> Point {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width as i32),
            y: rng.gen_range(0..height as i32),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.set_pixel(self.x as i32, self.y as i32, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 0)
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: &Point, end: &Point) -> Line {
        Line {
            start: start.clone(),
            end: end.clone(),
        }
    }

    pub fn random(width: u32, height: u32) -> Line {
        let mut rng = rand::thread_rng();
        let start_x = rng.gen_range(0..width as i32);
        let start_y = rng.gen_range(0..height as i32);
        let end_x = rng.gen_range(0..width as i32);
        let end_y = rng.gen_range(0..height as i32);
        Line::new(&Point::new(start_x, start_y), &Point::new(end_x, end_y))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let dx = (self.end.x - self.start.x) as f32;
        let dy = (self.end.y - self.start.y) as f32;
        let steps = dx.abs().max(dy.abs()) as u32;

        for i in 0..=steps {
            let x = (self.start.x as f32 + (dx * (i as f32 / steps as f32))).round() as u32;
            let y = (self.start.y as f32 + (dy * (i as f32 / steps as f32))).round() as u32;
            image.set_pixel(x as i32, y as i32, self.color());
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 255, 0)
    }
}

pub struct Triangle {
    vertices: [Point; 3],
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Triangle {
        Triangle {
            vertices: [p1.clone(), p2.clone(), p3.clone()],
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let mut edges = vec![
            Line::new(&self.vertices[0], &self.vertices[1]),
            Line::new(&self.vertices[1], &self.vertices[2]),
            Line::new(&self.vertices[2], &self.vertices[0]),
        ];

        for edge in edges {
            edge.draw(image);
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 0, 255)
    }
}

pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: &Point, bottom_right: &Point) -> Rectangle {
        Rectangle {
            top_left: top_left.clone(),
            bottom_right: bottom_right.clone(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let top_left = (self.top_left.x as u32, self.top_left.y as u32);
        let bottom_right = (self.bottom_right.x as u32, self.bottom_right.y as u32);

        for x in top_left.0..bottom_right.0 {
            for y in top_left.1..bottom_right.1 {
                image.set_pixel(x as i32, y as i32, self.color());
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(255, 255, 0)
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Circle {
        Circle {
            center: center.clone(),
            radius,
        }
    }

    pub fn random(width: u32, height: u32) -> Circle {
        let mut rng = rand::thread_rng();
        let center_x = rng.gen_range(0..width as i32);
        let center_y = rng.gen_range(0..height as i32);
        let radius = rng.gen_range(10..100);
        Circle::new(&Point::new(center_x, center_y), radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let center_x = self.center.x as f32;
        let center_y = self.center.y as f32;
        let radius = self.radius as f32;

        for angle in 0..=360 {
            let radian = angle as f32 * std::f32::consts::PI / 180.0;
            let x = (center_x + radius * radian.cos()) as u32;
            let y = (center_y + radius * radian.sin()) as u32;

            if x < image.width.try_into().unwrap() && y < image.height.try_into().unwrap() {
                image.set_pixel(x as i32, y as i32, self.color());
            }
        }
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
    }
}