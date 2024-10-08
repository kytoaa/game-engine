use super::*;
use query::Query;

pub struct ECS {
    systems: Vec<System>,
}

pub enum System {
    Parallel(Vec<System>),
    Single(Box<dyn Fn(Box<dyn Query>)>),
}

impl ECS {
    pub fn new() -> ECS {
        ECS { systems: vec![] }
    }
}
impl ECS {
    pub fn add_system(&mut self, system: System) -> &mut Self {
        self.systems.push(system);
        self
    }
}
