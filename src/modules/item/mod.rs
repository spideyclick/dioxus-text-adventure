use crate::modules::action::Action;
use std::clone::Clone;

#[derive(Clone, PartialEq)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub actions: Vec<Action>,
}

impl Item {
    pub fn get_action(self, id: String) -> Action {
        self.actions.iter().find(|i| &i.id == &id)
            .expect(format!("Could not find action {} for item {}", &id, self.id).as_str())
            .clone()
    }
}
