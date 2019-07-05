use crate::helper::tritvector::TritVector;
use crate::dispatcher::environment::Environment;

pub struct Effect
{
    pub delay: usize,
    pub environment: Environment
}

impl Effect {
    pub fn new(environment: Environment, delay: usize) -> Effect {
        Effect {
            environment,
            delay
        }
    }

    pub fn queue_environment_events(&self, value: TritVector) {
        // Transform the effect into one or more entity events in the event queue
        self.environment.affect(value, self.delay);
    }

}