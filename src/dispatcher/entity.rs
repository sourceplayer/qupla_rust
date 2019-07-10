use crate::helper::tritvector::{ TritVector };
use crate::dispatcher::effect::Effect;
use crate::dispatcher::environment::Environment;
use crate::dispatcher::event::Event;
use crate::dispatcher::supervisor::Supervisor;

#[derive(Clone)]
pub struct Entity<'a> {
    supervisor: &'a mut Supervisor<'a>,
    effects: Vec<Effect<'a>>,
    id: String,
    invoked: usize,
    limit: usize
}

impl<'a> Entity<'a> {
    pub fn new(supervisor: &'a mut Supervisor<'a>, limit: usize) -> Entity {
        Entity {
            supervisor,
            limit,
            id: String::new(),
            invoked: 0,
            effects: vec![]
        }
    }

    pub fn start(&self, entity_name: String) {
        // let entity = Entity::new(0);
    }

    pub fn affect(&mut self, env: &'a mut Environment<'a>, delay: usize) {
        let effect = Effect::new(env, delay);
        self.effects.push(effect);
    }

    pub fn join(&'a mut self, env: &'a mut Environment<'a>) {
        env.join(self);
    }

    pub fn process_effect(&'a mut self, effect: TritVector) {

        // Have entity process the effect
        let result: Option<TritVector> = self.on_effect(Some(effect));
        if result == None {
            // Propagation stops if null is returned (no data flow)
            return
        }

        // queue any effects that entity triggered
        self.queue_effect_events(result.unwrap());
    }

    pub fn queue_effect_events(&'a mut self, value: TritVector) {
        // All effects for this entity have been predetermined already
        // from the metadata, so we just need to queue them as events
        for effect in self.effects.iter_mut() {
            effect.queue_environment_events(value.clone());
        }
    }

    pub fn queue_event(&'a mut self, value: TritVector, delay: usize) {
        // Queue an event for this entity with proper delay
        if delay == 0 && self.invoked < self.limit {
            // Can do another invocation during the current quant
            self.invoked += 1;
            let mut event: Event = Event::new(self, value, delay);
        } else {
            // Invocation limit exceeded, schedule for next quant
            let mut event: Event = Event::new(self, value, delay);
            event.quant += 1;
        }
    }

    pub fn reset_limit(&mut self) {
        self.limit = 0;
    }

    pub fn stop(&mut self) {

    }

}

trait OnEffect {
    fn on_effect(&self, effect: Option<TritVector>) -> Option<TritVector>;
}

impl<'a> OnEffect for Entity<'a> {
    fn on_effect(&self, effect: Option<TritVector>) -> Option<TritVector>
    {
        None
    }
}

