use crate::helper::tritvector::TritVector;
use crate::dispatcher::entity::Entity;
use crate::qupla::statement::typestmt::TypeStmt;

#[derive(Clone)]
pub struct Environment<'a> {
    entities: Vec<&'a mut Entity<'a>>,
    id: String,
    name: String,
    pub type_info: Option<TypeStmt>
}

impl<'a> Environment<'a> {
   pub fn new(name: String, type_info: Option<TypeStmt>) -> Environment<'a> {
        Environment {
            entities: vec![],
            id: String::new(),
            name: name.clone(),
            type_info
        }
    }

    pub fn affect(&'a mut self, value: TritVector, delay: usize) {
        for entity in self.entities.iter_mut() {
            entity.queue_event(value.clone(), delay);
        }
    }

    pub fn join(& mut self, entity: &'a mut Entity<'a>) {
        //TODO insert ordered by entity id to be deterministic
        self.entities.push(entity);
    }

    pub fn reset_entity_limits(& mut self) {
        for entity in self.entities.iter_mut() {
            entity.reset_limit();
        }
    }
}
