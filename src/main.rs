#![allow(dead_code, unused_variables)]

use nalgebra::{Point2, Vector2};

mod simulation;

fn main() {
    let target = Target::new(25_000.0, 25_000.0);
    let missile = Missile::new(Point2::new(0.0, 0.0), target, 0.0, 858.0, 400.0);

    let entities: Vec<Box<dyn simulation::Simulatable>> = vec![Box::new(target), Box::new(missile)];

    let mut simulation = simulation::Simulation::new(entities);

    println!("Simulation State:");
    println!("Target at coordinates {}", target);
    for i in 0..42 {
        println!("Timestep {}", simulation.timestep);
        simulation.render().iter().for_each(|e| println!("{}", e));
        println!("\n");
        simulation.advance();
    }
}

type Target = Point2<f32>;

impl simulation::Simulatable for Target {
    fn happen(&mut self) -> simulation::Entity {
        self.render()
    }

    fn render(&self) -> simulation::Entity {
        simulation::Entity::new(*self, None, None, simulation::EntityType::Target)
    }
}

struct Missile {
    /// Unit vector of the direction
    direction: Vector2<f32>,
    /// Speed in m/s
    current_speed: f32,
    /// Maximum speed in m/s
    max_speed: f32,
    /// In m/s^2
    acceleration: f32,

    current_position: Point2<f32>,
    target_position: Point2<f32>,
}

impl Missile {
    fn new(
        current_position: Point2<f32>,
        target_position: Point2<f32>,
        current_speed: f32,
        max_speed: f32,
        acceleration: f32,
    ) -> Self {
        let dir = target_position - current_position;
        // Naive approach: Always normalize
        let unit = dir.normalize();

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
        self.direction = (self.target_position - self.current_position).normalize();
    }

    fn update_position(&mut self, position: Point2<f32>) {
        self.current_position = position;
    }
}

impl simulation::Simulatable for Missile {
    fn happen(&mut self) -> simulation::Entity {
        // Move one unit towards this direction
        let new_position = self.current_position + self.current_speed * self.direction;

        self.current_position = new_position;

        // Update speed according using acceleration
        (*self).accelerate();
        // Calculate adjusted distance for (target - current) vector
        (*self).update_direction();

        simulation::Entity::new(
            self.current_position,
            Some(self.direction),
            Some(self.current_speed),
            simulation::EntityType::Missile,
        )
    }

    fn render(&self) -> simulation::Entity {
        simulation::Entity::new(
            self.current_position,
            Some(self.direction),
            Some(self.current_speed),
            simulation::EntityType::Missile,
        )
    }
}
