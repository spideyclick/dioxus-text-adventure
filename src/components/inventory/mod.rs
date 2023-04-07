#![allow(non_snake_case)]

use crate::modules::game::Game;
use crate::modules::item::Item;
use crate::modules::action::Action;
use crate::modules::turn::Turn;
use dioxus::prelude::*;

pub fn ItemCards(cx: Scope) -> Element {
    let binding = use_shared_state::<Game>(cx).unwrap();
    let game = binding.read();
    let inventory = &game
        .scenes.iter().find(|i| i.id == game.current_scene)
        .expect("Could not find current scene")
        .actors.iter().find(|i| i.id == game.current_player)
        .expect("Could not find current player")
        .inventory;

    cx.render(rsx! {
        style { include_str!("./style.css") }
        h2 { "Inventory" }
        for item in inventory.iter() {
            ItemCard { data: item.clone() }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct ItemCardData {
    data: Item,
}
pub fn ItemCard(cx: Scope<ItemCardData>) -> Element {
    cx.render(rsx! {
        div {
            class: "card inventory_item",
            "{cx.props.data.title}",
            for action in cx.props.data.actions.iter() {
                ActionCard {
                    source_item_id: cx.props.data.id.clone(),
                    action: action.clone(),
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct ActionCardData {
    source_item_id: String,
    action: Action,
}
pub fn ActionCard(cx: Scope<ActionCardData>) -> Element {
    cx.render(rsx! {
        button {
            onclick: move |_| {
                select_action(
                    cx,
                    cx.props.source_item_id.clone(),
                    cx.props.action.clone(),
                );
            },
            "{cx.props.action.title}",
        }
    })
}

fn select_action(
    cx: Scope<ActionCardData>,
    source_item_id: String,
    action: Action,
) {
    let game_state_binding = use_shared_state::<Game>(cx).unwrap();
    let mut game_state = game_state_binding.write();
    let current_scene = game_state.clone().get_current_scene();
    let available_targets = current_scene.get_targets(
        game_state.clone().get_player().id,
        action.targets
    );
    let turn = Turn{
        initiator: game_state.current_player.clone(),
        item: source_item_id,
        action: action.id,
        targets: available_targets,
    };
    game_state.apply_turn(turn);
}
