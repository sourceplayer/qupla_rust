use crate::helper::tritvector::{ TritVector };
use crate::dispatcher::effect::Effect;
use crate::dispatcher::environment::Environment;
use crate::dispatcher::event::Event;

#[allow(unused_variables)] 
pub struct Entity<'a> {
    effects: Vec<Effect<'a>>,
    events: Vec<Event>,
    id: String,
    invoked: usize,
    limit: usize
}

impl<'a> From<usize> for Entity<'a> {
    fn from(limit: usize) -> Entity<'a> {
        Entity {
            limit,
            id: String::new(),
            invoked: 0,
            effects: vec![],
            events: vec![]
        }
    }
}

impl<'a> Entity<'a> {
    pub fn start(&self, entity_name: String) {
        let entity = Entity::from(0);
    }

    pub fn affect(&mut self, env: &'a mut Environment<'a>, delay: usize) {
        let effect = Effect::new(env, delay);
        self.effects.push(effect);
    }

    pub fn join(self, env: &mut Environment<'a>) {
        env.join(self);
    }

    pub fn process_effect(&mut self, effect: TritVector) {

        // Have entity process the effect
        let result: Option<TritVector> = self.on_effect(Some(effect));
        if result == None {
            // Propagation stops if null is returned (no data flow)
            return
        }

        // queue any effects that entity triggered
        self.queue_effect_events(result.unwrap());
    }

    pub fn queue_effect_events(&mut self, value: TritVector) {
        // All effects for this entity have been predetermined already
        // from the metadata, so we just need to queue them as events
        for effect in self.effects.iter_mut() {
            effect.queue_environment_events(value.clone());
        }
    }

    pub fn queue_event(& mut self, value: TritVector, delay: usize) {
        // Queue an event for this entity with proper delay
        let mut event: Event = Event::new(value, delay);
        self.events.push(event.clone());
        if delay == 0 && self.invoked < self.limit {
            // Can do another invocation during the current quant
            self.invoked += 1;
        } else {
            // Invocation limit exceeded, schedule for next quant
            event.quant += 1;
        }
    }

    pub fn reset_limit(&mut self) {
        self.limit = 0;
    }

    pub fn stop() {

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

