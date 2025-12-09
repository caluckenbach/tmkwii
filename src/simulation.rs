use crate::Point;

pub struct Simulation {
    /// Current timestep of a running simulation
    pub timestep: u32,
    grid: SimulationGrid,
    pub entities: Vec<Box<dyn Simulatable>>,
}

impl Simulation {
    pub fn new(entities: Vec<Box<dyn Simulatable>>) -> Self {
        Self {
            timestep: 0,
            grid: SimulationGrid::default(),
            entities,
        }
    }

    pub fn advance(&mut self) {
        self.entities.iter_mut().enumerate().for_each(|(i, e)| {
            e.happen();
            let entity = e.render();
            if let Some(row) = self.grid.cells.get_mut(entity.position.x as usize)
                && let Some(cell) = row.get_mut(entity.position.y as usize)
            {
                *cell = Some(i);
            }
        });
        self.timestep += 1;
    }

    pub fn render(&self) -> SimulationGrid {
        self.grid
    }
}

type EntityIndex = usize;

// Not sure if i even need this. Could also be implicit.
#[derive(Debug, Clone, Copy)]
pub struct SimulationGrid {
    cells: [[Option<EntityIndex>; 64]; 64],
}

impl Default for SimulationGrid {
    fn default() -> Self {
        Self {
            cells: [[None; 64]; 64],
        }
    }
}

#[derive(Debug)]
pub enum EntityType {
    Missile,
    Target,
}

#[derive(Debug)]
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
    fn happen(&mut self);
    fn render(&self) -> Entity;
}
