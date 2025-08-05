use leptos::{ev, html::*, prelude::*};

use crate::{GuessContext, UIManager};

pub fn victory() -> impl IntoView {
    let GuessContext(guess_reader, _) = use_context::<GuessContext>().unwrap();
    let ui = use_context::<RwSignal<UIManager>>().unwrap();
    let answer = move || {
        guess_reader
            .get()
            .correct
            .map_or("Unknown".to_string(), |e| e.name.to_string())
    };

    let gueess_count = guess_reader.get().len();
    let counter_text = if gueess_count > 1 {
        format!("After {gueess_count} guesses.")
    } else {
        format!("First try, baby.")
    };

    div().id("victory").child((
        b().child("That's it! The answer was"),
        p().child(format!("{}!", answer())),
        button().child("Back").on(ev::click, move |_| {
            ui.update(|ui| ui.show_victory = false);
        }),
        b().id("guesses-counter")
            .child(counter_text),
    ))
}
