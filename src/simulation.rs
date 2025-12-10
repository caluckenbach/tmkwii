use std::collections::HashSet;

use nalgebra::Point2;

pub struct Simulation {
    /// Current timestep of a running simulation
    pub timestep: usize,
    state: SimulationEntities,
    pub entities: Vec<Box<dyn Simulatable>>,
}

impl Simulation {
    pub fn new(entities: Vec<Box<dyn Simulatable>>) -> Self {
        let state = entities
            .iter()
            .map(|e| SimulationEntity::new(e.render(), EntityState::Functional))
            .collect::<SimulationEntities>();

        Self {
            timestep: 0,
            state,
            entities,
        }
    }

    pub fn advance(&mut self) {
        if self.entities.is_empty() {
            self.timestep += 1;
            return;
        }

        // Advance the simulation state of all entities.
        self.entities.iter_mut().for_each(|e| e.happen());

        // Find entity indices that got destroyed.
        let mut destroyed_indices: HashSet<usize> = HashSet::new();
        for i in 0..self.entities.len() - 1 {
            for j in i + 1..self.entities.len() {
                let ent_one = self.state[i];
                let ent_two = self.state[j];

                if ent_one.data.r#type == ent_two.data.r#type {
                    continue;
                }

                // TODO: Extract the epsilon.
                if nalgebra::distance(&ent_one.data.position, &ent_two.data.position) < 0.5 {
                    destroyed_indices.insert(i);
                    destroyed_indices.insert(j);
                }
            }
        }

        // Update state for destroyed entities
        destroyed_indices.iter().for_each(|index| {
            self.state[*index].state = EntityState::Destroyed;
        });

        self.timestep += 1;
    }

    pub fn render(&self) -> SimulationEntities {
        self.state.clone()
    }
}

pub type SimulationEntities = Vec<SimulationEntity>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityType {
    Missile,
    Target,
}

#[derive(Debug, Clone, Copy)]
pub enum EntityState {
    Functional,
    Destroyed,
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    position: Point2<f32>,
    r#type: EntityType,
}

impl Entity {
    pub fn new(position: Point2<f32>, r#type: EntityType) -> Self {
        Self { position, r#type }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SimulationEntity {
    data: Entity,
    state: EntityState,
}

impl SimulationEntity {
    pub fn new(data: Entity, state: EntityState) -> Self {
        Self { data, state }
    }
}

pub trait Simulatable {
    fn happen(&mut self);
    fn render(&self) -> Entity;
}
