#![allow(dead_code, unused_variables)]

mod simulation;

fn main() {
    let target = Target::new(12, 12);
    let missile = Missile::new(Point::new(42, 42), target, 0, 858, 400);

    let entities: Vec<Box<dyn simulation::Simulatable>> = vec![Box::new(target), Box::new(missile)];

    let mut simulation = simulation::Simulation::new(entities);

    println!("Simulation State:");
    for i in 0..10 {
        println!("Timestep :{}\n", simulation.timestep);
        simulation
            .entities
            .iter()
            .for_each(|e| println!("{:#?}", e.render()));
        simulation.advance();
    }
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

#[derive(Debug)]
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
    fn happen(&mut self) {}

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
            target_position.x as i32 - current_position.x as i32,
            target_position.y as i32 - current_position.y as i32,
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

    fn accelerate(&mut self) {
        if self.current_speed >= self.max_speed {
            return;
        }

        // We assume uniform acceleration for simplicity.
        self.current_speed += self.acceleration;
    }

    fn update_direction(&mut self) {
        let course = Vector::new(
            self.target_position.x as i32 - self.current_position.x as i32,
            self.target_position.y as i32 - self.current_position.y as i32,
        );
        // Naive approach: Always normalize
        self.direction = normalize_vector(course)
    }

    fn update_position(&mut self, position: Point) {
        self.current_position = position;
    }
}

impl simulation::Simulatable for Missile {
    fn happen(&mut self) {
        // Move one unit towards this direction
        let relative_speed = (self.current_speed / 64) as i32;
        let new_position = Point::new(
            (self.current_position.x as i32 + (self.direction.x * relative_speed)) as u32,
            (self.current_position.y as i32 + (self.direction.y * relative_speed)) as u32,
        );

        println!(
            "Speed: {}\nDirection: {:?}\nNew Position: {:?}",
            self.current_speed, self.direction, new_position
        );
        self.current_position = new_position;

        // Update speed according using acceleration
        (*self).accelerate();
        // Calculate adjusted distance for (target - current) vector
        (*self).update_direction();
    }

    fn render(&self) -> simulation::Entity {
        simulation::Entity::new(self.current_position, simulation::EntityType::Missile)
    }
}

fn normalize_vector(v: Vector) -> Vector {
    let norm = f32::sqrt((v.x.pow(2) + v.y.pow(2)) as f32);

    // Round down to the nearest int, because we are using a int grid.
    let normalized_x = (v.x as f32 / norm).floor() as i32;
    let normalized_y = (v.x as f32 / norm).floor() as i32;
    Vector::new(normalized_x, normalized_y)
}
