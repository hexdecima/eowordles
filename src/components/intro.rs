use leptos::{html::*, prelude::*};

pub fn intro() -> impl IntoView {
    div().id("intro").child((
        h1().child("Eater of Wordles"),
        p().child("Guess the daily Terraria NPC"),
    ))
}
