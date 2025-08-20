use eowordle_lib::{enemies::list_enemies, prelude::Enemy};
use leptos::{attr::Value, ev, html::*, prelude::*, task, IntoView};

use crate::{api, data::Guess, GuessContext, UIManager};

pub fn searcher() -> impl IntoView {
    let enemies = list_enemies();

    let query = RwSignal::new(String::new());
    let filtered = RwSignal::new(Vec::<Enemy>::new());

    Effect::new(move |_| {
        if query.get().len() == 0 {
            filtered.set(vec![]);
        } else {
            filtered.set(
                enemies
                .iter()
                .filter(|e| e.name.to_lowercase().contains(&query.get()))
                .cloned()
                .collect(),
            );
        }
    });

    div().class("searcher").child((
        input()
            .r#type("text")
            .id("searcher-input")
            .value(query)
            .bind(Value, query)
            .placeholder("Type here to search..."),
        div().id("searcher-picker").child(move || {
            filtered
                .get()
                .iter()
                .cloned()
                .map(picker_item)
                .collect_view()
        }),
    ))
}



pub fn picker_item(enemy: Enemy) -> impl IntoView {
    let text = format!("{} / {} life / {} defence / {}", enemy.name, enemy.life, enemy.defence, enemy.rarity);

    let GuessContext(guess_reader, guess_writer) = use_context::<GuessContext>().unwrap();
    let ui = use_context::<RwSignal<UIManager>>().unwrap();

    button()
        .class("picker-item")
        .child((
                img().src(format!("assets/enemies/{}.gif", enemy.id)).aria_label(enemy.name.clone()),
                text))
        .on(ev::click, move |_| {
            let enemy = enemy.clone();

            task::spawn_local(async move {
                let guesses = guess_reader.get_untracked();
                if guesses.has(&enemy) {
                    return;
                };

                let Some(diff) = api::get_daily_diff(enemy.id).await else {
                    return;
                };

                let guess = Guess {
                    enemy: enemy.clone(),
                    diff: diff.clone(),
                };
                guess_writer.update(|g| g.add(guess));

                if diff.is_same() {
                    guess_writer.update(|g| g.correct = Some(enemy.clone()));
                    ui.update(|ui| ui.show_victory = true);
                }
            });

        })
}
