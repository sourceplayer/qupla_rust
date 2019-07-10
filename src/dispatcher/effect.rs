use crate::helper::tritvector::TritVector;
use crate::dispatcher::environment::Environment;

#[derive(Clone)]
pub struct Effect<'a> {
    pub env: &'a mut Environment<'a>,
    pub delay: usize,

}

impl<'a> Effect<'a> {
    pub fn new(env: &'a mut Environment<'a>, delay: usize) -> Effect<'a> {
        Effect {
            env,
            delay
        }
    }

    pub fn queue_environment_events(&'a mut self, value: TritVector) {
        // Transform the effect into one or more entity events in the event queue
        self.env.affect(value, self.delay);
    }

}