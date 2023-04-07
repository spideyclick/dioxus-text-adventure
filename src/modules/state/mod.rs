use crate::modules::action::{Action, ActionType, TargetType};
use crate::modules::actor::{Actor, Alignment};
use crate::modules::game::Game;
use crate::modules::item::Item;
use crate::modules::scene::Scene;

pub fn initialize_state() -> Game {
    let health_potion = Item {
        id: "potion_health".to_owned(),
        title: "Health Potion".to_owned(),
        actions: vec![Action {
            id: "drink".to_owned(),
            title: "Drink".to_owned(),
            command: ActionType::Heal,
            hp: 5,
            targets: TargetType::YourSelf,
            consumes_item: true,
        }],
    };
    let poison_potion: Item = Item {
        id: "potion_poison".to_owned(),
        title: "Vial of Poison".to_owned(),
        actions: vec![Action {
            id: "drink".to_owned(),
            title: "Drink".to_owned(),
            command: ActionType::Harm,
            hp: 15,
            targets: TargetType::YourSelf,
            consumes_item: true,
        }],
    };
    let claws: Item = Item {
        id: "claws".to_owned(),
        title: "Claws".to_owned(),
        actions: vec![Action {
            id: "slash".to_owned(),
            title: "Slash".to_owned(),
            command: ActionType::Harm,
            hp: 3,
            targets: TargetType::Hostile,
            consumes_item: false,
        }],
    };
    let knife: Item = Item {
        id: "knife".to_owned(),
        title: "Small, Dull Knife".to_owned(),
        actions: vec![Action {
            id: "slash".to_owned(),
            title: "Slash".to_owned(),
            command: ActionType::Harm,
            hp: 3,
            targets: TargetType::Hostile,
            consumes_item: false,
        }],
    };
    let shield: Item = Item {
        id: "shield".to_owned(),
        title: "Iron Shield".to_owned(),
        actions: vec![
            Action {
                id: "defend".to_owned(),
                title: "Defend".to_owned(),
                command: ActionType::Heal,
                hp: 1,
                targets: TargetType::YourSelf,
                consumes_item: false,
            },
            Action {
                id: "push".to_owned(),
                title: "push".to_owned(),
                command: ActionType::Harm,
                hp: 1,
                targets: TargetType::Hostile,
                consumes_item: false,
            },
        ],
    };
    let mut game: Game = Game {
        current_player: "player".to_owned(),
        current_scene: "overgrown_cavern".to_owned(),
        pending_turn: None,
        history: vec![],
        alerts: vec![],
        scenes: vec![
            Scene {
                id: "starting_cave".to_owned(),
                title: "Starting Room".to_owned(),
                connectors: vec![
                    String::from("overgrown_cavern"),
                    String::from("sewer_ruins"),
                ],
                description: String::from(
                    r#"

You wake up in a dimly lit cave.

You are carrying a satchel that seems to contain some items."#,
                ),
                ..Default::default()
            },
            Scene {
                id: "overgrown_cavern".to_owned(),
                title: "Overgrown Cavern".to_owned(),
                actors: vec![
                    Actor {
                        id: "player".to_owned(),
                        hp: 10,
                        title: "Player".to_owned(),
                        inventory: vec![
                            knife.clone(),
                            shield.clone(),
                            health_potion.clone(),
                            health_potion.clone(),
                            poison_potion.clone(),
                        ],
                        alignment: Alignment::PlayerCharacter,
                    },
                ],
                connectors: vec![
                    "starting_cave".to_owned(),
                ],
                description: String::from(
                    r#"

A thick, musty smell permeates this room. Roots from the forest above reach down into this room, covering the ceiling and walls with their searching vines. The dirt floor beneath you turns to mud towards the center of the room. A pale shaft of light streams through an opening in the roof at the far end of the room."#,
                ),
                ..Default::default()
            },
            Scene {
                id: "sewer_ruins".to_owned(),
                title: "Sewer Ruins".to_owned(),
                connectors: vec![
                    "starting_cave".to_owned(),
                    "overgrown_cavern".to_owned(),
                ],
                actors: vec![
                    Actor {
                        id: "harmless_rat".to_owned(),
                        title: "Harmless Rat".to_owned(),
                        hp: 4,
                        inventory: vec![claws.clone()],
                        alignment: Alignment::NeutralCreature,
                    },
                    Actor {
                        id: "black_rat".to_owned(),
                        title: "Black Rat".to_owned(),
                        hp: 6,
                        inventory: vec![claws.clone()],
                        alignment: Alignment::HostileCreature,
                    },
                    Actor {
                        id: "brown_rat".to_owned(),
                        title: "Brown Rat".to_owned(),
                        hp: 6,
                        inventory: vec![claws.clone()],
                        alignment: Alignment::HostileCreature,
                    },
                ],
                description: String::from(
                    r#"

A rancid smell fills your lungs as you enter what appears to be the crumbling remains of an old sewer system. Ancient pillars from a bygone era hold the roof about 30 feet above your head, where faint sunlight streams through a grate. In one corner, you see part of the floor has crumbled and caved in to another room below.

You hear a few rats scurrying in a corner."#,
                ),
                ..Default::default()
            },
        ]
    };
    game.set_current_scene("starting_cave".to_owned()).unwrap();
    game
}
