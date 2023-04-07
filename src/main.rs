#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types

mod components;
mod layout;
mod modules;
mod modals;

use dioxus::prelude::*;

use crate::components::characters::CharacterCards;
use crate::components::inventory::ItemCards;
use crate::components::locations::LocationCards;
use crate::components::text_display::Console;
use crate::layout::footer::main as Footer;
use crate::layout::header::main as Header;
use crate::layout::sidebar::main as SideBar;
use crate::modules::state::initialize_state;
use crate::modals::modal_container::ModalContainer;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let game = initialize_state();
    let starting_text = game.scenes[0].description.clone();
    use_shared_state_provider(cx, || game);
    use_shared_state_provider(cx, || starting_text);
    cx.render(rsx! {
        meta { name:"viewport", content:"width=device-width, initial-scale=1"}
        link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" }
        style { include_str!("./style.css") }
        style { include_str!("./theme.css") }
        Header {}
        SideBar {}
        div {
            id: "side1",
            CharacterCards {}
            ItemCards {}
            LocationCards {}
        }
        div {
            id: "side2",
            Console {}
        }
        Footer {}
        ModalContainer {}
    })
}
