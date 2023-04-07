// use crate::modules::item::Item;
// use crate::modules::action::Action;
// use crate::modules::character::Alignment;
// use crate::modules::game::Game;
// use rand::seq::SliceRandom;
// use std::io::{stdin, stdout, Write};

// pub fn get_user_input() -> String {
//     print!("> ");
//     let _ = stdout().flush();
//     let mut user_input = String::new();
//     stdin()
//         .read_line(&mut user_input)
//         .expect("Did not enter a correct string");
//     user_input.trim().to_string()
// }

// pub fn main_loop(game: &mut Game) {
//     let mut clear_terminal: bool = false;
//     let help_text = String::from(r#"Available commands:
// - go
// - use
// - inventory
// - status
// - ? (help)
// - exit
// "#);
//     loop {
//         if game.get_current_player().hp < 1 {
//             println!("You have died");
//             break;
//         }
//         if clear_terminal {
//             print!("\x1B[2J\x1B[1;1H");
//             clear_terminal = false;
//         }
//         let user_input = get_user_input().to_ascii_lowercase();
//         let command = user_input.split_whitespace().nth(0);
//         match command {
//             None => {
//                 println!("{}", help_text);
//                 continue;
//             },
//             Some(command) => {
//                 match command {
//                     "exit" => break,
//                     "inventory" => game.get_current_player().print_inventory(),
//                     "status" => {
//                         for character in &game.get_current_scene().characters {
//                             println!("{} HP: {}", &character.name, &character.hp);
//                         }
//                     },
//                     "go" => {
//                         println!("Where would you like to go?");
//                         for (i, connector) in game.get_current_scene().connectors.iter().enumerate() {
//                             println!("{}: {}", i, connector);
//                         }
//                         let target_location = get_user_input();
//                         game.set_current_scene(&target_location);
//                     },
//                     "use" => { select_item(game); },
//                     "?" => println!("{}", help_text),
//                     "help" => println!("{}", help_text),
//                     &_ => println!("{}", help_text),
//                 }
//             },
//         }
//     }
// }

// fn select_item(game: &mut Game) {
//     println!("Use what?");
//     for (i, item) in game.get_current_player().inventory.iter().enumerate() {
//         println!("{}: {}", i, item.id);
//     }
//     let target_item = get_user_input();
//     let binding = game.get_current_player().get_item(&target_item);
//     match binding {
//         None => {
//             println!("Could not find that action");
//             return;
//         }
//         Some(item) => {
//             // let action = item.actions.first().expect("Item has no actions!");
//             let action = select_action(&item);
//             match action {
//                 None => return,
//                 Some(action) => {
//                     let character = game.get_current_player().clone();
//                     game.apply_character_action(&character, &action);
//                     if item.consumable {
//                         let index = game.get_current_player().inventory.iter().position(|i| i.id == item.id).unwrap();
//                         game.get_current_player().inventory.remove(index);
//                     }
//                 }
//             }
//         }
//     }
//     process_npc_actions(game);
// }

// fn select_action(item: &Item) -> Option<Action> {
//     if item.actions.len() == 0 {
//         println!("Item has no available actions");
//         return None;
//     } else if item.actions.len() == 1 {
//         return Some(item.actions.first().unwrap().clone());
//     }
//     println!("What would you like to do with {}?", item.id);
//     for (i, action) in item.actions.iter().enumerate() {
//         println!("{}: {}", i, action.name);
//     }
//     let user_input = get_user_input();
//     let target_action = item.actions.iter().find(|i| i.name == user_input);
//     match target_action {
//         None => {
//             println!("Could not find that action");
//             return None;
//         },
//         Some(action) => {
//             return Some(action.clone());
//         }
//     }
// }

// fn process_npc_actions(game: &mut Game) {
//     let scene = game.get_current_scene().clone();
//     for character in scene.characters.iter() {
//         if character.id == "player" { continue }
//         if !character.alive() { continue }
//         if vec![Alignment::PassiveCreature, Alignment::NeutralCreature].iter().any(|i| i == &character.alignment) { continue }
//         let selected_item = character.inventory.choose(&mut rand::thread_rng());
//         match selected_item {
//             None => continue,
//             Some(item) => {
//                 let selected_action = item.actions.choose(&mut rand::thread_rng());
//                 match selected_action {
//                     None => continue,
//                     Some(action) => {
//                         game.apply_character_action(character, action);
//                     }
//                 }
//             },
//         }
//     }
// }
