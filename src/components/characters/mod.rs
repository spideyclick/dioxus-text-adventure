#![allow(non_snake_case)]

use crate::modules::game::Game;
use crate::modules::actor::Actor;
use dioxus::prelude::*;

pub fn CharacterCards(cx: Scope) -> Element {
    let binding = use_shared_state::<Game>(cx).unwrap();
    let game = binding.read();

    let characters = &game
        .scenes.iter().find(|i| i.id == game.current_scene)
        .expect("Could not find current scene")
        .actors;
    let player = characters
        .iter().find(|i| i.id == "player")
        .expect("Could not find current player")
        ;
    let other_characters: Vec<Actor> = characters
        .iter().filter(|i| i.id != "player").cloned().collect()
        ;
    cx.render(rsx! {
        style { include_str!("./style.css") }
        h2 { "Player" }
        CharacterCard { data: player.clone() }
        h2 { "Targets" }
        for character in other_characters.iter() {
            CharacterCard { data: character.clone() }
        }
    })
}

#[derive(PartialEq, Props)]
 pub struct CharacterCardData {
    data: Actor,
}
pub fn CharacterCard(cx: Scope<CharacterCardData>) -> Element {
    cx.render(rsx! {
        div {
            class: "card character",
            "{cx.props.data.title}",
            small {
                "HP: {cx.props.data.hp}",
            }
        }
    })
}
