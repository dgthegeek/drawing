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
        image.display(self.x, self.y, self.color());
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
        image.draw_line(
            self.start.x,
            self.start.y,
            self.end.x,
            self.end.y,
            self.color(),
        );
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
        image.draw_polygon(
            &[
                (self.vertices[0].x, self.vertices[0].y),
                (self.vertices[1].x, self.vertices[1].y),
                (self.vertices[2].x, self.vertices[2].y),
            ],
            self.color(),
        );
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
        image.draw_rectangle(
            self.top_left.x,
            self.top_left.y,
            self.bottom_right.x,
            self.bottom_right.y,
            self.color(),
        );
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
        image.draw_ellipse(
            self.center.x,
            self.center.y,
            self.radius,
            self.radius,
            self.color(),
        );
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(0..256),
            rng.gen_range(0..256),
            rng.gen_range(0..256),
        )
    }
}