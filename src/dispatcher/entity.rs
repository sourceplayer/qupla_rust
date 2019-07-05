use crate::helper::tritvector::{ TritVector };
use crate::dispatcher::effect::Effect;
use crate::dispatcher::environment::Environment;

pub struct Entity {
    entities: Vec<Entity>,
    effects: Vec<Effect>,
    id: String,
    invoked: usize,
    limit: usize
}

impl From<usize> for Entity {
    fn from(limit: usize) -> Entity {
        let mut entity = Entity {
            limit,
            id: String::new(),
            invoked: 0,
            entities: vec![],
            effects: vec![]
        };
        entity.entities.push(entity);
        entity
    }
}

impl Entity {
    pub fn start(&self, entity_name: String) {
        let entity = Entity::from(0);
    }

    pub fn affect(&self, env: Environment, delay: usize) {

        let effect = Effect::new(env, delay);
        self.effects.push(effect);
    }

    pub fn join(&self, env: Environment) {
        env.join(&self);
    }

    pub fn process_effect(&self, effect: TritVector) {

        // Have entity process the effect
        let result: TritVector = self.on_effect(effect);
        if result == None {
            // Propagation stops if null is returned (no data flow)
            return
        }

        // queue any effects that entity triggered
        self.queue_effect_events(result);
    }

    pub fn queue_effect_events(&self, value: TritVector) {
        // All effects for this entity have been predetermined already
        // from the metadata, so we just need to queue them as events
        for effect in self.effects {
            effect.queue_environment_events(value);
        }
    }

    pub fn queue_event(&self, value: TritVector, delay: usize) {
        // Queue an event for this entity with proper delay
        let event: Event = Event::new(&self, value, delay);
        if delay == 0 && self.invoked < self.limit {
            // Can do another invocation during the current quant
            self.invoked += 1;
        } else {
            // Invocation limit exceeded, schedule for next quant
            event.quant += 1;
        }
    }

    pub fn reset_limit(&self) {
        self.limit = 0;
    }

    pub fn stop() {

    }

}

trait OnEffect {
    fn on_effect(&self, effect: TritVector) -> TritVector;
}

impl OnEffect for Entity {
    fn on_effect(&self, effect: TritVector) -> TritVector
    {
        TritVector::from(0);
    }
}

