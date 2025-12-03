#![allow(dead_code, unused_variables)]

fn main() {
    let target_pos = Point::new(42, 10);
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
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

struct Missile {
    /// Unit vector of the direction
    direction: Vector,
    /// Speed in m/s
    current_speed: u32,
    /// Maximum speed in m/s
    max_speed: u32,
    /// In m/s^2
    acceleration: u32,

    current_position: Point,
    target_position: Point,
}

impl Missile {
    fn new(
        current_position: Point,
        target_position: Point,
        current_speed: u32,
        max_speed: u32,
        acceleration: u32,
    ) -> Self {
        let dir = Vector::new(
            target_position.x - current_position.x,
            target_position.y - current_position.y,
        );

        // Naive approach: Always normalize
        let unit = normalize_vector(dir);

        Self {
            direction: unit,
            current_speed,
            max_speed,
            acceleration,
            current_position,
            target_position,
        }
    }

    fn accelerate(&mut self, to: u32) {
        if self.current_speed >= self.max_speed {
            return;
        }

        // We assume uniform acceleration for simplicity.
        self.current_speed = self.current_speed + self.acceleration;
    }

    fn change_direction(&mut self, x: i32, y: i32) {
        // Naive approach: Always normalize
        self.direction = normalize_vector(Vector::new(x, y))
    }
}

fn normalize_vector(v: Vector) -> Vector {
    let norm = f32::sqrt((v.x.pow(2) + v.y.pow(2)) as f32);

    // Round down to the nearest int, because we are using a int grid.
    let normalized_x = (v.x as f32 / norm).floor() as i32;
    let normalized_y = (v.x as f32 / norm).floor() as i32;
    Vector::new(normalized_x, normalized_y)
}
