use dioxus::prelude::*;

pub fn main(cx: Scope) -> Element {
    cx.render(rsx!(
      footer {
        About {},
      }
    ))
}

pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(
        p {
          "Connected to session: "
          b {"localhost"}
        }
    ))
}
