use crate::Point;

pub struct Simulation {
    /// Current timestep of a running simulation
    timestep: u32,
    grid: SimulationGrid,
    entity: Vec<Box<dyn Simulatable>>,
}

impl Simulation {
    pub fn advance(mut self) -> Self {
        self.entity.iter().for_each(|e| {
            e.happen();
            let entity = e.render();
            if let Some(row) = self.grid.cells.get_mut(entity.position.x as usize)
                && let Some(cell) = row.get_mut(entity.position.y as usize)
            {
                *cell = entity;
            }
        });
        self
    }
}

// Not sure if i even need this. Could also be implicit.
struct SimulationGrid {
    cells: [[Entity; 64]; 64],
}

enum EntityType {
    Missile,
    Target,
}

struct Entity {
    position: Point,
    r#type: EntityType,
}

trait Simulatable {
    fn happen(&self);
    fn render(&self) -> Entity;
}
