use crate::helper::tritvector::{ TritVector };
use crate::dispatcher::entity::Entity;

// static mut current_quant: usize = 0;
// // static mut queue: Vec<&Event> = vec![];
#[derive(Clone)]
pub struct Event<'a> {
    pub entity: &'a mut Entity<'a>,
    pub quant: usize,
    pub value: TritVector
}

impl<'a> Event<'a> {
    pub fn new(entity: &'a mut Entity<'a>, value: TritVector, delay: usize) -> Event<'a> {
        let event = Event {
            entity,
            value,
            quant: delay
        };
        // queue.push(&event);
        event
    }

    pub fn dispatch(&'a mut self) {
        self.entity.process_effect(self.value.clone());
    }

}