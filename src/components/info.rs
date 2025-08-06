use leptos::{html::*, prelude::*};

pub fn info() -> impl IntoView {
    div().id("info").child((
        p().child((
            "Made with 💜 by ",
            a().child("Hexdecima").href("https://mai.tilde.team/en/"),
            "."
        )),
        p().child((
            "This is an open source project under the ",
            a().child("Unlicense").href("https://unlicense.org"),
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
            "."
        )),
    ))
}
