use dioxus::prelude::*;

pub fn main(cx: Scope) -> Element {
    cx.render(rsx!(
      style { include_str!("./style.css") }
      nav {
        ul {
          li { a { href: "#", "Campaigns", } },
          li { a { href: "#", "Characters", } },
          li { a { href: "#", "Party", } },
          li { a { href: "#", "Journal", } },
        }
      }
    ))
}
