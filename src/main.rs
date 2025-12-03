#![allow(dead_code, unused_variables)]

fn main() {
    let target_pos = Point::new(42, 10);
}

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Direction {
    /// Unit vector of the direction
    unit: Vector,
    /// Speed in m/s
    speed: u32,
}

impl Direction {
    fn new(x: i32, y: i32, speed: u32) -> Self {
        // Naive approach: Always normalize
        let unit = normalize_vector(Vector::new(x, y));

        Self { unit, speed }
    }

    fn accelerate(&mut self, to: u32) {
        self.speed = to;
    }

    fn change(&mut self, x: i32, y: i32) {
        // Naive approach: Always normalize
        self.unit = normalize_vector(Vector::new(x, y))
    }
}

fn normalize_vector(v: Vector) -> Vector {
    let norm = f32::sqrt((v.x.pow(2) + v.y.pow(2)) as f32);

    // Round down to the nearest int, because we are using a int grid.
    let normalized_x = (v.x as f32 / norm).floor() as i32;
    let normalized_y = (v.x as f32 / norm).floor() as i32;
    Vector::new(normalized_x, normalized_y)
}
