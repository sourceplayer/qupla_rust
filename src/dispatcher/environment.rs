use crate::helper::tritvector::TritVector;
use crate::dispatcher::entity::Entity;

pub struct Environment<'a> {
    entities: Vec<Entity<'a>>,
    id: String,
    name: String,
    // type_info: TypeStatement
}

impl<'a> Environment<'a> {
    fn new(name: String) -> Environment<'a> {
        Environment {
            entities: vec![],
            id: String::new(),
            name
        }
    }

    pub fn affect(&mut self, value: TritVector, delay: usize) {
        for entity in self.entities.iter_mut() {
            entity.queue_event(value.clone(), delay);
        }
    }

    pub fn join(& mut self, entity: Entity<'a>) {
        //TODO insert ordered by entity id to be deterministic
        self.entities.push(entity);
    }

    pub fn reset_entity_limits(& mut self) {
        for entity in self.entities.iter_mut() {
            entity.reset_limit();
        }
    }
}
