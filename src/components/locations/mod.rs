#![allow(non_snake_case)]

use crate::modules::game::Game;
use crate::modules::scene::Scene;
use dioxus::prelude::*;

pub fn LocationCards(cx: Scope) -> Element {
    let binding = use_shared_state::<Game>(cx).unwrap();
    let game = binding.read();
    let location_names = &game
        .scenes.iter().find(|i| i.id == game.current_scene)
        .expect("Could not find current scene")
        .connectors;
    let mut locations = vec![];
    for location in location_names.iter() {
      let scene = game.scenes.iter()
        .find(|i| i.id == *location)
        .expect("Could not find current scene");
      locations.push(scene.clone());
    }
    cx.render(rsx! {
        style { include_str!("./style.css") }
        h2 { "Locations" }
        for scene in locations.iter() {
            SceneCard { data: scene.clone() }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct SceneCardData {
    data: Scene,
}
pub fn SceneCard(cx: Scope<SceneCardData>) -> Element {
  cx.render(rsx! {
        div {
            class: "card scene",
            "{cx.props.data.title}",
            button {
              onclick: move |_| {
                change_scene(cx, cx.props.data.id.clone());
              },
              "Go"
            }
        }
    })
}

fn change_scene(cx: Scope<SceneCardData>, id: String) {
    let game_state = use_shared_state::<Game>(cx).unwrap();
    let console_state = use_shared_state::<String>(cx).unwrap();
    let mut game = game_state.write();
    let mut console = console_state.write();

    game.set_current_scene(id.clone()).expect(format!("Could not find scene {}", &id).as_str());
    let scene_text = game.get_current_scene_mut().description.clone();
    console.push_str(&scene_text.as_str());
}
