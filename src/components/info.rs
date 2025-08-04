use leptos::{html::*, prelude::*};

pub fn info() -> impl IntoView {
    div().id("info").child((
        p().child((
            "Made with 💜 by ",
            a().child("Hexdecima").href("https://github.com/hexdecima"),
            " // ",
            "Source (",
            a().child("web")
                .href("https://github.com/hexdecima/eowordles"),
            "/",
            a().child("server")
                .href("https://github.com/hexdecima/eowordles-server"),
            ")"
        )),
        p().child((
            a().href("https://github.com/hexdecima/eowordles/issues")
                .child("Open an issue"),
            " // ",
            a().child("Buy me a coffee")
                .href("https://patreon.com/Hexdecima"),
        )),
        p().child((
            "Inspired by ",
            a().child("Terradle").href("https://www.terradle.com/"),
        )),
    ))
}
