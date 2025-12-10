use std::{collections::HashSet, fmt::Display};

use nalgebra::{Point2, Vector2};

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
        self.entities.iter_mut().enumerate().for_each(|(i, e)| {
            let updated_entity_data = e.happen();
            self.state[i] = SimulationEntity::new(updated_entity_data, self.state[i].state);
        });

        // Find entity indices that got destroyed.
        let mut destroyed_indices: HashSet<usize> = HashSet::new();
        let missiles = self
            .state
            .iter()
            .filter(|e| e.data.r#type == EntityType::Missile);
        // Check if something like 'split_by_predicate' exists.
        // Partition in missiles and targets and only check if missiles hit the targets using
        // raytracing.
        for i in 0..self.entities.len() - 1 {
            for j in i + 1..self.entities.len() {
                let ent_one = self.state[i];
                let ent_two = self.state[j];

                if ent_one.data.r#type == ent_two.data.r#type {
                    continue;
                }

                // TODO: Raycasting colision detection using direction, speed and current_pos
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
    direction: Option<Vector2<f32>>,
    speed: Option<f32>,
    r#type: EntityType,
}

impl Entity {
    pub fn new(
        position: Point2<f32>,
        direction: Option<Vector2<f32>>,
        speed: Option<f32>,
        r#type: EntityType,
    ) -> Self {
        Self {
            position,
            r#type,
            direction,
            speed,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SimulationEntity {
    data: Entity,
    state: EntityState,
}

impl Display for SimulationEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Type '{:?}'\tCoordinates {:?}\tState '{:?}'",
            self.data.r#type, self.data.position, self.state
        )
    }
}

impl SimulationEntity {
    pub fn new(data: Entity, state: EntityState) -> Self {
        Self { data, state }
    }
}

pub trait Simulatable {
    fn happen(&mut self) -> Entity;
    fn render(&self) -> Entity;
}
