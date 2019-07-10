use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::dispatcher::effect::Effect;
use crate::dispatcher::environment::Environment;
use crate::dispatcher::event::Event;
use crate::dispatcher::entity::Entity;
use crate::qupla::statement::typestmt::TypeStmt;

#[derive(Clone)]
pub struct Supervisor<'a> {
    environments: HashMap<String, Environment<'a>>,
    events: Vec<Event<'a>>,
    entities: Vec<Entity<'a>>,
    stop_running: bool,
    pub current_quant: usize
}

impl<'a> Supervisor<'a> {
    pub fn new() -> Supervisor<'a> {
        Supervisor {
            environments: HashMap::new(),
            events: vec![],
            entities: vec![],
            stop_running: false,
            current_quant: 0
        }
    }

    pub fn cancel(&mut self) {
        self.stop_running = true;
    } 

    fn finished(&mut self) {
        self.environments.clear();

        for entity in self.entities.iter_mut() {
            entity.stop();
        }

        self.entities.clear();
    }

    pub fn get_environment(&mut self, name: &str, type_info: Option<TypeStmt>) -> &mut Environment<'a> {
        self.environments.entry(name.to_owned())
        .and_modify(|e| { if e.type_info.is_none() && !type_info.is_none() { e.type_info = type_info } })
        .or_insert_with(|| Environment::new(name.to_owned(), type_info));
        self.environments.get_mut(name).unwrap()
    }

    pub fn list_environments(&self) -> Vec<String> {
        self.environments.keys().cloned().collect()
    }

    pub fn run(&'a mut self) {
        while !self.stop_running {
            self.run_quants();
            // self.sleep(200);
        }
        self.finished();
    }

    fn run_quants(&'a mut self) {
        // keep running as long as there are still events in the queue
        while self.dispatch_current_quant_events() {
          // reset all invocation limits for the next quant
          for (_, environment) in self.environments.iter_mut() {
            environment.reset_entity_limits();
          }
        }
    }

    pub fn dispatch_current_quant_events(&'a mut self) -> bool {

        // if self.events.len() == 0 {
        //     return false
        // }

        // self.events.iter_mut().map(|e| { 
        //     if e.quant <= self.current_quant {
        //         e.dispatch();
        //         self.events.remove_item(e); // Nighly only. How to remove item in stabile channel
        //     }
        // });

        for i in 0..self.events.len() {
            if self.events.get(i).unwrap().quant <= self.current_quant {
                let event: &'a mut Event = self.events.get_mut(i).unwrap();
                event.dispatch();
                self.events.remove(i);
                continue;
            }
        }

        // self.current_quant += 1;
        true
    }

}