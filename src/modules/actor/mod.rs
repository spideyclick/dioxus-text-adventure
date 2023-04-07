use std::error::Error;
use std::clone::Clone;
use derivative::Derivative;
use crate::modules::item::Item;

#[derive(Clone, PartialEq)]
pub enum Alignment {
    PlayerCharacter,
    NonPlayerCharacter,
    PassiveCreature,
    NeutralCreature,
    HostileCreature,
    Monster,
}

#[derive(Derivative, Clone, PartialEq)]
pub struct Actor {
    pub id: String,
    pub title: String,
    pub hp: i32,
    pub inventory: Vec<Item>,
    pub alignment: Alignment,
}

impl Default for Actor {
    fn default() -> Actor {
        Actor {
            id: String::from("missing_id"),
            title: String::from("Unnamed Monster"),
            hp: 10,
            inventory: vec![],
            alignment: Alignment::Monster,
        }
    }
}

impl Actor {
    pub fn alive(&self) -> bool {
        self.hp > 0
    }
    pub fn remove_item(&mut self, id: String) {
        let index = self
            .inventory.iter().position(|i| &i.id == &id)
            .expect(format!("Could not find item {} to remove from inventory of {}", &id, self.id).as_str())
            ;
        self.inventory.remove(index);
    }
    pub fn get_item(self, id: String) -> Item {
        self.inventory.iter().find(|i| &i.id == &id)
            .expect(format!("Could not find item {} in inventory of {}", &id, self.id).as_str())
            .clone()
    }
}
