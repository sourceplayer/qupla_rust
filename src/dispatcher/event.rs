use crate::helper::tritvector::{ TritVector };
use crate::dispatcher::entity::Entity;

// static mut current_quant: usize = 0;
// // static mut queue: Vec<&Event> = vec![];

#[derive(Clone)]
pub struct Event {
    pub quant: usize,
    pub value: TritVector
}

impl Event {
    pub fn new(value: TritVector, delay: usize) -> Event {
        let event = Event {
            value,
            quant: delay
        };
        // queue.push(&event);
        event
    }


}