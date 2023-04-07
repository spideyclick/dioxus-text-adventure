use std::clone::Clone;
use std::error::Error;
use crate::modules::actor::{Actor, Alignment};
use crate::modules::scene::Scene;
use crate::modules::history::{HistoryEntry, HistoryEntryType};
use crate::modules::action::ActionType;
use crate::modules::alert::{Alert, AlertType};
use crate::modules::turn::Turn;

#[derive(Clone)]
pub struct Game {
    pub current_player: String,
    pub current_scene: String,
    pub scenes: Vec<Scene>,
    pub pending_turn: Option<Turn>,
    pub history: Vec<HistoryEntry>,
    pub alerts: Vec<Alert>,
}

impl Game {
    pub fn get_scene(self, id: String) -> Scene {
        self.scenes.iter().find(|i| i.id == id)
        .expect(format!("Could not find scene {}", &id).as_str())
        .clone()
    }
    pub fn get_scene_mut(&mut self, id: String) -> &mut Scene {
        self.scenes.iter_mut().find(|i| i.id == id)
            .expect(format!("Could not find scene {}", &id).as_str())
    }
    pub fn get_current_scene(self) -> Scene {
        let id = self.current_scene.clone();
        self.get_scene(id)
    }
    pub fn get_current_scene_mut(&mut self) -> &mut Scene {
        let id = self.current_scene.clone();
        self.get_scene_mut(id)
    }
    pub fn get_player(self) -> Actor {
        let id = self.current_player.clone();
        self.get_current_scene().get_actor(id)
    }
    pub fn get_player_mut(&mut self) -> &mut Actor {
        let id = self.current_player.clone();
        self.get_current_scene_mut().get_actor_mut(id)
    }
    pub fn set_current_scene(&mut self, id: String) -> Result<(), Box<dyn Error>> {
        let current_scene = self.get_current_scene_mut();
        assert!(
            &current_scene.connectors.contains(&id),
            "Can not get to scene {} from here", &id
        );
        let player = self.clone().get_player();
        if !player.alive() {
            let alert = Alert {
                content: "Player is dead and cannot act.".to_string(),
                type_: AlertType::Error,
            };
            self.alerts.push(alert);
            return Ok(());
        }
        let target_scene = self.scenes.iter_mut()
            .find(|i| &i.id == &id)
            .expect(format!("Could not find target scene {}", &id).as_str())
            ;
        target_scene.actors.push(player.clone());
        let mut message = format!("{} moved to {}", self.current_player, id);
        if !&target_scene.visited {
            message.push_str(&target_scene.description.clone());
            target_scene.visited = true;
        }
        let entry = HistoryEntry{
            content: message,
            type_: HistoryEntryType::Move,
        };
        self.get_current_scene_mut().remove_actor(player.id.clone());
        self.current_scene = id;
        self.history.push(entry);
        Ok(())
    }
    pub fn apply_turn(&mut self, turn: Turn) {
        if !self.clone().get_current_scene().get_actor(turn.clone().initiator).alive() {
            let alert = Alert {
                content: format!("Actor {} is dead and cannot act.", turn.clone().initiator),
                type_: AlertType::Error,
            };
            self.alerts.push(alert);
            return;
        }
        if turn.clone().targets.is_empty() {
            let alert = Alert {
                content: format!("No valid targets available for action {}", turn.clone().action),
                type_: AlertType::Error,
            };
            self.alerts.push(alert);
            return;
        }
        if turn.clone().targets.len() > 1 {
            self.pending_turn = Some(turn);
            return;
        }
        let action = self.clone().get_current_scene().get_actor(turn.clone().initiator).get_item(turn.clone().item).get_action(turn.clone().action).clone();

        // Apply action to all targets
        let current_scene = self.get_current_scene_mut();
        for id in turn.clone().targets.iter() {
            let mut target = current_scene.get_actor_mut(id.clone());
            let mut hp = action.hp.clone();
            if action.command == ActionType::Harm {hp *= -1};
            target.hp += hp;
        }

        // Remove item if it is consumed
        if action.consumes_item {
            let initiator = current_scene.get_actor_mut(turn.clone().initiator);
            initiator.remove_item(turn.clone().item);
        }

        // Update history
        let entry = HistoryEntry{
            content: format!("Action {} applied to {}", turn.clone().action, turn.targets.join(", ")),
            type_: HistoryEntryType::Action,
        };
        self.history.push(entry);

        // Remove pending turn
        self.pending_turn = None;

        // Run AI
        if turn.clone().initiator == self.current_player {
            self.run_ai();
        }
    }
    fn run_ai(&mut self) {
        for actor in self.clone().get_current_scene().actors.iter() {
            if actor.id == "player" {continue};
            if actor.alignment != Alignment::HostileCreature {continue};
            if !actor.alive() {continue};
            let turn = Turn {
                initiator: actor.id.clone(),
                item: actor.inventory[0].id.clone(),
                action: actor.inventory[0].actions[0].id.clone(),
                targets: vec!["player".to_string()],
            };
            self.apply_turn(turn);
        }
    }
}
