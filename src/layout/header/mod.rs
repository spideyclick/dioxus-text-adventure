use dioxus::prelude::*;

pub fn main(cx: Scope) -> Element {
    cx.render(rsx!(
      header {
        h1 {
            "Character Sheet"
        }
      }
    ))
}
