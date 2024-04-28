# Geometric Shapes Drawing

This project provides a set of geometric shapes that can be drawn on an image. It includes structs for representing points, lines, triangles, rectangles, and circles, along with trait implementations for drawing these shapes using the `raster` crate.

## Dependencies

- `rand` crate (version 0.8.5) - for generating random numbers
- `raster` crate (version 0.2.0) - for image manipulation

## Modules

### `geometrical_shapes`

This module defines the following traits and structs:

#### Traits

- `Drawable`: This trait requires implementors to provide methods for drawing the shape on an image (`draw(&self, image: &mut Image)`) and retrieving the color of the shape (`color(&self) -> Color`).
- `Displayable`: This trait requires implementors to provide a method for setting a pixel at a specific coordinate with a given color (`display(&mut self, x: i32, y: i32, color: Color)`).

#### Structs

- `Point`: Represents a 2D point with `x` and `y` coordinates.
- `Line`: Represents a line segment defined by two points.
- `Triangle`: Represents a triangle defined by three points.
- `Rectangle`: Represents a rectangle defined by two points (top-left and bottom-right corners).
- `Circle`: Represents a circle defined by a center point and a radius.

Each struct provides an associated `new` function for creating instances and implements the `Drawable` trait. Additionally, `Point`, `Line`, and `Circle` provide an associated `random` function for generating random instances within specified bounds.

### `main`

This module contains the `main` function, which creates a blank image, draws various shapes on it, and saves the resulting image as "image.png". It also provides an implementation of the `Displayable` trait for the `Image` struct from the `raster` crate.

## Usage

1. Ensure you have the required dependencies installed by running `cargo build` in the project directory.
2. Run the project with `cargo run`.
3. The resulting image will be saved as "image.png" in the project directory.

## Example

```rust
use geometrical_shapes as gs;
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);
    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(50, 50), &gs::Point::new(150, 150));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

This example draws a random line, a random point, a rectangle, a triangle, and 50 random circles on a 1000x1000 image, and then saves the image as "image.png".