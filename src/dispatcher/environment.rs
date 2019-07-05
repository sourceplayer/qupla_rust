use crate::helper::tritvector::TritVector;
use crate::dispatcher::entity::Entity;
use std::sync::{Arc, Barrier};
use std::thread;

pub struct Environment {
    entities: Vec<Entity>,
    id: String,
    name: String,
    type_info: TypeStatement;
}

impl Environment {
    fn new(name: String, type_info: TypeStatement) -> Environment {
        Environment {
            name,
            type_info
        }
    }

    pub fn affect(&self, value: TritVector, delay: usize) {
        let barrier = Arc::new(Barrier::new(self.entities.len()));
        let handles = Vec::with_capacity(self.entities.len());
        for entity in self.entities {
            let c = barrier.clone();
            handles.push(thread::spawn(move || {
                entity.queue_event(value, delay);
                c.wait();
            }));
        }

        // Wait for other threads to finish.
        for handle in handles {
            handle.join().unwrap();
        }
    }

    pub fn join(&self, entity: Entity) {
        //TODO insert ordered by entity id to be deterministic
        self.entities.push(entity);
    }

    pub fn reset_entity_limits(&self) {
        let barrier = Arc::new(Barrier::new(self.entities.len()));
        let handles = Vec::with_capacity(self.entities.len());
        for entity in self.entities {
            let c = barrier.clone();
            handles.push(thread::spawn(move || {
                entity.reset_limit();
                c.wait();
            }));
        }

        // Wait for other threads to finish.
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
