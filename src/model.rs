use crate::dispatcher::event::Event;

pub struct Model<'a> {
    pub current_quant: usize,
    pub queue: Vec<&'a Event>

}

impl<'a> Model<'a> {
    pub fn new() -> Model<'a> {
        Model {
            current_quant: 0,
            queue: vec![],

        }
    }
}