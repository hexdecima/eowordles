use eowordle_lib::prelude::Enemy;
use leptos::{html::*, prelude::*};

pub fn info() -> impl IntoView {
    let yesterday = use_context::<RwSignal<Option<Enemy>>>().unwrap();
    let el = if let Some(enemy) = yesterday.get() {
        (
            p().child("Yesterday's NPC was..."),
            img().src(format!("assets/enemies/{}.gif", enemy.id)),
            b().child(enemy.name)
        ).into_any()
    } else {
        ().into_any()
    };

    div().id("info").child(
        (
            div().id("yesterday").child(el),
            div().id("text").child((
            p().child((
                "Made with 💜 by ",
                a().child("Hexdecima").href("https://mai.tilde.team/en/"),
                ". ",
                " <",
                a().child("web")
                    .href("https://github.com/hexdecima/eowordles"),
                "/",
                a().child("server")
                    .href("https://github.com/hexdecima/eowordles-server"),
                ">",
            )),
            p().child((
                "For bug reports and feedback, ",
                a().href("https://github.com/hexdecima/eowordles/issues")
                    .child("open an issue"),
                " or ",
                a().child("e-mail me").href("mailto:mai@tilde.team"),
                ".",
            )),
            p().child((
                "Inspired by ",
                a().child("Terradle").href("https://www.terradle.com/"),
                " and ",
                a().child("YGOrdle").href("https://ygordle.yugioh.app"),
                ".",
            )),
        ))),
    )
}
