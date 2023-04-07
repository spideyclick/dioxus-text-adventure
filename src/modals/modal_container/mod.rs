#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::modules::game::Game;
use crate::modals::select_target::SelectTargetModal;

pub fn ModalContainer(cx: Scope) -> Element {
    let game_state_binding = use_shared_state::<Game>(cx).unwrap();
    let game_state = &game_state_binding.read();
    let modal_visibility = game_state.pending_turn.is_some();

    let mut hidden_class = "hidden";
    if modal_visibility {
        hidden_class = "";
    }

    cx.render(rsx! {
        style { include_str!("./style.css") }
        div {
            id: "modal-backdrop",
            class: hidden_class,
            SelectTargetModal {}
        }
    })
}
