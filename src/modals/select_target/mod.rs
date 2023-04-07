#![allow(non_snake_case)]

use crate::modules::game::Game;
use dioxus::prelude::*;

pub fn SelectTargetModal(cx: Scope) -> Element {
    let game_state_binding = use_shared_state::<Game>(cx).unwrap();
    let game_state = game_state_binding.read();

    if game_state.pending_turn.is_none() {
        return cx.render(rsx! { div {} });
    } else {
        let turn = game_state.pending_turn.clone().unwrap();
        let targets = turn.targets.clone();
        return cx.render(rsx! {
            style { include_str!("./style.css") }
            div {
                class: "modal card",
                p {
                    format!("Who would you like to target with action \"{}\"?", turn.clone().action),
                }
                div {
                    class: "horizontal-button-list",
                    for target_id in targets.iter() {
                        TargetSelectionButton { data: target_id.clone() }
                    }
                }
            }
        });
    }
}

#[derive(PartialEq, Props)]
pub struct TargetSelectionData {
    data: String,
}
pub fn TargetSelectionButton(cx: Scope<TargetSelectionData>) -> Element {
    cx.render(rsx! {
        button {
            onclick: move |_| { select_target(cx, cx.props.data.clone()); },
            format!("{}", cx.props.data.clone()),
        }
    })
}

fn select_target(cx: Scope<TargetSelectionData>, target_id: String) {
    let game_state_binding = use_shared_state::<Game>(cx).unwrap();
    let mut game_state = game_state_binding.write();
    let mut turn = game_state.pending_turn.clone().expect(format!("No pending turn found!").as_str());
    turn.targets = vec![target_id.clone()];
    game_state.apply_turn(turn);
}
