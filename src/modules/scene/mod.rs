use crate::modules::actor::{Actor, Alignment};
use crate::modules::action::{TargetType};
use crate::modules::item::Item;
use derivative::Derivative;

#[derive(Derivative, Clone, PartialEq)]
pub struct Scene {
    pub id: String,
    pub title: String,
    pub description: String,
    pub visited: bool,
    pub actors: Vec<Actor>,
    pub traps: Vec<Item>,
    pub items: Vec<Item>,
    pub connectors: Vec<String>,
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            id: String::from("unnamed_scene"),
            title: String::from("Unnamed Scene"),
            description: String::from(""),
            visited: false,
            actors: vec![],
            traps: vec![],
            items: vec![],
            connectors: vec![],
        }
    }
}

impl Scene {
    pub fn get_actor(self, id: String) -> Actor {
        self.actors.iter().find(|i| &i.id == &id)
            .expect(format!("Could not find actor with id {} in scene {}", &id, self.id).as_str())
            .clone()
        }
    pub fn get_actor_mut(&mut self, id: String) -> &mut Actor {
        self.actors.iter_mut().find(|i| i.id == id)
            .expect(format!("Could not find actor with id {} in scene {}", &id, self.id).as_str())
    }
    pub fn remove_actor(&mut self, id: String) {
        let index = self
            .actors.iter()
            .position(|i| &i.id == &id)
            .expect(format!("Could not find actor {} in scene {}", &id, self.id).as_str())
            .clone()
            ;
        self.actors.remove(index);
    }
    pub fn get_targets(&self, initiator_id: String, target_type: TargetType) -> Vec<String> {
        let initiator = self.clone().get_actor(initiator_id);
        let mut output = vec![];
        match target_type {
            TargetType::YourSelf => {
                output.push(initiator.id.clone());
            },
            TargetType::Hostile => {
                for actor in self.actors.iter() {
                    if !actor.alive() { continue }
                    if actor.id == initiator.id { continue }
                    if actor.alignment == initiator.alignment { continue }
                    if actor.alignment == Alignment::PassiveCreature { continue }
                    if actor.alignment == Alignment::NeutralCreature { continue }
                    output.push(actor.id.clone());
                }
            }
            TargetType::Anyone => {
                for character in self.actors.iter() {
                    if !character.alive() { continue }
                    output.push(character.id.clone());
                }
            }
            TargetType::AnyoneElse => {
                for character in self.actors.iter() {
                    if !character.alive() { continue }
                    if character.id == initiator.id { continue }
                    output.push(character.id.clone());
                }
            }
            TargetType::Friendly => {
                for character in self.actors.iter() {
                    if !character.alive() { continue }
                    if character.alignment != initiator.alignment { continue }
                    output.push(character.id.clone());
                }
            }
        }
        output
    }
}
