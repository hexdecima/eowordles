use chrono::{NaiveTime, Utc};
use eowordle_lib::prelude::Enemy;
use leptos::{html::*, prelude::*};

pub fn info() -> impl IntoView {
    let yesterday = use_context::<RwSignal<Option<Enemy>>>().unwrap();

    let now = Utc::now().time();
    let midnight = NaiveTime::from_hms_opt(23, 59, 0).unwrap();
    let time_diff = {
        let diff = midnight - now;
        let hours = diff.num_hours();
        let mins = diff.num_minutes() - (hours * 60);

        format!("{}h{}m", hours, mins)
    };

    let el = if let Some(enemy) = yesterday.get() {
        (
            p().child("Yesterday's NPC was..."),
            img().src(format!("assets/enemies/{}.gif", enemy.id)),
            b().child(enemy.name),
            p().child((
                "A new creature spawns in ",
                b().child(format!("{time_diff}...")),
            )),
        )
            .into_any()
    } else {
        ().into_any()
    };

    div().id("info").child((
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
                "Last updated on ",
                b().child("16/10/2025"),
                " (",
                a().href("https://github.com/hexdecima/eowordles/issues")
                    .child("Open an issue"),
                ").",
            )),
            p().child((
                "Inspired by ",
                a().child("Terradle").href("https://www.terradle.com/"),
                " and ",
                a().child("YGOrdle").href("https://ygordle.yugioh.app"),
                ".",
            )),
        )),
    ))
}
