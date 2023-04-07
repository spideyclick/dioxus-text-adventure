use dioxus::prelude::{dioxus_elements, rsx, use_shared_state, Element, Scope, fc_to_builder, GlobalAttributes, Props};
use crate::modules::game::Game;
use crate::modules::alert::Alert;
use crate::modules::history::HistoryEntry;

pub fn Console(cx: Scope) -> Element {
    let game_state_binding = use_shared_state::<Game>(cx).unwrap();
    let game_state = &game_state_binding.read();
    let alerts = &game_state.alerts.clone();
    let history = &game_state.history.clone();

    cx.render(rsx! {
        style { include_str!("./style.css") }
        div {
            id: "console",
            div {
                id: "alerts",
                for (index, entry) in alerts.iter().rev().enumerate() {
                    AlertCard {
                        index: index,
                        data: entry.clone()
                    }
                }
            }
            div {
                id: "history",
                for entry in history.iter().rev() {
                    HistoryEntryCard { data: entry.clone() }
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct AlertData {
    index: usize,
    data: Alert,
}
pub fn AlertCard(cx: Scope<AlertData>) -> Element {
    cx.render(rsx! {
        div {
            class: "alert alert-{cx.props.data.type_}",
            "{cx.props.data.content}",
            button {
                class: "material-icons muted",
                onclick: move |_| {
                    let game_state_binding = use_shared_state::<Game>(cx).unwrap();
                    let mut game_state = game_state_binding.write();
                    game_state.alerts.remove(cx.props.index);
                },
                "close"
            },
        }
    })
}

#[derive(PartialEq, Props)]
pub struct HistoryEntryData {
    data: HistoryEntry,
}
pub fn HistoryEntryCard(cx: Scope<HistoryEntryData>) -> Element {
    cx.render(rsx! {
        div {
            class: "card history-entry history-entry-{cx.props.data.type_}",
            "{cx.props.data.content}"
        }
    })
}
