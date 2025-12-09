use crate::Point;

pub struct Simulation {
    /// Current timestep of a running simulation
    timestep: u32,
    grid: SimulationGrid,
    entities: Vec<Box<dyn Simulatable>>,
}

impl Simulation {
    pub fn new(entities: Vec<Box<dyn Simulatable>>) -> Self {
        Self {
            timestep: 0,
            grid: SimulationGrid::default(),
            entities: entities,
        }
    }

    pub fn advance(mut self) -> Self {
        self.entities.iter().enumerate().for_each(|(i, e)| {
            e.happen();
            let entity = e.render();
            if let Some(row) = self.grid.cells.get_mut(entity.position.x as usize)
                && let Some(cell) = row.get_mut(entity.position.y as usize)
            {
                *cell = Some(i);
            }
        });
        self
    }
}

type EntityIndex = usize;

// Not sure if i even need this. Could also be implicit.
struct SimulationGrid {
    cells: [[Option<EntityIndex>; 64]; 64],
}

impl Default for SimulationGrid {
    fn default() -> Self {
        Self {
            cells: [[None; 64]; 64],
        }
    }
}

pub enum EntityType {
    Missile,
    Target,
}

pub struct Entity {
    position: Point,
    r#type: EntityType,
}

impl Entity {
    pub fn new(position: Point, r#type: EntityType) -> Self {
        Self { position, r#type }
    }
}

pub trait Simulatable {
    fn happen(&self);
    fn render(&self) -> Entity;
}
