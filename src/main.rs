#![allow(dead_code, unused_variables)]

mod simulation;

fn main() {
    let target = Target::new(12, 12);
    let missile = Missile::new(Point::new(42, 42), target, 0, 858, 400);

    let entities: Vec<Box<dyn simulation::Simulatable>> = vec![Box::new(target), Box::new(missile)];

    let simulation = simulation::Simulation::new(entities);
}

#[derive(Debug, Clone, Copy)]
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

type Target = Point;

impl simulation::Simulatable for Target {
    fn happen(&self) {
        return;
    }

    fn render(&self) -> simulation::Entity {
        simulation::Entity::new(*self, simulation::EntityType::Target)
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
            (target_position.x - current_position.x).try_into().unwrap(),
            (target_position.y - current_position.y).try_into().unwrap(),
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
        self.current_speed += self.acceleration;
    }

    fn change_direction(&mut self, x: i32, y: i32) {
        // Naive approach: Always normalize
        self.direction = normalize_vector(Vector::new(x, y))
    }
}

impl simulation::Simulatable for Missile {
    fn happen(&self) {
        todo!()
    }

    fn render(&self) -> simulation::Entity {
        todo!()
    }
}

fn normalize_vector(v: Vector) -> Vector {
    let norm = f32::sqrt((v.x.pow(2) + v.y.pow(2)) as f32);

    // Round down to the nearest int, because we are using a int grid.
    let normalized_x = (v.x as f32 / norm).floor() as i32;
    let normalized_y = (v.x as f32 / norm).floor() as i32;
    Vector::new(normalized_x, normalized_y)
}
